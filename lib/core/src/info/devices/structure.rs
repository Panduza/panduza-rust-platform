pub mod element;

pub use element::attribute::AttributeMode;
pub use element::ElementAttribute;
pub use element::ElementInterface;
pub use element::StructuralElement;
use serde_json::json;

use crate::Error;

pub struct DeviceStructure {
    ///
    /// Name of the device
    ///
    device_name: String,

    ///
    /// Elements of the device
    ///
    elements: Vec<StructuralElement>,
}

impl DeviceStructure {
    pub fn new<A: Into<String>>(name: A) -> Self {
        DeviceStructure {
            device_name: name.into(),
            elements: Vec::new(),
        }
    }

    pub fn into_json_value() -> serde_json::Value {
        let p = serde_json::Map::new();

        p.into()
    }

    pub fn breakdown_topic<A: Into<String>>(&self, topic: A) -> Vec<String> {
        // Split the topic
        let topic_string = topic.into();
        let mut layers: Vec<&str> = topic_string.split('/').collect();

        // Remove layers before device name
        while !layers.is_empty() {
            if let Some(value) = layers.get(0) {
                if value == &self.device_name.as_str() {
                    break;
                } else {
                    layers.remove(0);
                }
            }
        }

        layers.into_iter().map(|s| s.to_string()).collect()
    }

    ///
    /// Check if there is an element that match the given name
    ///
    fn is_element_exist_from_name<A: Into<String>>(&self, name: A) -> Result<bool, Error> {
        let n = name.into();
        for element in &self.elements {
            if element.name() == &n {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    ///
    ///
    ///
    pub fn is_element_exist<A: Into<String>>(&self, topic: A) -> Result<bool, Error> {
        //
        // Breakdown the topic into layers
        let mut layers = self.breakdown_topic(topic);
        layers.remove(0);

        if layers.len() == 1 {
            //
            // Check on elements for this layer
            let name = layers.get(0).ok_or(Error::Wtf)?;
            return self.is_element_exist_from_name(name);
        } else {
            //
            // Check sub layers
            let name = layers.get(0).ok_or(Error::Wtf)?;
            let sublayer = self
                .find_layer(&name)
                .ok_or(Error::InternalLogic(format!("layer '{}' not found", name)))?;

            let mut new_la = layers;
            new_la.remove(0);
            return sublayer.is_element_exist(new_la);
        }
    }

    ///
    ///
    ///
    pub fn find_layer(&self, name: &str) -> Option<&StructuralElement> {
        self.elements.iter().find(|element| element.name() == name)
    }

    ///
    ///
    ///
    pub fn find_layer_mut(&mut self, name: &str) -> Option<&mut StructuralElement> {
        self.elements
            .iter_mut()
            .find(|element| element.name() == name)
    }

    pub fn insert(&mut self, topic: String, element: StructuralElement) -> Result<(), Error> {
        println!("pok");

        let mut layers: Vec<&str> = topic.split('/').collect();

        while !layers.is_empty() {
            match layers.get(0) {
                Some(value) => {
                    if value == &self.device_name.as_str() {
                        break;
                    } else {
                        layers.remove(0);
                    }
                }
                None => {
                    // Should never go there
                }
            }
        }
        println!("{:?}", layers);

        if layers.is_empty() {
            // error
        }

        // Remove device name
        layers.remove(0);

        if layers.is_empty() {
            // error insertion need at least a name
        }

        if layers.len() == 1 {
            //
            //
            let layer_name = layers
                .get(0)
                .ok_or(Error::Wtf)
                .and_then(|s| Ok(s.to_string()))?;

            //
            // basic check
            if &layer_name != element.name() {
                return Err(Error::InternalLogic(format!(
                    "Element that need to be inserted does not match the topic name {} ! {}",
                    element.name(),
                    layer_name
                )));
            }

            //
            //
            self.elements.push(element);

            // insert here
        } else {
            //
            // Insert inside a sub layer
            //
            // Get the sub layer name
            let sub_layer_name = layers
                .get(0)
                .ok_or(Error::Wtf)
                .and_then(|s| Ok(s.to_string()))?;

            //
            //
            let sublayer = self
                .find_layer_mut(&sub_layer_name)
                .ok_or(Error::InternalLogic(format!(
                    "#0001 Cannot find layer '{}'",
                    sub_layer_name
                )))?;

            //
            // Remove a layer
            layers.remove(0);

            sublayer.insert(layers, element)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    ///
    /// Test that we can insert a interface element
    ///
    fn test_insert_element_basic() {
        // inputs
        let device_name = "my_device";
        let topic = "namespace/pza/my_device/topic1";

        // operation
        let mut structure = DeviceStructure::new(device_name);
        structure
            .insert(
                topic.to_string(),
                StructuralElement::Interface(ElementInterface::new("topic1", Vec::new())),
            )
            .unwrap();

        // checks
        let is_element_exist = structure.is_element_exist(topic).unwrap();
        assert!(is_element_exist);
    }

    #[test]
    ///
    /// Test that we can insert a interface element
    ///
    fn test_insert_element_multiple_layer() {
        // inputs
        let device_name = "my_device";
        let topic1 = "namespace/pza/my_device/topic1";
        let topic2 = "namespace/pza/my_device/topic1/topic2";

        // operation
        let mut structure = DeviceStructure::new(device_name);
        structure
            .insert(
                topic1.to_string(),
                StructuralElement::Interface(ElementInterface::new("topic1", Vec::new())),
            )
            .unwrap();

        structure
            .insert(
                topic2.to_string(),
                StructuralElement::Attribute(ElementAttribute::new(
                    "topic2",
                    "string",
                    AttributeMode::AttOnly,
                )),
            )
            .unwrap();

        // checks
        let is_element1_exist = structure.is_element_exist(topic1).unwrap();
        assert!(is_element1_exist);
        let is_element2_exist = structure.is_element_exist(topic2).unwrap();
        assert!(is_element2_exist);
    }

    #[test]
    ///
    /// Just perform a very basic breakdown topic operation
    ///
    fn test_breakdown_topic_basic() {
        let device_name = "my_device";
        let topic = "namespace/pza/my_device/topic1/subtopic";
        let expected = vec!["my_device", "topic1", "subtopic"];

        let mut structure = DeviceStructure::new(device_name);
        let result = structure.breakdown_topic(topic);
        assert_eq!(result, expected);
    }
}
