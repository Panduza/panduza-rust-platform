mod inner;
pub mod root;

use std::{fmt::Display, future::Future, sync::Arc, time::Duration};
use tokio::{task::JoinHandle, time::sleep};

pub use inner::DeviceInner;

use crate::{
    reactor::{self, Reactor},
    DeviceLogger, DeviceOperations, DeviceSettings, Error, TaskResult, TaskSender,
};

use serde_json;
use tokio::sync::Mutex;

use crate::InterfaceBuilder;
use futures::FutureExt;
pub mod monitor;

// use crate::interface::listener::Listener;

// use crate::interface::fsm::Fsm;
// use crate::platform::TaskReceiverLoader;

// use futures::FutureExt;
// // use crate::device::traits::DeviceActions;
// use crate::link::AmManager as AmLinkManager;

// use crate::interface::Builder as InterfaceBuilder;

// use crate::{subscription, FunctionResult, __platform_error_result};

// use crate::interface::Interface;

// use super::logger::{self, Logger};

/// States of the main Interface FSM
///
#[derive(Clone, Debug)]
pub enum State {
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
        spawner: TaskSender<Result<(), Error>>,
        name: String,
        operations: Box<dyn DeviceOperations>,
        settings: DeviceSettings,
    ) -> Device {
        // Create the object
        Device {
            logger: DeviceLogger::new(name.clone()),
            reactor: reactor.clone(),
            inner: DeviceInner::new(reactor.clone(), settings).into(),
            inner_operations: Arc::new(Mutex::new(operations)),
            topic: format!("{}/{}", reactor.root_topic(), name),
            state: State::Initializating,
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
}
