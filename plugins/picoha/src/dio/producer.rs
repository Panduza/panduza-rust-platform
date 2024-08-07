use super::device::PicoHaDioDevice;
use panduza_platform_core::{DeviceOperations, Producer};

pub struct PiochaDio {}

impl PiochaDio {
    pub fn new() -> Box<PiochaDio> {
        Box::new(PiochaDio {})
    }
}

impl Producer for PiochaDio {
    fn manufacturer(&self) -> String {
        "panduza".to_string()
    }

    fn model(&self) -> String {
        "picoha-dio".to_string()
    }

    fn produce(&self) -> Result<Box<dyn DeviceOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(PicoHaDioDevice::new()));
    }
}
