
use serde_json::json;

use crate::platform::PlatformError;
// use crate::interface;
use crate::device::{ traits::DeviceActions, traits::Producer };

use crate::interface::builder::Builder as InterfaceBuilder;


mod itf_fake_bpc;
mod itf_fake_voltmeter;


struct FakePowerSupply;


impl DeviceActions for FakePowerSupply {

    /// Create the interfaces
    fn interface_builders(&self, _device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        list.push(
            itf_fake_bpc::build("channel")
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
        return Ok(Box::new(FakePowerSupply{}));
    }

}

