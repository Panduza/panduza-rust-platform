
mod json;
mod info;
mod raw;

pub type JsonAttribute = json::JsonAttribute;
pub type InfoAttribute = info::InfoAttribute;
pub type RawAttribute = raw::RawAttribute;

use crate::FunctionResult as PlatformFunctionResult;

// Enum to mqtt payload, String format json or byte array
pub enum MqttPayload {
    Json(String),
    Bytes(Vec<u8>)
}

// impl From<MqttPayload> for Vec<u8> {
//     fn from(payload: MqttPayload) -> Self {
//         match payload {
//             MqttPayload::Bytes(_) => payload.into(),
//             MqttPayload::Json(_) => payload.into()
//         }
//     }
// }

// required for `&MqttPayload` to implement `Into<Vec<u8>>

impl Into<Vec<u8>> for &MqttPayload {
    fn into(self) -> Vec<u8> {
        match self {
            MqttPayload::Json(v) => v.to_string().into(),
            MqttPayload::Bytes(v) => v.to_vec()
        }
    }
}

/// Attribute Interface
///
/// Allow the platform to publish the attribute to the MQTT broker
///
pub trait AttributeInterface : Send + Sync {
    fn name(&self) -> &String;
    fn retain(&self) -> &bool;
    fn to_mqtt_payload(&self) -> MqttPayload;
    fn from_mqtt_payload(&mut self, payload: &str);

    fn need_publication(&self) -> bool;
    fn publication_done(&mut self);

    fn update_field_with_f64(&mut self, field: &str, value: f64);
    fn update_field_with_bool(&mut self, field: &str, value: bool) -> PlatformFunctionResult;
    fn update_field_with_string(&mut self, field: &str, value: &String);
    fn update_field_with_json(&mut self, field: &str, value: &serde_json::Value);
    fn update_field_with_bytes(&mut self, value: &Vec<u8>);
}


