use std::env;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use dirs;
use futures::future::BoxFuture;
use futures::Future;
use serde_json::json;
use tokio::signal;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::net::UdpSocket;
use crate::device;
use crate::connection;

mod connection_info;
pub mod error;
pub mod services;
mod task_pool_loader;

use services::{Services, AmServices};

use crate::platform_error;


use self::services::boot::execute_service_boot;
use self::services::hunt::execute_service_hunt;



pub type TaskPoolLoader = task_pool_loader::TaskPoolLoader;

/// Platform error type
///
pub type PlatformError = error::PlatformError;

/// Platform result type
///
pub type PlatformTaskResult = Result<(), PlatformError>;

/// Macro to create a platform error
/// 
#[macro_export]
macro_rules! platform_error {
    ($msg:expr, $parent:expr) => {
        Err(crate::platform::error::PlatformError::new(file!(), line!(), $msg.to_string(), $parent))
    };
}

/// Platform main object
/// 
pub struct Platform
{
    /// Task pool to manage all tasks
    task_pool: JoinSet<PlatformTaskResult>,

    /// Task loader
    // task_loader: TaskPoolLoader,

    /// Task pool receiver
    task_pool_rx: Arc<Mutex< tokio::sync::mpsc::Receiver<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>> >>,

    /// Services
    services: AmServices,

    /// Device manager
    devices: device::AmManager,

    /// Connection manager
    connection: connection::AmManager
}

impl Platform {

    /// Create a new instance of the Platform
    /// 
    pub fn new(name: &str) -> Platform {

        // Create the channel
        let (tx, rx) =
            tokio::sync::mpsc::channel::<BoxFuture<'static, PlatformTaskResult>>(5);
        
        let tl = TaskPoolLoader::new(tx);

        let srvs = Services::new(tl.clone());

        return Platform {
            task_pool: JoinSet::new(),
            // task_loader: tl.clone(),
            task_pool_rx: Arc::new(Mutex::new(rx)),
            services: srvs.clone(),
            devices: device::Manager::new(tl.clone(), srvs.clone()),
            connection: connection::Manager::new(tl.clone(), name)
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
        let c = self.connection.clone();
        self.task_pool.spawn(async move {
            Platform::services_task(s, d, c).await
        });

        // Start local discovery at the start of the application
        self.task_pool.spawn(
            Platform::local_service_discovery_task()
        );

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


            // self.services.lock().await.stop_requested();

            match join_result {

                Ok(task_result) => {
                    match task_result {
                        Ok(_) => {
                            tracing::warn!("Task completed");
                        },
                        Err(e) => {
                            tracing::error!("Task failed: {}", e);
                            self.task_pool.abort_all();

                        }
                    }
                },
                Err(e) => {
                    tracing::error!("Join failed: {}", e);
                }
            }

        }
    }

    /// Start the local service discovery 
    ///
    /// > COVER:PLATF_REQ_LSD_0000_00 - Service Port
    /// > COVER:PLATF_REQ_LSD_0010_00 - Request Payload
    /// > COVER:PLATF_REQ_LSD_0020_00 - Answer Payload
    ///
    pub async fn local_service_discovery_task() -> PlatformTaskResult {

        // Get port and address of broker used 
        // let broker_info_json = Platform::load_network_file_content().await;

        // If panic send the message expected 
        // start the connection
        let socket = UdpSocket::bind("0.0.0.0:53035").await.expect("creation local discovery socket failed");
        tracing::trace!(class="Platform", "Local discovery service start");

        let mut buf = [0; 1024];
        let json_reply_bytes = "{\"name\": \"panduza_platform\",\"version\": 1.0}".as_bytes();

        loop {
            // Receive request and answer it 
            // Error who didn't depend of the user so user unwrap or expect
            let (nbr_bytes, src_addr) = socket.recv_from(&mut buf).await.expect("receive local discovery failed");
            let filled_buf = &mut buf[..nbr_bytes];

            // need to manage if conversion from utf8 fail (with log)
            let buf_utf8 = std::str::from_utf8(&filled_buf);

            match buf_utf8 {
                Ok(buf) => {
                    let json_content: Result<serde_json::Value, serde_json::Error>  = serde_json::from_str(&buf);
                    match json_content {
                        Ok(content) => {
                            if content["search"] != json!(true) {
                                tracing::trace!(class="Platform", "Local discovery request message incorrect");
                                continue;
                            }
                            let _ = socket.send_to(json_reply_bytes, &src_addr).await;
                            tracing::trace!(class="Platform", "Local discovery reply send success");
                        },
                        Err(_e) => {
                            tracing::trace!(class="Platform", "Json request not correctly formatted");
                        }
                    }
                },
                Err(_e) => {
                    tracing::trace!(class="Platform", "Request need to be send to UTF-8 format");
                }
            }
        }
    }

    /// Services task
    /// 
    async fn services_task(services: AmServices, devices: device::AmManager, connection: connection::AmManager) -> PlatformTaskResult {
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
                        if execute_service_boot(services.clone()).await.is_err() {
                            return platform_error!("Failed to boot", None);
                        }
                        // , devices.clone(), connection.clone()

                        // Load the tree file
                        if let Err(e) = Platform::load_tree_file(services.clone()).await
                        {
                            tracing::warn!(class="Platform", "Failed to load tree: {}", e);
                            tracing::warn!(class="Platform", "Continue with default configuration");
                        }

                        // Sart minimal connection and devices
                        Platform::boot_minimal_services(services.clone(), devices.clone(), connection.clone()).await;

                        // log
                        tracing::info!(class="Platform", "Boot Success!");
                    }
                    
                    // --------------------------------------------------------
                    // --- RELOAD ---
                    if services.lock().await.reload_tree_requested() {
                        tracing::info!(class="Platform", "Reloading Configuration Tree...");

                        // Try to reload the tree
                        if let Err(e) = Platform::reload_tree(
                            services.clone(), devices.clone(), connection.clone()).await {
                            tracing::error!(class="Platform", "Failed to reload tree: {}", e);
                        }
                        
                        tracing::info!(class="Platform", "Reloading Success!");
                    }

                    // --------------------------------------------------------
                    // --- HUNT ---
                    if services.lock().await.hunt_requested() {

                        if execute_service_hunt(services.clone()).await.is_err() {
                            return platform_error!("Failed to hunt", None);
                        }
                    }

                    // --------------------------------------------------------
                    // --- STOP ---
                    if services.lock().await.stop_requested() {

                        
                        return Ok(());
                    }

                }
            }
        }
    }



    /// Start the broker connection
    /// 
    async fn start_broker_connection(services: AmServices, devices: device::AmManager, connection: connection::AmManager) {




        let sss = services.lock().await;
        let oci = sss.connection_info();

        if oci.is_none() {
            return;
        }

        let ci = oci.as_ref()
            .unwrap()
            .clone();

        connection.lock().await.start_connection(&ci.host_addr(), ci.host_port()).await;
        devices.lock().await.set_connection_link_manager(connection.lock().await.connection().unwrap().lock().await.link_manager());


    }

    /// Load the tree file from system into service data
    ///
    async fn load_tree_file(services: AmServices) -> Result<(), error::PlatformError> {

        // Get the tree file path
        let mut tree_file_path = PathBuf::from(dirs::home_dir().unwrap()).join("panduza").join("tree.json");
        match env::consts::OS {
            "linux" => {
                tree_file_path = PathBuf::from("/etc/panduza/tree.json");
                // println!("We are running linux!");
            }
            "windows" => {

            }
            _ => {
                tracing::error!("Unsupported system!");
            }
        }
        tracing::info!(class="Platform", "Loading tree file: {:?}", tree_file_path);

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
    async fn boot_minimal_services(services: AmServices, devices: device::AmManager, connection: connection::AmManager) {

        // Start the broker connection
        Platform::start_broker_connection(services.clone(), devices.clone(), connection.clone()).await;

        // Lock managers to create connection and device
        let mut d = devices.lock().await;

        // Server hostname
        let hostname = hostname::get().unwrap().to_string_lossy().to_string();

        // Create server device
        if let Err(e) = d.create_device(&json!({
            "name": hostname,
            "ref": "panduza.server"
        })).await {
            tracing::error!(class="Platform", "Failed to create device:\n{}", e);
        }

        // Mount devices
        d.start_devices().await;
    }

    /// Reload tree inside platform configuration
    /// 
    async fn reload_tree(
        services: AmServices, 
        devices_manager: device::AmManager, 
        connections_manager: connection::AmManager) -> Result<(), PlatformError>
    {

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
                                return platform_error!(
                                    format!("Failed to create device: {}", serde_json::to_string_pretty(&device_definition).unwrap()), 
                                    Some(Box::new(e))
                                );
                            },
                            Ok(new_device_name) => {
                                let mut d = devices_manager.lock().await;
                                let mut c = connections_manager.lock().await;
                        
                                let server_device = d.get_device(new_device_name).unwrap();
                                let connection = c.connection().unwrap();
                                // server_device.set_default_connection(default_connection.clone()).await;

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

        // Success
        Ok(())
    }



}

