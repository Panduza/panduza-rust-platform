use std::sync::Arc;

use tokio::sync::Mutex;

use crate::Node;

pub struct DeviceInner {}

impl DeviceInner {
    pub fn new() -> DeviceInner {
        DeviceInner {}
    }
}

/// Allow mutation into Arc pointer
impl Into<Arc<Mutex<Node>>> for DeviceInner {
    fn into(self) -> Arc<Mutex<Node>> {
        Arc::new(Mutex::new(Node::Device(self)))
    }
}
