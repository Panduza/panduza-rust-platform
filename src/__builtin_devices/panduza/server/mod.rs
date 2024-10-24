
use panduza_core::device::Device;
use serde_json::json;

use panduza_core::interface::builder::Builder as InterfaceBuilder;
use panduza_core::Error as PlatformError;
use panduza_core::device::{ traits::DeviceActions, traits::Producer };



mod itf_platform;

struct ServerDeviceActions;


impl DeviceActions for ServerDeviceActions {


    /// Create the interfaces
    /// 
    fn interface_builders(&self, _: &Device) 
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

    
    fn settings_props(&self) -> serde_json::Value {
        return json!([
        ]);
    }

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(ServerDeviceActions{}));
    }

}

