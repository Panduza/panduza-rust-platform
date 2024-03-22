use std::env;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use dirs;

use futures::future::BoxFuture;
use futures::Future;
use futures::FutureExt;
use serde_json::json;
use tokio::signal;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use crate::device;
use crate::connection;

pub mod error;
mod services;

use services::{Services, AmServices};

use crate::platform_error;

pub type Error = error::Error;

#[macro_export]
macro_rules! platform_error {
    ($msg:expr, $parent:expr) => {
        Err(crate::platform::error::Error::new(file!(), line!(), $msg.to_string(), $parent))
    };
}


#[derive(Clone)]
pub struct TaskPoolLoader {

    task_pool_tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = ()> + Send>>>

}

impl TaskPoolLoader {

    pub fn new(tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = ()> + Send>>>) -> TaskPoolLoader {
        return TaskPoolLoader {
            task_pool_tx: tx
        }
    }

    pub fn load(&mut self, future: Pin<Box<dyn Future<Output = ()> + Send>>) -> Result<(), error::Error>{
        let r = self.task_pool_tx.try_send(future);
        match r {
            Ok(_) => {
                return Ok(());
            },
            Err(e) => {
                return platform_error!("Failed to send task to task pool", None);
            }
        }
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Platform main object
/// 
pub struct Platform
{

    /// Task pool to manage all tasks
    task_pool: JoinSet<()>,

    task_pool_rx: Arc<Mutex< tokio::sync::mpsc::Receiver<Pin<Box<dyn Future<Output = ()> + Send>>> >>,

    task_loader: TaskPoolLoader,

    /// Services
    services: AmServices,

    /// Device manager
    devices: device::AmManager,

    /// Connection manager
    connections: connection::AmManager
}

impl Platform {

    /// Create a new instance of the Platform
    /// 
    pub fn new(name: &str) -> Platform {

        // Create the channel
        let (tx, rx) =
            tokio::sync::mpsc::channel::<BoxFuture<'static, ()>>(5);
        
        let tl = TaskPoolLoader::new(tx);

        return Platform {
            task_pool: JoinSet::new(),
            task_pool_rx: Arc::new(Mutex::new(rx)),
            task_loader: tl.clone(),
            services: Services::new(tl.clone()),
            devices: device::Manager::new(tl.clone()),
            connections: connection::Manager::new(tl.clone(), name)
        }
    }

    /// Main platform run loop
    /// 
    pub async fn work(&mut self) {
        // Info log
        tracing::info!("Platform Version ...");

        // Start service task
        let s = self.services.clone();
        let d = self.devices.clone();
        let c = self.connections.clone();

        self.task_loader.load(async move {
                Platform::services_task(s, d, c).await;
            }.boxed()
        ).unwrap();
        

        let p = self.task_pool_rx.clone();
        let mut task_pool_rx = p.lock().await;

        let task_option = task_pool_rx.recv().await;
        match task_option {
            Some(task) => {
                tracing::warn!("new task !!!");
                self.task_pool.spawn(task);
            },
            None => {
                tracing::warn!("???!!!!!");
            }
        }


        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    tracing::warn!("End by user ctrl-c");
                    break;
                },
                task = task_pool_rx.recv() => {
                    tracing::warn!("new task !!!");
                    self.task_pool.spawn(task.unwrap());
                },
                _ = self.end_of_all_tasks() => {
                    tracing::warn!("End by all tasks completed");
                    break;
                }
            }            
        }

    }

    /// Wait for all tasks to complete
    /// 
    async fn end_of_all_tasks( &mut self) {
        while let Some(result) = self.task_pool.join_next().await {
            tracing::info!("End task with result {:?}", result);
        }
    }

    /// Services task
    /// 
    async fn services_task(services: AmServices, devices: device::AmManager, connections: connection::AmManager) {
        let requests_change_notifier = services.lock().await.get_requests_change_notifier();
        loop {
            // Wait for an event
            requests_change_notifier.notified().await;

            tracing::info!("Services task notified");
            {
                if services.lock().await.has_pending_requests() {

                    // --- BOOT ---
                    if services.lock().await.booting_requested() {
                        // log
                        tracing::info!("Booting...");

                        // Load the tree file
                        let r = Platform::load_tree_file(services.clone()).await;
                        match r {
                            Ok(_) => {
                                tracing::info!("Tree loaded");
                            },
                            Err(e) => {
                                tracing::error!("Failed to load tree: {}", e);
                            }
                        }

                        // Sart minimal connection and devices
                        Platform::boot_minimal_services(services.clone(), devices.clone(), connections.clone()).await;

                        // log
                        tracing::info!("Boot Success!");
                    }
                    
                    // --- RELOAD ---
                    if services.lock().await.reload_tree_requested() {
                        tracing::info!("Reload");

                        Platform::reload_tree(services.clone(), devices.clone(), connections.clone()).await;
                        
                    }

                }
            }
        }
    }

    /// Load the tree file from system into service data
    ///
    async fn load_tree_file(services: AmServices) -> Result<(), error::Error> {

        // Get the tree file path
        let tree_file_path = PathBuf::from(dirs::home_dir().unwrap()).join("panduza").join("tree.json");
        match env::consts::OS {
            "linux" => {
                // println!("We are running linux!");
            }
            "windows" => {

            }
            _ => {
                tracing::error!("Unsupported system!");
            }
        }

        // Try to read the file content
        let file_content = tokio::fs::read_to_string(&tree_file_path).await;
        match file_content {
            Ok(content) => {
                return Platform::load_tree_string(services.clone(), &content).await;
            },
            Err(e) => {
                return platform_error!(
                    format!("Failed to read {:?} file content: {}", tree_file_path, e), None)
            }
        }
    }

    /// Load a tree string into service data
    ///
    async fn load_tree_string(services: AmServices, content: &String) -> Result<(), error::Error> {
        // Parse the JSON content
        let json_content = serde_json::from_str::<serde_json::Value>(&content);
        match json_content {
            Ok(json) => {
                tracing::info!("JSON content: {:?}", serde_json::to_string_pretty(&json));

                services.lock().await.set_tree_content(json);

                return Ok(());
            },
            Err(e) => {
                return platform_error!(
                    format!("Failed to parse JSON content: {}", e), None)
            }
        }
    }

    /// Boot default connection and platform device
    ///
    async fn boot_minimal_services(services: AmServices, devices: device::AmManager, connections: connection::AmManager) {

        // Lock managers to create connection and device
        let mut d = devices.lock().await;
        let mut c = connections.lock().await;

        // Server hostname
        let hostname = hostname::get().unwrap().to_string_lossy().to_string();

        // Create default connection
        c.create_connection("default", "localhost", 1883).await;

        // Create server device
        match d.create_device( &json!({
                "name": hostname,
                "ref": "panduza.server"
            })).await {
            Ok(_) => {
                tracing::info!("Device created");
            },
            Err(e) => {
                tracing::error!("Failed to create device: {}", e);
            }
        }

        // attach
        let server_device = d.get_device(hostname).unwrap();
        let default_connection = c.get_connection(&"default".to_string());
        server_device.attach_connection(default_connection).await;

        // Start connection
        c.start_connection("default").await;

        // Mount devices
        d.mount_devices().await;
    }

    /// Reload tree inside platform configuration
    /// 
    async fn reload_tree(services: AmServices, devices: device::AmManager, connections: connection::AmManager) {


    }



}

