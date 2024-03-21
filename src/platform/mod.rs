use std::env;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use dirs;
use futures::channel::mpsc;
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

                    if services.lock().await.booting_requested() {
                        tracing::info!("Booting requested");

                        Platform::load_tree_file(services.clone()).await;
                    }
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
    async fn load_tree_file(services: AmServices) {

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

        // Read the file content
        let file_content = tokio::fs::read_to_string(&tree_file_path).await;
        if let Ok(content) = file_content {
            // Parse the JSON content
            let json_content = serde_json::from_str::<serde_json::Value>(&content);
            if let Ok(json) = json_content {
                
                tracing::info!("JSON content: {:?}", json);

                services.lock().await.set_tree_content(json);

            } else {
                tracing::error!("Failed to parse JSON content");
            }
        } else {
            tracing::error!("Failed to read file content");
        }


    }

    /// Reload tree inside platform configuration
    /// 
    async fn reload_tree(services: AmServices, devices: device::AmManager, connections: connection::AmManager) {
    
        let mut c = connections.lock().await;
        
        c.create_connection("default", "localhost", 1883).await;

        let mut d = devices.lock().await;
        match d.create_device( &json!({
            "name": "host",
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
        let devvv = d.get_device("host".to_string()).unwrap();
        devvv.attach_connection(c.get_connection(&"default".to_string())).await;


        c.start_connection("default").await;

        d.mount_devices().await;

    }



}

