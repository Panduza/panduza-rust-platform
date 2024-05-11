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


mod itf_voxpower_inhibiter;


struct VoxpowerInhibiter;


impl DeviceActions for VoxpowerInhibiter {

    /// Create the interfaces
    fn interface_builders(&self, device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        for n in 2..10 {
            list.push(
                itf_voxpower_inhibiter::build(format!("channel_{}", n))
            );
        }

        return Ok(list);
    }
}




pub struct DeviceProducer;

impl Producer for DeviceProducer {

    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(VoxpowerInhibiter{}));
    }

}

