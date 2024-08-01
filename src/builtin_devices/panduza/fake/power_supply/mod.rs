
use panduza_core::device::Device;
use serde_json::json;

use panduza_core::Error as PlatformError;
// use panduza_core::interface;
use panduza_core::device::{ traits::DeviceActions, traits::Producer };

use panduza_core::attribute::*;
use tracing::instrument::WithSubscriber;

use panduza_core::interface::interface::*;

pub struct FakePowerSupply  {
    base: Device
}

impl DeviceActions for FakePowerSupply {

    /// Create the interfaces
    fn interface_builders(&self, _device: &Device) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {

        let enable: AttributeBool = AttributeBoolBuilder::new()
            .with_name("enable")
            .build();

        let control: InterfaceControl = InterfaceBuilder::new()
            .with_name("control")
            .with_attribute(enable)
            .build();

        

        let list = Vec::new();
        //list.push(
        //    itf_fake_bpc::build("channel")
        //);

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
