
use crate::interface::Builder as InterfaceBuilder;
use crate::platform::PlatformError;
use crate::device::{ traits::DeviceActions, traits::Producer };

struct PlatformInterfaceSubscriber;


mod itf_fake_relay;


struct FakeRelay;


impl DeviceActions for FakeRelay {

    // fn hunt(&self) -> LinkedList<Value> {
    //     return LinkedList::new();
    // }

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        list.push(
            itf_fake_relay::build("channel")
        );

        return Ok(list);
    }
}




pub struct DeviceProducer;

impl Producer for DeviceProducer {

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(FakeRelay{}));
    }

}

