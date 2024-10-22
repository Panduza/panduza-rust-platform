mod structure;

use std::sync::Arc;

use crate::underscore_device::element::InfoElement;
use panduza_platform_core::{device::State, StateNotification, StructuralNotification};
use std::collections::HashMap;
use tokio::sync::{Mutex, Notify};

use super::{element::InfoElementInstance, Topic};
// use panduza_platform_core

///
/// Dynamic information that must be provided by the device to maintain a status inside
/// the main platform device "_".
///
/// The device "_" will always be up and will be able to display state and notification
/// if the device crash and is no longer able to communicate.
///
pub struct InfoDynamicDeviceStatus {
    ///
    /// Current state of the device
    state: State,

    ///
    /// Main Notifications from the device to the user
    // notifications: Vec<Notification>,

    ///
    /// True if the object has been updated
    /// The info device set it back to false when changes are published
    has_been_updated: bool,

    ///
    /// This allow this object to notify its parent
    /// This will trigger actions to manage a status notification
    /// The excepted action is a new publication on device "_"
    device_status_change_notifier: Arc<Notify>,

    device_structure_change_notifier: Arc<Notify>,
    // structure: DeviceStructure,
}

///
/// Thread safe wrapper
///
pub type ThreadSafeInfoDynamicDeviceStatus = Arc<Mutex<InfoDynamicDeviceStatus>>;

impl InfoDynamicDeviceStatus {
    pub fn new(
        device_status_change_notifier: Arc<Notify>,
        device_structure_change_notifier: Arc<Notify>,
    ) -> InfoDynamicDeviceStatus {
        let new_instance = InfoDynamicDeviceStatus {
            state: State::Booting,
            // notifications: Vec::new(),
            has_been_updated: true,
            device_status_change_notifier: device_status_change_notifier,
            device_structure_change_notifier: device_structure_change_notifier,
            // structure: DeviceStructure::new(),
        };
        new_instance.device_status_change_notifier.notify_waiters();
        new_instance
    }

    pub fn state_as_string(&self) -> String {
        format!("{}", self.state)
    }

    // pub fn structure_into_json_value(&self) -> serde_json::Value {
    //     self.structure.into_json_value()
    // }

    ///
    ///
    ///  
    pub fn change_state(&mut self, new_state: State) {
        self.state = new_state;
        self.has_been_updated = true;
        self.device_status_change_notifier.notify_waiters();
    }

    pub fn has_been_updated(&mut self) -> bool {
        if self.has_been_updated {
            self.has_been_updated = false;
            return true;
        }
        return false;
    }

    // pub fn structure_insert(&mut self, topic: String, element: InfoElement) -> Result<(), Error> {
    //     // println!("{:?}", self.structure.into_json_value());

    //     let res = self.structure.insert(topic, element);
    //     self.device_structure_change_notifier.notify_waiters();
    //     res
    // }

    // pub fn structure_remove()
}

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

                let instance = self.instances.get_mut(&instance_name).unwrap();
                if let InfoElement::Instance(info_element_instance) = instance {
                    println!("pooooooooooooookkkkk");
                }
                // match instance {
                //     InfoElement::Instance(info_element_instance) => {
                //         // info_element_instance.set_state(n.state.clone());
                //     }
                //     InfoElement::Attribute(element_attribute) => todo!(),
                //     InfoElement::Interface(element_interface) => todo!(),
                // }
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
    pub async fn structure_into_json_value(&self) -> serde_json::Value {
        let mut p = serde_json::Map::new();

        // for e in &self.devs {
        //     p.insert(e.0.clone(), e.1.lock().await.structure_into_json_value());
        // }

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

    ///
    ///
    pub fn get_dev_info(&self, name: &String) -> Option<Arc<Mutex<InfoDynamicDeviceStatus>>> {
        // match self.devs.get(name) {
        //     Some(o) => Some(o.clone()),
        //     None => None,
        // }
        None
    }

    ///
    /// Go trough status and check for update
    ///
    /// WARNING
    /// Maybe not the best way of doing this feature, it will force a thread to run useless
    /// periodic actions
    ///
    pub async fn check_for_status_update(&self) -> Vec<ThreadSafeInfoDynamicDeviceStatus> {
        let mut updated_status = Vec::new();
        // for d in &self.devs {
        //     if d.1.lock().await.has_been_updated {
        //         updated_status.push(d.1.clone())
        //     }
        // }
        updated_status
    }
}
