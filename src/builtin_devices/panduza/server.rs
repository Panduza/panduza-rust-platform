use crate::device::Device;
use crate::device::Producer;






struct ServerDevice{

}

impl Device for ServerDevice {
    fn get_name(&self) -> &str {
        return "aaa";
    }

}


pub struct DeviceProducer {

}

impl Producer for DeviceProducer {

    fn create_device(&self) -> Result<Box<dyn Device>, String> {
        return Ok(Box::new(ServerDevice{}));
    }

}

