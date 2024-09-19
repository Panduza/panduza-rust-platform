pub mod devices;
pub mod pack;

use std::{collections::HashMap, sync::Arc, time::Duration};

use async_trait::async_trait;
use futures::lock::Mutex;
use pack::InfoPack;
use serde_json::json;
use tokio::time::sleep;

use crate::{AttOnlyMsgAtt, Device, DeviceOperations, Error, JsonCodec};

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

    ///
    /// Each device have an attribute to share its state
    /// This Map hold those attribute, the name of the device is the key.
    ///
    devices_status_attributes: Arc<Mutex<HashMap<String, AttOnlyMsgAtt<JsonCodec>>>>,
}

impl InfoDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> (InfoDevice, InfoPack) {
        let pack = InfoPack::new();

        let device = InfoDevice {
            pack: pack.clone(),
            devices_status_attributes: Arc::new(Mutex::new(HashMap::new())),
        };

        (device, pack)
    }
}

#[async_trait]
impl DeviceOperations for InfoDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        //
        // state of each devices
        let mut interface_devices = device.create_interface("devices").finish().await;

        //
        // Here the device interface must provide an attribute for each device mounted on the platform
        // When the device boot, it must send a creation request to this task and wait for the 'InfoDevice'
        // a validation. Once validated, the device can continue to run and report its status through an 'Arc<Mutex<InfoDev"
        //
        let pack_clone = self.pack.clone();
        let devices_status_attributes_clone = self.devices_status_attributes.clone();
        device
            .spawn(async move {
                //
                // Clone the notifier from info pack
                let new_request = pack_clone.new_request_notifier().await;

                //
                loop {
                    let devices = pack_clone.devices();
                    let request = devices.lock().await.pop_next_request();
                    match request {
                        Some(r) => {
                            //
                            //
                            println!("********{:?}", r);

                            let att = interface_devices
                                .create_attribute(r.name.clone())
                                .message()
                                .with_att_only_access()
                                .finish_with_codec::<JsonCodec>()
                                .await;

                            // att => att only

                            devices_status_attributes_clone
                                .lock()
                                .await
                                .insert(r.name.clone(), att);

                            // Here I must create a attribute inside interface_devices
                            // when the request is a creation request
                            // else delete the object
                            let _info = devices.lock().await.validate_creation_request(r);
                        }
                        None => {}
                    }
                    //
                    // Wait for more request
                    new_request.notified().await;
                }

                // Ok(())
            })
            .await;

        // I need to spawn a task to watch if a device status has changed, if yes update
        // It is a better design to create a task that will always live here
        let pack_clone2 = self.pack.clone();
        let devices_status_attributes_clone2 = self.devices_status_attributes.clone();
        device
            .spawn(async move {
                //
                // Clone the notifier from info pack
                let device_status_change = pack_clone2.device_status_change_notifier().await;

                //
                loop {
                    //
                    // Wait for next status change
                    device_status_change.notified().await;

                    println!("$$$$$$$$$$ status change");

                    let status_attributes = devices_status_attributes_clone2.lock().await;

                    // Update each status attribute here
                    for d in pack_clone2.devices().lock().await.devs() {
                        let mut status = d.1.lock().await;
                        if status.has_been_updated() {
                            status_attributes[d.0]
                                .set(JsonCodec::from(json!({
                                    "state": status.state_as_string()
                                })))
                                .await?;
                        }
                    }
                }
                // Ok(())
            })
            .await;

        //
        // Structure of the devices
        let structure_att = device
            .create_attribute("structure")
            .message()
            .with_att_only_access()
            .finish_with_codec::<JsonCodec>()
            .await;

        let pack_clone3 = self.pack.clone();

        device
            .spawn(async move {
                //
                //
                let structure_change = pack_clone3.device_structure_change_notifier().await;

                loop {
                    //
                    // Wait for next status change
                    structure_change.notified().await;

                    println!("$$$$$$$$$$ structure change ****");

                    let structure = pack_clone3.device_structure_as_json_value().await;
                    // println!("{:?}", structure);

                    structure_att.set(JsonCodec::from(structure)).await.unwrap();
                }

                // Ok(())
            })
            .await;

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
