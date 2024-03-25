use std::collections::HashMap;

use crate::builtin_devices;
use crate::platform_error;
use crate::platform::PlatformError;

use super::Device;
use super::Producer;



pub struct Factory {
    producers : HashMap<String, Box<dyn Producer>>
    
}

impl Factory {

    /// Create a new factory
    /// 
    pub fn new() -> Factory {
        // New object
        let mut obj = Factory {
            producers: HashMap::new()
        };

        // Info log
        tracing::info!(class="Factory", "# Device factory initialization");
        tracing::info!(class="Factory", "List of producers:");

        // Load builtin device producers
        builtin_devices::import_plugin_producers(&mut obj);
        return obj;
    }

    /// Add a producer to the factory
    /// 
    pub fn add_producer(&mut self, device_ref: &str, producer: Box<dyn Producer>) {
        // Info log
        tracing::info!(class="Factory", "  - {}", device_ref);

        // Append the producer
        self.producers.insert(device_ref.to_string(), producer);
    }


    // pub fn get_producer(self, device_ref: String) -> Result<Box<dyn Producer>, String>  {
    // }


    /// Create a new device instance
    /// 
    pub fn create_device(&self, device_def: &serde_json::Value) -> Result<Device, PlatformError> {

        // Try to get the name
        let mut name = String::from("changeme");
        if let Some(value) = device_def.get("name") {
            name = value.as_str().unwrap().to_string();
        }

        // Try to get ref
        let ref_option = device_def.get("ref");
        match ref_option {
            None => {
                // tracing::error!("Device definition does not have a 'ref'");
                // return Err(());
                
                return platform_error!("Device definition does not have a 'ref'", None);
            },
            Some(ref_value) => {

                let producer = self.producers.get(ref_value.as_str().unwrap());
                match producer {
                    None => {
                        // tracing::error!("Producer not found: {}", ref_value);
                        return platform_error!("Producer not found", None);
                    },
                    Some(producer) => {
                        let mut dev = producer.create_device().unwrap();
                        dev.set_name(name);
                        return Ok(dev);
                    }
                }

            }
        }


    }

}



