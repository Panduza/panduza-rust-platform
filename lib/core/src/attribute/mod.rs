pub mod bool;

pub enum AttributeType {
    Bool,
    String
}

pub trait Attribute {
    fn get_name(&self) -> &str;
    fn get_type(&self) -> AttributeType; 
}
//
//macro_rules! impl_get_name {
//    ($t: ty) => {
//        impl Attribute for $t {
//            fn get_name(&self) -> &str {
//                &self.name
//            }
//        }
//    };
//}

//mod a1;
//mod a3;
//
//mod json;
//mod info;
//mod raw;
//
//pub type JsonAttribute = json::JsonAttribute;
//pub type InfoAttribute = info::InfoAttribute;
//pub type RawAttribute = raw::RawAttribute;
//
//use crate::FunctionResult as PlatformFunctionResult;
//
//
//pub type A0 = a1::A1; // A0 has the same payload management than A1
//pub type A1 = a1::A1;
//pub type A3 = a3::A3;
//
//
//#[derive(Clone, Debug)]
//pub enum Attribute {
//    A0(A0),
//    A1(A1),
//    A3(A3),
//}
//
///// Thread safe connection object
//pub type ThreadSafeAttribute = std::sync::Arc<
//                                    tokio::sync::Mutex<
//                                        Attribute
//                                    >
//                                >;
//
//pub fn pack_attribute_as_thread_safe(attr: Attribute) -> ThreadSafeAttribute {
//    return std::sync::Arc::new(tokio::sync::Mutex::new(attr));
//}
//
//impl Attribute {
//    pub fn name(&self) -> String {
//        match self {
//            Attribute::A0(attr) => attr.name(),
//            Attribute::A1(attr) => attr.name(),
//            Attribute::A3(attr) => attr.name()
//        }
//    }
//
//    pub fn to_vec(&self) -> &Vec<u8> {
//        match self {
//            Attribute::A0(attr) => attr.to_vec(),
//            Attribute::A1(attr) => attr.to_vec(),
//            Attribute::A3(attr) => attr.to_vec()
//        }
//    }
//
//    pub fn retain(&self) -> bool {
//        match self {
//            Attribute::A0(attr) => *attr.retain(),
//            Attribute::A1(attr) => *attr.retain(),
//            Attribute::A3(_) => true
//        }
//    }
//}
//
//
//
//// Enum to mqtt payload, String format json or byte array
//pub enum MqttPayload {
//    Json(String),
//    Bytes(Vec<u8>)
//}
//
//// impl From<MqttPayload> for Vec<u8> {
////     fn from(payload: MqttPayload) -> Self {
////         match payload {
////             MqttPayload::Bytes(_) => payload.into(),
////             MqttPayload::Json(_) => payload.into()
////         }
////     }
//// }
//
//// required for `&MqttPayload` to implement `Into<Vec<u8>>
//
//impl Into<Vec<u8>> for &MqttPayload {
//    fn into(self) -> Vec<u8> {
//        match self {
//            MqttPayload::Json(v) => v.to_string().into(),
//            MqttPayload::Bytes(v) => v.to_vec()
//        }
//    }
//}
//
///// Attribute Interface
/////
///// Allow the platform to publish the attribute to the MQTT broker
/////
//pub trait AttributeInterface : Send + Sync {
//    fn name(&self) -> &String;
//    fn retain(&self) -> &bool;
//    fn to_mqtt_payload(&self) -> MqttPayload;
//    fn from_mqtt_payload(&mut self, payload: &str);
//
//    fn need_publication(&self) -> bool;
//    fn publication_done(&mut self);
//
//    fn update_field_with_f64(&mut self, field: &str, value: f64);
//    fn update_field_with_bool(&mut self, field: &str, value: bool) -> PlatformFunctionResult;
//    fn update_field_with_string(&mut self, field: &str, value: &String);
//    fn update_field_with_json(&mut self, field: &str, value: &serde_json::Value);
//
//
//    fn push_byte_stream(&mut self, value: &Vec<u8>);
//
//
//    // fn update_stream(&mut self, value: &Vec<u8>);
//}
//
