use std::collections::HashMap;

use crate::builtin_devices;
use crate::platform_error;
use crate::platform::PlatformError;

use super::Device;
use crate::device::traits::Producer;


/// Factory to create devices from a configuration json
/// 
pub struct Factory {
    /// List of known producers
    /// 
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

    /// Create a new device instance
    /// 
    pub fn create_device(&self, device_def: &serde_json::Value) -> Result<Device, PlatformError> {

        // Try to get the name
        // 
        let mut dev_name = String::new();

        let dev_name_value = device_def.get("name");
        match dev_name_value {
            None => {
                return platform_error!("Device definition does not have a 'name'", None);
            },
            Some(value) => {
                dev_name = value.as_str().unwrap().to_string();
            }
        }

        // Default if bench name not found
        let bench_name = String::from("default");

        // Try to get ref
        let ref_option = device_def.get("ref");
        match ref_option {
            None => {
                return platform_error!("Device definition does not have a 'ref'", None);
            },
            Some(ref_value) => {

                return self.find_producer_and_produce_device(&name, &bench_name, &ref_value.as_str().unwrap().to_string());

            }
        }


    }

    /// Find the producer and produce the device
    /// 
    fn find_producer_and_produce_device(&self,
        dev_name: &String, bench_name: &String, device_ref: &String
    )
        -> Result<Device, PlatformError>
    {
        let producer = self.producers.get(device_ref);
        match producer {
            None => {
                return platform_error!("Producer not found", None);
            },
            Some(producer) => {
                return Self::produce_device(dev_name, bench_name, producer);
            }
        }
    }

    /// Create a new device instance with all the required data
    ///
    fn produce_device(dev_name: &String, bench_name: &String, producer: &Box<dyn Producer>)
        -> Result<Device, PlatformError>
    {
        let actions = producer.produce();
        match actions {
            Err(e) => {
                return platform_error!("Fail to produce device actions", Some(Box::new(e)));
            },
            Ok(actions) => {
                return Ok(Device::new(dev_name, bench_name, actions));
            }
        }
    }
    
}


