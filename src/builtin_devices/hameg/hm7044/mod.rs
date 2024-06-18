use serde_json::json;

use crate::platform::PlatformError;
use crate::device::{ traits::DeviceActions, traits::Producer };

use crate::interface::builder::Builder as InterfaceBuilder;

use panduza_connectors::serial::tty::Config as SerialConfig;

mod itf_bpc;

use itf_bpc::Hm7044Channel;

struct Hm7044;

impl DeviceActions for Hm7044 {

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value)
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {

        println!("hm7044::interface_builders");
        println!("{}", device_settings);

        let mut serial_conf = SerialConfig::new();
        serial_conf.import_from_json_settings(device_settings);

        serial_conf.serial_baudrate = Some(9600);

        let mut list = Vec::new();
        list.push(
            itf_bpc::build("channel1", &serial_conf, Hm7044Channel::Channel1)
        );
        list.push(
            itf_bpc::build("channel2", &serial_conf, Hm7044Channel::Channel2)
        );
        list.push(
            itf_bpc::build("channel3", &serial_conf, Hm7044Channel::Channel3)
        );
        list.push(
            itf_bpc::build("channel4", &serial_conf, Hm7044Channel::Channel4)
        );
        return Ok(list);
    }
}

pub struct DeviceProducer;

impl Producer for DeviceProducer {

    // fn manufacturer(&self) -> String {
    //     return "hameg".to_string();
    // }
    // fn model(&self) -> String {
    //     return "hm7044".to_string();
    // }

    fn settings_props(&self) -> serde_json::Value {
        return json!([
            {
                "name": "usb_vendor",
                "type": "string",
                "default": ""
            },
            {
                "name": "usb_model",
                "type": "string",
                "default": ""
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
        return Ok(Box::new(Hm7044{}));
    }

}

