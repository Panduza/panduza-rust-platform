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


pub struct TaskPoolLoader {

    task_pool_tx: mpsc::Sender<Pin<Box<dyn Future<Output = ()> + Send>>>

}

impl TaskPoolLoader {

    pub fn new(tx: mpsc::Sender<Pin<Box<dyn Future<Output = ()> + Send>>>) -> TaskPoolLoader {
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

    task_pool_rx: mpsc::Receiver<Pin<Box<dyn Future<Output = ()> + Send>>>,

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
        let (tx, mut rx) =
            mpsc::channel::<BoxFuture<'static, ()>>(5);
        
        return Platform {
            task_pool: JoinSet::new(),
            services: Services::new(),
            devices: device::Manager::new(),
            connections: connection::Manager::new(name)
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


        // let a: Pin<Box<dyn Future<Output = ()> + Send>> = async move {
        //     Platform::services_task(s, d, c).await;
        // }.boxed();
        // tx.send(a).await;


        // let pppp = rx.recv().await.unwrap();

        tracing::warn!("pok");

        self.task_pool.spawn(pppp);

        // Wait for either a signal or all tasks to complete
        tokio::select! {
            _ = signal::ctrl_c() => {
                tracing::warn!("End by user ctrl-c");
            },
            _ = self.end_of_all_tasks() => {
                tracing::warn!("End by all tasks completed");
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

        // c.start_connection("default", &mut self.task_pool).await;

        // self.attach_device_to_connection("host", "default").await;

        // self.devices.mount_devices(&mut self.task_pool).await;

    }

    /// Attach a device to a connection
    /// 
    async fn attach_device_to_connection(&mut self, device: &str, connection: &str) {

        // // get device
        // let devvv = self.devices.get_device(&device.to_string()).unwrap();

        // devvv.attach_connection(self.connections.get_connection(connection)).await;


        // get connection
        // attach device to connection
        
        // self.devices.attach_connection(device, connection);

    }




}

