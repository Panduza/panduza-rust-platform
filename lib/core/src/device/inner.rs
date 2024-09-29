use crate::{DeviceSettings, Reactor};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Inner implementation of the device
pub struct DeviceInner {
    ///
    ///
    pub reactor: Reactor,

    /// Settings of the device, provided by the user
    ///
    pub settings: Option<DeviceSettings>,
}

impl DeviceInner {
    pub fn new(reactor: Reactor, settings: Option<DeviceSettings>) -> DeviceInner {
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
