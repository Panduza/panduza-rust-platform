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

    let devices = device.lock().await;
    let hunters = devices.hunters();

    tracing::info!(class="Platform", "Hunting...");


    for hunter in hunters {
        let devices = hunter.hunt().await;
        if devices.is_some() {
            tracing::info!(class="Platform", "Hunting Success!");
            return Ok(());
        }
    }


    tracing::info!(class="Platform", "Hunting Success!");
    Ok(())
}


