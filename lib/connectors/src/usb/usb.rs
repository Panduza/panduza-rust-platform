use std::{collections::HashMap, sync::Arc};
use nusb::{transfer::Direction, transfer::EndpointType, Interface};

use tokio;
use lazy_static::lazy_static;
use futures_lite::future::block_on;

use panduza_core::FunctionResult as PlatformFunctionResult;
use panduza_core::Error as PlatformError;
use panduza_core::platform_error_result;

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

    pub fn import_from_json_settings(&mut self, settings: &serde_json::Value) -> PlatformFunctionResult {

        // Get serial number
        self.usb_serial = match settings.get("usb_serial")
        {
            Some(val) => match val.as_str()
            {
                Some(s) => Some(s.to_string()),
                None => return platform_error_result!("usb_serial not a String")
            },
            None => return platform_error_result!("Missing usb_serial from tree.json")
        };

        Ok(())
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
        if config.usb_serial.is_some() {
            return config.usb_serial.clone();
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

    pub async fn init(&mut self) -> PlatformFunctionResult {
        let _ = match self.core.as_ref() {
            Some(val) => val.lock().await.init().await,
            None => return platform_error_result!("Unable to initialize USB connector")
        };

        Ok(())
    }


    pub async fn write(&mut self, command: &[u8]) -> PlatformFunctionResult {
        match self.core.as_ref() {
            Some(val) => val.lock().await.write(command).await,
            None => platform_error_result!("Unable to write")
        }
    }

    pub async fn read(&mut self) -> Result<String, PlatformError> {
        match self.core.as_ref() {
            Some(val) => val.lock().await.read().await,
            None => platform_error_result!("Unable to write")
        }
    }

}


struct UsbCore {
    config: Config,
    interface: Option<Interface>,
}

impl UsbCore {

    fn new(config: Config) -> UsbCore {
        UsbCore {
            config: config,
            interface: None,
        }
    }

    async fn init(&mut self) -> PlatformFunctionResult {

        // Open device with to provided VID and PID
        // TODO find the device with the serial number
        let devices_option = match nusb::list_devices() {
            Ok(val) => val,
            Err(_e) => return platform_error_result!("Unable to list USB devices")
        }
            .find(|d| d.serial_number() == self.config.usb_serial.as_deref());

        let devices = match devices_option{
            Some(v) => v,
            None => return platform_error_result!("Unable to list USB devices")
        };

        let device = match devices.open() {
            Ok(val) => val,
            Err(_e) => return platform_error_result!("Unable to open USB device")
        };

        self.interface = match device.claim_interface(0) {
            Ok(val) => Some(val),
            Err(_e) => return platform_error_result!("Unable to create USB device interface")
        };

        Ok(())
    }


    async fn write(&mut self, command: &[u8]) -> PlatformFunctionResult {
        
        let itf = match self.interface.as_ref() {
            Some(val) => val,
            None => return platform_error_result!("No USB interface")
        };
        
        // find the Bulk Out endpoint to send the message
        for interface_descriptor in itf.descriptors() {
            for endpoint in interface_descriptor.endpoints() {
                if endpoint.direction() == Direction::Out && endpoint.transfer_type() == EndpointType::Bulk {
                    // Send the command on the usb
                    match block_on(itf.bulk_out(endpoint.address(), command.to_vec())).into_result() {
                        Ok(_v) => return Ok(()),
                        Err(_e) => return platform_error_result!("Unable to write on USB")
                    }
                }
            }
        }
        Ok(())
    }


    async fn read(&mut self) -> Result<String, PlatformError> {

        let mut msg = String::new();

        // find the Bulk In endpoint to receive the message
        let itf = match self.interface.as_ref() {
            Some(val) => val,
            None => return platform_error_result!("No USB interface")
        };

        for interface_descriptor in itf.descriptors() {
            for endpoint in interface_descriptor.endpoints() {
                if endpoint.direction() == Direction::In && endpoint.transfer_type() == EndpointType::Bulk {
                    let response = nusb::transfer::RequestBuffer::new(32 as usize);
                    let data = match block_on(itf.bulk_in(endpoint.address(), response)).into_result() {
                        Ok(val) => val,
                        Err(_e) => return platform_error_result!("Unable to read USB data")
                    };
                    
                    msg = match String::from_utf8(data) {
                        Ok(val) => val,
                        Err(_e) => return platform_error_result!("Unable to decode received USB data")
                    };
                }
            }
        }

        Ok(msg)
    }

}