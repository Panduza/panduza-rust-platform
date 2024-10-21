use std::collections::HashMap;

use super::InfoElement;

///
///
///
pub struct InfoElementInstance {
    ///
    ///
    ///
    name: String,

    ///
    ///
    ///
    instances: HashMap<String, InfoElement>,
}

impl InfoElementInstance {
    pub fn new<A: Into<String>>(name: A) -> Self {
        Self {
            name: name.into(),
            instances: HashMap::new(),
        }
    }
}
