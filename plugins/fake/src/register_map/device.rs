use std::time::Duration;

use async_trait::async_trait;
use panduza_platform_core::{
    spawn_on_command, Device, DeviceOperations, Error, MemoryCommandCodec, RwMessageAttribute,
    TaskResult, UIntergerCodec, WoMessageAttribute,
};
use tokio::time::sleep;

///
/// This device is a simulation of a register map that you can access through commands
///
pub struct RegisterMapDevice {
    array: Vec<WoMessageAttribute<UIntergerCodec>>,
}

impl RegisterMapDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> RegisterMapDevice {
        RegisterMapDevice { array: Vec::new() }
    }

    ///
    /// Triggered when a new command is received
    ///
    async fn on_command_action(attr_command: RwMessageAttribute<MemoryCommandCodec>) -> TaskResult {
        println!("cooucou");
        let _dat = attr_command.get().await.unwrap();
        println!("cooucou {} ", _dat);
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
        spawn_on_command!(
            device,
            attr_command,
            Self::on_command_action(attr_command.clone())
        );
    }
}

#[async_trait]
impl DeviceOperations for RegisterMapDevice {
    /// Mount the device
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        // commands [json Codec] (Ro)
        // N topic avec 1 valeur de registre [int or array codec] (Wo -> write only)

        // //
        // device.logger.info("pooook");

        // let mut interface = device
        //     .create_interface("pok")
        //     .with_tags("examples;tests")
        //     .finish();

        // for n in 1..20 {
        //     let a = interface
        //         .create_attribute(format!("cell_{}", n))
        //         .message()
        //         .with_wo_access()
        //         .finish_with_codec::<UIntergerCodec>()
        //         .await;
        //     a.set(2).await.unwrap();
        //     self.array.push(a);
        // }

        // let attribut = interface
        //     .create_attribute("test")
        //     .message()
        //     .with_rw_access()
        //     .finish_with_codec::<BooleanCodec>()
        //     .await;

        // attribut.set(true).await.unwrap();
        // //
        // device.logger.info("pooook 2 ");
        // // Task that run an action every time the value of the attribute change

        // let _aa = attribut.clone();
        // spawn_loop!(device, {
        //     println!("start wait");
        //     let attribut_bis = _aa.clone();
        //     on_command!(_aa, {
        //         println!("cooucou");
        //         let _dat = attribut_bis.get().await.unwrap();
        //         println!("cooucou {} ", _dat);
        //         Ok(())
        //     });
        // });

        // device.logger.info("pooook 3 ");

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
