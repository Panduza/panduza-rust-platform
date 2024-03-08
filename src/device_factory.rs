
use std::collections::HashMap;


// pub mod math { 


pub trait Device {
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str;
}


pub trait DeviceProducer {
    fn create_device(&self, device_ref: &str) -> Result<Box<dyn Device>, String>;
}


pub struct DeviceFactory {
    producers : HashMap<String, Box<dyn DeviceProducer>>
    
}

impl DeviceFactory {

    pub fn new() -> DeviceFactory {   
        return DeviceFactory {
            producers: HashMap::new()
        }
    }


    // append_producer

}


// }
