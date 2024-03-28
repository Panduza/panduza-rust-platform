

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
    /// return the name of the device if success
    ///
    pub async fn create_device(&mut self, device_def: &serde_json::Value) -> Result<String, PlatformError> {
        // Debug log
        tracing::debug!(class="Platform", " - Try to create device -\n{}", serde_json::to_string_pretty(&device_def).unwrap() );

        // Create the device
        let result = self.factory.create_device(device_def);
        match result {
            Err(e) => {
                return platform_error!("Device not created", Some(Box::new(e)));
            },
            Ok(device_object) => {
                let name = device_object.get_name().clone();
                self.instances.insert(device_object.get_name().clone(), device_object);
                return Ok(name);
            }
        }
    }



    pub async fn start_devices(&mut self)
    {
        for(_, device) in self.instances.iter_mut() {
            device.start_interfaces(&mut self.task_loader).await;
        }
    }


    pub fn get_device(&mut self, device_ref: String) -> Option<&mut Device> {
        return self.instances.get_mut(&device_ref);
    }


}
