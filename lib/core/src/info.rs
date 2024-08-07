mod devices;
mod pack;

use std::time::Duration;

use async_trait::async_trait;
use pack::InfoPack;
use tokio::time::sleep;

use crate::{device, Device, DeviceOperations, Error};

///
/// Main device of the platform
/// Provides the informations about the platform
///
pub struct InfoDevice {
    ///
    /// Object that allow other elements of the platform to
    /// communicate with this device
    ///
    pack: InfoPack,
}

impl InfoDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> (InfoDevice, InfoPack) {
        let pack = InfoPack::new();

        let device = InfoDevice { pack: pack.clone() };

        (device, pack)
    }
}

#[async_trait]
impl DeviceOperations for InfoDevice {
    ///
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        //
        //
        let mut interface_devices = device.create_interface("devices").finish();

        // spawn loop
        //      wait notify on devices structure
        //      add request => create attribute for the device name
        //      validate request in info pack => info pack must have a fifo of request

        // Chaque nouvelle interface pour le nouveau device attend la notif de sa structure

        //

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
