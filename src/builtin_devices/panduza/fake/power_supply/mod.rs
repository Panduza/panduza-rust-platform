use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, DeviceActions, Producer };

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
        // list.push(
        //     Interface::new(
        //         "platform", dev_name, bench_name,
        //         Box::new(TestIdentityProvider{}),
        //         Box::new(TestInterfaceStates{}),
        //         Box::new(PlatformInterfaceSubscriber{})
        //     )
        // );

        return list;
    }
}


pub struct DeviceProducer {

}

impl Producer for DeviceProducer {

    fn create_device(&self) -> Result<Device, String> {
        return Ok(Device::new(Box::new(FakePowerSupply{})));
    }

}

