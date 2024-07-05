// use std::ops::DerefMut;

use serde_json::{json, Value};

use super::AmServices;
use crate::device;

/// 
/// 
pub async fn execute_service_hunt(
    services: AmServices,
    device: device::AmManager
)
    -> Result<(),  &'static str >
{

    services.lock().await.start_hunting_set_flag();

    let devices = device.lock().await;
    let hunters = devices.hunters();

    let mut cur_device_hunt;
    let mut devices_hunt = Vec::new();

    let mut store = devices.create_an_empty_store();

    tracing::info!(class="Platform", "Hunting...");

    for hunter in hunters {
        cur_device_hunt = hunter.hunt().await;

        match cur_device_hunt {
            Some(device) => {
                // If a device has been found
                devices_hunt.push(device);
            },
            None => {
                // If device not found
            }
        }
    }

    // update store
    store = json!(devices_hunt);
    
    tracing::info!(class="Platform", "store : {}", store);

    services.lock().await.update_device_store(store);

    tracing::info!(class="Platform", "Hunting Success!");
    Ok(())
}


