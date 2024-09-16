use crate::Error;

enum AttributeMode {
    AttOnly,
    CmdOnly,
    Bidir,
}

pub struct ElementAttribute {
    name: String,
    typee: String,
    mode: AttributeMode,
}

pub struct ElementInterface {
    name: String,
    tags: Vec<String>,
    elements: Vec<StructuralElement>,
}

impl ElementInterface {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            tags: Vec::new(),
            elements: Vec::new(),
        }
    }

    pub fn insert(topic: String, element: StructuralElement) {}
}

///
/// Element at the basis of device structure
///
pub enum StructuralElement {
    Attribute(ElementAttribute),
    Interface(ElementInterface),
}

impl StructuralElement {
    pub fn insert(&self, topic: String, element: StructuralElement) {
        match self {
            StructuralElement::Attribute(_) => todo!(),
            StructuralElement::Interface(_) => Err(Error),
        }
    }
}

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

    pub fn insert(&mut self, topic: String, element: StructuralElement) {
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
            // new element name = layers.get(0)
            // insert here
        } else {
            // next layer = layers.get(0)
            // layers.remove(0);
            // next_element_layer = elements.get(next layer)
            // next_element_layer.insert(layers, element)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_element() {
        let mut structure = DeviceStructure::new("device");
        structure.insert(
            "namespace/pza/device/truc/machin".to_string(),
            StructuralElement::Interface(ElementInterface::new()),
        );

        assert!(false);
    }
}
