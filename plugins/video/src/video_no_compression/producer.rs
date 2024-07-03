use serde_json::json;

use panduza_core::Error as PlatformError;
use panduza_core::device::{ traits::DeviceActions, traits::Producer };

use super::device::Video;

pub struct DeviceProducer;

impl DeviceProducer {
    pub fn new_boxed() -> Box<dyn Producer> {
        return Box::new(DeviceProducer{});
    }
}

impl Producer for DeviceProducer {

    fn settings_props(&self) -> serde_json::Value {
        return json!([
        ]);
    }


    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError> {
        return Ok(Box::new(Video{}));
    }

}