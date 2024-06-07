// use async_trait::async_trait;
use serde_json::json;

use crate::platform::PlatformError;
use crate::device::{ traits::DeviceActions, traits::Producer };//, traits::Hunter };

use crate::interface::builder::Builder as InterfaceBuilder;


use crate::connector::serial::tty::Config as SerialConfig;

// use tokio_serial;

mod itf_pm100a_powermeter;



static VID: u16 = 0x1313;
static PID: u16 = 0x8079;

// pub struct DeviceHunter;


// #[async_trait]
// impl Hunter for DeviceHunter {

//     async fn hunt(&self) -> Option<Vec<serde_json::Value>> {

//         let mut bag = Vec::new();

//         println!("DeviceHunter::hunt");

//         let ports = tokio_serial::available_ports();
//         for port in ports.unwrap() {
//             println!("{:?}", port);

//             match port.port_type {
//                 tokio_serial::SerialPortType::UsbPort(info) => {
//                     if info.vid == VID && info.pid == PID {
//                         println!("Found device");

//                         bag.push(json!(
//                             {
//                                 "name": "Korad KA3005",
//                                 "ref": "korad.ka3005",
//                                 "settings": {
//                                     "usb_vendor": format!("{:04x}", info.vid),
//                                     "usb_model": format!("{:04x}", info.pid),
//                                     "usb_serial": info.serial_number,
//                                 }
//                             }
//                         ))
//                     }
//                 },
//                 _ => {}
//             }
//         }

//         if bag.is_empty() {
//             return None;
//         }
//         else {
//             return Some(bag);
//         }
//     }

// }

struct PM100A;

impl DeviceActions for PM100A {

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {

        println!("PM100A::interface_builders");
        println!("{}", device_settings);

        let mut serial_conf = SerialConfig::new();
        serial_conf.import_from_json_settings(device_settings);

        // serial_conf.serial_baudrate = Some(9600);

        let mut list = Vec::new();
        list.push(
            // itf_pm100a_powermeter::build("channel", &serial_conf)
            itf_pm100a_powermeter::build("channel")
        );
        return Ok(list);
    }
}




pub struct DeviceProducer;

impl Producer for DeviceProducer {

    fn settings_props(&self) -> serde_json::Value {
        return json!([
            {
                "name": "usb_vendor",
                "type": "string",
                "default": format!("{:04x}", VID)
            },
            {
                "name": "usb_model",
                "type": "string",
                "default": format!("{:04x}", PID)
            },
            {
                "name": "usb_serial",
                "type": "string",
                "default": ""
            }
            // {
            //     "name": "serial_port_name",
            //     "type": "string",
            //     "default": ""
            // }
        ]);
    }


    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(PM100A{}));
    }

}

