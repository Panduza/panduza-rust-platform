use panduza_platform_core::{device::State, Error};
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
}

impl InstanceElement {
    ///
    /// Define the state
    ///
    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    ///
    /// Convert the instance into a json format
    ///
    pub fn into_json_value(&self) -> serde_json::Value {
        let mut p = serde_json::Map::new();

        // for (name, e) in &self.children {
        //     p.insert(name.clone(), e.into_json_value());
        // }

        p.into()
    }

    // ///
    // /// Insert a sub element inside this instance
    // ///
    // pub fn insert(&mut self, layers: Vec<String>, element: InfoElement) -> Result<(), Error> {
    //     if layers.len() == 1 {
    //         // Insert HERE
    //         // new element name = layers.get(0)
    //         let _layer_name = match layers.get(0) {
    //             Some(value) => {
    //                 self.children.insert(value.clone(), element);
    //             }
    //             None => {
    //                 // None
    //                 // TODO SO UGLY
    //                 return Err(Error::Generic("layer name not found 2".to_string()));
    //                 // cannot find the layer => error
    //             }
    //         };

    //         // insert here
    //     } else {
    //         // Insert inside the sub layer
    //         let layer_name = match layers.get(0) {
    //             Some(value) => Some(value.to_string()),
    //             None => {
    //                 None
    //                 // Err(Error::Generic("layer name not found".to_string()))
    //                 // cannot find the layer => error
    //             }
    //         };

    //         match layer_name {
    //             Some(n) => {
    //                 let mut new_layers = layers;
    //                 new_layers.remove(0);
    //                 let sublayer = self.children.get_mut(&n).unwrap();
    //                 sublayer.insert(new_layers, element)?;
    //             }
    //             None => todo!(),
    //         }
    //     }

    //     Ok(())
    // }
}
