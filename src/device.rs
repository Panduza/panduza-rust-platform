
use std::collections::HashMap;


// pub struct Device {

// }


pub trait Device {
    fn get_name(&self) -> &str;
}




pub trait Producer {
    fn create_device(&self, device_ref: &str) -> Result<Box<dyn Device>, String>;
}

struct CustommmDevice{

}

impl Device for CustommmDevice {
    fn get_name(&self) -> &str {
        return "aaa";
    }

}

struct CustommmProducer{

}

impl Producer for CustommmProducer {

    fn create_device(&self, device_ref: &str) -> Result<Box<dyn Device>, String> {
        return Ok(Box::new(CustommmDevice{}));
    }

}

pub struct Factory {
    producers : HashMap<String, Box<dyn Producer>>
    
}

impl Factory {

    pub fn new() -> Factory {   
        return Factory {
            producers: HashMap::new()
        }
    }

    pub fn add_producer(&mut self, device_ref: String, producer: Box<dyn Producer>) {
        self.producers.insert(device_ref, producer);
    }

    // pub fn get_producer(self, device_ref: String) -> Box<dyn Producer> {
    //     return self.producers.get(device_ref);
    // }

}

