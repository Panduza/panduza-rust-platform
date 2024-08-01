pub mod fsm;
pub mod basic;

pub mod logger;
pub mod builder;
pub mod listener;
pub mod subscriber;

pub struct InterfaceBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl InterfaceBuilder {
    pub fn new() -> InterfaceBuilder{
        InterfaceBuilder {
            name: None,
            description: None
        }
    }

    pub fn with_name(mut self, name: &str) -> InterfaceBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_description(mut self, description: &str) -> InterfaceBuilder {
        self.description = Some(description.to_string());
        self
    }

    pub fn build(self) -> Result<Interface, &'static str> {
        if let Some(name) = self.name {
            Ok(Interface {
                name,
                description: self.description
            })
        } else {
            Err("Interface name is required")
        }
    }
}

pub struct Interface {
    name: String,
    description: Option<String>,
}


impl Interface {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
