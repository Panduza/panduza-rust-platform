#[cfg(feature = "built-in-drivers")]
use crate::built_in;

use crate::device_tree::DeviceTree;
use crate::local_broker_discovery;
use crate::plugins_manager::PluginsManager;
use crate::underscore_device::pack::InfoPack;
use crate::underscore_device::scanner::data::ScannerDriver;
use crate::underscore_device::store::data::SharedStore;
use crate::underscore_device::UnderscoreDevice;
use futures::FutureExt;
use panduza_platform_core::{
    create_task_channel, env, log_debug, log_warn, Factory, InstanceMonitor, Logger, Notification,
    NotificationGroup, ProductionOrder, Runtime, Store, TaskReceiver, TaskResult, TaskSender,
};
use panduza_platform_core::{Reactor, ReactorSettings};
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

use panduza_platform_core::log_info;

///
///
///
static REQUEST_CHANNEL_SIZE: usize = 256;

pub enum ServiceRequest {
    Boot,
    ReadConfig,
    StartBroker,
    StartLocalBrokerDiscovery,
    LoadPlugins,
    LoadDeviceTree,
    LoadLocalRuntime,
    LoadUnderscoreDevice,
    ProduceDevice(ProductionOrder),
    StartScanning,
}

/// Platform
///
/// Shareable wrapper around its inner implementation
///
pub struct Platform {
    /// Main logger
    logger: Logger,

    ///
    ///
    config: crate::config::Config,

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

    // -- Device monitoring
    ///
    /// Notifications that comes from devices
    /// They will help the underscore device to give informations to the user
    ///
    notifications: Arc<Mutex<Vec<Notification>>>,
    ///
    ///
    ///
    new_notifications_notifier: Arc<Notify>,

    ///
    ///
    ///
    store: SharedStore,

    built_in_store: Store,

    ///
    ///
    ///
    scanner_driver: ScannerDriver,

    local_runtime_po_sender: Option<tokio::sync::mpsc::Sender<ProductionOrder>>,
    local_runtime_notifications: Option<Arc<std::sync::Mutex<NotificationGroup>>>,
}

impl Platform {
    /// Create a new instance of the Platform
    ///
    pub fn new(enable_stdout: bool, debug: bool, trace: bool) -> Self {
        //
        // Task creation request channel
        let (main_tx, main_rx) = create_task_channel::<TaskResult>(20);
        let (rqst_tx, rqst_rx) = channel::<ServiceRequest>(REQUEST_CHANNEL_SIZE);
        //
        // Create object
        return Self {
            logger: Logger::new_for_platform(),

            config: crate::config::Config::default(),

            keep_alive: Arc::new(AtomicBool::new(true)),
            must_stop: Arc::new(AtomicBool::new(false)),

            task_pool: JoinSet::new(),
            task_sender: main_tx,
            task_receiver: Some(main_rx),
            new_task_notifier: Arc::new(Notify::new()),

            request_sender: rqst_tx.clone(),
            request_receiver: Some(rqst_rx),

            reactor: None,
            plugin_manager: PluginsManager::new(enable_stdout, debug, trace),

            notifications: Arc::new(Mutex::new(Vec::new())),
            new_notifications_notifier: Arc::new(Notify::new()),

            store: SharedStore::new(),
            built_in_store: Store::default(),
            scanner_driver: ScannerDriver::new(),

            local_runtime_po_sender: None,
            local_runtime_notifications: None,
        };
    }

    ///
    ///
    pub fn log_starting_info(
        &self,
        args: &super::Args,
        platform_version: &str,
        rustc_version: &str,
    ) {
        log_info!(self.logger, "-- Platform Start --");
        log_info!(self.logger, "Args: {:?}", args);
        log_info!(self.logger, "Platform Version: {}", platform_version);
        log_info!(self.logger, "Rustc Version: {}", rustc_version);
    }

    /// Main platform run loop
    ///
    pub async fn run(&mut self) {
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
                    log_warn!(self.logger, "User ctrl-c, abort requested");
                    self.task_pool.abort_all();
                    self.must_stop.store(true, Ordering::Relaxed);
                    self.new_task_notifier.notify_waiters();
                },
                //
                // Manage new task creation requests
                //
                request = task_receiver.rx.recv() => {
                    match request {
                        Some(task) => {
                            // Function to effectily spawn tasks requested by the system
                            let ah = self.task_pool.spawn(task.future);
                            log_debug!(self.logger, "New task created [{:?} => {:?}]", ah.id(), task.name);
                            self.new_task_notifier.notify_waiters();
                        },
                        None => {
                            log_warn!(self.logger, "Empty Task Request Received !");
                        }
                    }
                },
                //
                //
                //
                request = request_receiver.recv() => {
                    //
                    // Manage service requests
                    let request_value = request.unwrap();
                    match request_value {
                        ServiceRequest::Boot => {
                            self.service_boot().await;
                        },
                        ServiceRequest::ReadConfig => {
                            self.service_read_config().await;
                        },
                        ServiceRequest::StartBroker => {
                            self.service_start_broker().await;
                        }
                        ServiceRequest::StartLocalBrokerDiscovery => {
                            self.service_start_local_discovery().await;
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
                            self.service_produce_device(order).await;
                        },
                        ServiceRequest::StartScanning => {
                            self.service_start_scanning(self.scanner_driver.clone()).await;
                        },
                    }
                },
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    self.pull_notifications().await;
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
        log_debug!(self.logger, "All tasks completed");
        match self.must_stop.load(Ordering::Relaxed) {
            true => {
                // No task and stop request => quit this loop
                false
            }
            false => {
                // Wait for an other task to be loaded
                log_debug!(self.logger, "Wait for new tasks");
                self.new_task_notifier.notified().await;
                true
            }
        }
    }

    /// -------------------------------------------------------------
    ///
    async fn pull_notifications(&mut self) {
        let result = self.plugin_manager.pull_notifications();
        match result {
            Ok(new_notifications) => {
                let mut n = self.notifications.lock().await;
                n.extend(new_notifications);
                self.new_notifications_notifier.notify_waiters();
            }
            Err(e) => {
                self.logger
                    .error(format!("error while pulling notifis {:?}", e));
            }
        }

        //
        // Local notifications
        let local_notifs = self
            .local_runtime_notifications
            .as_mut()
            .unwrap()
            .lock()
            .unwrap()
            .pull();
        if !local_notifs.is_empty() {
            let mut n = self.notifications.lock().await;
            n.extend(local_notifs);
            self.new_notifications_notifier.notify_waiters();
        }
    }

    /// -------------------------------------------------------------
    ///
    async fn service_boot(&mut self) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : BOOT -----");
        //
        //
        self.request_sender
            .try_send(ServiceRequest::ReadConfig)
            .unwrap();
        //
        //
        self.request_sender
            .try_send(ServiceRequest::ReadConfig)
            .unwrap();
        //
        //
        self.request_sender
            .try_send(ServiceRequest::StartBroker)
            .unwrap();
        //
        //
        self.request_sender
            .try_send(ServiceRequest::StartLocalBrokerDiscovery)
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
    async fn service_read_config(&mut self) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : READ CONFIG -----");

        self.config = crate::config::get_platform_config(self.logger.clone());
    }

    /// -------------------------------------------------------------
    ///
    async fn service_start_broker(&mut self) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : START BROKER -----");

        let addr = self
            .config
            .broker
            .as_ref()
            .and_then(|b| b.addr.clone())
            .unwrap_or("127.0.0.1".to_string());

        let port = self
            .config
            .broker
            .as_ref()
            .and_then(|b| b.port.clone())
            .unwrap_or(1883);

        let listen_addr = format!("{}:{}", addr, port);

        let mut router: std::collections::HashMap<String, config::Value> = config::Map::new();
        router.insert("id".to_string(), config::Value::new(None, 0));
        router.insert(
            "max_connections".to_string(),
            config::Value::new(None, 20480),
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
            config::Value::new(None, 2000480),
        );
        server_connections.insert(
            "max_inflight_count".to_string(),
            config::Value::new(None, 20480),
        );
        server_connections.insert(
            "dynamic_filters".to_string(),
            config::Value::new(None, true),
        );

        let mut server: std::collections::HashMap<String, config::Value> = config::Map::new();
        server.insert("name".to_string(), config::Value::new(None, "v4-1"));
        server.insert(
            "listen".to_string(),
            config::Value::new(None, listen_addr.clone()),
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

        //
        // this is where we deserialize it into Config
        let rumqttd_config: Config = config.try_deserialize().unwrap();
        let mut broker = Broker::new(rumqttd_config);

        //
        // start broker
        log_info!(self.logger, "Broker listen on: {}", listen_addr);
        let _jh = std::thread::spawn(move || {
            broker.start().unwrap();
            println!("BROKER STOPPPED !!!!!!!!!!!!!!!!!");
        });
    }

    /// -------------------------------------------------------------
    ///
    async fn service_start_local_discovery(&mut self) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : START LOCAL DISCOVERY -----");

        //
        // Check in config if we must start the local discovery
        let plbd_is_enabled = self
            .config
            .services
            .as_ref()
            .and_then(|s| s.enable_plbd)
            .unwrap_or(false);

        if plbd_is_enabled {
            log_info!(self.logger, "PLBD is enabled");
            self.task_sender
                .spawn_with_name(
                    "local_broker_discovery",
                    local_broker_discovery::task(self.config.platform_name.clone()).boxed(),
                )
                .unwrap();
        } else {
            log_info!(self.logger, "PLBD is disabled");
        }
    }

    /// -------------------------------------------------------------
    ///
    async fn service_load_plugins(&mut self) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : LOAD PLUGINS -----");

        self.plugin_manager.load_system_plugins().unwrap();

        self.store
            .set_stores(self.plugin_manager.merge_stores())
            .await;
    }

    /// -------------------------------------------------------------
    ///
    async fn service_load_device_tree(&mut self) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : LOAD DEVICE TREE -----");

        //
        // Get path
        let tree_path = env::system_default_device_tree_file().unwrap();

        //
        // info
        self.logger
            .info(format!("TREE PATH: \"{}\"", tree_path.display()));

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
        log_info!(self.logger, "----- SERVICE : LOAD LOCAL RUNTIME -----");

        //
        // Allow mut here because it depend of features enabled
        #[allow(unused_mut)]
        let mut factory = Factory::new();

        //
        // Append built-in drivers
        #[cfg(feature = "built-in-drivers")]
        factory.add_producers(crate::built_in::plugin_producers());

        //
        //
        self.built_in_store = factory.store();

        //
        let settings = ReactorSettings::new("localhost", 1883, None);
        let mut reactor = Reactor::new(settings);
        reactor.start(self.task_sender.clone()).unwrap();

        self.reactor = Some(reactor.clone());

        //
        //
        let runtime: Runtime = Runtime::new(factory, reactor);

        //
        //
        self.local_runtime_notifications = Some(runtime.clone_notifications());

        //
        //
        self.local_runtime_po_sender = Some(runtime.clone_production_order_sender());

        //
        // Start thread
        self.task_sender
            .spawn_with_name("platform_runtime", runtime.task().boxed())
            .unwrap();
    }

    /// -------------------------------------------------------------
    ///
    async fn service_load_underscore_device(&mut self) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : LOAD UNDERSCORE DEVICE -----");

        //
        //
        let (underscore_device_operations, info_pack) =
            UnderscoreDevice::new(self.store.clone(), self.scanner_driver.clone());

        //
        //
        let (mut monitor, mut device) = InstanceMonitor::new(
            self.reactor.as_ref().unwrap().clone(),
            None, // this device will manage info_pack and cannot use it to boot like other devices
            Box::new(underscore_device_operations),
            ProductionOrder::new("_", "_"),
        );

        self.task_sender
            .spawn_with_name(
                "_/fsm",
                async move {
                    device.run_fsm().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();

        self.task_sender
            .spawn_with_name(
                "_/monitor",
                async move {
                    monitor.run().await;
                    Ok(())
                }
                .boxed(),
            )
            .unwrap();

        // // self.info_pack.
        let n_n = self.notifications.clone();
        let n_notifier = self.new_notifications_notifier.clone();
        self.task_sender
            .spawn_with_name(
                "notifications",
                Self::task_process_notifications(info_pack, n_notifier, n_n).boxed(),
            )
            .unwrap();

        //
        //
        self.task_sender
            .spawn_with_name(
                "scanner_request_processor",
                Self::task_process_scanner(
                    self.scanner_driver.clone(),
                    self.request_sender.clone(),
                )
                .boxed(),
            )
            .unwrap();
    }

    /// -------------------------------------------------------------
    ///
    async fn service_produce_device(&mut self, po: ProductionOrder) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : PRODUCE DEVICE -----");
        log_info!(self.logger, "ORDER: {:?}", po);

        if self.built_in_store.contains(&po.dref()) {
            log_info!(self.logger, "LOCAL PRODUCER");
            self.local_runtime_po_sender
                .as_ref()
                .unwrap()
                .try_send(po)
                .unwrap();
        } else {
            log_info!(self.logger, "PLUGIN PRODUCER");
            let _res = self.plugin_manager.produce(&po).unwrap();
        }
    }

    /// -------------------------------------------------------------
    ///
    async fn service_start_scanning(&mut self, mut scanner_shared_data: ScannerDriver) {
        //
        // info
        log_info!(self.logger, "----- SERVICE : START SCANNING -----");

        let mut orders = Vec::new();

        orders.extend(self.plugin_manager.scan().unwrap());

        #[cfg(feature = "built-in-drivers")]
        for scanner in built_in::plugin_scanners() {
            orders.extend(scanner.scan());
        }

        log_info!(self.logger, "Found instances : {:?}", orders);

        scanner_shared_data.store_instances(orders).await;
        scanner_shared_data.stop_running().await;
    }

    /// -------------------------------------------------------------
    ///
    async fn task_process_notifications(
        mut info_pack: InfoPack,
        n_notifier: Arc<Notify>,
        n_notifications: Arc<Mutex<Vec<Notification>>>,
    ) -> TaskResult {
        loop {
            n_notifier.notified().await;
            let mut lock = n_notifications.lock().await;
            let copy_notifs = lock.clone();
            lock.clear();
            drop(lock);
            info_pack.process_notifications(copy_notifs);
        }
    }

    /// -------------------------------------------------------------
    ///
    async fn task_process_scanner(
        driver: ScannerDriver,
        request_sender: Sender<ServiceRequest>,
    ) -> TaskResult {
        loop {
            driver.request_notifier.notified().await;
            if !driver.is_already_running().await {
                request_sender
                    .try_send(ServiceRequest::StartScanning)
                    .unwrap();
            }
        }
    }
}
