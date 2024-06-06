use async_trait::async_trait;
use std::mem::swap;

use crate::platform::PlatformError;
use crate::meta::video;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


/// Fake Laser Channel Data
/// 
struct VideoActions {
    mode_value: Bytes,
}

#[async_trait]
impl video::VideoActions for VideoActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the mode value
    /// 
    async fn read_mode_value(&mut self, interface: &AmInterface) -> Result<String, PlatformError> {
        interface.lock().await.log_info(
            format!("Video - read_mode_value: {}", self.mode_value)
        );

        let mut mode_val = String::new();
        swap(&mut mode_val, &mut self.mode_value);
        return Ok(mode_val);
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

