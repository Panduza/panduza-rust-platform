

// ------------------------------------------------------------------------------------------------

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{platform::{PlatformError, TaskPoolLoader}, platform_error};

use super::{factory::Factory, Device};

pub struct Manager {
    
    // Device factory
    factory: Factory,

    // Lits of device instances
    instances: HashMap<String, Device>,

    task_loader: TaskPoolLoader

}
pub type AmManager = Arc<Mutex<Manager>>;

impl Manager {

    pub fn new(task_loader: TaskPoolLoader) -> AmManager {
        return Arc::new(Mutex::new(Manager {
            factory: Factory::new(),
            instances: HashMap::new(),
            task_loader: task_loader
        }));
    }

    // pub fn add_producer(&mut self, device_ref: &str, producer: Box<dyn Producer>) {
    //     self.factory.add_producer(device_ref, producer);
    // }

    // pub async fn create_device(&mut self, device_name: &str, device_ref: &str) {

    //     let device = self.factory.create_device(device_ref);

    //     self.instances.insert(device_name.to_string(), device.unwrap());
    // }


    /// Create a new device instance
    /// 
    pub async fn create_device(&mut self, device_def: &serde_json::Value) -> Result<(), PlatformError> {

        // Debug log
        tracing::debug!(class="Platform", "Create device: {:?}", device_def);

        let dev = self.factory.create_device(device_def);
        match dev {
            Err(e) => {
                return platform_error!("Device not created", Some(Box::new(e)));
            },
            Ok(dev) => {

                self.instances.insert(dev.get_name().clone(), dev);

            }
        }



        return  Ok(());

    }



    pub async fn mount_devices(&mut self)
    {
        for(_, device) in self.instances.iter_mut() {
            device.mount_interfaces(&mut self.task_loader).await;
        }
    }


    pub fn get_device(&mut self, device_ref: String) -> Option<&mut Device> {
        return self.instances.get_mut(&device_ref);
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

