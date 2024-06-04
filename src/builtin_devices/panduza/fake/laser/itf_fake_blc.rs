use async_trait::async_trait;
use std::mem::swap;

use crate::platform::PlatformError;
use crate::meta::blc;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


/// Fake Laser Channel Data
/// 
struct FakeBlcActions {
    mode_value: String,
    enable_value: bool,
    power_value: f64,
    current_value: f64,
}

#[async_trait]
impl blc::BlcActions for FakeBlcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the mode value
    /// 
    async fn read_mode_value(&mut self, interface: &AmInterface) -> Result<String, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeBpc - read_mode_value: {}", self.mode_value)
        );

        let mut mode_val = String::new();
        swap(&mut mode_val, &mut self.mode_value);
        return Ok(mode_val);
    }

    async fn write_mode_value(&mut self, interface: &AmInterface, v: String) {
        interface.lock().await.log_info(
            format!("FakeBpc - write_mode_value: {}", self.mode_value)
        );
        self.mode_value = v;
    }

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeBpc - read_enable_value: {}", self.enable_value)
        );
        return Ok(self.enable_value);
    }

    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {
        interface.lock().await.log_info(
            format!("FakeBpc - write_enable_value: {}", self.enable_value)
        );
        self.enable_value = v;
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeBlc - read_power_value: {}", self.power_value)
        );
        return Ok(self.power_value);
    }

    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) {
        interface.lock().await.log_info(
            format!("FakeBlc - write_power_value: {}", v)
        );
        self.power_value = v;
    }
 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        interface.lock().await.log_info(
            format!("FakeBlc - read_current_value: {}", self.current_value)
        );
        return Ok(self.current_value);
    }

    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) {
        interface.lock().await.log_info(
            format!("FakeBlc - write_current_value: {}", v)
        );
        self.current_value = v;
    }

}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A
) -> InterfaceBuilder {

    return blc::build(
        name, 
        blc::BlcParams {
            power_min: 0.0,
            power_max: 0.3,
            power_decimals: 3,

            current_min: 0.0,
            current_max: 0.5,
            current_decimals: 1,
        }, 
        Box::new(FakeBlcActions {
            mode_value: "no_regulation".to_string(),
            enable_value: false,
            power_value: 0.0,
            current_value: 0.0,
        })
    )
}

