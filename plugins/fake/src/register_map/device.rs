use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use panduza_platform_core::{
    spawn_on_command, Device, DeviceLogger, DeviceOperations, Error, MemoryCommandCodec,
    NumberCodec, RwMessageAttribute, TaskResult, WoMessageAttribute,
};
use tokio::time::sleep;

///
/// This device is a simulation of a register map that you can access through commands
///
pub struct RegisterMapDevice {
    logger: Option<DeviceLogger>,
    array: Arc<Vec<WoMessageAttribute<NumberCodec>>>,
}

impl RegisterMapDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> RegisterMapDevice {
        RegisterMapDevice {
            logger: None,
            array: Arc::new(Vec::new()),
        }
    }

    ///
    /// Triggered when a new command is received
    ///
    async fn on_command_action(
        logger: DeviceLogger,
        array: Arc<Vec<WoMessageAttribute<NumberCodec>>>,
        attr_command: RwMessageAttribute<MemoryCommandCodec>,
    ) -> TaskResult {
        logger.info("new incoming command");
        let command = attr_command.get().await.unwrap();
        // println!("cooucou {} ", command);

        array[1].set(14).await?;

        Ok(())
    }

    ///
    /// Register map can be updated through memory command
    ///
    async fn create_memory_command_attribute(&mut self, mut device: Device) {
        //
        // Create the attribute
        let attr_command = device
            .create_attribute("command")
            .message()
            .with_rw_access()
            .finish_with_codec::<MemoryCommandCodec>()
            .await;

        //
        // Execute action on each command received
        let logger = self.logger.as_ref().unwrap().clone();
        let array = self.array.clone();
        spawn_on_command!(
            device,
            attr_command,
            Self::on_command_action(logger.clone(), array.clone(), attr_command.clone())
        );
    }

    ///
    ///
    ///
    async fn create_registers(&mut self, mut device: Device) {
        //
        // Get the logger
        self.logger = Some(device.logger.clone());

        //
        // Register interface
        let mut interface = device.create_interface("registers").finish();

        //
        // Create 20 register
        let mut array = Vec::new();
        for n in 1..20 {
            let a = interface
                .create_attribute(format!("{}", n))
                .message()
                .with_wo_access()
                .finish_with_codec::<NumberCodec>()
                .await;
            a.set(2).await.unwrap();
            array.push(a);
        }
        self.array = Arc::new(array);
    }
}

#[async_trait]
impl DeviceOperations for RegisterMapDevice {
    ///
    /// Mount the device
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        //
        // First create registers because command will need them
        self.create_registers(device.clone()).await;
        //
        // Create command
        self.create_memory_command_attribute(device.clone()).await;
        Ok(())
    }

    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}

// use panduza_core::device::traits::DeviceActions;
// use panduza_core::device::Device;
// use panduza_core::interface::builder::Builder as InterfaceBuilder;

// use super::itf_registers;

// pub struct RegisterMap;
// impl DeviceActions for RegisterMap {

//     /// Create the interfaces
//     fn interface_builders(&self, device: &Device)
//         -> Result<Vec<InterfaceBuilder>, panduza_core::Error>
//     {

//         // println!("Ka3005::interface_builders");
//         // println!("{}", device_settings);

//         // let mut serial_conf = SerialConfig::new();
//         // serial_conf.import_from_json_settings(device_settings);

//         // const_settings = {
//         //     "usb_vendor": '0416',
//         //     "usb_model": '5011',
//         //     "serial_baudrate": 9600
//         // }

//         // serial_conf.serial_baudrate = Some(9600);

//         let mut list = Vec::new();
//         list.push(
//             itf_registers::build("map")
//         );
//         return Ok(list);
//     }
// }
