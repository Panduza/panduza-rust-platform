use async_trait::async_trait;
use panduza_core::device::Device;
use serde_json::json;

use panduza_core::Error as PlatformError;
use panduza_core::device::{ traits::DeviceActions, traits::Producer, traits::Hunter };
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use panduza_connectors::usb::usbtmc::Config as UsbtmcConfig;

mod itf_pm100a_powermeter;


static VID: u16 = 0x1313;
static PID: u16 = 0x8079;
// serial number = P1006194

pub struct DeviceHunter;


#[async_trait]
impl Hunter for DeviceHunter {

    async fn hunt(&self) -> Option<Vec<serde_json::Value>> {

        let mut bag = Vec::new();

        // println!("DeviceHunter::hunt");

        // usb type device
        let option_device_info = nusb::list_devices()
            .unwrap()
            .find(|d| d.vendor_id() == VID && d.product_id() == PID);
        
        match option_device_info {
            Some(device_info) => {
                println!("Found device : Thorlab");

                bag.push(json!(
                    {
                        "name": "Thorlabs PM100A",
                        "ref": "thorlabs.pm100a",
                        "settings": {
                            "usb_vendor": format!("{:04x}", device_info.vendor_id()),
                            "usb_model": format!("{:04x}", device_info.product_id()),
                            "usb_serial": device_info.serial_number(),
                        }
                    }
                ))
            },
            None => {
                println!("Thorlab not connected");
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

struct PM100A;

impl DeviceActions for PM100A {

    /// Create the interfaces
    fn interface_builders(&self, device: &Device) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let device_settings = device.settings.clone();

        println!("PM100A::interface_builders");
        println!("{}", device_settings);

        let mut serial_conf = UsbtmcConfig::new();
        serial_conf.import_from_json_settings(&device_settings);

        // serial_conf.serial_baudrate = Some(9600);

        let mut list = Vec::new();
        list.push(
            itf_pm100a_powermeter::build("channel", &serial_conf)
            // itf_pm100a_powermeter::build("channel")
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

