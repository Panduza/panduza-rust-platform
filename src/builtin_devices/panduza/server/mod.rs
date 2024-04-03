use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::interface::builder::Builder as InterfaceBuilder;
use crate::platform::PlatformError;
use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, traits::DeviceActions, traits::Producer };



mod itf_platform;

struct ServerDeviceActions;


impl DeviceActions for ServerDeviceActions {

    // fn hunt(&self) -> LinkedList<Value> {
    //     return LinkedList::new();
    // }


    /// Create the interfaces
    /// 
    fn interface_builders(&self, device_settings: &serde_json::Value) 
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

