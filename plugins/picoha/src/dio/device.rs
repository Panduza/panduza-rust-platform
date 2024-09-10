use std::fmt::format;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use panduza_platform_connectors::serial::slip::get as get_connector;
use panduza_platform_connectors::serial::slip::Connector;

use panduza_platform_connectors::SerialSettings;
use panduza_platform_connectors::UsbSettings;
use panduza_platform_core::Interface;
use panduza_platform_core::StringCodec;
use panduza_platform_core::StringListCodec;
use panduza_platform_core::{Device, DeviceOperations, Error};
use prost::Message;
use tokio::time::sleep;

use crate::dio::api_dio::PicohaDioAnswer;

use super::api_dio::PicohaDioRequest;
use super::api_dio::RequestType;

static PICOHA_VENDOR_ID: u16 = 0x16c0;
static PICOHA_PRODUCT_ID: u16 = 0x05e1;
static PICOHA_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

///
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
        logger.info(format!("JSON settings: {}", json_settings));

        // Usb settings
        let usb_settings = UsbSettings::new()
            .set_vendor(PICOHA_VENDOR_ID)
            .set_model(PICOHA_PRODUCT_ID)
            .optional_set_serial_from_json_settings(&json_settings);
        logger.info(format!("USB settings: {}", usb_settings));

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

    ///
    ///
    ///
    pub async fn pico_get_direction(&self) -> Result<(), Error> {
        //
        let mut request = PicohaDioRequest::default();
        request.set_type(RequestType::GetPinDirection);
        request.pin_num = 2;

        //
        let answer_buffer = &mut [0u8; 1024];
        let size = self
            .connector
            .as_ref()
            .ok_or(Error::BadSettings(
                "Connector is not initialized".to_string(),
            ))?
            .lock()
            .await
            .write_then_read(&request.encode_to_vec(), answer_buffer)
            .await?;

        // Decode the answer
        let answer_slice = answer_buffer[..size].as_ref();
        println!("Received {} bytes -> {:?}", size, answer_slice);
        let answer = PicohaDioAnswer::decode(answer_slice).unwrap();

        println!("{:?}", answer);

        Ok(())
    }

    ///
    ///
    ///
    pub async fn create_io_interface_enum_direction(
        &mut self,
        mut device: Device,
        mut parent_interface: Interface,
    ) -> Result<(), Error> {
        //
        // Create interface direction
        let mut direction = parent_interface.create_interface("direction").finish();

        // meta : enum ?

        let choices = direction
            .create_attribute("choices")
            .message()
            .with_wo_access()
            .finish_with_codec::<StringListCodec>()
            .await;

        // choices.set(["input", "output"]);

        let value = direction
            .create_attribute("value")
            .message()
            .with_wo_access()
            .finish_with_codec::<StringCodec>()
            .await;

        // read a first time here then only set when a new value arrive
        value.set("input").await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn create_io_interface_enum_value(
        &mut self,
        mut device: Device,
        mut parent_interface: Interface,
    ) -> Result<(), Error> {
        // io_%d/value           (enum/string) set/get (when input cannot be set)

        Ok(())
    }

    ///
    /// Create io interfaces
    ///
    pub async fn create_io_interface(
        &mut self,
        mut device: Device,
        mut parent_interface: Interface,
    ) -> Result<(), Error> {
        //
        //
        // io_%d/direction              meta : enum
        // io_%d/direction/choices      list of string
        // io_%d/direction/value        string
        // io_%d/value           (enum/string) set/get (when input cannot be set)
        // io_%d/trigger_read    (boolean) start an input reading (oneshot)

        Ok(())
    }

    ///
    /// Create io interfaces
    ///
    pub async fn create_io_interfaces(&mut self, mut device: Device) -> Result<(), Error> {
        // Get the device logger
        let logger = device.logger.clone();

        //
        // Register interface
        let mut interface = device.create_interface("io").finish();

        //
        //
        // let mut array = Vec::new();
        for n in 0..5 {
            // Debug log
            logger.debug(format!("Create io_{}", n));

            //
            self.create_io_interface(device.clone(), interface.clone())
                .await?;

            // let a = interface
            //     .create_attribute(format!("{}", n))
            //     .message()
            //     .with_wo_access()
            //     .finish_with_codec::<NumberCodec>()
            //     .await;

            // array.push(a);
        }
        // self.array = Arc::new(array);

        Ok(())
    }
}

#[async_trait]
impl DeviceOperations for PicoHaDioDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        self.prepare_settings(device.clone()).await?;
        self.mount_connector().await?;

        // une interface pour chaque io_%d
        //
        // io_%d/direction              meta : enum
        // io_%d/direction/choices      list of string
        // io_%d/direction/value        string
        // io_%d/value           (enum/string) set/get (when input cannot be set)
        // io_%d/trigger_read    (boolean) start an input reading (oneshot)
        //

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
