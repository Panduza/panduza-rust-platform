mod inner;

use std::{fmt::Display, future::Future, sync::Arc, time::Duration};
use tokio::time::sleep;

pub use inner::DeviceInner;

use crate::{
    info::devices::InfoDev, reactor::Reactor, DeviceLogger, DeviceOperations, DeviceSettings,
    Error, InfoPack, TaskResult, TaskSender,
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
    /// Specific for device info
    info_dev: Option<Arc<Mutex<InfoDev>>>,

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
            info_dev: None,
            inner: DeviceInner::new(reactor.clone(), settings).into(),
            inner_operations: Arc::new(Mutex::new(operations)),
            topic: format!("{}/{}", reactor.root_topic(), name),
            state: State::Booting,
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
    ///
    pub fn create_interface<N: Into<String>>(&mut self, name: N) -> InterfaceBuilder {
        InterfaceBuilder::new(
            self.reactor.clone(),
            Arc::downgrade(&self.inner),
            format!("{}/{}", self.reactor.root_topic(), name.into()),
        )
    }

    pub fn create_attribute<N: Into<String>>(&mut self, name: N) {}

    // pub async fn run(&mut self) {}

    ///
    /// Run the FSM of the device
    ///
    pub async fn run_fsm(&mut self) {
        // wait for notify event
        // then lock inner
        // use inner once
        // loop

        loop {
            // Helper log
            self.logger.debug(format!("FSM State {}", self.state));

            // Perform state task
            match self.state {
                State::Booting => {
                    if let Some(mut info_pack) = self.info_pack.clone() {
                        self.info_dev = Some(info_pack.add_device(self.name()).await);
                    }
                    self.state = State::Initializating;
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
                            self.state = State::Running;
                        }
                        Err(e) => {
                            self.logger.error(format!("FSM Mount Failure {}", e));
                            self.state = State::Error;
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
                    self.state = State::Initializating;
                }
                State::Warning => {}
                State::Cleaning => {}
                State::Stopping => {}
            }

            sleep(Duration::from_secs(1)).await;
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
}
