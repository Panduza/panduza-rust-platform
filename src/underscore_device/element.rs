pub mod attribute;
mod device;
mod interface;

pub use attribute::ElementAttribute;
pub use device::InfoElementDevice;
pub use interface::ElementInterface;
use panduza_platform_core::Error;

///
/// Element at the basis of device structure
///
pub enum InfoElement {
    Device(InfoElementDevice),
    Attribute(ElementAttribute),
    Interface(ElementInterface),
}

impl InfoElement {
    pub fn name(&self) -> &String {
        match self {
            InfoElement::Attribute(a) => &a.name(),
            InfoElement::Interface(i) => &i.name(),
            InfoElement::Device(info_element_device) => todo!(),
        }
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        match self {
            InfoElement::Attribute(a) => a.into_json_value(),
            InfoElement::Interface(i) => i.into_json_value(),
            InfoElement::Device(info_element_device) => todo!(),
        }
    }

    pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
        match self {
            InfoElement::Attribute(a) => a.is_element_exist(layers),
            InfoElement::Interface(i) => i.is_element_exist(layers),
            InfoElement::Device(info_element_device) => todo!(),
        }
    }

    ///
    ///
    ///
    pub fn insert(&mut self, layers: Vec<String>, element: InfoElement) -> Result<(), Error> {
        match self {
            InfoElement::Attribute(_) => Err(Error::InternalLogic(
                "Cannot insert an element inside an Attribute".to_string(),
            )),
            InfoElement::Interface(interface) => interface.insert(layers, element),
            InfoElement::Device(info_element_device) => todo!(),
        }
    }
}
