use crate::{
    info::devices::ThreadSafeInfoDynamicDeviceStatus, AttributeBuilder, InterfaceBuilder, Reactor,
};

pub mod builder;

#[derive(Clone)]
pub struct Interface {
    ///
    reactor: Reactor,
    //
    pub device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
    ///
    topic: String,
}

impl Interface {
    ///
    /// Create a new interface from this device
    ///
    pub fn create_interface<N: Into<String>>(&mut self, name: N) -> InterfaceBuilder {
        InterfaceBuilder::new(
            self.reactor.clone(),
            self.device_dyn_info.clone(),
            format!("{}/{}", self.topic, name.into()), // take the device topic as root
        )
    }

    pub fn create_attribute<N: Into<String>>(&mut self, name: N) -> AttributeBuilder {
        self.reactor
            .create_new_attribute(self.device_dyn_info.clone())
            .with_topic(format!("{}/{}", self.topic, name.into()))
    }
}

impl From<builder::InterfaceBuilder> for Interface {
    fn from(builder: builder::InterfaceBuilder) -> Self {
        Interface {
            reactor: builder.reactor,
            device_dyn_info: builder.device_dyn_info,
            topic: builder.topic,
        }
    }
}
