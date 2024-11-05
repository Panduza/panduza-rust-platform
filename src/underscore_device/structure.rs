use super::element::instance::InstanceElement;
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
    info: String,
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
}
