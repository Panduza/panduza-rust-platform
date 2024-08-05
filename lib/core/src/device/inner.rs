use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{DeviceOperations, Reactor};

pub struct DeviceInner {
    reactor: Reactor,
    pub operations: Box<dyn DeviceOperations>,
}

impl DeviceInner {
    pub fn new(reactor: Reactor, operations: Box<dyn DeviceOperations>) -> DeviceInner {
        DeviceInner {
            reactor: reactor,
            operations: operations,
        }
    }
}

/// Allow mutation into Arc pointer
impl Into<Arc<Mutex<DeviceInner>>> for DeviceInner {
    fn into(self) -> Arc<Mutex<DeviceInner>> {
        Arc::new(Mutex::new(self))
    }
}
