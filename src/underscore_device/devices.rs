use std::{collections::HashMap, sync::Arc};

use super::pack::InfoPack;
use panduza_platform_core::{log_trace, Error, Instance};
use serde_json::json;
use tokio::sync::Mutex;

///
///
///
pub async fn mount(mut instance: Instance, pack: InfoPack) -> Result<(), Error> {
    //
    // Get logger
    let logger = instance.logger.clone();

    // Each device have an attribute to share its state
    // This Map hold those attribute, the name of the device is the key.
    // instance_attributes: Arc<Mutex<HashMap<String, JsonAttServer>>>,
    let instance_attributes = Arc::new(Mutex::new(HashMap::new()));

    //
    // state of each devices
    let mut interface_devices = instance.create_class("devices").finish();

    // I need to spawn a task to watch if a device status has changed, if yes update
    // It is a better design to create a task that will always live here
    let pack_clone2 = pack.clone();
    let instance_attributes_clone = instance_attributes.clone();
    instance
        .spawn(async move {
            //
            // Clone the notifier from info pack
            let device_status_change = pack_clone2.instance_status_change_notifier();

            //
            loop {
                //
                // Wait for next status change
                device_status_change.notified().await;
                log_trace!(logger, "status change notification");

                let pack_status = pack_clone2.pack_instance_status();
                log_trace!(logger, "{:?}", pack_status);

                let mut lock = instance_attributes_clone.lock().await;
                for status in pack_status {
                    if !lock.contains_key(&status.0) {
                        let att = interface_devices
                            .create_attribute(status.0.clone())
                            .with_ro()
                            .finish_as_json()
                            .await?;

                        lock.insert(status.0.clone(), att);
                    }

                    lock.get_mut(&status.0)
                        .unwrap()
                        .set(json!({
                            "state": status.1.to_string(),
                            "alerts": status.2
                        }))
                        .await?;
                }
                drop(lock);
            }
            // Ok(())
        })
        .await;

    // //
    // // Structure of the devices
    // let structure_att = instance
    //     .create_attribute("structure")
    //     .with_ro()
    //     .finish_as_json()
    //     .await?;

    // let pack_clone3 = pack.clone();
    // instance
    //     .spawn(async move {
    //         //
    //         //
    //         let structure_change = pack_clone3.instance_structure_change_notifier().await;
    //         // let pack_clone4 = pack_clone3.clone();

    //         loop {
    //             //
    //             // Wait for next status change
    //             structure_change.notified().await;
    //             log_trace!(logger, "structure change notification");

    //             let structure = pack_clone3.device_structure_as_json_value().await.unwrap();
    //             log_trace!(logger, "new structure {:?}", structure);

    //             structure_att.set(structure).await.unwrap();
    //         }
    //         // Ok(())
    //     })
    //     .await;

    //
    //
    Ok(())
}
