

use super::Attribute;



#[derive(Clone, Debug)]
pub struct A3 {
    // Attribute Name
    name: String,

}

impl A3 {

    pub fn new<A: Into<String>>(name: A) -> A3 {
        A3 {
            name: name.into()
        }
    }


    pub fn name(&self) -> String {
        self.name.clone()
    }


}

