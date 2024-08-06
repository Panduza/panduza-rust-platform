use serde_json::json;

pub type DeviceSettings = serde_json::Value;

pub struct ProductionOrder {
    /// Name of the device to be produced
    pub device_name: String,

    /// Reference of driver device producer
    pub device_ref: String,

    ///
    pub device_settings: DeviceSettings,
}

impl ProductionOrder {
    /// Constructor
    ///
    pub fn new<A: Into<String>, B: Into<String>>(d_ref: A, d_name: B) -> ProductionOrder {
        ProductionOrder {
            device_name: d_name.into(),
            device_ref: d_ref.into(),
            device_settings: serde_json::Value::Null,
        }
    }

    /// From a json value
    ///
    pub fn from_json(value: &serde_json::Value) -> ProductionOrder {
        ProductionOrder {
            device_name: "test".to_string(),
            device_ref: "rtok".to_string(),
            device_settings: json!({}),
        }
    }

    pub fn device_ref(&self) -> &String {
        &self.device_ref
    }
}
