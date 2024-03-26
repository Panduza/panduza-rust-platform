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

/// Platform error type
///
pub type PlatformError = error::PlatformError;

/// Platform result type
///
pub type PlatformTaskResult = Result<(), PlatformError>;



#[macro_export]
macro_rules! platform_error {
    ($msg:expr, $parent:expr) => {
        Err(crate::platform::error::PlatformError::new(file!(), line!(), $msg.to_string(), $parent))
    };
}




#[derive(Clone)]
pub struct TaskPoolLoader {

    task_pool_tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>>

}

impl TaskPoolLoader {

    pub fn new(tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>>) -> TaskPoolLoader {
        return TaskPoolLoader {
            task_pool_tx: tx
        }
    }

    pub fn load(&mut self, future: Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>) -> Result<(), error::PlatformError>{
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
    task_pool: JoinSet<PlatformTaskResult>,

    /// Task loader
    task_loader: TaskPoolLoader,

    /// Task pool receiver
    task_pool_rx: Arc<Mutex< tokio::sync::mpsc::Receiver<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>> >>,

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
            tokio::sync::mpsc::channel::<BoxFuture<'static, PlatformTaskResult>>(5);
        
        let tl = TaskPoolLoader::new(tx);

        return Platform {
            task_pool: JoinSet::new(),
            task_loader: tl.clone(),
            task_pool_rx: Arc::new(Mutex::new(rx)),
            services: Services::new(tl.clone()),
            devices: device::Manager::new(tl.clone()),
            connections: connection::Manager::new(tl.clone(), name)
        }
    }

    /// Main platform run loop
    /// 
    pub async fn work(&mut self) {
        // Info log
        tracing::info!(class="Platform", "Platform Version ...");

        // Start the main service task directly
        // it acts as a idle task for the platform to avoid the platform to stop if no other task
        let s = self.services.clone();
        let d = self.devices.clone();
        let c = self.connections.clone();
        self.task_pool.spawn(async move {
            Platform::services_task(s, d, c).await
        });

        // Main loop
        // Run forever and wait for:
        // - ctrl-c: to stop the platform after the user request it
        // - a new task to start in the task pool
        // - all tasks to complete
        let task_pool_rx_clone = self.task_pool_rx.clone();
        let mut task_pool_rx = task_pool_rx_clone.lock().await;
        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    tracing::warn!(class="Platform", "User ctrl-c, abort requested");
                    self.task_pool.abort_all();
                },
                task = task_pool_rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.task_pool.spawn(task.unwrap());
                    tracing::debug!(class="Platform", "New task created ! [{:?}]", ah );
                },
                _ = self.end_of_all_tasks() => {
                    tracing::warn!(class="Platform", "All tasks completed, stop the platform");
                    break;
                }
            }
        }

    }

    /// Wait for all tasks to complete
    /// 
    async fn end_of_all_tasks( &mut self) {
        while let Some(join_result) = self.task_pool.join_next().await {

            match join_result {

                Ok(task_result) => {
                    match task_result {
                        Ok(_) => {
                            tracing::warn!("Task completed");
                        },
                        Err(e) => {
                            tracing::error!("Task failed: {}", e);
                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Join failed: {}", e);
                }
            }

        }
    }

    /// Services task
    /// 
    async fn services_task(services: AmServices, devices: device::AmManager, connections: connection::AmManager) -> PlatformTaskResult {
        let requests_change_notifier = services.lock().await.get_requests_change_notifier();
        loop {
            // Wait for an event
            requests_change_notifier.notified().await;
            tracing::trace!(class="Platform", "Services task notified");
            {
                if services.lock().await.has_pending_requests() {

                    // --------------------------------------------------------
                    // --- BOOT ---
                    if services.lock().await.booting_requested() {
                        // log
                        tracing::info!(class="Platform", "Booting...");

                        // Load the tree file
                        if let Err(e) = Platform::load_tree_file(services.clone()).await
                        {
                            tracing::warn!(class="Platform", "Failed to load tree: {}", e);
                            tracing::warn!(class="Platform", "Continue with default configuration");
                        }

                        // Sart minimal connection and devices
                        Platform::boot_minimal_services(services.clone(), devices.clone(), connections.clone()).await;

                        // log
                        tracing::info!(class="Platform", "Boot Success!");
                    }
                    
                    // --------------------------------------------------------
                    // --- RELOAD ---
                    if services.lock().await.reload_tree_requested() {
                        tracing::info!(class="Platform", "Reloading Configuration Tree...");

                        Platform::reload_tree(services.clone(), devices.clone(), connections.clone()).await;
                        
                        tracing::info!(class="Platform", "Reloading Success!");
                    }

                }
            }
        }
    }

    /// Load the tree file from system into service data
    ///
    async fn load_tree_file(services: AmServices) -> Result<(), error::PlatformError> {

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
    async fn load_tree_string(services: AmServices, content: &String) -> Result<(), error::PlatformError> {
        // Parse the JSON content
        let json_content = serde_json::from_str::<serde_json::Value>(&content);
        match json_content {
            Ok(json) => {
                // log
                tracing::info!(class="Platform", " - Tree Json content -\n{}", serde_json::to_string_pretty(&json).unwrap());

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
        if let Err(e) = d.create_device(&json!({
            "name": hostname,
            "ref": "panduza.server"
        })).await {
            tracing::error!(class="Platform", "Failed to create device: {}", e);
        }

        // attach
        let server_device = d.get_device(hostname).unwrap();
        let default_connection = c.get_connection(&"default".to_string());
        server_device.set_default_connection(default_connection.clone()).await;
        server_device.set_operational_connection(default_connection.clone()).await;

        // Start connection
        c.start_connection("default").await;

        // Mount devices
        d.start_devices().await;
    }

    /// Reload tree inside platform configuration
    /// 
    async fn reload_tree(services: AmServices, devices_manager: device::AmManager, connections_manager: connection::AmManager) {

        let services_lock = services.lock().await;

        let tree_ref = services_lock.get_tree_content();


        let devices_definitions= tree_ref.get("devices");
        match devices_definitions {
            Some(devices) => {
                // Iterate over the devices
                if let Some(devices) = devices.as_array() {
                    for device_definition in devices {


                        let result = devices_manager.lock().await.create_device(device_definition).await;
                        match result {
                            Err(e) => {
                                tracing::error!("Failed to create device {}: {}", "ppp", e);
                            },
                            Ok(new_device_name) => {
                                let mut d = devices_manager.lock().await;
                                let mut c = connections_manager.lock().await;
                        
                                let server_device = d.get_device(new_device_name).unwrap();
                                let default_connection = c.get_connection(&"default".to_string());
                                server_device.set_default_connection(default_connection.clone()).await;
                                server_device.set_operational_connection(default_connection.clone()).await;
                            }
                        }

                    }
                }
            },
            None => {
                tracing::warn!("No devices found in the tree");
            }
        }


        let mut d = devices_manager.lock().await;
        d.start_devices().await;



    }



}

