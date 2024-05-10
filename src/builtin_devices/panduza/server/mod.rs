
use crate::interface::builder::Builder as InterfaceBuilder;
use crate::platform::PlatformError;
use crate::device::{ traits::DeviceActions, traits::Producer };



mod itf_platform;

struct ServerDeviceActions;


impl DeviceActions for ServerDeviceActions {

    fn hunt(&self) -> Option<Vec<serde_json::Value>> {
        // let mut list = Vec::new();
        // list.push(serde_json::json!({
        //     "name": "Server",
        //     "type": "platform",
        //     "id": "server",
        //     "settings": {
        //         "host": "localhost",
        //         "port": 8080
        //     }
        // }));
        return None;
    }


    /// Create the interfaces
    /// 
    fn interface_builders(&self, _device_settings: &serde_json::Value) 
        -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        list.push(
            itf_platform::new("platform")
        );

        return Ok(list);
    }
}


pub struct DeviceProducer;
impl Producer for DeviceProducer {

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(ServerDeviceActions{}));
    }

}

