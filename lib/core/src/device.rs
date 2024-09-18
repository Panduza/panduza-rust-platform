mod inner;

use std::{fmt::Display, future::Future, sync::Arc};
use tokio::sync::Notify;

pub use inner::DeviceInner;

use crate::{
    info::devices::InfoDynamicDeviceStatus, reactor::Reactor, AttributeBuilder, DeviceLogger,
    DeviceOperations, DeviceSettings, Error, InfoPack, TaskResult, TaskSender,
};

use tokio::sync::Mutex;

use crate::InterfaceBuilder;
use futures::FutureExt;
pub mod monitor;

/// States of the main Interface FSM
///
#[derive(Clone, Debug)]
pub enum State {
    Booting,
    Connecting,
    Initializating,
    Running,
    Warning,
    Error,
    Cleaning,
    Stopping,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Booting => write!(f, "Booting"),
            State::Connecting => write!(f, "Connecting"),
            State::Initializating => write!(f, "Initializating"),
            State::Running => write!(f, "Running"),
            State::Error => write!(f, "Error"),
            State::Warning => write!(f, "Warning"),
            State::Cleaning => write!(f, "Cleaning"),
            State::Stopping => write!(f, "Stopping"),
        }
    }
}

/// A device manage a set of interfaces
///
#[derive(Clone)]
pub struct Device {
    // pub settings: serde_json::Value,
    pub logger: DeviceLogger,

    //
    reactor: Reactor,

    // Object to provide data to the info device
    /// Main pack
    info_pack: Option<InfoPack>,

    ///
    /// Device must share its status with the device "_" through this info object
    info_dyn_dev_status: Option<Arc<Mutex<InfoDynamicDeviceStatus>>>,

    // started: bool,
    /// Inner object
    inner: Arc<Mutex<DeviceInner>>,

    /// Operations of the devices
    ///
    inner_operations: Arc<Mutex<Box<dyn DeviceOperations>>>,

    ///
    topic: String,

    // platform_services: crate::platform::services::AmServices,
    // // logger: Logger,
    state: State,
    state_change_notifier: Arc<Notify>,
    //
    //
    spawner: TaskSender<Result<(), Error>>,
}

impl Device {
    //
    // reactor

    /// Create a new instance of the Device
    ///
    pub fn new(
        reactor: Reactor,
        info_pack: Option<InfoPack>,
        spawner: TaskSender<Result<(), Error>>,
        name: String,
        operations: Box<dyn DeviceOperations>,
        settings: DeviceSettings,
    ) -> Device {
        // Create the object
        Device {
            logger: DeviceLogger::new(name.clone()),
            reactor: reactor.clone(),
            info_pack: info_pack,
            info_dyn_dev_status: None,
            inner: DeviceInner::new(reactor.clone(), settings).into(),
            inner_operations: Arc::new(Mutex::new(operations)),
            topic: format!("{}/{}", reactor.root_topic(), name),
            state: State::Booting,
            state_change_notifier: Arc::new(Notify::new()),
            spawner: spawner,
        }
    }

    pub async fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = TaskResult> + Send + 'static,
    {
        self.spawner.spawn(future.boxed()).unwrap();
    }

    ///
    /// Create a new interface from this device
    ///
    pub fn create_interface<N: Into<String>>(&mut self, name: N) -> InterfaceBuilder {
        InterfaceBuilder::new(
            self.reactor.clone(),
            self.info_dyn_dev_status.clone(),
            format!("{}/{}", self.topic, name.into()), // take the device topic as root
        )
    }

    ///
    /// Device can directly create some attribute on its root
    ///
    pub fn create_attribute<N: Into<String>>(&mut self, name: N) -> AttributeBuilder {
        self.reactor
            .create_new_attribute(self.info_dyn_dev_status.clone())
            .with_topic(format!("{}/{}", self.topic, name.into())) // take the device topic as root
    }

    // pub async fn run(&mut self) {}

    ///
    /// Run the FSM of the device
    ///
    pub async fn run_fsm(&mut self) {
        //
        // First start by booting the device to give him a connection with the info_pack
        // and allow the InfoDevice to send device information on MQTT
        self.move_to_state(State::Booting).await;

        //
        // Start the main loop of the device
        // TODO => Maybe we should give a way to stop properly this task instead of canceling the task brutally
        loop {
            self.state_change_notifier.notified().await;

            // Helper log
            self.logger.debug(format!("FSM State {}", self.state));

            // Perform state task
            match self.state {
                State::Booting => {
                    if let Some(mut info_pack) = self.info_pack.clone() {
                        self.logger.debug("FSM try to add_deivce in info pack");
                        self.info_dyn_dev_status = Some(info_pack.add_device(self.name()).await);
                        self.logger.debug("FSM finish info pack");
                    } else {
                        self.logger.debug("FSM NO INFO PACK !");
                    }
                    self.move_to_state(State::Initializating).await;
                }
                State::Connecting => {} // wait for reactor signal
                State::Initializating => {
                    //
                    // Try to mount the device
                    let mount_result = self.inner_operations.lock().await.mount(self.clone()).await;
                    //
                    // Manage mount result
                    match mount_result {
                        Ok(_) => {
                            self.logger.debug("FSM Mount Success ");
                            self.move_to_state(State::Running).await;
                        }
                        Err(e) => {
                            self.logger.error(format!("FSM Mount Failure {}", e));
                            self.move_to_state(State::Error).await;
                        }
                    }
                }
                State::Running => {} // do nothing, watch for inner tasks
                State::Error => {
                    //
                    // Wait before reboot
                    self.inner_operations
                        .lock()
                        .await
                        .wait_reboot_event(self.clone())
                        .await;
                    self.logger.info("try to reboot");
                    self.move_to_state(State::Initializating).await;
                }
                State::Warning => {}
                State::Cleaning => {}
                State::Stopping => {}
            }
        }

        // Ok(())
    }

    ///
    /// Clone settings of the device
    ///
    pub async fn settings(&self) -> DeviceSettings {
        self.inner.lock().await.settings.clone()
    }

    pub fn name(&self) -> String {
        match self.topic.split('/').last() {
            Some(value) => value.to_string(),
            None => "noname".to_string(),
        }
    }

    ///
    /// Function to change the current state of the device FSM
    ///
    pub async fn move_to_state(&mut self, new_state: State) {
        // Set the new state
        self.state = new_state;

        // Alert monitoring device "_"
        if let Some(sts) = &mut self.info_dyn_dev_status {
            sts.lock().await.change_state(self.state.clone());
        }

        // Notify FSM
        self.state_change_notifier.notify_one();
    }
}
