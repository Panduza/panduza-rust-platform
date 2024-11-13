use panduza_platform_core::device::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{attribute::AttributElement, class::ClassElement};

///
/// Represent an instance in the structure
///
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InstanceElement {
    ///
    /// State of the instance
    ///
    #[serde(skip)]
    pub state: State,

    ///
    /// Sub classes
    ///
    pub classes: HashMap<String, ClassElement>,

    ///
    /// Sub attributes
    ///
    pub attributes: HashMap<String, AttributElement>,

    ///
    /// User information about the structure
    ///
    info: String,
}

impl InstanceElement {
    ///
    /// Define the state
    ///
    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    ///
    ///
    ///
    pub fn insert_class(&mut self, name: String, class: ClassElement) {
        self.classes.insert(name, class);
    }

    ///
    ///
    ///
    pub fn insert_attribute(&mut self, name: String, attribute: AttributElement) {
        self.attributes.insert(name, attribute);
    }

    ///
    /// Get a class from its layers, it means that it will dig to find a sub class if needed
    ///
    pub fn get_mut_class_from_layers(&mut self, layers: &Vec<String>) -> Option<&mut ClassElement> {
        //
        // low level debug
        println!("instance::get_mut_class_from_layers({:?})", layers);

        if layers.len() == 1 {
            let name = layers.first().unwrap();
            self.classes.get_mut(name)
        } else if layers.len() > 1 {
            let name = layers.first().unwrap();
            let mut sub_layers = layers.clone();
            sub_layers.remove(0);
            self.classes
                .get_mut(name)
                .unwrap()
                .get_mut_class_from_layers(sub_layers)
        } else {
            None
        }
    }
}