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




struct FakeBpcActions {
    enable_value: bool,
    voltage_value: f32,
    current_value: f32,
}

#[async_trait]
impl bpc::BpcActions for FakeBpcActions {

    async fn initializating(&mut self, core: &interface::AmCore) -> Result<(), PlatformError> {
        
        return Ok(());
    }

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, core: &interface::AmCore) -> Result<bool, PlatformError> {
        core.lock().await.log_info(
            format!("FakeBpc - read_enable_value: {}", self.enable_value)
        );
        return Ok(self.enable_value);
    }

    async fn write_enable_value(&mut self, core: &interface::AmCore, v: bool) {
        core.lock().await.log_info(
            format!("FakeBpc - write_enable_value: {}", self.enable_value)
        );
        self.enable_value = v;
    }

    /// Read the voltage value
    /// 
    async fn read_voltage_value(&mut self, core: &interface::AmCore) -> Result<f32, PlatformError> {
        core.lock().await.log_info(
            format!("FakeBpc - read_voltage_value: {}", self.voltage_value)
        );
        return Ok(self.voltage_value);
    }

    async fn write_voltage_value(&mut self, core: &interface::AmCore, v: f32) {
        println!("write_voltage_value: {}", v);
    }
 
    async fn read_current_value(&mut self, core: &interface::AmCore) -> Result<f32, PlatformError> {
        core.lock().await.log_info(
            format!("FakeBpc - read_current_value: {}", self.current_value)
        );
        return Ok(self.current_value);
    }

    async fn write_current_value(&mut self, core: &interface::AmCore, v: f32) {

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
            enable_value: false,
            voltage_value: 0.0,
            current_value: 0.0,
        })
    )
}

