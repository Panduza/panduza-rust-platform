
use serde_json::json;

use super::AttributeInterface;
// use super::FieldF64;

pub struct JsonAttribute {
    // 
    name: String,

    retain: bool,

    // 
    data: serde_json::Value,

    // 
    orign: serde_json::Value,
}

impl JsonAttribute {
    pub fn new<A: Into<String>>(name: A, retain: bool) -> JsonAttribute {

        let name_str = name.into();

        let data = json!({
            name_str.clone(): {}
        });

        return JsonAttribute {
            name: name_str,
            retain: retain,
            data: data,
            orign: serde_json::Value::Null,
        };
    }

    /// Create a new instance of the object inside a box
    ///
    #[inline]
    pub fn new_boxed<A: Into<String>>(name: A, retain: bool)
        -> Box<dyn AttributeInterface>
    {
        return Box::new(JsonAttribute::new(name, retain));
    }
}


impl AttributeInterface for JsonAttribute {

    fn name(&self) -> &String {
        return &self.name;
    }
    
    fn retain(&self) -> &bool {
        return &self.retain;
    }
    
    fn to_mqtt_payload(&self) -> String {
        return self.data.to_string();
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

    fn update_field_with_f64(&mut self, field: &str, value: f64)
    {
        let n = self.name.clone();
        let d = self.data.get_mut(n);
        if d.is_none() {
            return;
        }
        let value_as_number = serde_json::Number::from_f64(value as f64).unwrap();
        d.unwrap().as_object_mut().unwrap().insert(field.into(), 
            serde_json::Value::Number(value_as_number)
        );
    }

    fn update_field_with_bool(&mut self, field: &str, value: bool)
    {
        let n = self.name.clone();
        let d = self.data.get_mut(n);
        if d.is_none() {
            return;
        }
        d.unwrap().as_object_mut().unwrap().insert(field.into(), 
            serde_json::Value::Bool(value)
        );
    }

    fn update_field_with_string(&mut self, field: &str, value: &String) {
        let n = self.name.clone();
        let d = self.data.get_mut(n);
        if d.is_none() {
            return;
        }
        d.unwrap().as_object_mut().unwrap().insert(field.into(), 
            serde_json::Value::String(value.into())
        );
    }

    fn update_field_with_json(&mut self, field: &str, value: &serde_json::Value) {
        let n = self.name.clone();
        let d = self.data.get_mut(n);
        if d.is_none() {
            return;
        }
        d.unwrap().as_object_mut().unwrap().insert(field.into(), value.clone());
    }

}

