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

    let store = devices.create_an_empty_store();

    tracing::info!(class="Platform", "Hunting...");


    for hunter in hunters {
        let devices = hunter.hunt().await;
        if devices.is_some() {
            tracing::info!(class="Platform", "Hunting Success!");

        }
    }

    
    services.lock().await.update_device_store(store);

    tracing::info!(class="Platform", "Hunting Success!");
    Ok(())
}


