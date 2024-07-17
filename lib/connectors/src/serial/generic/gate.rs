use panduza_core::platform_error;
use panduza_core::Error as PlatformError;

use std::collections::HashMap;
use std::rc::Rc;

use lazy_static::lazy_static;

use crate::SerialSettings;


use crate::GateLogger;
use super::SerialConnector;


lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate> 
        = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
}

// get should return an error message
pub async fn get(serial_settings: &SerialSettings) -> Result<SerialConnector, PlatformError> {
    let mut gate = GATE.lock().await;
    gate.get(serial_settings)
}

pub async fn run_garbage_collector() {
    let mut gate = GATE.lock().await;
    // gate.run_garbage_collector();
}


/// Main entry point to acces connectors
/// 
pub struct Gate {
    instances: HashMap<String, SerialConnector>
}

impl Gate {


    fn get(&mut self, serial_settings: &SerialSettings)
        -> Result<SerialConnector, PlatformError>
    {


        // Get the key
        let key = serial_settings.port_name
            .as_ref()
            .ok_or(platform_error!("Port name is not set"))?;


        // if the instance is not found, it means that the port is not opened yet
        if ! self.instances.contains_key(key) {

            // Create a new instance
            let new_instance = SerialConnector::new(&self);

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());
            tracing::info!(class="Platform", "connector created");

            
            return Ok(new_instance.clone());
        }

        // Try to find the instance
        let instance = self.instances.get(key)
            .ok_or(platform_error!(
                format!("Unable to find the tty connector \"{}\"", key)
            ))?;


        println!("c -----> {}", instance.count_refs());

        // Return the instance
        Ok(instance.clone())
    }

}