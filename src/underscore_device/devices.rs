use std::{collections::HashMap, sync::Arc};

use super::pack::InfoPack;
use panduza_platform_core::{log_trace, Container, Error, Instance};
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
    let mut interface_devices = instance.create_class("devices").finish().await;

    // I need to spawn a task to watch if a device status has changed, if yes update
    // It is a better design to create a task that will always live here
    let pack_clone2 = pack.clone();
    let instance_attributes_clone = instance_attributes.clone();
    instance
        .spawn("devices/watcher", async move {
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

    //
    //
    Ok(())
}
