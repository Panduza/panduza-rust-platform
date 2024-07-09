// use std::ops::DerefMut;

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

    let mut store = devices.create_an_empty_store();

    tracing::info!(class="Platform", "Hunting...");


    for hunter in hunters {
        let devices = hunter.hunt().await;

        match devices {
            Some(device) => {
                // Here add in instances device hunted
                match device.clone().get(0) {
                    Some(first_instancce) => {
                        match first_instancce["ref"].as_str() {
                            Some(device_name) => {
                                store[device_name]["instances"] = serde_json::Value::Array(device);
                                tracing::info!(class="Platform", "Hunting Success!");
                            },
                            None => {
                                // Never supposed to happended because else None would be get earlier
                                // If this strange comportement happen just didn't had instance of this device
                            }
                        }
                    },
                    None => {
                        // Never supposed to happended because else None would be get earlier
                        // If this strange comportement happen just didn't had instance of this device
                    }
                }
            },
            None => {
                tracing::info!(class="Platform", "No device found !");
            }
        }
    }


    services.lock().await.update_device_store(store);

    tracing::info!(class="Platform", "Hunting Success!");
    Ok(())
}