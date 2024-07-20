use panduza_core::platform_error;
use panduza_core::Error as PlatformError;

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::SerialSettings;


use crate::GateLogger;
use super::SlipConnector;


lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate> 
        = tokio::sync::Mutex::new(Gate {
            logger: GateLogger::new("serial-generic"),
            instances: HashMap::new()
        });
}

// get should return an error message
pub async fn get(serial_settings: &SerialSettings) -> Result<SlipConnector, PlatformError> {
    let mut gate = GATE.lock().await;
    gate.get(serial_settings)
}

pub async fn garbage_collector() {
    let mut gate = GATE.lock().await;
    gate.garbage_collector();
}


/// Main entry point to acces connectors
/// 
pub struct Gate {
    logger: GateLogger,
    instances: HashMap<String, SlipConnector>
}

impl Gate {


    fn get(&mut self, serial_settings: &SerialSettings)
        -> Result<SlipConnector, PlatformError>
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
            let new_instance = SlipConnector::from_settings(serial_settings);

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


    /// Garbage collector
    /// 
    fn garbage_collector(&mut self) {
        let mut keys_to_remove = Vec::new();
        for (key, instance) in self.instances.iter() {
            // If there is only left one reference, we can remove it (it is the gate)
            if instance.count_refs() == 1 {
                keys_to_remove.push(key.clone());
            }
        }
        for key in keys_to_remove {
            self.instances.remove(&key);
        }
    }

}