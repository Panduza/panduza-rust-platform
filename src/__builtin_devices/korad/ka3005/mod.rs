use async_trait::async_trait;
use panduza_core::device::Device;
use serde_json::json;

use panduza_core::Error as PlatformError;
use panduza_core::device::{ traits::DeviceActions, traits::Producer, traits::Hunter };

use panduza_core::interface::builder::Builder as InterfaceBuilder;


use panduza_connectors::serial::tty2::Config as SerialConfig;

use tokio_serial;

mod itf_bpc;
mod itf_voltmeter;
mod itf_ammeter;


static VID: u16 = 0x0416;
static PID: u16 = 0x5011;
static BAUDRATE: u32 = 115200;

pub struct DeviceHunter;


#[async_trait]
impl Hunter for DeviceHunter {

    async fn hunt(&self) -> Option<Vec<serde_json::Value>> {

        let mut bag = Vec::new();

        // println!("DeviceHunter::hunt : Korad");

        let ports = tokio_serial::available_ports();
        for port in ports.unwrap() {
            // println!("{:?}", port);

            match port.port_type {
                tokio_serial::SerialPortType::UsbPort(info) => {
                    if info.vid == VID && info.pid == PID {
                        println!("Found device : Korad");

                        bag.push(json!(
                            {
                                "name": "Korad KA3005",
                                "ref": "korad.ka3005",
                                "settings": {
                                    "usb_vendor": format!("{:04x}", info.vid),
                                    "usb_model": format!("{:04x}", info.pid),
                                    "usb_serial": info.serial_number,
                                }
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

struct Ka3005p;

impl DeviceActions for Ka3005p {

    /// Create the interfaces
    fn interface_builders(&self, device: &Device) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {

        let device_settings = &device.settings;

        println!("Ka3005p::interface_builders");
        println!("{}", device_settings);

        let mut serial_conf = SerialConfig::new();
        let port = device_settings["serial_port_name"].as_str().unwrap_or_else(|| {
            panic!("serial_port_name is not defined");
        }).to_string();


        serial_conf.fill(SerialConfig {
            serial_port_name: Some(port),
            serial_baudrate: Some(BAUDRATE),
            usb_model: Some(PID),
            usb_vendor: Some(VID),
            time_lock_duration: Some(tokio::time::Duration::from_millis(500)),
            ..Default::default()
        });
        
        let mut list = Vec::new();
        list.push(itf_voltmeter::build("channel_0:_voltmeter", &serial_conf));
        list.push(itf_ammeter::build("channel_0:_ammeter", &serial_conf));
        list.push(itf_bpc::build("channel_0:_control", &serial_conf));
        return Ok(list);
    }
}




pub struct DeviceProducer;

impl Producer for DeviceProducer {

    // fn manufacturer(&self) -> String {
    //     return "korad".to_string();
    // }
    // fn model(&self) -> String {
    //     return "KA3005".to_string();
    // }

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
            },
            {
                "name": "serial_port_name",
                "type": "string",
                "default": ""
            }
        ]);
    }


    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(Ka3005p{}));
    }

}
