use nusb::{transfer::Direction, transfer::EndpointType, Interface};
use std::{collections::HashMap, sync::Arc};

use futures_lite::future::block_on;
use lazy_static::lazy_static;
use tokio;
use usbtmc_message::Sequencer;

use panduza_platform_core::platform_error_result;
use panduza_platform_core::Error as PlatformError;
use panduza_platform_core::FunctionResult as PlatformFunctionResult;

lazy_static! {
    static ref GATE: tokio::sync::Mutex<Gate> = tokio::sync::Mutex::new(Gate {
        instances: HashMap::new()
    });
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
            usb_serial: None,
        }
    }

    pub fn import_from_json_settings(
        &mut self,
        settings: &serde_json::Value,
    ) -> PlatformFunctionResult {
        // Get serial number
        self.usb_serial = match settings.get("usb_serial") {
            Some(val) => match val.as_str() {
                Some(s) => Some(s.to_string()),
                None => return platform_error_result!("usb_serial not a String"),
            },
            None => return platform_error_result!("Missing usb_serial from tree.json"),
        };

        Ok(())
    }
}

struct Gate {
    instances: HashMap<String, UsbtmcConnector>,
}

impl Gate {
    fn get(&mut self, config: &Config) -> Option<UsbtmcConnector> {
        // First try to get the key
        let key_string = Gate::generate_unique_key_from_config(config)?;
        let key = key_string.as_str();

        // if the instance is not found, it means that the port is not opened yet
        if !self.instances.contains_key(key) {
            // Create a new instance
            let new_instance = UsbtmcConnector::new(Some(config.clone()));

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());
            tracing::info!(class = "Platform", "connector created");
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
pub struct UsbtmcConnector {
    core: Option<Arc<tokio::sync::Mutex<UsbtmcCore>>>,
}

impl UsbtmcConnector {
    pub fn new(config: Option<Config>) -> UsbtmcConnector {
        match config {
            Some(config) => UsbtmcConnector {
                core: Some(Arc::new(tokio::sync::Mutex::new(UsbtmcCore::new(config)))),
            },
            None => UsbtmcConnector { core: None },
        }
    }

    pub async fn init(&mut self) -> PlatformFunctionResult {
        let _ = match self.core.as_ref() {
            Some(val) => val.lock().await.init().await,
            None => platform_error_result!("Unable to initialize USBTMC connector"),
        };

        Ok(())
    }

    pub async fn ask(&mut self, command: String) -> Result<String, PlatformError> {
        match self.core.as_ref() {
            Some(val) => val.lock().await.write_then_read(command).await,
            None => platform_error_result!("Unable to write then read"),
        }
    }
}

struct UsbtmcCore {
    config: Config,
    interface: Option<Interface>,
}

impl UsbtmcCore {
    fn new(config: Config) -> UsbtmcCore {
        UsbtmcCore {
            config: config,
            interface: None,
        }
    }

    async fn init(&mut self) -> PlatformFunctionResult {
        let devices_option = match nusb::list_devices() {
            Ok(val) => val,
            Err(_e) => return platform_error_result!("Unable to list USB devices"),
        }
        .find(|d| d.serial_number() == self.config.usb_serial.as_deref());

        let devices = match devices_option {
            Some(v) => v,
            None => return platform_error_result!("Unable to list USB devices"),
        };

        let device = match devices.open() {
            Ok(val) => val,
            Err(_e) => return platform_error_result!("Unable to open USB device"),
        };

        self.interface = match device.claim_interface(0) {
            Ok(val) => Some(val),
            Err(_e) => return platform_error_result!("Unable to create USB device interface"),
        };

        Ok(())
    }

    async fn write_then_read(&mut self, command: String) -> Result<String, PlatformError> {
        let itf = match self.interface.as_ref() {
            Some(val) => val,
            None => return platform_error_result!("No USB interface"),
        };

        let mut endpoint_out = 0;
        let mut endpoint_in = 0;

        // Get the usb endpoints
        for interface_descriptor in itf.descriptors() {
            for endpoint in interface_descriptor.endpoints() {
                if endpoint.direction() == Direction::Out
                    && endpoint.transfer_type() == EndpointType::Bulk
                {
                    endpoint_out = endpoint.address()
                }
                if endpoint.direction() == Direction::In
                    && endpoint.transfer_type() == EndpointType::Bulk
                {
                    endpoint_in = endpoint.address()
                }
            }
        }

        // Create a sequencer with a max_sequence_length of 64 (depend on your device)
        let mut sequencer = Sequencer::new(64);

        // Create a message sequence from a command
        let sequence = sequencer.command_to_message_sequence(command.clone());

        // Send the sequence on the usb
        for i in 0..sequence.len() {
            let message = sequence[i].to_vec();
            // SEND TO USB
            match block_on(itf.bulk_out(endpoint_out, message.to_vec())).into_result() {
                Ok(val) => val,
                Err(_e) => return platform_error_result!("Unable to write on USB"),
            };
        }

        let response = nusb::transfer::RequestBuffer::new(64 as usize);

        // Receive data form the usb
        let data = match block_on(itf.bulk_in(endpoint_in, response)).into_result() {
            Ok(val) => val,
            Err(_e) => return platform_error_result!("Unable to read on USB"),
        };

        // Parse the received data
        let msg = usbtmc_message::BulkInMessage::from_u8_array(&data);

        Ok(msg.payload_as_string())
    }
}
