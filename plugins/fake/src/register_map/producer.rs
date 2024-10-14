use super::device::RegisterMapDevice;
use panduza_platform_core::{DeviceOperations, Producer};

pub struct RegisterMapProducer {}

impl RegisterMapProducer {
    pub fn new() -> Box<RegisterMapProducer> {
        Box::new(RegisterMapProducer {})
    }
}

impl Producer for RegisterMapProducer {
    fn manufacturer(&self) -> String {
        "panduza".to_string()
    }

    fn model(&self) -> String {
        "fake_register_map".to_string()
    }

    fn produce(&self) -> Result<Box<dyn DeviceOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(RegisterMapDevice::new()));
    }
}

// impl Into<Box<RegisterMapProducer>> for RegisterMapProducer {
//     fn into(self) -> Box<RegisterMapProducer> {
//         Box::new(self)
//     }
// }

// use serde_json;
// use serde_json::json;
// use panduza_core::device::traits::{DeviceActions, Producer};

// use super::device::RegisterMap;

// pub struct DeviceProducer;

// impl DeviceProducer {
//     pub fn new_boxed() -> Box<dyn Producer> {
//         return Box::new(DeviceProducer{});
//     }
// }

// impl Producer for DeviceProducer {

//     // fn manufacturer(&self) -> String {
//     //     return "korad".to_string();
//     // }
//     // fn model(&self) -> String {
//     //     return "KA3005".to_string();
//     // }

//     fn settings_props(&self) -> serde_json::Value {
//         return json!([
//             {
//                 "name": "usb_vendor",
//                 "type": "string",
//                 "default": ""
//             },
//             {
//                 "name": "usb_model",
//                 "type": "string",
//                 "default": ""
//             },
//             {
//                 "name": "usb_serial",
//                 "type": "string",
//                 "default": ""
//             },
//             {
//                 "name": "serial_port_name",
//                 "type": "string",
//                 "default": ""
//             }
//         ]);
//     }

//     fn produce(&self) -> Result<Box<dyn DeviceActions>, panduza_core::Error> {
//         return Ok(Box::new(RegisterMap{}));
//     }

// }
