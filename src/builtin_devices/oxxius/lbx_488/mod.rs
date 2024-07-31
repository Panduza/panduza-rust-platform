use async_trait::async_trait;
use panduza_core::device::Device;
use serde_json::json;

use panduza_core::device::traits::DeviceActions;
use panduza_core::device::traits::Hunter;
use panduza_core::device::traits::Producer;
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use panduza_core::platform_error;
use panduza_core::Error as PlatformError;

use panduza_connectors::UsbSettings as UsbConfig;
// use panduza_connectors::usb::usb::Config as UsbConfig;

mod itf_lbx_488_blc;

static VID: u16 = 0x0403;
static PID: u16 = 0x90d9;

pub struct DeviceHunter;

#[async_trait]
impl Hunter for DeviceHunter {
    async fn hunt(&self) -> Option<Vec<serde_json::Value>> {
        let mut bag = Vec::new();

        // println!("DeviceHunter::hunt Oxxius");

        // usb type device
        let option_device_info = nusb::list_devices()
            .unwrap()
            .find(|d| d.vendor_id() == VID && d.product_id() == PID);

        match option_device_info {
            Some(device_info) => {
                // println!("Found device : Oxxius");

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
            }
            None => {
                println!("Oxxius not connected");
            }
        }

        if bag.is_empty() {
            return None;
        } else {
            return Some(bag);
        }
    }
}

struct LBX488;

impl DeviceActions for LBX488 {
    /// Create the interfaces
    fn interface_builders(&self, device: &Device) -> Result<Vec<InterfaceBuilder>, PlatformError> {
        let device_settings = device.settings.clone();

        // println!("S0501::interface_builders");
        // println!("{}", device_settings);

        let usb_conf = UsbConfig::new()
            .set_vendor(VID)
            .set_model(PID)
            .set_serial_from_json_settings(&device_settings)
            .map_err(|e| platform_error!("Unable to get usb configuration: {}", e))?;
        // let mut usb_conf = UsbConfig::new();
        // usb_conf.import_from_json_settings(&device_settings)?;

        let mut list = Vec::new();
        list.push(itf_lbx_488_blc::build("blc", &usb_conf));
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
        ]);
    }

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(LBX488 {}));
    }
}
