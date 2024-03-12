use crate::device::DeviceCallbacks;
use crate::device::Producer;






struct ServerDevice{

}

impl DeviceCallbacks for ServerDevice {
    fn get_name(&self) -> &str {
        return "aaa";
    }

    fn mount_interfaces(&self, task_pool: &mut tokio::task::JoinSet<()>)
    {
        println!("mounting interfaces");
    }

}


pub struct DeviceProducer {

}

impl Producer for DeviceProducer {

    fn create_device(&self) -> Result<Box<dyn DeviceCallbacks>, String> {
        return Ok(Box::new(ServerDevice{}));
    }

}

