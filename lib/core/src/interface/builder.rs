use crate::info::devices::ElementInterface;
use crate::info::devices::StructuralElement;

use crate::{info::devices::ThreadSafeInfoDynamicDeviceStatus, Reactor};

use super::Interface;

pub struct InterfaceBuilder {
    //
    pub reactor: Reactor,
    ///
    /// Option because '_' device will not provide one
    ///
    pub device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
    ///
    pub topic: String,

    pub tags: Vec<String>,
}

impl InterfaceBuilder {
    pub fn new<N: Into<String>>(
        reactor: Reactor,
        device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
        topic: N,
    ) -> Self {
        Self {
            reactor: reactor,
            device_dyn_info: device_dyn_info,
            topic: topic.into(),
            tags: Vec::new(),
        }
    }

    pub fn with_tag<T: Into<String>>(mut self, tag: T) -> Self {
        self.tags.push(tag.into());
        self
    }

    ///
    ///
    ///
    pub async fn finish(self) -> Interface {
        let bis = self.topic.clone();
        let name = bis.split('/').last().unwrap();
        if let Some(device_dyn_info) = self.device_dyn_info.clone() {
            device_dyn_info
                .lock()
                .await
                .structure_insert(
                    self.topic.clone(),
                    StructuralElement::Interface(ElementInterface::new(
                        name.to_string(),
                        self.tags.clone(),
                    )),
                )
                .unwrap();
        }
        // insert in status
        Interface::from(self)
    }
}
