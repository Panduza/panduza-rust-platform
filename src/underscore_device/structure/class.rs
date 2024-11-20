use super::attribute::AttributElement;
use panduza_platform_core::InterfaceNotification;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// use panduza_platform_core::not

///
///
///
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassElement {
    ///
    ///
    ///
    tags: Vec<String>,

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
    info: Option<String>,
}

impl ClassElement {
    ///
    /// Constructor
    ///
    pub fn new(tags: Vec<String>, info: Option<String>) -> Self {
        Self {
            tags,
            classes: HashMap::default(),
            attributes: HashMap::default(),
            info: info,
        }
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
    ///
    ///
    pub fn get_mut_class_from_layers(&mut self, layers: Vec<String>) -> Option<&mut ClassElement> {
        //
        // low level debug
        println!("class::get_mut_class_from_layers({:?})", layers);

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

///
///
///
impl From<InterfaceNotification> for ClassElement {
    fn from(notif: InterfaceNotification) -> Self {
        ClassElement::new(notif.tags, None)
    }
}
