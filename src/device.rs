use std::collections::{HashMap, LinkedList};

// use tokio::{task::yield_now, time::{sleep, Duration}};

use crate::connection::SafeLinkConnectionManager;
use crate::builtin_devices;
use crate::interface::SafeInterface;

use crate::connection::SafeConnection;
use crate::connection::LinkInterfaceHandle;

use serde_json::Value;
use tokio::task::JoinSet;


pub trait DeviceActions {

    fn hunt(&self) -> LinkedList<Value>;
    // list de device definition
    //   ref / name / settings
    fn create_interfaces(&self) -> LinkedList<SafeInterface>;

}

pub struct Device {

    task_pool: JoinSet<()>,

    actions: Box<dyn DeviceActions>,

    interfaces: LinkedList<SafeInterface>,

    connections: LinkedList<SafeLinkConnectionManager>

}

impl Device {

    /// Create a new instance of the Device
    pub fn new(actions: Box<dyn DeviceActions>) -> Device {
        return Device {
            task_pool: JoinSet::new(),
            actions: actions,
            interfaces: LinkedList::new(),
            connections: LinkedList::new()
        }
    }




    pub async fn mount_interfaces(&mut self, task_pool: &mut JoinSet<()>) {
        self.interfaces = self.actions.create_interfaces();

        for interface in self.interfaces.iter_mut() {
            let itf = interface.clone();

            for connection in self.connections.iter_mut() {
                let mut interface_lock = interface.lock().await;


                let requests = interface_lock.get_subscription_requests().await;

                let x: LinkInterfaceHandle = connection.lock().await.request_link(requests).await.unwrap();

                interface_lock.add_link(x).await;
            }

            itf.lock().await.start(&mut self.task_pool).await;
        }


    }


    pub async fn attach_connection(&mut self, connection: SafeConnection) {
        self.connections.push_back(connection.lock().await.clone_link_manager());
    }

}



// pub trait DeviceCallbacks {
//     fn get_name(&self) -> &str;

    
//     // hunt
//     fn mount_interfaces(&self, task_pool: &mut tokio::task::JoinSet<()>);
    
// }


pub trait Producer {
    fn create_device(&self) -> Result<Device, String>;
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


    pub fn create_device(&self, device_ref: &str) -> Result<Device, String> {
        return self.producers.get(device_ref).unwrap().create_device();
    }

}



// ------------------------------------------------------------------------------------------------

pub struct Manager {
    
    // Device factory
    factory: Factory,

    // Lits of device instances
    instances: HashMap<String, Device>,

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

    pub async fn create_device(&mut self, device_name: &str, device_ref: &str) {

        let device = self.factory.create_device(device_ref);

        self.instances.insert(device_name.to_string(), device.unwrap());
    }




    pub async fn mount_devices(&mut self, task_pool: &mut JoinSet<()>)
    {
        for(_, device) in self.instances.iter_mut() {
            device.mount_interfaces(task_pool).await;
        }
    }


    pub fn get_device(&mut self, device_ref: &String) -> Option<&mut Device> {
        return self.instances.get_mut(device_ref);
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

