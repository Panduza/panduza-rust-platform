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

pub struct DeviceStructure {
    elements: Vec<StructuralElement>,
}

impl DeviceStructure {
    pub fn new() -> Self {
        DeviceStructure {
            elements: Vec::new(),
        }
    }

    pub fn insert(&mut self, topic: String, element: StructuralElement) {
        println!("pok");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_element() {
        let mut structure = DeviceStructure::new();
        structure.insert(
            "truc/machin".to_string(),
            StructuralElement::Interface(ElementInterface::new()),
        );
    }
}
