use panduza_core::device::traits::DeviceActions;
use panduza_core::device::Device;
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use serde_json::de;

use super::itf_digital_input;

use panduza_connectors::serial::tty;


pub struct PicoHaDio;



// 0x16c0, 0x05E1

impl DeviceActions for PicoHaDio {

    /// Create the interfaces
    fn interface_builders(&self, device: &Device)
        -> Result<Vec<InterfaceBuilder>, panduza_core::Error>
    {

        let device_settings = device.settings.clone();

        // println!("Ka3005::interface_builders");
        // println!("{}", device_settings);

        let mut serial_conf = tty::Config::new();
        serial_conf.import_from_json_settings(&device_settings);

        serial_conf.serial_baudrate = Some(9600);
        serial_conf.usb_vendor = Some(0x16c0);
        serial_conf.usb_model = Some(0x05E1);

        // const_settings = {
        //     "usb_vendor": '0416',
        //     "usb_model": '5011',
        //     "serial_baudrate": 9600
        // }

        // serial_conf.serial_baudrate = Some(9600);

        let mut list = Vec::new();
        list.push(
            itf_digital_input::Builder::new()
                .with_name("io0")
                .with_serial_config(serial_conf.clone())
                .build()
        );
        return Ok(list);
    }
}



