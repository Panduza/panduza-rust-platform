use async_trait::async_trait;

use panduza_platform_connectors::serial::slip::get as get_connector;
use panduza_platform_connectors::serial::slip::Connector;

use panduza_platform_connectors::SerialSettings;
use panduza_platform_connectors::UsbSettings;
use panduza_platform_core::{Device, DeviceOperations, Error};

static PICOHA_VENDOR_ID: u16 = 0x16c0;
static PICOHA_PRODUCT_ID: u16 = 0x05e1;
static PICOHA_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

/// Device to control PicoHA Dio Board
///
pub struct PicoHaDioDevice {
    serial_settings: Option<SerialSettings>,
    connector: Option<Connector>,
}

impl PicoHaDioDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        PicoHaDioDevice {
            serial_settings: None,
            connector: None,
        }
    }

    ///
    /// Prepare settings of the device
    ///
    pub async fn prepare_settings(&mut self, device: Device) -> Result<(), Error> {
        // Get the device logger
        let logger = device.logger.clone();

        // Get the device settings
        let json_settings = device.settings().await;

        // Log debug info
        logger.info("Build interfaces for \"picoha.dio\" device");
        logger.info(format!("json_settings: {}", json_settings));

        // Usb settings
        let usb_settings = UsbSettings::new()
            .set_vendor(PICOHA_VENDOR_ID)
            .set_model(PICOHA_PRODUCT_ID)
            .optional_set_serial_from_json_settings(&json_settings);
        logger.info(format!("usb_settings: {}", usb_settings));

        // Serial settings
        self.serial_settings = Some(
            SerialSettings::new()
                .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)?
                .set_baudrate(PICOHA_SERIAL_BAUDRATE),
        );

        Ok(())
    }

    ///
    /// Try to mount the connector to reach the device
    ///
    pub async fn mount_connector(&mut self) -> Result<(), Error> {
        //
        // Recover settings
        let settings = self.serial_settings.as_ref().ok_or(Error::BadSettings(
            "Serial Settings not provided".to_string(),
        ))?;
        //
        // Try to get connector
        self.connector = Some(get_connector(settings).await?);
        //
        // Try to init it
        self.connector
            .as_ref()
            .ok_or(Error::BadSettings(
                "Connector is not initialized".to_string(),
            ))?
            .lock()
            .await
            .init()
            .await?;
        Ok(())
    }
}

#[async_trait]
impl DeviceOperations for PicoHaDioDevice {
    /// Mount device and give him its structure
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        self.prepare_settings(device.clone()).await?;
        // self.mount_connector().await?;
        Ok(())
    }
}
// impl DeviceActions for PicoHaDio {

//     /// Create the interfaces
//     fn interface_builders(&self, device: &Device)
//         -> Result<Vec<InterfaceBuilder>, panduza_core::Error>
//     {
//         // Get the device logger
//         let logger = device.clone_logger().clone();

//         // Get the device settings
//         let device_settings = device.settings.clone();

//         // Log debug info
//         logger.log_info("Build interfaces for \"picoha.dio\" device");
//         logger.log_info(format!("settings: {}", device_settings));

//         let mut serial_conf = SerialConfig::new();
//         serial_conf.import_from_json_settings(&device_settings)?;

//         serial_conf.serial_baudrate = Some(9600);
//         serial_conf.usb_vendor = Some(PICOHA_VENDOR_ID);
//         serial_conf.usb_model = Some(PICOHA_PRODUCT_ID);

//         // const_settings = {
//         //     "usb_vendor": '0416',
//         //     "usb_model": '5011',
//         //     "serial_baudrate": 9600
//         // }

//         // serial_conf.serial_baudrate = Some(9600);

//         let mut list = Vec::new();
//         list.push(
//             itf_digital_input::Builder::new()
//                 .with_name("io0")
//                 .with_serial_config(serial_conf.clone())
//                 .build()
//         );
//         return Ok(list);
//     }
// }
