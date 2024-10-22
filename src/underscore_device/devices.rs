use std::sync::Arc;

use crate::underscore_device::element::InfoElement;
use panduza_platform_core::{device::State, StateNotification, StructuralNotification};
use std::collections::HashMap;
use tokio::sync::Notify;

use super::{
    element::{self, InfoElementInstance},
    Topic,
};
// use panduza_platform_core

pub struct InfoPackInner {
    ///
    ///
    ///
    instances: HashMap<String, InfoElement>,

    ///
    /// Notified when a device status change
    ///
    instance_status_change_notifier: Arc<Notify>,

    instance_structure_change_notifier: Arc<Notify>,
}

impl InfoPackInner {
    ///
    ///
    pub fn new() -> InfoPackInner {
        InfoPackInner {
            instances: HashMap::new(),
            instance_status_change_notifier: Arc::new(Notify::new()),
            instance_structure_change_notifier: Arc::new(Notify::new()),
        }
    }

    ///
    ///
    ///
    pub fn process_state_changed(&mut self, n: &StateNotification) {
        let topic = Topic::from_string(n.topic.clone());
        // println!("{:?}", p.device);

        let instance_name = topic.device;

        // if the instance does not exist, create it
        if !self.instances.contains_key(&instance_name) {
            self.instances.insert(
                instance_name.clone(),
                InfoElement::Instance(InfoElementInstance::new(instance_name.clone())),
            );
        }

        let instance = self.instances.get_mut(&instance_name).unwrap();
        match instance {
            InfoElement::Instance(info_element_instance) => {
                info_element_instance.set_state(n.state.clone());
            }
            InfoElement::Attribute(element_attribute) => todo!(),
            InfoElement::Interface(element_interface) => todo!(),
        }

        self.instance_status_change_notifier.notify_waiters();
    }

    ///
    ///
    ///
    pub fn process_element_creation(&mut self, n: &StructuralNotification) {
        let topic = Topic::from_string(n.topic());
        println!("topic :::: {:?}", topic);
        match n {
            StructuralNotification::Attribute(attribute_notification) => {
                let instance_name = topic.device;

                if !self.instances.contains_key(&instance_name) {
                    self.instances.insert(
                        instance_name.clone(),
                        InfoElement::Instance(InfoElementInstance::new(instance_name.clone())),
                    );
                }

                let instance: &mut InfoElement = self.instances.get_mut(&instance_name).unwrap();

                let o = InfoElement::from(n.clone());

                instance.insert(topic.layers, o).unwrap();
            }
            StructuralNotification::Interface(interface_notification) => {
                let instance_name = topic.device;

                if !self.instances.contains_key(&instance_name) {
                    self.instances.insert(
                        instance_name.clone(),
                        InfoElement::Instance(InfoElementInstance::new(instance_name.clone())),
                    );
                }
            }
        }

        self.instance_structure_change_notifier.notify_waiters();
    }

    ///
    ///
    pub fn pack_instance_status(&self) -> Vec<(String, State)> {
        let mut r = Vec::new();
        // instance name
        // instance state
        for (key, value) in (&self.instances).into_iter() {
            match value {
                InfoElement::Instance(info_element_instance) => {
                    r.push((
                        info_element_instance.name.clone(),
                        info_element_instance.state.clone(),
                    ));
                }
                InfoElement::Attribute(element_attribute) => todo!(),
                InfoElement::Interface(element_interface) => todo!(),
            }
        }
        r
    }

    ///
    ///
    ///
    pub fn structure_into_json_value(&self) -> serde_json::Value {
        let mut p = serde_json::Map::new();

        for (name, e) in &self.instances {
            p.insert(name.clone(), e.into_json_value());
        }

        p.into()
    }

    ///
    ///
    pub fn instance_status_change_notifier(&self) -> Arc<Notify> {
        self.instance_status_change_notifier.clone()
    }

    ///
    ///
    pub fn instance_structure_change_notifier(&self) -> Arc<Notify> {
        self.instance_structure_change_notifier.clone()
    }
}
