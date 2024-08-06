use std::collections::HashMap;

use lazy_static::lazy_static;

use panduza_platform_core::Error as PlatformError;

use crate::ConnectorLogger;
use crate::SerialSettings;

use super::driver::Driver;
use super::Connector;

lazy_static! {
    static ref GATE: tokio::sync::Mutex<Gate> = tokio::sync::Mutex::new(Gate {
        logger: ConnectorLogger::new("serial", "generic", ""),
        instances: HashMap::new()
    });
}

// get should return an error message
pub async fn get(serial_settings: &SerialSettings) -> Result<Connector, PlatformError> {
    let mut gate = GATE.lock().await;
    gate.get(serial_settings)
}

/// Main entry point to acces connectors
///
pub struct Gate {
    logger: ConnectorLogger,
    instances: HashMap<String, Connector>,
}

impl Gate {
    fn get(&mut self, serial_settings: &SerialSettings) -> Result<Connector, PlatformError> {
        // Debug
        self.logger.debug("GET a new serial-slip connector");
        self.logger
            .debug(format!("- port_name: {:?}", serial_settings.port_name));

        // Get the key
        let key = serial_settings
            .port_name
            .as_ref()
            .ok_or(PlatformError::BadSettings(
                "Port name is not set".to_string(),
            ))?;

        // if the instance is not found, it means that the port is not opened yet
        if !self.instances.contains_key(key) {
            //
            self.logger
                .info(format!("Creating a new serial connector for {}", key));

            // Create a new instance
            let new_instance = Driver::new(serial_settings).into_connector();

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());

            return Ok(new_instance.clone());
        }

        // Try to find the instance
        let instance = self
            .instances
            .get(key)
            .ok_or(PlatformError::BadSettings(format!(
                "Unable to find the tty connector \"{}\"",
                key
            )))?;

        // Return the instance
        Ok(instance.clone())
    }
}
