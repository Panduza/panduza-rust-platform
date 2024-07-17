use std::sync::Arc;
use panduza_connectors::serial::slip::SlipConnector;
use panduza_connectors::SerialSettings;
use prost::Message;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration;

use async_trait::async_trait;

use panduza_core::{interface, Error as PlatformError};
use panduza_core::meta::digital_input;
use panduza_core::interface::ThreadSafeInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use serde_json::Error;

use futures::FutureExt;


// use panduza_connectors::serial::slip::get as ConnectorSlipGet;
use panduza_connectors::serial::generic::get as ConnectorSlipGet;
use panduza_connectors::serial::tty3::get as ConnectorGet;
// use panduza_connectors::serial::slip::Connector as SlipConnector;
use panduza_connectors::serial::tty3::Config as SerialConfig;
use panduza_connectors::serial::generic::SerialConnector;

use panduza_connectors::serial::generic::garbage_collector;

use panduza_connectors::serial::slip::SlipDriver;
use panduza_connectors::serial::slip::get as SlipGetConnector;

use super::api_dio::PicohaDioRequest;
use super::api_dio::RequestType;


use panduza_core::platform_error;


///
/// 
struct InterfaceActions {

    config: SerialConfig,

    connector: SlipConnector,
    
    // pub fake_values: Arc<Mutex<Vec<u64>>>,
}

#[async_trait]
impl digital_input::MetaActions for InterfaceActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface :&ThreadSafeInterface) -> Result<(), PlatformError> {

        // let logger = interface.lock().await.clone_logger();


        // self.connector = 

        let seee = SerialSettings::new()
            .set_port_name("port_name")
            .set_baudrate(115200);


        println!("before creatino");
        self.connector = SlipGetConnector(&seee).await?;
        println!("after creatino");


        // self.connector.init().await?;


        garbage_collector().await;

        

        // self.connector =  None;
        // self.connector.as_mut().unwrap().init().await?;


        // let request = PicohaDioRequest {
        //     r#type: RequestType::Ping as i32,
        //     pin_num: 5,
        //     value: 0,
        // };
        // // let request = PicohaDioRequest {
        // //     r#type: RequestType::GetPinDirection as i32,
        // //     pin_num: 0,
        // //     value: 0,
        // // };
        
        // println!("=====");
        // // let mut buf = vec![0;20];
        // let buf = request.encode_to_vec();
        // // if p.is_err() {
        // //     println!("------*** Error: {:?}", p.err());
        // // }
        // // else {
        // //     println!("------*** Ok");
        // // };
        // println!("Sending: {:?}", buf);
        // println!("=====");

        // let respond = &mut [0; 20];
        // self.connector.as_mut().unwrap().write_then_read(&buf, respond).await.unwrap();

        return Ok(());
    }
    
    
    async fn read(&mut self, interface: &ThreadSafeInterface) -> Result<u8, String>
    {
        // let values = self.fake_values.lock().await;
        return Ok(0 as u8);
    }



    // async fn read(&mut self, interface: &ThreadSafeInterface, index:usize, size:usize) -> Result<Vec<u64>, String>
    // {
    //     if let Some(sub_vec) = self.fake_values.lock().await.get(index..index+size) {
    //         // Étape 4: Utiliser sub_vec ici
    //         println!("Sous-vecteur: {:?}", sub_vec);
    //         Ok(sub_vec.to_vec())
    //     } else {
    //         // Gérer l'erreur si la plage est invalide
    //         println!("Plage invalide");
    //         Err("invalid".to_string())
    //     }
    // }

    // async fn write(&mut self, interface: &ThreadSafeInterface, index:usize, v: &Vec<u64>)
    // {
    //     self.fake_values.lock().await.splice(index..index+v.len(), v.iter().cloned());
    //     println!("InterfaceActions - write: {:?}", v);
    // }


}




/// Builder
/// 
pub struct Builder {
    /// Name of the interface
    name: String,
    /// Serial configuration
    serial_config: Option<SerialConfig>,
}
impl Builder {
    /// Create a new builder with default values
    pub fn new() -> Builder {
        return Builder {
            name: "digital_input".to_string(),
            serial_config: None,
        }
    }

    /// Set the name of the interface
    pub fn with_name<A: Into<String>>(mut self, name: A) -> Self {
        self.name = name.into();
        self
    }

    /// Set the serial configuration
    pub fn with_serial_config(mut self, serial_config: SerialConfig) -> Self {
        self.serial_config = Some(serial_config);
        self
    }

    /// Build the interface
    pub fn build(self) -> InterfaceBuilder {
        digital_input::build(
            self.name,
            Box::new(InterfaceActions {
                config: self.serial_config.unwrap(),
                connector: SlipConnector::new(),
            })
        )
    }
}

