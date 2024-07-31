use std::collections::HashMap;

use lazy_static::lazy_static;

use panduza_core::platform_error;
use panduza_core::Error as PlatformError;

use crate::GateLogger;
use crate::UsbSettings;

use super::driver::Driver;
use super::Connector;

static CONNECTOR_CLASS_NAME: &str = "usb-generic";

lazy_static! {
    static ref GATE: tokio::sync::Mutex<Gate> = tokio::sync::Mutex::new(Gate {
        logger: GateLogger::new(CONNECTOR_CLASS_NAME),
        instances: HashMap::new()
    });
}

// get should return an error message
pub async fn get(usb_settings: &UsbSettings) -> Result<Connector, PlatformError> {
    let mut gate = GATE.lock().await;
    gate.get(usb_settings)
}

/// Main entry point to acces connectors
///
pub struct Gate {
    logger: GateLogger,
    instances: HashMap<String, Connector>,
}

impl Gate {
    fn get(&mut self, usb_settings: &UsbSettings) -> Result<Connector, PlatformError> {
        // Debug
        self.logger.log_debug("GET a new usb connector");
        self.logger
            .log_debug(format!("- serial_number: {:?}", usb_settings.serial));

        // Get the key
        let key = usb_settings
            .serial
            .as_ref()
            .ok_or(platform_error!("Port name is not set"))?;

        // if the instance is not found, it means that the port is not opened yet
        if !self.instances.contains_key(key) {
            //
            self.logger
                .log_info(format!("Creating a new usb connector for {}", key));

            // Create a new instance
            let new_instance = Driver::new(usb_settings).into_connector();

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());

            return Ok(new_instance.clone());
        }

        // Try to find the instance
        let instance = self.instances.get(key).ok_or(platform_error!(format!(
            "Unable to find the usb connector \"{}\"",
            key
        )))?;

        // Return the instance
        Ok(instance.clone())
    }
}
