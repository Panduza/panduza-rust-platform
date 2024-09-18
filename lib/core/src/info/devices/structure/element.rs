pub mod attribute;
mod interface;

use crate::Error;

pub use attribute::ElementAttribute;
pub use interface::ElementInterface;

///
/// Element at the basis of device structure
///
pub enum StructuralElement {
    Attribute(ElementAttribute),
    Interface(ElementInterface),
}

impl StructuralElement {
    pub fn name(&self) -> &String {
        match self {
            StructuralElement::Attribute(a) => &a.name(),
            StructuralElement::Interface(i) => &i.name(),
        }
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        match self {
            StructuralElement::Attribute(a) => a.into_json_value(),
            StructuralElement::Interface(i) => i.into_json_value(),
        }
    }

    pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
        match self {
            StructuralElement::Attribute(a) => a.is_element_exist(layers),
            StructuralElement::Interface(i) => i.is_element_exist(layers),
        }
    }

    ///
    ///
    ///
    pub fn insert(&mut self, layers: Vec<String>, element: StructuralElement) -> Result<(), Error> {
        match self {
            StructuralElement::Attribute(_) => Err(Error::InternalLogic(
                "Cannot insert an element inside an Attribute".to_string(),
            )),
            StructuralElement::Interface(interface) => interface.insert(layers, element),
        }
    }
}
