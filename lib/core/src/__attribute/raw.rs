use super::{AttributeInterface, MqttPayload};

use crate::FunctionResult as PlatformFunctionResult;
pub struct RawAttribute {
    // 
    name: String,

    retain: bool,

    // 
    data: Vec<u8>,

    // 
    orign: Vec<u8>,
}

impl RawAttribute {
    pub fn new<A: Into<String>>(name: A, retain: bool) -> RawAttribute {

        let name_str = name.into();

        let data: Vec<u8> = Vec::new(); 
        let data2: Vec<u8> = Vec::new(); 

        return RawAttribute {
            name: name_str,
            retain: retain,
            data: data,
            orign: data2,
        };
    }

    /// Create a new instance of the object inside a box
    ///
    #[inline]
    pub fn new_boxed<A: Into<String>>(name: A, retain: bool)
        -> Box<dyn AttributeInterface>
    {
        return Box::new(RawAttribute::new(name, retain));
    }
}


impl AttributeInterface for RawAttribute {

    fn name(&self) -> &String {
        return &self.name;
    }
    
    fn retain(&self) -> &bool {
        return &self.retain;
    }
    
    fn to_mqtt_payload(&self) -> MqttPayload {
        return MqttPayload::Bytes(self.data.clone());
    }
    
    fn from_mqtt_payload(&mut self, _payload: &str) {
        todo!()
    }

    fn need_publication(&self) -> bool {
        return self.orign != self.data;
    }

    fn publication_done(&mut self) {
        self.orign = self.data.clone();
    }

    fn update_field_with_f64(&mut self, _field: &str, _value: f64)
    {

    }

    fn update_field_with_bool(&mut self, _field: &str, _value: bool) -> PlatformFunctionResult
    {
        Ok(())
    }

    fn update_field_with_string(&mut self, _field: &str, _value: &String) {
        
    }

    fn push_byte_stream(&mut self, value: &Vec<u8>) {
        self.data = value.to_vec();
    }

    fn update_field_with_json(&mut self, _field: &str, _value: &serde_json::Value) {
        
    }
}

