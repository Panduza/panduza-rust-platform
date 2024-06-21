use serde_json;
use panduza_core::device::traits::{DeviceActions, Producer};

use super::device::SerialPort;

// use super::Ka3005;

pub struct DeviceProducer;

impl Producer for DeviceProducer {

    // fn manufacturer(&self) -> String {
    //     return "korad".to_string();
    // }
    // fn model(&self) -> String {
    //     return "KA3005".to_string();
    // }

    fn settings_props(&self) -> serde_json::Value {
        return serde_json::Value::Null;
    //     return json!([
    //         {
    //             "name": "usb_vendor",
    //             "type": "string",
    //             "default": format!("{:04x}", VID)
    //         },
    //         {
    //             "name": "usb_model",
    //             "type": "string",
    //             "default": format!("{:04x}", PID)
    //         },
    //         {
    //             "name": "usb_serial",
    //             "type": "string",
    //             "default": ""
    //         },
    //         {
    //             "name": "serial_port_name",
    //             "type": "string",
    //             "default": ""
    //         }
    //     ]);
    }


    fn produce(&self) -> Result<Box<dyn DeviceActions>, panduza_core::Error> {
        return Ok(Box::new(SerialPort{}));
    }

}

