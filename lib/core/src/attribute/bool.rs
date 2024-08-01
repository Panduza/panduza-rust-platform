use crate::attribute::{Attribute, AttributeType};

impl Attribute for AttributeBool {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> super::AttributeType {
        AttributeType::Bool
    }
}

pub struct AttributeBool {
    name: String,
    value: bool
}

pub struct AttributeBoolBuilder {
    name: String,
    value: bool
}

impl AttributeBoolBuilder {
    pub fn new() -> AttributeBoolBuilder {
        AttributeBoolBuilder {
            name: String::new(),
            value: false
        }
    }

    pub fn with_name(mut self, name: &str) -> AttributeBoolBuilder {
        self.name = name.to_string();
        self
    }

    pub fn with_value(mut self, value: bool) -> AttributeBoolBuilder{
        self.value = value;
        self
    }

    pub fn build(&self) -> AttributeBool {
        AttributeBool {
            name: self.name.clone(),
            value: self.value
        }
    }
}

impl AttributeBool {
    fn set_value(&mut self, value: bool) {
        self.value = value
    }

    fn get_value(&self) -> bool {
        self.value
    }
}
