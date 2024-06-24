



#[derive(Clone, Debug)]
pub struct A1 {
    // Attribute Name
    name: String,

    payload: Vec<u8>
}

impl A1 {

    pub fn new<A: Into<String>>(name: A) -> A1 {
        A1 {
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



