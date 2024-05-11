use std::collections::HashMap;
use std::vec;

use super::Device;

use crate::link;

use crate::device::traits::Producer;

use crate::device::traits::Hunter;

use crate::platform_error;
use crate::platform::PlatformError;

use crate::builtin_devices;

/// Factory to create devices from a configuration json
/// 
pub struct Factory {
    /// List of known producers
    /// 
    producers: HashMap<String, Box<dyn Producer>>,


    hunters: Vec<Box<dyn Hunter>>,

    /// Connection link manager
    /// 
    connection_link_manager: Option<link::AmManager>,

    platform_services: crate::platform::services::AmServices
}

impl Factory {

    /// Create a new factory
    /// 
    pub fn new(platform_services: crate::platform::services::AmServices) -> Factory {
        // New object
        let mut obj = Factory {
            producers: HashMap::new(),
            hunters: Vec::new(),
            connection_link_manager: None,
            platform_services: platform_services
        };

        // Info log
        tracing::info!(class="Factory", "# Device factory initialization");
        tracing::info!(class="Factory", "List of producers:");

        // Load builtin device producers
        builtin_devices::import_plugin_producers(&mut obj);
        return obj;
    }

    /// Set the connection link manager
    /// 
    pub fn set_connection_link_manager(&mut self, connection_link_manager: link::AmManager) {
        self.connection_link_manager = Some(connection_link_manager);
    }

    /// Add a producer to the factory
    /// 
    pub fn add_producer(&mut self, device_ref: &str, producer: Box<dyn Producer>) {
        // Info log
        tracing::info!(class="Factory", "  - {}", device_ref);

        // Append the producer
        self.producers.insert(device_ref.to_string(), producer);
    }

    /// Add a hunter to the factory
    /// 
    pub fn add_hunter(&mut self, hunter: Box<dyn Hunter>) {
        // Info log
        tracing::info!(class="Factory", "  - new hunter");

        // Append the producer
        self.hunters.push(hunter);
    }

    /// Create a new device instance
    /// 
    pub fn create_device(&self, device_def: &serde_json::Value) -> Result<Device, PlatformError> {

        // Try to get the name
        // Error if not found or badly formated
        let dev_name_opt = device_def.get("name");
        if dev_name_opt.is_none() {
            return platform_error!("Device definition does not have a 'name'", None);
        }
        let dev_name_str = dev_name_opt.unwrap().as_str();
        if dev_name_str.is_none() {
            return platform_error!("Device definition 'name' is not a string", None);
        }
        let dev_name = String::from(dev_name_str.unwrap());

        // Try to get ref
        // Error if not found or badly formated
        let ref_opt = device_def.get("ref");
        if ref_opt.is_none() {
            return platform_error!("Device definition does not have a 'ref'", None);
        }
        let ref_str = ref_opt.unwrap().as_str();
        if ref_str.is_none() {
            return platform_error!("Device definition 'ref' is not a string", None);
        }
        let ref_string = String::from(ref_str.unwrap());

        // Default if bench name not found
        let bench_name = String::from("default");


        let settings = device_def.get("settings")
            .unwrap_or(&serde_json::Value::Null)
            .clone();

        // Try to get the producer
        return self.find_producer_and_produce_device(&dev_name, &bench_name, &ref_string, settings);
    }

    /// Find the producer and produce the device
    /// 
    fn find_producer_and_produce_device(&self,
        dev_name: &String, bench_name: &String, device_ref: &String, settings: serde_json::Value
    )
        -> Result<Device, PlatformError>
    {
        let producer = self.producers.get(device_ref);
        match producer {
            None => {
                let error_text = format!("Producer not found for {}", device_ref);
                return platform_error!(error_text , None);
            },
            Some(producer) => {
                return Self::produce_device(dev_name, bench_name, producer, self.connection_link_manager.as_ref().unwrap(), settings
                , self.platform_services.clone());
            }
        }
    }

    /// Create a new device instance with all the required data
    ///
    fn produce_device(dev_name: &String, bench_name: &String, producer: &Box<dyn Producer>, connection_link_manager: &link::AmManager, settings: serde_json::Value
        ,platform_services: crate::platform::services::AmServices)
        -> Result<Device, PlatformError>
    {
        let actions = producer.produce();
        match actions {
            Err(e) => {
                return platform_error!("Fail to produce device actions", Some(Box::new(e)));
            },
            Ok(actions) => {
                return Ok(Device::new(dev_name, bench_name, settings, actions, connection_link_manager.clone()
                , platform_services));
            }
        }
    }
    

    pub fn hunters(&self) -> &Vec<Box<dyn Hunter>> {
        return &self.hunters;
    }


}


