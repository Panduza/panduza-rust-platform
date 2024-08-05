use async_trait::async_trait;
use panduza_platform_core::{BooleanCodec, Device, DeviceOperations, Error};

pub struct RegisterMapDevice {}

#[async_trait]
impl DeviceOperations for RegisterMapDevice {
    /// Mount the device
    ///
    async fn mount(&self, mut device: Device) -> Result<(), Error> {
        //
        device.logger.info("pooook");

        let mut interface = device
            .create_interface("pok")
            .with_tags("examples;tests")
            .finish();

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

        device
            .spawn(async move {
                println!("start loop");
                loop {
                    println!("start wait");
                    let attribut_bis = attribut.clone();
                    attribut
                        .wait_one_command_then(async move {
                            println!("cooucou");
                            let _dat = attribut_bis.get().await.unwrap();
                            println!("cooucou {} ", _dat);
                        })
                        .await;
                }
            })
            .await;

        // we have to store the handle in an object that will survive the function
        // device.store_handle(h).await;

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
