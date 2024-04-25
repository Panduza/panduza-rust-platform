use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::platform::PlatformError;
use crate::subscription;
use crate::interface::{self, Runner};
use crate::interface::AmInterface;
use crate::interface::AmRunner;
use crate::device::{ Device, traits::DeviceActions, traits::Producer };

use crate::interface::builder::Builder as InterfaceBuilder;
struct PlatformInterfaceSubscriber;


mod itf_bpc;


struct Ka3005;


impl DeviceActions for Ka3005 {

    // fn hunt(&self) -> LinkedList<Value> {
    //     return LinkedList::new();
    // }

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        // list.push(
        //     itf_fake_bpc::build("channel")
        // );

        return Ok(list);
    }
}




pub struct DeviceProducer;

impl Producer for DeviceProducer {

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(Ka3005{}));
    }

}

