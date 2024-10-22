pub mod attribute;
mod instance;
mod interface;

pub use attribute::ElementAttribute;
pub use instance::InfoElementInstance;
pub use interface::ElementInterface;
use panduza_platform_core::{Error, StructuralNotification};

///
/// Element at the basis of Instance structure
///
#[derive(Debug)]
pub enum InfoElement {
    Instance(InfoElementInstance),
    Attribute(ElementAttribute),
    Interface(ElementInterface),
}

impl InfoElement {
    pub fn name(&self) -> &String {
        match self {
            InfoElement::Attribute(a) => &a.name(),
            InfoElement::Interface(i) => &i.name(),
            InfoElement::Instance(info_element_instance) => todo!(),
        }
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        match self {
            InfoElement::Attribute(a) => a.into_json_value(),
            InfoElement::Interface(i) => i.into_json_value(),
            InfoElement::Instance(inn) => inn.into_json_value(),
        }
    }

    pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
        match self {
            InfoElement::Attribute(a) => a.is_element_exist(layers),
            InfoElement::Interface(i) => i.is_element_exist(layers),
            InfoElement::Instance(info_element_instance) => todo!(),
        }
    }

    ///
    ///
    ///
    pub fn insert(&mut self, layers: Vec<String>, element: InfoElement) -> Result<(), Error> {
        println!(">>>>>>> {:?} -- {:?}", layers, element);
        match self {
            InfoElement::Attribute(_) => Err(Error::InternalLogic(
                "Cannot insert an element inside an Attribute".to_string(),
            )),
            InfoElement::Interface(interface) => interface.insert(layers, element),
            InfoElement::Instance(info_element_Instance) => {
                info_element_Instance.insert(layers, element)
            }
        }
    }
}

///
///
///
impl From<StructuralNotification> for InfoElement {
    fn from(value: StructuralNotification) -> Self {
        match value {
            StructuralNotification::Attribute(attribute_notification) => {
                InfoElement::Attribute(ElementAttribute::new(
                    attribute_notification.name(),
                    attribute_notification.typee(),
                    attribute_notification.mode().clone(),
                ))
            }
            StructuralNotification::Interface(interface_notification) => InfoElement::Interface(
                ElementInterface::new(interface_notification.topic(), Vec::new()),
            ),
        }
    }
}
