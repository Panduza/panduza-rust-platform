use std::sync::Weak;

use tokio::sync::Mutex;

use crate::{info::devices::ThreadSafeInfoDynamicDeviceStatus, DeviceInner, Reactor};

use super::Interface;

pub struct InterfaceBuilder {
    //
    pub reactor: Reactor,
    ///
    pub device_dyn_info: ThreadSafeInfoDynamicDeviceStatus,
    ///
    pub topic: String,
}

impl InterfaceBuilder {
    pub fn new<N: Into<String>>(
        reactor: Reactor,
        device_dyn_info: ThreadSafeInfoDynamicDeviceStatus,
        topic: N,
    ) -> Self {
        Self {
            reactor: reactor,
            device_dyn_info: device_dyn_info,
            topic: topic.into(),
        }
    }

    pub fn with_tags<T: Into<String>>(self, tags: T) -> Self {
        self
    }

    pub fn finish(self) -> Interface {
        // device_dyn_info
        // insert in status
        Interface::from(self)
    }
}
