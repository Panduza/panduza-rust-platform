pub mod pack;
pub mod pack_inner;
pub mod structure;
pub mod topic;

use async_trait::async_trait;
use futures::lock::Mutex;
use pack::InfoPack;
use panduza_platform_core::{AttOnlyMsgAtt, Device, DeviceOperations, Error, JsonCodec};
use serde_json::json;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::sleep;
pub use topic::Topic;

///
/// Main device of the platform
/// Provides the informations about the platform
///
pub struct UnderscoreDevice {
    ///
    ///
    ///
    pack: InfoPack,

    ///
    /// Each device have an attribute to share its state
    /// This Map hold those attribute, the name of the device is the key.
    ///
    instance_attributes: Arc<Mutex<HashMap<String, AttOnlyMsgAtt<JsonCodec>>>>,
}

impl UnderscoreDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> (UnderscoreDevice, InfoPack) {
        let pack = InfoPack::new();

        let device = UnderscoreDevice {
            pack: pack.clone(),
            instance_attributes: Arc::new(Mutex::new(HashMap::new())),
        };

        (device, pack)
    }
}

#[async_trait]
impl DeviceOperations for UnderscoreDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error> {
        //
        // state of each devices
        let mut interface_devices = device.create_interface("devices").finish();

        // I need to spawn a task to watch if a device status has changed, if yes update
        // It is a better design to create a task that will always live here
        let pack_clone2 = self.pack.clone();
        let instance_attributes_clone = self.instance_attributes.clone();
        device
            .spawn(async move {
                //
                // Clone the notifier from info pack
                let device_status_change = pack_clone2.instance_status_change_notifier();

                //
                loop {
                    //
                    // Wait for next status change
                    device_status_change.notified().await;

                    println!("$$$$$$$$$$ status change");

                    let pack_status = pack_clone2.pack_instance_status();

                    println!("{:?}", pack_status);

                    let mut lock = instance_attributes_clone.lock().await;
                    for status in pack_status {
                        if !lock.contains_key(&status.0) {
                            let att = interface_devices
                                .create_attribute(status.0.clone())
                                .message()
                                .with_att_only_access()
                                .finish_with_codec::<JsonCodec>()
                                .await;

                            lock.insert(status.0.clone(), att);
                        }

                        lock.get_mut(&status.0)
                            .unwrap()
                            .set(JsonCodec::from(json!({
                                "state": status.1.to_string()
                            })))
                            .await?;
                    }
                    drop(lock);
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
                let structure_change = pack_clone3.instance_structure_change_notifier().await;
                // let pack_clone4 = pack_clone3.clone();

                loop {
                    //
                    // Wait for next status change
                    structure_change.notified().await;

                    println!("$$$$$$$$$$ structure change ****");

                    let structure = pack_clone3.device_structure_as_json_value().await.unwrap();
                    println!("structure {:?}", structure);

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
