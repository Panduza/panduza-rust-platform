use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, DeviceActions, Producer };



mod itf_platform;

struct ServerDeviceActions;


impl DeviceActions for ServerDeviceActions {

    // fn hunt(&self) -> LinkedList<Value> {
    //     return LinkedList::new();
    // }


    /// Create the interfaces
    /// 
    fn create_interfaces(&self, settings: &serde_json::Value)
        -> Vec<AmInterface> {
        let mut list = Vec::new();
        list.push(
            itf_platform::new("platform")
        );

        return list;
    }
}


pub struct DeviceProducer {

}

impl Producer for DeviceProducer {

    fn create_device(&self) -> Result<Device, String> {
        return Ok(Device::new(Box::new(ServerDeviceActions{})));
    }

}

