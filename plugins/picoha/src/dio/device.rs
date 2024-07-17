use panduza_core::device::traits::DeviceActions;
use panduza_core::device::Device;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use super::itf_digital_input;

use panduza_connectors::UsbSettings;
use panduza_connectors::SerialSettings;

pub struct PicoHaDio;


static PICOHA_VENDOR_ID: u16 = 0x16C0;
static PICOHA_PRODUCT_ID: u16 = 0x05E1;


impl DeviceActions for PicoHaDio {

    /// Create the interfaces
    fn interface_builders(&self, device: &Device)
        -> Result<Vec<InterfaceBuilder>, panduza_core::Error>
    {
        // Get the device logger
        let logger = device.clone_logger().clone();

        // Get the device settings
        let json_settings = device.settings.clone();

        // Log debug info
        logger.log_info("Build interfaces for \"picoha.dio\" device");
        logger.log_info(format!("settings: {}", json_settings));

        // Usb settings
        let usb_settings = UsbSettings::new()
            .set_vendor(PICOHA_VENDOR_ID)
            .set_model(PICOHA_PRODUCT_ID)
            .optional_set_serial_from_json_settings(&json_settings);

        // Serial settings
        let serial_settings = SerialSettings::new()
            .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)?
            .set_baudrate(9600);

        //
        let mut list = Vec::new();
        list.push(
            itf_digital_input::Builder::new()
                .with_name("io0")
                .with_serial_settings(serial_settings)
                .build()
        );
        return Ok(list);
    }
}



