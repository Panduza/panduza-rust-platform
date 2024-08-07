mod inner;
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
    Cleaning,
    Stopping,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::Connecting => write!(f, "Connecting"),
            State::Initializating => write!(f, "Initializating"),
            State::Running => write!(f, "Running"),
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
            logger: DeviceLogger::new(),
            reactor: reactor.clone(),
            inner: DeviceInner::new(reactor.clone(), operations, settings).into(),
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

    pub async fn run(&mut self) {
        // wait for notify event
        // then lock inner
        // use inner once
        // loop
        loop {
            // Perform state task
            match self.state {
                State::Connecting => {} // wait for reactor signal
                State::Initializating => {
                    self.inner
                        .lock()
                        .await
                        .operations
                        .mount(self.clone())
                        .await
                        .unwrap();

                    self.state = State::Running
                }

                State::Running => {} // do nothing, watch for inner tasks
                State::Warning => {}
                State::Cleaning => {}
                State::Stopping => {}
            }

            sleep(Duration::from_secs(1)).await;
        }
    }

    ///
    /// Clone settings of the device
    ///
    pub async fn settings(&self) -> DeviceSettings {
        self.inner.lock().await.settings.clone()
    }
}
