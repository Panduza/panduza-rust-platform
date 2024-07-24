use panduza_connectors::serial::slip::Connector as SlipConnector;
use panduza_connectors::SerialSettings;
use prost::Message;

use async_trait::async_trait;

use panduza_core::interface::builder::Builder as InterfaceBuilder;
use panduza_core::interface::ThreadSafeInterface;
use panduza_core::meta::digital_input;
use panduza_core::Error as PlatformError;

use panduza_connectors::serial::generic::garbage_collector;

use panduza_connectors::serial::slip::get as get_connector;

use super::api_dio::PicohaDioAnswer;

use super::api_dio::PicohaDioRequest;
use super::api_dio::RequestType;

use panduza_core::platform_error;

///
///
struct InterfaceActions {
    serial_settings: SerialSettings,

    connector: Option<SlipConnector>,
}

impl InterfaceActions {
    // // Timeout reading
    // self.in_buf_size += timeout(Duration::from_secs(5), read_chunk)
    //     .await
    //     .map_err(|e| platform_error!("Timeout reading {:?}", e))?
    //     .map_err(|e| platform_error!("Unable to read on serial stream {:?}", e))?;

    async fn execute_request(
        &mut self,
        request: &PicohaDioRequest,
    ) -> Result<PicohaDioAnswer, PlatformError> {
        let buf = request.encode_to_vec();
        let respond = &mut [0; 64];
        let size = self
            .connector
            .as_ref()
            .ok_or(platform_error!("Connector is not initialized"))?
            .lock()
            .await
            .write_then_read(&buf, respond)
            .await?;
        let answerr = PicohaDioAnswer::decode(&respond[0..size]).unwrap();
        return Ok(answerr);
    }
}

#[async_trait]
impl digital_input::MetaActions for InterfaceActions {
    /// Initialize the interface
    ///
    async fn initializating(
        &mut self,
        interface: &ThreadSafeInterface,
    ) -> Result<(), PlatformError> {
        // let logger = interface.lock().await.clone_logger();

        self.connector = Some(get_connector(&self.serial_settings).await?);
        self.connector
            .as_ref()
            .ok_or(platform_error!("Connector is not initialized"))?
            .lock()
            .await
            .init()
            .await?;

        // // garbage_collector().await;

        // // self.connector =  None;
        // // self.connector.as_mut().unwrap().init().await?;

        // let request = PicohaDioRequest {
        //     r#type: RequestType::Ping as i32,
        //     pin_num: 5,
        //     value: 0,
        // };
        // // // let request = PicohaDioRequest {
        // // //     r#type: RequestType::GetPinDirection as i32,
        // // //     pin_num: 0,
        // // //     value: 0,
        // // // };

        // // println!("=====");
        // // // let mut buf = vec![0;20];
        // let buf = request.encode_to_vec();
        // // // if p.is_err() {
        // // //     println!("------*** Error: {:?}", p.err());
        // // // }
        // // // else {
        // // //     println!("------*** Ok");
        // // // };
        // // println!("Sending: {:?}", buf);
        // // println!("=====");

        // let respond = &mut [0; 64];

        // // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
        // let size = self.connector.write_then_read(&buf, respond).await.unwrap();

        // let answerr = PicohaDioAnswer::decode(&respond[0..size]).unwrap();

        // println!("Respond: {:?}", respond[0..size].to_vec());

        return Ok(());
    }

    async fn read(&mut self, interface: &ThreadSafeInterface) -> Result<u8, String> {
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
    serial_settings: Option<SerialSettings>,
}
impl Builder {
    /// Create a new builder with default values
    pub fn new() -> Builder {
        return Builder {
            name: "digital_input".to_string(),
            serial_settings: None,
        };
    }

    /// Set the name of the interface
    pub fn with_name<A: Into<String>>(mut self, name: A) -> Self {
        self.name = name.into();
        self
    }

    /// Set the serial configuration
    pub fn with_serial_settings(mut self, serial_settings: SerialSettings) -> Self {
        self.serial_settings = Some(serial_settings);
        self
    }

    /// Build the interface
    pub fn build(self) -> InterfaceBuilder {
        digital_input::build(
            self.name,
            Box::new(InterfaceActions {
                serial_settings: self.serial_settings.unwrap(),
                connector: None,
            }),
        )
    }
}
