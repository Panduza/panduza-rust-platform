use std::{future::Future, sync::Arc};

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{DeviceOperations, DeviceSettings, Reactor, TaskResult};
use tokio::task::JoinSet;

/// Inner implementation of the device
pub struct DeviceInner {
    ///
    ///
    pub reactor: Reactor,

    /// Settings of the device, provided by the user
    ///
    pub settings: DeviceSettings,
}

impl DeviceInner {
    pub fn new(reactor: Reactor, settings: DeviceSettings) -> DeviceInner {
        DeviceInner {
            reactor: reactor,
            settings: settings,
        }
    }
}

/// Allow mutation into Arc pointer
impl Into<Arc<Mutex<DeviceInner>>> for DeviceInner {
    fn into(self) -> Arc<Mutex<DeviceInner>> {
        Arc::new(Mutex::new(self))
    }
}
