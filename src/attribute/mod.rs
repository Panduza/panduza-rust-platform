
mod json;
mod info;

pub type JsonAttribute = json::JsonAttribute;
pub type InfoAttribute = info::InfoAttribute;

use crate::platform::FunctionResult as PlatformFunctionResult;

/// Attribute Interface
///
/// Allow the platform to publish the attribute to the MQTT broker
///
pub trait AttributeInterface : Send + Sync {
    fn name(&self) -> &String;
    fn retain(&self) -> &bool;
    fn to_mqtt_payload(&self) -> String;
    fn from_mqtt_payload(&mut self, payload: &str);

    fn need_publication(&self) -> bool;
    fn publication_done(&mut self);

    fn update_field_with_f64(&mut self, field: &str, value: f64);
    fn update_field_with_bool(&mut self, field: &str, value: bool) -> PlatformFunctionResult;
    fn update_field_with_string(&mut self, field: &str, value: &String);
    fn update_field_with_json(&mut self, field: &str, value: &serde_json::Value);
}


