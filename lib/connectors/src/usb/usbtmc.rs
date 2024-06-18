use std::{collections::HashMap, sync::Arc};
use nusb::Interface;

use tokio;
use lazy_static::lazy_static;
use usbtmc_message::Sequencer;
use futures_lite::future::block_on;

lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate>
        = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
}

pub async fn get(config: &Config) -> Option<UsbtmcConnector> {
    let mut gate = GATE.lock().await;
    gate.get(config)
}


#[derive(Clone, Debug)]
pub struct Config {
    pub usb_vendor: Option<u16>,
    pub usb_model: Option<u16>,
    pub usb_serial: Option<String>,


}

impl Config {
    pub fn new() -> Config {
        Config {
            usb_vendor: None,
            usb_model: None,
            usb_serial: None
        }
    }

    pub fn import_from_json_settings(&mut self, settings: &serde_json::Value) {

        self.usb_vendor =
            settings.get("usb_vendor")
                .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

        self.usb_model =
            settings.get("usb_model")
                .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

        self.usb_serial =
            settings.get("usb_serial")
                .map(|v| v.as_str().unwrap().to_string());

    }
}



struct Gate {
    instances: HashMap<String, UsbtmcConnector>
}

impl Gate {


    fn get(&mut self, config: &Config) -> Option<UsbtmcConnector> {
        // First try to get the key
        let key_string = Gate::generate_unique_key_from_config(config)?;
        let key= key_string.as_str();

        // if the instance is not found, it means that the port is not opened yet
        if ! self.instances.contains_key(key) {

            // Create a new instance
            let new_instance = UsbtmcConnector::new(Some(config.clone()));

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());
            tracing::info!(class="Platform", "connector created");
        }

        // Try to find the instance
        let instance = self.instances.get(key)?;

        // Return the instance
        Some(instance.clone())
    }

    /// Try to generate a unique key from the config
    /// This key will be used to find back the tty connector
    ///
    fn generate_unique_key_from_config(config: &Config) -> Option<String> {
        // Check if the usb vendor and model are provided
        if let Some(k) = Some(format!("{}_{}_{}", config.usb_vendor.unwrap(), config.usb_model.unwrap(), config.usb_serial.as_ref().unwrap())) {
            return Some(k.clone());
        }

        // Finally unable to generate a key with the config
        return None;
    }

}



#[derive(Clone)]
pub struct UsbtmcConnector {
    core: Option<Arc<tokio::sync::Mutex<UsbtmcCore>>>,
}

impl UsbtmcConnector {
    
    pub fn new(config: Option<Config>) -> UsbtmcConnector {
        match config {
            Some(config)    => {
                UsbtmcConnector {
                    core: Some(
                        Arc::new(tokio::sync::Mutex::new(
                            UsbtmcCore::new(config)))
                    )
                }
            }
            None            => {
                UsbtmcConnector {
                    core: None
                }
            }
        }
    }

    pub async fn init(&mut self) {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .init()
            .await;
    }


    pub async fn ask(&mut self, command: String) 
            -> String {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .write_then_read(command)
            .await
    }

}


struct UsbtmcCore {
    _config: Config,
    interface: Option<Interface>,
}

impl UsbtmcCore {

    fn new(config: Config) -> UsbtmcCore {
        UsbtmcCore {
            _config: config,
            interface: None,
        }
    }

    async fn init(&mut self) {

        let devices = nusb::list_devices()
            .unwrap()
            .find(|d| d.vendor_id() == 0x1313 && d.product_id() == 0x8079)//format!("{:04x}", self.config.usb_vendor.unwrap()).parse::<u16>().unwrap() && d.product_id() == format!("{:04x}", self.config.usb_model.unwrap()).parse::<u16>().unwrap())
            .expect("device is not connected");

        let device = devices.open().unwrap();
        self.interface = Some(device.claim_interface(0).unwrap());

    }


    async fn write_then_read(&mut self, command: String) 
            -> String{

        // Create a sequencer with a max_sequence_length of 64 (depend on your device)
        let mut sequencer = Sequencer::new(64);

        // Create a message sequence from a command
        let sequence = sequencer.command_to_message_sequence(command);
        
        // Send the sequence on the usb
        for i in 0..sequence.len() {
            let message = sequence[i].to_vec();
            // SEND TO USB
            block_on(self.interface.as_ref().unwrap().bulk_out(0x02, message.to_vec()))
                .into_result()
                .unwrap();
            
        }

        
        let response = nusb::transfer::RequestBuffer::new(64 as usize);
        let data = block_on(self.interface.as_ref().unwrap().bulk_in(0x82, response)).into_result().unwrap();

        // Parse the received data
        let msg = usbtmc_message::BulkInMessage::from_u8_array(&data);

        msg.payload_as_string()
        
    }

}