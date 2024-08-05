



#[derive(Clone, Debug)]
pub struct A3 {
    // Attribute Name
    name: String,

    payload: Vec<u8>
}

impl A3 {

    pub fn new<A: Into<String>>(name: A) -> A3 {
        A3 {
            name: name.into(),
            payload: vec![]
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }
    
    pub fn to_vec(&self) -> &Vec<u8> {
        &self.payload
    }

}



