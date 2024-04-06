use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use crate::platform::PlatformError;
use crate::subscription;
use crate::interface::{self, Interface};
use crate::interface::core::AmCore;
use crate::interface::AmInterface;
use crate::device::{ Device, traits::DeviceActions, traits::Producer };


use crate::meta::bpc;


use crate::interface::builder::Builder as InterfaceBuilder;




struct FakeBpcActions;

#[async_trait]
impl bpc::BpcActions for FakeBpcActions {

    async fn read_enable_value(&self) -> Result<bool, PlatformError> {
        return Ok(true);
    }

    async fn write_enable_value(&self, v: bool) {
        println!("write_enable_value: {}", v);
    }

    async fn read_voltage_value(&self) -> Result<f32, PlatformError> {
        return Ok(3.3);
    }

    async fn write_voltage_value(&self, v: f32) {
        println!("write_voltage_value: {}", v);
    }

}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    return bpc::build(
        name, 
        bpc::BpcParams {
            voltage_min: 0.0,
            voltage_max: 5.0,
        }, 
    Box::new(FakeBpcActions {

        })
    )
}

