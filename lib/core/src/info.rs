pub mod devices;
pub mod pack;

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

        //
        // Here the device interface must provide an attribute for each device mounted on the platform
        // When the device boot, it must send a creation request to this task and wait for the 'InfoDevice'
        // a validation. Once validated, the device can continue to run and report its status through an 'Arc<Mutex<InfoDev"
        //
        let pack_clone = self.pack.clone();
        device
            .spawn(async move {
                let new_request = pack_clone.new_request_notifier().await;

                loop {
                    let devices = pack_clone.devices();
                    let request = devices.lock().await.pop_next_request();
                    match request {
                        Some(r) => {
                            // Here I must create a attribute inside interface_devices
                            // when the request is a creation request
                            // else delete the object
                            devices.lock().await.validate_request(r);
                        }
                        None => {}
                    }
                    //
                    // Wait for more request
                    new_request.notified().await;
                }

                Ok(())
            })
            .await;

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
