pub mod element;

pub use element::attribute::AttributeMode;
pub use element::ElementAttribute;
pub use element::ElementInterface;
pub use element::StructuralElement;

use crate::Error;

struct Topic {
    pub namespace: String,
    pub host: String,
    pub device: String,
    pub layers: Vec<String>,
}

impl Topic {
    pub fn from_string<A: Into<String>>(topic: A) -> Self {
        // Split the topic
        let topic_string = topic.into();
        let mut layers: Vec<&str> = topic_string.split('/').collect();

        //
        //
        let mut namespace_parts: Vec<String> = Vec::new();
        while !layers.is_empty() {
            {
                let layer = layers.get(0).unwrap();
                if *layer == "pza" {
                    break;
                }
                namespace_parts.push(layer.to_string());
            }
            layers.remove(0);
        }

        // Remove pza
        layers.remove(0);

        //
        //
        let namespace = namespace_parts.join("/");
        let host = layers.remove(0).to_string();
        let device = layers.remove(0).to_string();

        Self {
            namespace,
            host,
            device,
            layers: layers.into_iter().map(|l| l.to_string()).collect(),
        }
    }
}

///
/// Meta Structure of all devices managed by the platform except '_'
///
pub struct DeviceStructure {
    ///
    /// Elements of the device
    ///
    elements: Vec<StructuralElement>,
}

impl DeviceStructure {
    pub fn new() -> Self {
        DeviceStructure {
            elements: Vec::new(),
        }
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        let mut p = serde_json::Map::new();

        for e in &self.elements {
            p.insert(e.name().clone(), e.into_json_value());
        }

        p.into()
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

        let t = Topic::from_string(topic);

        let layers = t.layers;

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
        //
        // Debug
        println!("structure::insert {:?}", topic);

        //
        //
        // let mut layers: Vec<&str> = topic.split('/').collect();
        let t = Topic::from_string(topic);
        let mut layers = t.layers;

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
    /// Just perform a very basic breakdown topic operation
    ///
    fn test_breakdown_topic_basic() {
        let topic = "namespace/pza/host/my_device/topic1/subtopic";

        let mut structure = Topic::from_string(topic);
        assert_eq!(structure.host, "host".to_string());
        assert_eq!(structure.device, "my_device".to_string());
        assert_eq!(
            structure.layers,
            vec!["topic1".to_string(), "subtopic".to_string()]
        );
    }

    #[test]
    ///
    /// Test that we can insert a interface element
    ///
    fn test_insert_element_basic() {
        // inputs
        let topic = "namespace/pza/host/my_device/topic1";

        // operation
        let mut structure = DeviceStructure::new();
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
        let topic1 = "namespace/pza/host/my_device/topic1";
        let topic2 = "namespace/pza/host/my_device/topic1/topic2";

        // operation
        let mut structure = DeviceStructure::new();
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
    ///
    ///
    fn test_into_json_basic() {
        // inputs
        let topic = "namespace/pza/host/my_device/topic1";

        // operation
        let mut structure = DeviceStructure::new();
        structure
            .insert(
                topic.to_string(),
                StructuralElement::Interface(ElementInterface::new("topic1", Vec::new())),
            )
            .unwrap();

        // checks
        let is_element_exist = structure.into_json_value();
        assert_eq!(is_element_exist, json!({}));
    }
}
