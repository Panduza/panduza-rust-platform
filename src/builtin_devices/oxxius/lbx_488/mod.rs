use async_trait::async_trait;
use serde_json::json;

use panduza_core::Error as PlatformError;
use panduza_core::device::{ traits::DeviceActions, traits::Producer, traits::Hunter };

use panduza_core::interface::builder::Builder as InterfaceBuilder;


// use panduza_connectors::serial::tty::Config as SerialConfig;
use panduza_connectors::usb::usb::Config as UsbConfig;

mod itf_lbx_488_blc;



static VID: u16 = 0x0403;
static PID: u16 = 0x90d9;

pub struct DeviceHunter;


#[async_trait]
impl Hunter for DeviceHunter {

    async fn hunt(&self) -> Option<Vec<serde_json::Value>> {

        let mut bag = Vec::new();

        println!("DeviceHunter::hunt Oxxius");

       // usb type device
       let option_device_info = nusb::list_devices()
        .unwrap()
        .find(|d| d.vendor_id() == VID && d.product_id() == PID);
    
        match option_device_info {
            Some(device_info) => {
                println!("Found device : Oxxius");

                bag.push(json!(
                    {
                        "name": "Oxxius LBX_488",
                        "ref": "oxxius.lbx_488",
                        "settings": {
                            "usb_vendor": format!("{:04x}", device_info.vendor_id()),
                            "usb_model": format!("{:04x}", device_info.product_id()),
                            "usb_serial": device_info.serial_number(),
                        }
                    }
                ))
            },
            None => {
                println!("Oxxius not connected");
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

struct LBX488;

impl DeviceActions for LBX488 {

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {

        println!("S0501::interface_builders");
        println!("{}", device_settings);

        let mut serial_conf = UsbConfig::new();
        serial_conf.import_from_json_settings(device_settings);

        // serial_conf.serial_baudrate = Some(9600);

        let mut list = Vec::new();
        list.push(
            itf_lbx_488_blc::build("blc", &serial_conf)
        );
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
            }
            // {
            //     "name": "serial_port_name",
            //     "type": "string",
            //     "default": ""
            // }
        ]);
    }


    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(LBX488{}));
    }

}

