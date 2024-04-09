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

    fn update_field_with_f32(&mut self, field: &str, value: f32);
    fn update_field_with_bool(&mut self, field: &str, value: bool);
    fn update_field_with_string(&mut self, field: &str, value: &String);
}

