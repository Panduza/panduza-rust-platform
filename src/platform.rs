use futures::FutureExt;
use panduza_platform_core::{
    create_task_channel, env, DeviceMonitor, Factory, ProductionOrder, Runtime, TaskReceiver,
    TaskResult, TaskSender,
};
use panduza_platform_core::{PlatformLogger, Reactor, ReactorSettings};
use rumqttd::Broker;
use rumqttd::Config;
use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};
use tokio::task::JoinSet;
use tokio::time::sleep;

use crate::device_tree::DeviceTree;
use crate::plugins_manager::PluginsManager;
use crate::underscore_device::UnderscoreDevice;

///
///
///
static REQUEST_CHANNEL_SIZE: usize = 256;

pub enum ServiceRequest {
    Boot,
    StartBroker,
    LoadPlugins,
    LoadDeviceTree,
    LoadLocalRuntime,
    LoadUnderscoreDevice,
    ProduceDevice(ProductionOrder),
}

/// Platform
///
/// Shareable wrapper around its inner implementation
///
pub struct Platform {
    /// Main logger
    logger: PlatformLogger,

    ///
    /// Flag to know if we the platform must continue its work
    keep_alive: Arc<AtomicBool>,
    ///
    /// Flag to know alert the platform, it must stop
    must_stop: Arc<AtomicBool>,

    // -- tasks management
    // All the task that should never be stopped
    /// Pool
    task_pool: JoinSet<TaskResult>,
    /// Sender
    task_sender: TaskSender<TaskResult>,
    /// Receiver
    task_receiver: Option<TaskReceiver<TaskResult>>,
    /// Notify when a new task has been loaded
    new_task_notifier: Arc<Notify>,

    // -- Services management
    ///
    ///
    request_sender: Sender<ServiceRequest>,
    ///
    ///
    request_receiver: Option<Receiver<ServiceRequest>>,

    // -- Local Runtime
    ///
    ///
    // runtime: Option<Runtime>,
    ///
    ///
    reactor: Option<Reactor>,

    // -- Plugin management
    ///
    ///
    plugin_manager: PluginsManager,
}

impl Platform {
    /// Create a new instance of the Platform
    ///
    pub fn new() -> Self {
        //
        // Task creation request channel
        let (main_tx, main_rx) = create_task_channel::<TaskResult>(20);
        let (rqst_tx, rqst_rx) = channel::<ServiceRequest>(REQUEST_CHANNEL_SIZE);
        //
        // Create object
        return Self {
            logger: PlatformLogger::new(),

            keep_alive: Arc::new(AtomicBool::new(true)),
            must_stop: Arc::new(AtomicBool::new(false)),

            task_pool: JoinSet::new(),
            task_sender: main_tx,
            task_receiver: Some(main_rx),

            new_task_notifier: Arc::new(Notify::new()),

            request_sender: rqst_tx.clone(),
            request_receiver: Some(rqst_rx),

            reactor: None,
            plugin_manager: PluginsManager::new(),
        };
    }

    /// Main platform run loop
    ///
    pub async fn run(&mut self) {
        // Info log
        self.logger.info("Platform Version ...");

        // TODO: should be done thorugh connection.json
        // let settings = ReactorSettings::new("localhost", 1883, None);

        // //
        // let mut reactor = Reactor::new(settings);
        // reactor.start(self.task_sender.clone()).unwrap();

        // create the root device inside main task

        // // Creation à la mano du device de gestion de la platform
        // let (info_device_operations, info_pack) = InfoDevice::new();
        // let (mut info_monitor, info_device) = DeviceMonitor::new(
        //     reactor.clone(),
        //     None, // this device will manage info_pack and cannot use it to boot like other devices
        //     Box::new(info_device_operations),
        //     ProductionOrder::new("_", "_"),
        // );

        // let mut info_device_clone = info_device.clone();
        // self.task_sender
        //     .spawn(
        //         async move {
        //             info_device_clone.run_fsm().await;
        //             Ok(())
        //         }
        //         .boxed(),
        //     )
        //     .unwrap();

        // self.task_sender
        //     .spawn(
        //         async move {
        //             info_monitor.run().await;
        //             Ok(())
        //         }
        //         .boxed(),
        //     )
        //     .unwrap();

        //
        // Start service task
        // self.task_sender.spawn(service_task().boxed()).unwrap();

        //
        //
        self.request_sender.try_send(ServiceRequest::Boot).unwrap();

        //
        // Take the task reciever for usage in the main loop only
        let mut task_receiver = self.task_receiver.take().unwrap();
        let mut request_receiver = self.request_receiver.take().unwrap();

        //
        // Main running loop
        //
        while self.keep_alive.load(Ordering::Relaxed) {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    //
                    // Exit due to user request
                    self.logger.warn("User ctrl-c, abort requested");
                    self.task_pool.abort_all();
                    self.must_stop.store(true, Ordering::Relaxed);
                    self.new_task_notifier.notify_waiters();
                },
                task = task_receiver.rx.recv() => {
                    //
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.task_pool.spawn(task.unwrap());
                    self.logger.debug(format!( "New task created ! [{:?}]", ah ));
                },
                request = request_receiver.recv() => {
                    //
                    // Manage service requests
                    let request_value = request.unwrap();
                    match request_value {
                        ServiceRequest::Boot => {
                            self.service_boot().await;
                        },
                        ServiceRequest::StartBroker => {
                            self.service_start_broker().await;
                        }
                        ServiceRequest::LoadPlugins => {
                            self.service_load_plugins().await;
                        },
                        ServiceRequest::LoadDeviceTree => {
                            self.service_load_device_tree().await;
                        },
                        ServiceRequest::LoadLocalRuntime => {
                            self.service_load_local_runtime().await;
                        },
                        ServiceRequest::LoadUnderscoreDevice => {
                            self.service_load_underscore_device().await;
                        },
                        ServiceRequest::ProduceDevice(order) => {
                            self.service_produce_device(&order).await;
                        }
                    }
                },
                //
                // task to create monitor plugin manager notifications
                //
                continue_running = self.end_of_all_tasks() => {
                    //
                    // Manage platform end
                    if !continue_running {
                        break;
                    }
                }
            }
        }

        //
        self.logger.error("Platform EXIT");
    }

    /// Wait for all tasks to complete
    ///
    async fn end_of_all_tasks(&mut self) -> bool {
        //
        // Make tasks run
        while let Some(join_result) = self.task_pool.join_next().await {
            match join_result {
                Ok(jr) => match jr {
                    Ok(_) => {
                        self.logger.warn("Task completed successly");
                    }
                    Err(e) => {
                        self.logger.error(format!("Task end badly: {:?}", e));
                        self.task_pool.abort_all();
                    }
                },
                Err(e) => {
                    self.logger.error(format!("Task join_next error: {:?}", e));
                }
            }
        }
        //
        // Reaching here means that there is no task anymore
        self.logger.warn("All tasks completed");
        match self.must_stop.load(Ordering::Relaxed) {
            true => {
                // No task and stop request => quit this loop
                false
            }
            false => {
                // Wait for an other task to be loaded
                self.logger.warn("Wait for new tasks");
                self.new_task_notifier.notified().await;
                true
            }
        }
    }

    /// -------------------------------------------------------------
    ///
    async fn service_boot(&mut self) {
        //
        // info
        self.logger.info("----- SERVICE : BOOT -----");
        //
        //
        self.request_sender
            .try_send(ServiceRequest::StartBroker)
            .unwrap();
        //
        //
        self.request_sender
            .try_send(ServiceRequest::LoadPlugins)
            .unwrap();
        //
        //
        self.request_sender
            .try_send(ServiceRequest::LoadLocalRuntime)
            .unwrap();
        //
        //
        self.request_sender
            .try_send(ServiceRequest::LoadUnderscoreDevice)
            .unwrap();
        //
        //
        self.request_sender
            .try_send(ServiceRequest::LoadDeviceTree)
            .unwrap();
    }

    /// -------------------------------------------------------------
    ///
    async fn service_start_broker(&mut self) {
        //
        // info
        self.logger.info("----- SERVICE : START BROKER -----");

        let mut router: std::collections::HashMap<String, config::Value> = config::Map::new();
        router.insert("id".to_string(), config::Value::new(None, 0));
        router.insert(
            "max_connections".to_string(),
            config::Value::new(None, 10010),
        );
        router.insert(
            "max_outgoing_packet_count".to_string(),
            config::Value::new(None, 200),
        );
        router.insert(
            "max_segment_size".to_string(),
            config::Value::new(None, 104857600),
        );
        router.insert(
            "max_segment_count".to_string(),
            config::Value::new(None, 10),
        );

        let mut server_connections: std::collections::HashMap<String, config::Value> =
            config::Map::new();
        server_connections.insert(
            "connection_timeout_ms".to_string(),
            config::Value::new(None, 60000),
        );
        server_connections.insert(
            "max_payload_size".to_string(),
            config::Value::new(None, 20480),
        );
        server_connections.insert(
            "max_inflight_count".to_string(),
            config::Value::new(None, 10000),
        );
        server_connections.insert(
            "dynamic_filters".to_string(),
            config::Value::new(None, true),
        );

        let mut server: std::collections::HashMap<String, config::Value> = config::Map::new();
        server.insert("name".to_string(), config::Value::new(None, "v4-1"));
        server.insert(
            "listen".to_string(),
            config::Value::new(None, "0.0.0.0:1883"),
        );
        server.insert(
            "next_connection_delay_ms".to_string(),
            config::Value::new(None, 1),
        );
        server.insert(
            "connections".to_string(),
            config::Value::new(None, server_connections),
        );

        // see docs of config crate to know more
        let config = config::Config::builder()
            .set_default("id", 0)
            .unwrap()
            .set_default("router", router)
            .unwrap()
            .set_default("v4.1", server)
            .unwrap()
            .build()
            .unwrap();

        // this is where we deserialize it into Config
        let rumqttd_config: Config = config.try_deserialize().unwrap();
        let mut broker = Broker::new(rumqttd_config);

        self.logger.info("Start Broker");
        let jh = std::thread::spawn(move || {
            broker.start().unwrap();
            println!("BROKER STOPPPED !!!!!!!!!!!!!!!!!");
        });
    }

    /// -------------------------------------------------------------
    ///
    async fn service_load_plugins(&mut self) {
        //
        // info
        self.logger.info("----- SERVICE : LOAD PLUGINS -----");

        self.plugin_manager.load_system_plugins().unwrap();
    }

    /// -------------------------------------------------------------
    ///
    async fn service_load_device_tree(&mut self) {
        //
        // info
        self.logger.info("----- SERVICE : LOAD DEVICE TREE -----");

        let tree_path = env::system_default_device_tree_file().unwrap();

        // println!("search for tree in ({:?})", tree_path);

        let file = File::open(tree_path).unwrap();
        let dt: DeviceTree = serde_json::from_reader(&file).unwrap();

        for po in dt.devices {
            self.request_sender
                .try_send(ServiceRequest::ProduceDevice(po))
                .unwrap();
        }
    }

    /// -------------------------------------------------------------
    ///
    async fn service_load_local_runtime(&mut self) {
        //
        // info
        self.logger.info("----- SERVICE : LOAD LOCAL RUNTIME -----");

        //
        //
        let mut factory = Factory::new();
        // factory.add_producers(plugin_producers());

        //
        let settings = ReactorSettings::new("localhost", 1883, None);
        let mut reactor = Reactor::new(settings);
        reactor.start(self.task_sender.clone()).unwrap();

        self.reactor = Some(reactor.clone());

        //
        //
        let runtime = Runtime::new(factory, reactor);

        // //
        // //
        // POS = Some(runtime.clone_production_order_sender());

        //
        // Start thread
        self.task_sender.spawn(runtime.task().boxed()).unwrap();
    }

    /// -------------------------------------------------------------
    ///
    async fn service_load_underscore_device(&mut self) {
        //
        // info
        self.logger
            .info("----- SERVICE : LOAD UNDERSCORE DEVICE -----");

        let (underscore_device_operations, info_pack) = UnderscoreDevice::new();
        let (mut monitor, mut device) = DeviceMonitor::new(
            self.reactor.as_ref().unwrap().clone(),
            None, // this device will manage info_pack and cannot use it to boot like other devices
            Box::new(underscore_device_operations),
            ProductionOrder::new("_", "_"),
        );

        self.task_sender
            .spawn(
                async move {
                    device.run_fsm().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();

        self.task_sender
            .spawn(
                async move {
                    monitor.run().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();
    }

    /// -------------------------------------------------------------
    ///
    async fn service_produce_device(&mut self, po: &ProductionOrder) {
        //
        // info
        self.logger.info("----- SERVICE : PRODUCE DEVICE -----");
        self.logger.info(format!("ORDER: {:?}", po));

        let _res = self.plugin_manager.produce(po).unwrap();
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
