pub mod attribute;
pub mod class;
pub mod instance;

use instance::{Alert, InstanceElement};
use panduza_platform_core::driver_instance::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// Structure that represent the json maintained in '_/structure'
///
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Structure {
    ///
    /// Instances managed
    ///
    driver_instances: HashMap<String, InstanceElement>,

    ///
    /// User information about the structure
    ///
    info: Option<String>,
}

impl Structure {
    ///
    ///
    ///
    pub fn insert_instance(&mut self, name: String, instance: InstanceElement) {
        self.driver_instances.insert(name, instance);
    }

    // ///
    // ///
    // ///
    // pub fn insert_class(topic, class) {

    // }

    ///
    ///
    ///
    pub fn contains_instance(&mut self, name: &String) -> bool {
        self.driver_instances.contains_key(name)
    }

    ///
    ///
    ///
    pub fn get_mut_instance(&mut self, name: &String) -> Option<&mut InstanceElement> {
        self.driver_instances.get_mut(name)
    }

    ///
    ///
    ///
    pub fn pack_instance_status(&self) -> Vec<(String, State, Vec<Alert>)> {
        let mut r = Vec::new();
        for (_key, value) in (&self.driver_instances).into_iter() {
            r.push((_key.clone(), value.state.clone(), value.alerts.clone()));
        }
        r
    }
}
