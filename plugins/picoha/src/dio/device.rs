use std::fmt::format;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use panduza_platform_connectors::serial::slip::get as get_connector;
use panduza_platform_connectors::serial::slip::Connector;

use panduza_platform_connectors::SerialSettings;
use panduza_platform_connectors::UsbSettings;
use panduza_platform_core::spawn_on_command;
use panduza_platform_core::BidirMsgAtt;
use panduza_platform_core::DeviceLogger;
use panduza_platform_core::Interface;
use panduza_platform_core::StringCodec;
use panduza_platform_core::StringListCodec;
use panduza_platform_core::TaskResult;
use panduza_platform_core::{Device, DeviceOperations, Error};
use prost::Message;
use tokio::time::sleep;

use super::connector::PicoHaDioConnector;

use crate::dio::api_dio::PicohaDioAnswer;
use crate::dio::api_dio::PinValue;

use super::api_dio::PicohaDioRequest;
use super::api_dio::RequestType;

static PICOHA_VENDOR_ID: u16 = 0x16c0;
static PICOHA_PRODUCT_ID: u16 = 0x05e1;
static PICOHA_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

///
/// Device to control PicoHA Dio Board
///
pub struct PicoHaDioDevice {
    ///
    /// Device logger
    logger: Option<DeviceLogger>,

    ///
    /// Serial settings to connect to the pico
    serial_settings: Option<SerialSettings>,

    ///
    /// Connector to communicate with the pico
    connector: Option<Connector>,

    ///
    ///
    pico_connector: Option<PicoHaDioConnector>,
}

impl PicoHaDioDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        PicoHaDioDevice {
            logger: None,
            serial_settings: None,
            connector: None,
            pico_connector: None,
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

        //
        self.pico_connector = Some(PicoHaDioConnector::new(
            self.logger.as_ref().unwrap().clone(),
            self.connector.as_ref().unwrap().clone(),
        ));

        Ok(())
    }

    ///
    ///
    ///
    async fn on_direction_value_change_action(
        logger: DeviceLogger,
        connector: PicoHaDioConnector,
        mut direction_value_attr: BidirMsgAtt<StringCodec>,
        pin_num: u32,
    ) -> TaskResult {
        while let Some(command) = direction_value_attr.pop_cmd().await {
            logger.debug(format!("set direction command {:?}", command));

            if command.value == "input".to_string() {
                connector.pico_set_direction(pin_num, command.value).await?;
            } else if command.value == "output".to_string() {
                connector.pico_set_direction(pin_num, command.value).await?;
            }

            let read_direction = connector.pico_get_direction(pin_num).await?;

            direction_value_attr.set(read_direction).await?;
        }
        Ok(())
    }

    ///
    ///
    ///
    pub async fn create_io_interface_enum_direction(
        &mut self,
        mut device: Device,
        mut parent_interface: Interface,
        pin_num: u32,
    ) -> Result<(), Error> {
        //
        // Create interface direction
        let mut direction = parent_interface.create_interface("direction").finish();

        // meta : enum ?

        println!("creation direction");

        let choices = direction
            .create_attribute("choices")
            .message()
            .with_wo_access()
            .finish_with_codec::<StringListCodec>()
            .await;

        choices
            .set(vec!["input".to_string(), "output".to_string()])
            .await?;

        let mut value = direction
            .create_attribute("value")
            .message()
            .with_bidir_access()
            .finish_with_codec::<StringCodec>()
            .await;

        //
        // Execute action on each command received
        let logger = self.logger.as_ref().unwrap().clone();
        let value_attr = value.clone();
        let connector = self.pico_connector.as_ref().unwrap().clone();
        spawn_on_command!(
            device,
            value_attr,
            Self::on_direction_value_change_action(
                logger.clone(),
                connector.clone(),
                value_attr.clone(),
                pin_num
            )
        );

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
        pin_num: u32,
    ) -> Result<(), Error> {
        // Register interface
        let mut io_interface = parent_interface
            .create_interface(format!("{}", pin_num))
            .finish();

        //
        self.create_io_interface_enum_direction(device, io_interface, pin_num)
            .await?;

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
            self.create_io_interface(device.clone(), interface.clone(), n)
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
        //
        // Init logger
        self.logger = Some(device.logger.clone());

        self.prepare_settings(device.clone()).await?;
        self.mount_connector().await?;

        self.create_io_interfaces(device.clone()).await?;

        // self.pico_get_direction(2).await?;

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
