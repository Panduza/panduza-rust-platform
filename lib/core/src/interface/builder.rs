use std::sync::Weak;

use tokio::sync::Mutex;

use crate::{DeviceInner, Reactor};

use super::Interface;

pub struct InterfaceBuilder {
    //
    pub reactor: Reactor,
    ///
    pub parent: Weak<Mutex<DeviceInner>>,
    ///
    pub topic: String,
}

impl InterfaceBuilder {
    pub fn new<N: Into<String>>(
        reactor: Reactor,
        parent: Weak<Mutex<DeviceInner>>,
        topic: N,
    ) -> Self {
        Self {
            reactor: reactor,
            parent: parent,
            topic: topic.into(),
        }
    }

    pub fn with_tags<T: Into<String>>(self, tags: T) -> Self {
        self
    }

    pub fn finish(self) -> Interface {
        Interface::from(self)
    }
}
