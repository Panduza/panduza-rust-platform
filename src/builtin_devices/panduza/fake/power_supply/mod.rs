use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::platform::PlatformError;
use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, traits::DeviceActions, traits::Producer };

struct PlatformInterfaceSubscriber;


mod itf_fake_bpc;
mod itf_fake_voltmeter;


struct FakePowerSupply;


impl DeviceActions for FakePowerSupply {

    // fn hunt(&self) -> LinkedList<Value> {
    //     return LinkedList::new();
    // }

    /// Create the interfaces
    fn create_interfaces(&self, settings: &serde_json::Value)
        -> Vec<AmInterface> {
        let mut list = Vec::new();
        list.push(
            itf_fake_bpc::new()
        );

        return list;
    }
}




pub struct DeviceProducer;

impl Producer for DeviceProducer {

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(FakePowerSupply{}));
    }

}

