use async_trait::async_trait;
use serde_json::json;

use crate::platform::PlatformError;
use crate::device::{ traits::DeviceActions, traits::Producer, traits::Hunter };

use crate::interface::builder::Builder as InterfaceBuilder;


use crate::connector::serial::tty::Config as SerialConfig;

use tokio_serial;

mod itf_bpc;



static VID: u16 = 0x0416;
static PID: u16 = 0x5011;

pub struct DeviceHunter;


#[async_trait]
impl Hunter for DeviceHunter {

    async fn hunt(&self) -> Option<Vec<serde_json::Value>> {

        let mut bag = Vec::new();

        println!("DeviceHunter::hunt");

        let ports = tokio_serial::available_ports();
        for port in ports.unwrap() {
            println!("{:?}", port);

            match port.port_type {
                tokio_serial::SerialPortType::UsbPort(info) => {
                    if info.vid == VID && info.pid == PID {
                        println!("Found device");

                        // "settings" {
                        //     "usb_vendor": format!("{:04x}", info.vid),
                        //     "usb_model": format!("{:04x}", info.pid),
                        // }
                        bag.push(json!(
                            {

                            }
                        ))
                    }
                },
                _ => {}
            }
        }

        if bag.is_empty() {
            return None;
        }
        else {
            return Some(bag);
        }
    }

}

struct Ka3005;

impl DeviceActions for Ka3005 {

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {

        println!("Ka3005::interface_builders");
        println!("{}", device_settings);

        let mut serial_conf = SerialConfig::new();
        serial_conf.import_from_json_settings(device_settings);

        // const_settings = {
        //     "usb_vendor": '0416',
        //     "usb_model": '5011',
        //     "serial_baudrate": 9600
        // }

        serial_conf.serial_baudrate = Some(9600);

        let mut list = Vec::new();
        list.push(
            itf_bpc::build("channel", &serial_conf)
        );
        return Ok(list);
    }
}




pub struct DeviceProducer;

impl Producer for DeviceProducer {

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(Ka3005{}));
    }

}

