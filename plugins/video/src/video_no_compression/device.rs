use panduza_core::Error as PlatformError;
use panduza_core::device::traits::DeviceActions;

use panduza_core::interface::builder::Builder as InterfaceBuilder;

use super::itf_video;

pub struct Video;

impl DeviceActions for Video {

    /// Create the interfaces
    fn interface_builders(&self, _device_settings: &serde_json::Value) 
    -> Result<Vec<InterfaceBuilder>, PlatformError>
    {
        let mut list = Vec::new();
        list.push(
            itf_video::build("channel")
        );

        return Ok(list);
    }
    
}