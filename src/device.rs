use std::collections::{HashMap, LinkedList};


use crate::interfaces::Fsm as InterfaceFsm;
use crate::builtin_devices;

use serde_json::{Value};


pub trait DeviceCallbacks {

    fn hunt(&self) -> LinkedList<Value>;
    // list de device definition
    //   ref / name / settings
    fn create_interfaces(&self) -> LinkedList<InterfaceFsm>;
    
}

pub struct Device {
    
    interfaces: LinkedList<InterfaceFsm>
}

impl Device {

    /// Create a new instance of the Device
    pub fn new() -> Device {
        return Device {
            interfaces: LinkedList::new()
        }
    }
}



// pub trait DeviceCallbacks {
//     fn get_name(&self) -> &str;

    
//     // hunt
//     fn mount_interfaces(&self, task_pool: &mut tokio::task::JoinSet<()>);
    
// }


pub trait Producer {
    // fn create_device(&self) -> Result<Box<dyn DeviceCallbacks>, String>;
}

pub struct Factory {
    producers : HashMap<String, Box<dyn Producer>>
    
}

impl Factory {

    pub fn new() -> Factory {
        let mut obj = Factory {
            producers: HashMap::new()
        };

        // Load builtin device producers
        builtin_devices::import_plugin_producers(&mut obj);

        return obj;
    }

    pub fn add_producer(&mut self, device_ref: &str, producer: Box<dyn Producer>) {
        // Info log
        tracing::info!("Add producer to factory: {}", device_ref);

        // Append the producer
        self.producers.insert(device_ref.to_string(), producer);
    }


    // pub fn get_producer(self, device_ref: String) -> Result<Box<dyn Producer>, String>  {
    // }


    // pub fn create_device(&self, device_ref: &str) -> Result<Box<dyn DeviceCallbacks>, String> {

    //     // return Ok(
    //         // return 
    //         // self.producers.get(device_ref).unwrap().create_device();
    //     // )

    // }

}



// ------------------------------------------------------------------------------------------------

pub struct Manager {
    
    // Device factory
    factory: Factory,

    // Lits of device instances
    instances: HashMap<String, Box<dyn DeviceCallbacks>>,
    
}

impl Manager {

    pub fn new() -> Manager {
        return Manager {
            factory: Factory::new(),
            instances: HashMap::new()
        }
    }

    // pub fn add_producer(&mut self, device_ref: &str, producer: Box<dyn Producer>) {
    //     self.factory.add_producer(device_ref, producer);
    // }

    pub fn create_device(&mut self, device_name: &str, device_ref: &str) {
        // let device = self.factory.create_device(device_ref);

        // self.instances.insert(device_name.to_string(), device.unwrap());
    }




    pub fn mount_devices(&mut self, task_pool: &mut tokio::task::JoinSet<()>)
    {
        // for(_, device) in &self.instances {
        //     device.mount_interfaces(task_pool);
        // }
    }

    // pub fn get_device(&self, device_ref: &String) -> Option<&Box<dyn Device>> {
    //     return self.instances.iter().find(|&x| x.get_name() == device_ref);
    // }

    // pub fn get_devices(&self) -> &LinkedList<Box<dyn Device>> {
    //     return &self.instances;
    // }

    // pub fn get_factory(&self) -> &Factory {
    //     return &self.factory;
    // }

    // pub fn get_factory_mut(&mut self) -> &mut Factory {
    //     return &mut self.factory;
    // }

    // pub fn work(&mut self) {
    //     // Info log
    //     tracing::info!("Device Manager Starting...");

    //     // Create a device
    //     let device = self.create_device(&"panduza.server".to_string()).unwrap();

    //     // Info log
    //     tracing::info!("Device created: {}", device.get_name());

    //     // Get the device
    //     let device = self.get_device(&"panduza.server".to_string()).unwrap();

    //     // Info log
    //     tracing::info!("Device found: {}", device.get_name());

    //     // Get the factory
    //     let factory = self.get_factory();

    //     // Info log
    //     tracing::info!("Factory found: {:?}", factory);

    //     // Get the factory
    //     let factory = self.get_factory_mut();

    //     // Info log
    //     tracing::info!("Factory found: {:?}", factory);

    //     // Info log
    //     tracing::info!("Device Manager Stopping...");
    // }

}

