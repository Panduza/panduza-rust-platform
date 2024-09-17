pub mod element;

pub use element::ElementAttribute;
pub use element::ElementInterface;
pub use element::StructuralElement;

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

    pub fn is_element_exist<A: Into<String>>(&self, topic: A) -> Result<bool, Error> {
        //
        // Breakdown the topic into layers
        let mut layers = self.breakdown_topic(topic);
        layers.remove(0);

        Ok(false)
    }

    ///
    ///
    ///
    pub fn find_layer(&mut self, name: &str) -> Option<&mut StructuralElement> {
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
            // Insert HERE
            // new element name = layers.get(0)
            let layer_name = match layers.get(0) {
                Some(value) => {
                    self.elements.push(element);
                }
                None => {
                    // None
                    // TODO SO UGLY
                    return Err(Error::Generic("layer name not found 2".to_string()));
                    // cannot find the layer => error
                }
            };

            // insert here
        } else {
            // Insert inside the sub layer
            let layer_name = match layers.get(0) {
                Some(value) => Some(value.to_string()),
                None => {
                    None
                    // Err(Error::Generic("layer name not found".to_string()))
                    // cannot find the layer => error
                }
            };

            match layer_name {
                Some(n) => {
                    layers.remove(0);
                    let sublayer = self
                        .find_layer(&n)
                        .ok_or(Error::Generic("cannot find layer 5".to_string()))?;
                    sublayer.insert(layers, element)?;
                }
                None => todo!(),
            }
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
    fn test_insert_element() {
        let device_name = "my_device";
        let topic = "namespace/pza/my_device/topic1/subtopic";

        let mut structure = DeviceStructure::new(device_name);
        structure.insert(
            topic.to_string(),
            StructuralElement::Interface(ElementInterface::new("machin", Vec::new())),
        );

        let search = structure.is_element_exist(topic).unwrap();
        assert!(search);
    }

    #[test]
    ///
    /// Just perform a very basic breakdown topic operation
    ///
    fn test_basic_breakdown_topic() {
        let device_name = "my_device";
        let topic = "namespace/pza/my_device/topic1/subtopic";
        let expected = vec!["my_device", "topic1", "subtopic"];

        let mut structure = DeviceStructure::new(device_name);
        let result = structure.breakdown_topic(topic);
        assert_eq!(result, expected);
    }
}
