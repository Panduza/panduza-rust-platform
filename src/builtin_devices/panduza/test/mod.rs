
use serde_json::json;

use crate::interface::builder::Builder as InterfaceBuilder;
use crate::platform::PlatformError;
use crate::device::{ traits::DeviceActions, traits::Producer };



mod itf_ping;

struct TestDeviceActions;


impl DeviceActions for TestDeviceActions {


    /// Create the interfaces
    /// 
    fn interface_builders(&self, _device_settings: &serde_json::Value) 
        -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        list.push(
            itf_ping::new("ping_0")
        );

        return Ok(list);
    }
}


pub struct DeviceProducer;
impl Producer for DeviceProducer {

    
    fn settings_props(&self) -> serde_json::Value {
        return json!([
        ]);
    }

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(TestDeviceActions{}));
    }

}

