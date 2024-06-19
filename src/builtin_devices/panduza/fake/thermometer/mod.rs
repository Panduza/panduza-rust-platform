use serde_json::json;

use panduza_core::Error as PlatformError;
use panduza_core::device::{ traits::DeviceActions, traits::Producer };

use panduza_core::interface::builder::Builder as InterfaceBuilder;


mod itf_fake_thermometer;


struct FakeThermometer;


impl DeviceActions for FakeThermometer {

    /// Create the interfaces
    fn interface_builders(&self, _device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        list.push(
            itf_fake_thermometer::build("channel")
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
        return Ok(Box::new(FakeThermometer{}));
    }

}

