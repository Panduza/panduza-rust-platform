// use std::env;
// use std::path::PathBuf;
// use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use futures::FutureExt;
use serde_json::json;
// use dirs;
// use futures::future::BoxFuture;
// use futures::Future;
// use serde_json::json;
use tokio::signal;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::sleep;
// use tokio::net::UdpSocket;
// use crate::device;
// use crate::connection;

use crate::info::InfoDevice;
use crate::{task_channel::create_task_channel, TaskReceiver, TaskResult, TaskSender};

use crate::{
    Device, DeviceMonitor, Factory, PlatformLogger, ProductionOrder, Reactor, ReactorSettings,
};

/// Platform
///
/// Shareable wrapper around its inner implementation
///
pub struct Platform {
    /// Main logger
    logger: PlatformLogger,

    /// Factory
    factory: Factory,

    // Main tasks management
    // All the task that should never be stopped
    /// Pool
    main_task_pool: JoinSet<TaskResult>,
    /// Sender
    main_task_sender: TaskSender<TaskResult>,
    /// Receiver
    main_task_receiver: Arc<Mutex<TaskReceiver<TaskResult>>>,

    // Device tasks management
    // make it possible to cancel only device task when needed
    /// Devices
    devices: Vec<Device>,
    /// Pool
    device_task_pool: JoinSet<TaskResult>,
    /// Sender
    device_task_sender: TaskSender<TaskResult>,
    /// Receiver
    device_task_receiver: Arc<Mutex<TaskReceiver<TaskResult>>>,
}

impl Platform {
    /// Create a new instance of the Platform
    ///
    pub fn new(factory: Factory) -> Platform {
        // Main
        let (main_tx, main_rx) = create_task_channel::<TaskResult>(20);

        // Device
        let (device_tx, device_rx) = create_task_channel::<TaskResult>(50);

        // Create object
        return Platform {
            logger: PlatformLogger::new(),
            factory: factory,

            main_task_pool: JoinSet::new(),
            main_task_sender: main_tx,
            main_task_receiver: Arc::new(Mutex::new(main_rx)),

            devices: Vec::new(),
            device_task_pool: JoinSet::new(),
            device_task_sender: device_tx,
            device_task_receiver: Arc::new(Mutex::new(device_rx)),
        };
    }

    /// Main platform run loop
    ///
    pub async fn run(&mut self) {
        // Info log
        self.logger.info("Platform Version ...");

        // TODO: should be done thorugh connection.json
        let settings = ReactorSettings::new("localhost", 1883, None);

        //
        let mut reactor = Reactor::new(settings);
        reactor.start(self.main_task_sender.clone()).unwrap();

        // create the root device inside main task

        // Creation à la mano du device de gestion de la platform
        let (info_device_operations, info_pack) = InfoDevice::new();
        let (mut info_monitor, info_device) = DeviceMonitor::new(
            reactor.clone(),
            None, // this device will manage info_pack and cannot use it to boot like other devices
            Box::new(info_device_operations),
            ProductionOrder::new("_", "_"),
        );

        let mut info_device_clone = info_device.clone();
        self.main_task_sender
            .spawn(
                async move {
                    info_device_clone.run_fsm().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();

        self.main_task_sender
            .spawn(
                async move {
                    info_monitor.run().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();

        //
        let mut production_order = ProductionOrder::new("panduza.fake_register_map", "memory_map");
        // let mut production_order = ProductionOrder::new("panduza.picoha-dio", "testdevice");
        production_order.device_settings = json!({});
        let (mut monitor, mut dev) =
            self.factory
                .produce(reactor, Some(info_pack.clone()), production_order);

        // state machine + subtask monitoring

        let mut dddddd2 = dev.clone();
        self.main_task_sender
            .spawn(
                async move {
                    dddddd2.run_fsm().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();

        self.main_task_sender
            .spawn(
                async move {
                    monitor.run().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();

        //
        // need to spawn the idle task
        //
        // ugly but...
        self.main_task_pool.spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await
            }
        });

        // Main loop
        // Run forever and wait for:
        // - ctrl-c: to stop the platform after the user request it
        // - a new task to start in the task pool
        // - all tasks to complete
        let main_task_receiver_clone = self.main_task_receiver.clone();
        let mut main_task_receiver_clone_lock = main_task_receiver_clone.lock().await;
        let device_task_receiver_clone = self.device_task_receiver.clone();
        let mut device_task_receiver_clone_lock = device_task_receiver_clone.lock().await;
        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    self.logger.warn("User ctrl-c, abort requested");
                    self.main_task_pool.abort_all();
                },
                main_task = main_task_receiver_clone_lock.rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.main_task_pool.spawn(main_task.unwrap());
                    self.logger.debug(format!( "New task created ! [{:?}]", ah ));
                },
                device_task = device_task_receiver_clone_lock.rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.device_task_pool.spawn(device_task.unwrap());
                    self.logger.debug(format!( "New task created ! [{:?}]", ah ));
                },
                _ = self.end_of_all_main_tasks() => {
                    self.logger.warn("All tasks completed, stop the platform");
                    break;
                }
                // _ = self.end_of_all_device_tasks() => {
                //     // self.logger.warn("All tasks completed, stop the platform");
                //     break;
                // }
            }
        }
    }

    /// Wait for all tasks to complete
    ///
    async fn end_of_all_main_tasks(&mut self) {
        while let Some(join_result) = self.main_task_pool.join_next().await {
            // self.services.lock().await.stop_requested();

            match join_result {
                Ok(task_result) => match task_result {
                    Ok(_) => {
                        self.logger.warn("Task completed");
                    }
                    Err(e) => {
                        self.logger.error(format!("Task failed: {}", e));
                        self.main_task_pool.abort_all();
                    }
                },
                Err(e) => {
                    self.logger.error(format!("Join failed: {}", e));
                }
            }
        }
    }

    async fn end_of_all_device_tasks(&mut self) {
        while let Some(join_result) = self.device_task_pool.join_next().await {
            // self.services.lock().await.stop_requested();

            match join_result {
                Ok(task_result) => match task_result {
                    Ok(_) => {
                        self.logger.warn("Task completed");
                    }
                    Err(e) => {
                        self.logger.error(format!("Task failed: {}", e));
                        self.device_task_pool.abort_all();
                    }
                },
                Err(e) => {
                    self.logger.error(format!("Join failed: {}", e));
                }
            }
        }
    }
}

//         // Start the main service task directly
//         // it acts as a idle task for the platform to avoid the platform to stop if no other task
//         let s = self.services.clone();
//         let d = self.devices.clone();
//         let c = self.connection.clone();

//         // Quick and dirty store init
//         let store = d.lock().await.create_an_empty_store();
//         s.lock().await.update_device_store(store);

//         // Start the services task
//         self.task_pool.spawn(async move {
//             Platform::services_task(s, d, c).await
//         });

//         // Start local discovery at the start of the application
//         let plbd_platform_services = self.services.clone();
//         self.task_pool.spawn(
//             local_broker_discovery_task(plbd_platform_services)
//         );

//     }

//     /// Services task
//     ///
//     async fn services_task(services: AmServices, devices: device::AmManager, connection: connection::AmManager)
//         -> PlatformTaskResult
//     {
//         let requests_change_notifier = services.lock().await.get_requests_change_notifier();
//         loop {
//             // Wait for an event
//             requests_change_notifier.notified().await;
//             tracing::trace!(class="Platform", "Services task notified");
//             {
//                 if services.lock().await.has_pending_requests() {

//                     // --------------------------------------------------------
//                     // --- BOOT ---
//                     if services.lock().await.booting_requested() {

//                         execute_service_boot(services.clone()).await?;

//                         if let Err(e) = Platform::load_tree_file(services.clone()).await {
//                             tracing::warn!(class="Platform", "Failed to load tree: {}", e);
//                             tracing::warn!(class="Platform", "Continue with default configuration");
//                         }

//                         // Sart minimal connection and devices
//                         Platform::boot_minimal_services(services.clone(), devices.clone(), connection.clone()).await;

//                         // log
//                         tracing::info!(class="Platform", "Boot Success!");
//                     }

//                     // --------------------------------------------------------
//                     // --- RELOAD ---
//                     if services.lock().await.reload_tree_requested() {
//                         tracing::info!(class="Platform", "Reloading Configuration Tree...");

//                         // Try to reload the tree
//                         if let Err(e) = Platform::reload_tree(
//                             services.clone(), devices.clone(), connection.clone()).await {
//                             tracing::error!(class="Platform", "Failed to reload tree: {}", e);
//                         }

//                         tracing::info!(class="Platform", "Reloading Success!");
//                     }

//                     // --------------------------------------------------------
//                     // --- HUNT ---
//                     if services.lock().await.hunt_requested() {
//                         if execute_service_hunt(services.clone(), devices.clone()).await.is_err() {
//                             return __platform_error_result!("Failed to hunt");
//                         }
//                     }

//                     // --------------------------------------------------------
//                     // --- STOP ---
//                     if services.lock().await.stop_requested() {

//                         return Ok(());
//                     }

//                 }
//             }
//         }
//     }

//     /// Start the broker connection
//     ///
//     async fn start_broker_connection(services: AmServices, devices: device::AmManager, connection: connection::AmManager) {

//         let sss = services.lock().await;
//         let oci = sss.connection_info();

//         if oci.is_none() {
//             return;
//         }

//         let ci = oci.as_ref()
//             .unwrap()
//             .clone();

//         connection.lock().await.start_connection(&ci.broker_addr, ci.broker_port).await;
//         devices.lock().await.set_connection_link_manager(connection.lock().await.connection().unwrap().lock().await.link_manager());

//     }

//     /// Load the tree file from system into service data
//     ///
//     async fn load_tree_file(services: AmServices) -> Result<(), crate::Error> {

//         // Get the tree file path
//         let mut tree_file_path = PathBuf::from(dirs::public_dir().unwrap()).join("panduza").join("tree.json");
//         match env::consts::OS {
//             "linux" => {
//                 tree_file_path = PathBuf::from("/etc/panduza/tree.json");
//                 // println!("We are running linux!");
//             }
//             "windows" => {

//             }
//             _ => {
//                 tracing::error!("Unsupported system!");
//             }
//         }
//         tracing::info!(class="Platform", "Loading tree file: {:?}", tree_file_path);

//         // Try to read the file content
//         let file_content = tokio::fs::read_to_string(&tree_file_path).await;
//         match file_content {
//             Ok(content) => {
//                 return Platform::load_tree_string(services.clone(), &content).await;
//             },
//             Err(e) => {
//                 return __platform_error_result!(
//                     format!("Failed to read {:?} file content: {}", tree_file_path, e))
//             }
//         }
//     }

// /// Load a tree string into service data
// ///
// async fn load_tree_string(services: AmServices, content: &String) -> Result<(), crate::Error> {
//     // Parse the JSON content
//     let json_content = serde_json::from_str::<serde_json::Value>(&content);
//     match json_content {
//         Ok(json) => {
//             // log
//             tracing::info!(
//                 class = "Platform",
//                 " - Tree Json content -\n{}",
//                 serde_json::to_string_pretty(&json).unwrap()
//             );
//             services.lock().await.set_tree_content(json);

//             return Ok(());
//         }
//         Err(e) => return __platform_error_result!(format!("Failed to parse JSON content: {}", e)),
//     }
// }

//     /// Boot default connection and platform device
//     ///
//     async fn boot_minimal_services(services: AmServices, devices: device::AmManager, connection: connection::AmManager) {

//         // Start the broker connection
//         Platform::start_broker_connection(services.clone(), devices.clone(), connection.clone()).await;

//         // Lock managers to create connection and device
//         let mut d = devices.lock().await;

//         // Server hostname
//         let hostname = hostname::get().unwrap().to_string_lossy().to_string();

//         // Create server device
//         if let Err(e) = d.create_device(&json!({
//             "name": hostname,
//             "ref": "panduza.server"
//         })).await {
//             tracing::error!(class="Platform", "Failed to create device:\n{}", e);
//         }

//         // Mount devices
//         d.start_devices().await.unwrap();
//     }

//     /// Reload tree inside platform configuration
//     ///
//     async fn reload_tree(
//         services: AmServices,
//         devices_manager: device::AmManager,
//         connections_manager: connection::AmManager) -> Result<(), crate::Error>
//     {

//         let services_lock = services.lock().await;

//         let tree_ref = services_lock.get_tree_content();

//         tracing::info!(class="Platform", "store : {}", tree_ref);

//         let devices_definitions= tree_ref.get("devices");
//         match devices_definitions {
//             Some(devices) => {
//                 // Iterate over the devices
//                 if let Some(devices) = devices.as_array() {
//                     for device_definition in devices {

//                         let result = devices_manager.lock().await.create_device(device_definition).await;
//                         match result {
//                             Err(_e) => {
//                                 return __platform_error_result!(
//                                     format!("Failed to create device: {}", serde_json::to_string_pretty(&device_definition).unwrap())
//                                 );
//                             },
//                             Ok(new_device_name) => {
//                                 let mut d = devices_manager.lock().await;
//                                 let mut _c = connections_manager.lock().await;

//                                 let _server_device = d.get_device(new_device_name).unwrap();
//                                 let _connection = _c.connection().unwrap();
//                                 // server_device.set_default_connection(default_connection.clone()).await;

//                             }
//                         }

//                     }
//                 }
//             },
//             None => {
//                 tracing::warn!("No devices found in the tree");
//             }
//         }

//         let mut d = devices_manager.lock().await;
//         d.start_devices().await.unwrap();

//         // Success
//         Ok(())
//     }

//     pub fn devices(&self) -> &device::AmManager {
//         return &self.devices;
//     }

// }
