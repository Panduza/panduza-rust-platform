use panduza_core::platform_error;
use panduza_core::Error as PlatformError;

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::SerialSettings;


use crate::GateLogger;
use super::Connector;
use super::driver::Driver;

lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate> 
        = tokio::sync::Mutex::new(Gate {
            logger: GateLogger::new("serial-slip"),
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
    logger: GateLogger,
    instances: HashMap<String, Connector>
}

impl Gate {


    fn get(&mut self, serial_settings: &SerialSettings)
        -> Result<Connector, PlatformError>
    {
        // Debug
        self.logger.log_debug("GET a new serial-slip connector");
        self.logger.log_debug(format!("- port_name: {:?}", serial_settings.port_name));

        // Get the key
        let key = serial_settings.port_name
            .as_ref()
            .ok_or(platform_error!("Port name is not set"))?;


        // if the instance is not found, it means that the port is not opened yet
        if ! self.instances.contains_key(key) {

            //
            self.logger.log_info(format!("Creating a new serial connector for {}", key));

            // Create a new instance
            let new_instance = Driver::new(serial_settings).into_connector();

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());

            
            return Ok(new_instance.clone());
        }

        // Try to find the instance
        let instance = self.instances.get(key)
            .ok_or(platform_error!(
                format!("Unable to find the tty connector \"{}\"", key)
            ))?;


        // Return the instance
        Ok(instance.clone())
    }

}