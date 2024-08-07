pub mod production_order;

use std::collections::HashMap;

use crate::{Device, DeviceMonitor, FactoryLogger, InfoPack, Producer, ProductionOrder, Reactor};

/// Factory to create devices from a configuration json
///
pub struct Factory {
    /// Local logger
    logger: FactoryLogger,
    /// List of known producers
    producers: HashMap<String, Box<dyn Producer>>,
}

impl Factory {
    /// Create a new factory
    ///
    pub fn new() -> Factory {
        // New object
        let obj = Factory {
            logger: FactoryLogger::new(),
            producers: HashMap::new(),
        };
        // Info log
        obj.logger.info("# Device factory initialization");
        // Load builtin device producers
        return obj;
    }

    /// Add multiple producers
    ///
    pub fn add_producers(&mut self, producers: Vec<Box<dyn Producer>>) {
        for producer in producers {
            self.add_producer(producer);
        }
    }

    /// Add a single producer
    pub fn add_producer(&mut self, producer: Box<dyn Producer>) {
        // Info log
        self.logger.info(format!(
            "  - {}.{}",
            producer.manufacturer(),
            producer.model()
        ));

        self.producers.insert(
            format!("{}.{}", producer.manufacturer(), producer.model()),
            producer,
        );
    }

    ///
    /// production_order => json with ref, name, settings
    ///
    pub fn produce(
        &self,
        reactor: Reactor,
        info_pack: Option<InfoPack>,
        production_order: ProductionOrder,
    ) -> (DeviceMonitor, Device) {
        let producer = self.producers.get(production_order.device_ref()).unwrap();
        let device_operations = producer.produce().unwrap();

        // Box<dyn DeviceOperations>

        DeviceMonitor::new(
            reactor.clone(),
            info_pack,
            device_operations,
            production_order,
        )
    }

    // /// Create a new device instance
    // ///
    // pub fn create_device(&self, device_def: &serde_json::Value) -> Result<Device, PlatformError> {
    //     // Try to get the name
    //     // Error if not found or badly formated
    //     let dev_name_opt = device_def.get("name");
    //     if dev_name_opt.is_none() {
    //         return __platform_error_result!("Device definition does not have a 'name'");
    //     }
    //     let dev_name_str = dev_name_opt.unwrap().as_str();
    //     if dev_name_str.is_none() {
    //         return __platform_error_result!("Device definition 'name' is not a string");
    //     }
    //     let dev_name = String::from(dev_name_str.unwrap());

    //     // Try to get ref
    //     // Error if not found or badly formated
    //     let ref_opt = device_def.get("ref");
    //     if ref_opt.is_none() {
    //         return __platform_error_result!("Device definition does not have a 'ref'");
    //     }
    //     let ref_str = ref_opt.unwrap().as_str();
    //     if ref_str.is_none() {
    //         return __platform_error_result!("Device definition 'ref' is not a string");
    //     }
    //     let ref_string = String::from(ref_str.unwrap());

    //     // Default if bench name not found
    //     let bench_name = String::from("default");

    //     let settings = device_def
    //         .get("settings")
    //         .unwrap_or(&serde_json::Value::Null)
    //         .clone();

    //     // Try to get the producer
    //     return self.find_producer_and_produce_device(
    //         &dev_name,
    //         &bench_name,
    //         &ref_string,
    //         settings,
    //     );
    // }

    // /// Create a new device instance with all the required data
    // ///
    // fn produce_device(
    //     dev_name: &String,
    //     bench_name: &String,
    //     producer: &Box<dyn Producer>,
    //     connection_link_manager: &link::AmManager,
    //     settings: serde_json::Value,
    //     platform_services: crate::platform::services::AmServices,
    // ) -> Result<Device, PlatformError> {
    //     let actions = producer.produce();
    //     match actions {
    //         Err(_) => {
    //             return __platform_error_result!("Fail to produce device actions");
    //         }
    //         Ok(actions) => {
    //             return Ok(Device::new(
    //                 dev_name,
    //                 bench_name,
    //                 settings,
    //                 actions,
    //                 connection_link_manager.clone(),
    //                 platform_services,
    //             ));
    //         }
    //     }
    // }

    // pub fn hunters(&self) -> &Vec<Box<dyn Hunter>> {
    //     return &self.hunters;
    // }

    // pub fn create_an_empty_store(&self) -> serde_json::Value {
    //     let mut store_map = serde_json::Map::new();

    //     for producer in &self.producers {
    //         let mut product_map = serde_json::Map::new();

    //         let producer_ref = producer.0;
    //         let producer_obj = producer.1;

    //         product_map.insert("settings_props".to_string(), producer_obj.settings_props());
    //         product_map.insert("instances".to_string(), serde_json::Value::Array(vec![]));

    //         store_map.insert(
    //             producer_ref.to_string(),
    //             serde_json::Value::Object(product_map),
    //         );
    //     }

    //     return serde_json::Value::Object(store_map);
    // }
}
