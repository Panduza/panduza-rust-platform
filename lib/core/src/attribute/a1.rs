use serde_json::json;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct A1 {
    // Attribute Name
    name: String,

    
    retain: bool,

    // 
    data: serde_json::Value,

    // 
    orign: serde_json::Value,


    vec_data: Vec<u8>,
}

impl A1 {

    pub fn new<A: Into<String>>(name: A) -> A1 {
        A1 {
            name: name.into(),
            retain: true,
            data: json!({}),
            orign: serde_json::Value::Null,
            vec_data: vec![],
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    
    pub fn retain(&self) -> &bool {
        return &self.retain;
    }
    

    pub fn update_field<T>(&mut self, field: &str, value: T)
    where
        T: Serialize
    {
        self.data[field] = serde_json::to_value(value).unwrap();
    }
    
    pub fn to_vec(&self) -> &Vec<u8> {
        // &self.payload

        // &self.data.to_string().as_bytes().to_vec()

        &self.vec_data
    }

}



