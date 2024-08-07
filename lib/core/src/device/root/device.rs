use std::time::Duration;

use async_trait::async_trait;
use tokio::time::sleep;

use crate::{Device, DeviceOperations, Error};

///
/// Main device of the platform
/// Provides the root informations about the platform
///
pub struct RootDevice {}

impl RootDevice {
    pub fn new() -> RootDevice {
        RootDevice {}
    }
}

#[async_trait]
impl DeviceOperations for RootDevice {
    ///
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
