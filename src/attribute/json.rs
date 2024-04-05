use std::any::TypeId;

use serde_json::json;

use super::AttributeInterface;


pub struct JsonAttribute {
    // 
    name: String,

    retain: bool,
    
    // 
    data: serde_json::Value,
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
        };
    }

    // pub fn update_field(&mut self, field: &str, value: serde_json::Value) {

        
    // }

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
    
    fn from_mqtt_payload(&mut self, payload: &str) {
        todo!()
    }

    fn update_field<F: Into<String>, V: 'static>(&mut self, field: &F, value: &V) {
        if TypeId::of::<V>() == TypeId::of::<String>() {
            println!("V is a String");

            let n = self.name.clone();
            let d = self.data.get_mut(n);
            if d.is_none() {
                return;
            }
            // d.unwrap().as_object_mut().unwrap().insert(field.into(), 
            //     value
            // );

        }
    }
}

