

mod json;
mod info;


pub type JsonAttribute = json::JsonAttribute;
pub type InfoAttribute = info::InfoAttribute;

/// Attribute Interface
///
/// Allow the platform to publish the attribute to the MQTT broker
///
pub trait AttributeInterface : Send + Sync {
    fn name(&self) -> &String;
    fn retain(&self) -> &bool;
    fn to_mqtt_payload(&self) -> String;
    fn from_mqtt_payload(&mut self, payload: &str);

    // fn update_field_with_string<F: Into<String>, V: Into<String>>(&mut self, field: F, value: V);
    fn update_field_with_string(&mut self, field: &str, value: &String);
}

