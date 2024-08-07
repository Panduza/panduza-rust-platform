use super::device::RootDevice;
use crate::{DeviceOperations, Error, Producer};

pub struct RootProducer {}

impl RootProducer {
    pub fn new() -> Box<RootProducer> {
        Box::new(RootProducer {})
    }
}

impl Producer for RootProducer {
    fn manufacturer(&self) -> String {
        "_".to_string()
    }

    fn model(&self) -> String {
        "_".to_string()
    }

    fn produce(&self) -> Result<Box<dyn DeviceOperations>, Error> {
        return Ok(Box::new(RootDevice::new()));
    }
}
