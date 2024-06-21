use std::{collections::HashMap, sync::Arc};
use nusb::{transfer::Direction, Interface};

use tokio;
use lazy_static::lazy_static;
use futures_lite::future::block_on;

lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate>
        = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
}

pub async fn get(config: &Config) -> Option<UsbConnector> {
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

        let usb_vendor_str = 
            settings.get("usb_vendor")
                .map(|v| v.as_str().unwrap());

        // get VID hexadecimal value
        self.usb_vendor =
            Some(u16::from_str_radix(usb_vendor_str.as_ref().unwrap(), 16).unwrap());

        let usb_model_str = 
            settings.get("usb_model")
                .map(|v| v.as_str().unwrap());

        // get PID hexadecimal value
        self.usb_model =
            Some(u16::from_str_radix(usb_model_str.as_ref().unwrap(), 16).unwrap());

        self.usb_serial =
            settings.get("usb_serial")
                .map(|v| v.as_str().unwrap().to_string());

    }
}



struct Gate {
    instances: HashMap<String, UsbConnector>
}

impl Gate {


    fn get(&mut self, config: &Config) -> Option<UsbConnector> {
        // First try to get the key
        let key_string = Gate::generate_unique_key_from_config(config)?;
        let key= key_string.as_str();

        // if the instance is not found, it means that the port is not opened yet
        if ! self.instances.contains_key(key) {

            // Create a new instance
            let new_instance = UsbConnector::new(Some(config.clone()));

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
pub struct UsbConnector {
    core: Option<Arc<tokio::sync::Mutex<UsbCore>>>,
}

impl UsbConnector {
    
    pub fn new(config: Option<Config>) -> UsbConnector {
        match config {
            Some(config)    => {
                UsbConnector {
                    core: Some(
                        Arc::new(tokio::sync::Mutex::new(
                            UsbCore::new(config)))
                    )
                }
            }
            None            => {
                UsbConnector {
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


    pub async fn write(&mut self, command: &[u8]) {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .write(command)
            .await
    }

    pub async fn read(&mut self) 
            -> String {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .read()
            .await
    }

}


struct UsbCore {
    _config: Config,
    interface: Option<Interface>,
}

impl UsbCore {

    fn new(config: Config) -> UsbCore {
        UsbCore {
            _config: config,
            interface: None,
        }
    }

    async fn init(&mut self) {

        // Open device with to provided VID and PID
        // TODO find the device with the serial number
        let devices = nusb::list_devices()
            .unwrap()
            .find(|d| d.vendor_id() == self._config.usb_vendor.unwrap() && d.product_id() == self._config.usb_model.unwrap())
            .expect("device is not connected");

        let device = devices.open().unwrap();
        self.interface = Some(device.claim_interface(0).unwrap());

    }


    async fn write(&mut self, command: &[u8]) {
        
        // find the Bulk Out endpoint to send the message
        for interface_descriptor in self.interface.as_ref().unwrap().descriptors() {
            for endpoint in interface_descriptor.endpoints() {
                if endpoint.direction() == Direction::Out {
                    // Send the command on the usb
                    block_on(self.interface.as_ref().unwrap().bulk_out(endpoint.address(), command.to_vec()))
                        .into_result()
                        .unwrap();
                }
            }
        }
    }


    async fn read(&mut self) 
        -> String {

        let mut msg = String::new();

        // find the Bulk In endpoint to receive the message
        for interface_descriptor in self.interface.as_ref().unwrap().descriptors() {
            for endpoint in interface_descriptor.endpoints() {
                if endpoint.direction() == Direction::In {
                    let response = nusb::transfer::RequestBuffer::new(32 as usize);
                    let data = block_on(self.interface.as_ref().unwrap().bulk_in(endpoint.address(), response)).into_result().unwrap();
                    
                    msg = String::from_utf8(data).unwrap();
                }
            }
        }

        msg
    }

}