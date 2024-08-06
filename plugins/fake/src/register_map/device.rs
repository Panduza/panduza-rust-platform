use core::sync;

use async_trait::async_trait;
use panduza_platform_core::{
    BooleanCodec, Device, DeviceOperations, Error, UIntergerCodec, WoMessageAttribute,
};

pub struct RegisterMapDevice {
    array: Vec<WoMessageAttribute<UIntergerCodec>>,
}

impl RegisterMapDevice {
    pub fn new() -> RegisterMapDevice {
        RegisterMapDevice { array: Vec::new() }
    }

    async fn create_channel_n(&mut self, mut device: Device, i: u32) {
        let mut channel_0 = device
            .create_interface("channel_0")
            .with_tags("examples;tests")
            .finish();

        let enable = channel_0
            .create_attribute("enable")
            .message()
            .with_rw_access()
            .finish_with_codec::<BooleanCodec>()
            .await;

        let _clone = enable.clone();
        device
            .spawn(async move {
                loop {
                    println!("start wait");
                    let attribut_bis = _clone.clone();

                    _clone
                        .wait_one_command_then(async move {
                            // return Err(Error::Wtf);
                            println!("enable 0");
                            Ok(())
                        })
                        .await?
                }
            })
            .await;
    }
}

#[async_trait]
impl DeviceOperations for RegisterMapDevice {
    /// Mount the device
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        let test = std::sync::Arc::new(std::sync::Mutex::new(0u8));

        // commands [json Codec] (Ro)
        // N topic avec 1 valeur de registre [int or array codec] (Wo -> write only)

        //
        device.logger.info("pooook");

        let mut interface = device
            .create_interface("pok")
            .with_tags("examples;tests")
            .finish();

        for n in 1..20 {
            let a = interface
                .create_attribute(format!("cell_{}", n))
                .message()
                .with_wo_access()
                .finish_with_codec::<UIntergerCodec>()
                .await;
            a.set(2).await.unwrap();
            self.array.push(a);
        }

        let attribut = interface
            .create_attribute("test")
            .message()
            .with_rw_access()
            .finish_with_codec::<BooleanCodec>()
            .await;

        attribut.set(true).await.unwrap();
        //
        device.logger.info("pooook 2 ");
        // Task that run an action every time the value of the attribute change

        let _aa = attribut.clone();
        device
            .spawn(async move {
                loop {
                    println!("start wait");
                    let attribut_bis = _aa.clone();

                    _aa.wait_one_command_then(async move {
                        // return Err(Error::Wtf);
                        println!("cooucou");
                        let _dat = attribut_bis.get().await.unwrap();
                        println!("cooucou {} ", _dat);
                        Ok(())
                    })
                    .await?
                }
            })
            .await;

        device.logger.info("pooook 3 ");

        Ok(())
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
