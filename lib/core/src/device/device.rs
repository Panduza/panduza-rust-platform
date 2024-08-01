use serde_json;

use crate::interface::Interface;

struct InterfaceManager {
    interfaces: Vec<Interface>,
}

impl InterfaceManager {
    pub fn new() -> InterfaceManager {
        InterfaceManager {
            interfaces: Vec::new(),
        }
    }

    pub fn add_interface(&mut self, interface: Interface) {
        self.interfaces.push(interface);
    }
}

pub struct DeviceBuilder {
    name: Option<String>,
    model: String,
    manufacturer: String,

    interface_manager: InterfaceManager,
    attribute_manager: AttributeManager,
}

impl DeviceBuilder {
    pub fn new() -> DeviceBuilder {
        DeviceBuilder {
            name: None,
            model: "Unknown".to_string(), 
            manufacturer: "Unknown".to_string(),
        }
    }

    pub fn with_name(mut self, name: String) -> DeviceBuilder {
        self.name = Some(name);
        self
    }

    pub fn with_model(mut self, model: String) -> DeviceBuilder {
        self.model = model;
        self
    }

    pub fn with_manufacturer(mut self, manufacturer: String) -> DeviceBuilder {
        self.manufacturer = manufacturer;
        self
    }

    pub fn build(&self) -> Result<Device, &'static str> {
        if let Some(name) = &self.name {
            Ok(Device {
                name: name.clone(),
                model: self.model.clone(),
                manufacturer: self.manufacturer.clone(),
                device_settings: serde_json::Value::Null,
                connector_settings: serde_json::Value::Null,
            })
        } else {
            Err("Device name is required")
        }
    }
}

/// A device manage a set of interfaces
/// 
pub struct Device {

    /// Device name
    name: String,
    model: String,
    manufacturer: String,

    device_settings: serde_json::Value,
    connector_settings: serde_json::Value,
}
