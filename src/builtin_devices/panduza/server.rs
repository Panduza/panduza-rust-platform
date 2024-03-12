use crate::device::Device;
use crate::device::Producer;






struct ServerDevice{

}

impl Device for ServerDevice {
    fn get_name(&self) -> &str {
        return "aaa";
    }

    fn mount_interfaces(&self, task_pool: &mut tokio::task::JoinSet<()>)
    {
        println!("mounting interfaces");
    }

    fn unmount_interfaces(&self)
    {

    }
}


pub struct DeviceProducer {

}

impl Producer for DeviceProducer {

    fn create_device(&self) -> Result<Box<dyn Device>, String> {
        return Ok(Box::new(ServerDevice{}));
    }

}

