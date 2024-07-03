use panduza_core::device::traits::DeviceActions;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use super::itf_digital_input;

use panduza_connectors::serial::tty;


pub struct PicoHaDio;

impl DeviceActions for PicoHaDio {

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value)
        -> Result<Vec<InterfaceBuilder>, panduza_core::Error>
    {

        // println!("Ka3005::interface_builders");
        // println!("{}", device_settings);

        let mut serial_conf = tty::Config::new();
        serial_conf.import_from_json_settings(device_settings);

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



