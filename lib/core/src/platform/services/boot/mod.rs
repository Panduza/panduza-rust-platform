use std::ops::DerefMut;

// use crate::{connection, device};

use super::AmServices;

use crate::FunctionResult;
mod connection_info_loading_process;

use connection_info_loading_process::execute_connection_info_loading_process;



/// 
/// 
pub async fn execute_service_boot(
    services: AmServices,
    // devices: device::AmManager,
    // connection: connection::AmManager
)
    -> FunctionResult
{
    // log
    tracing::info!(class="Platform", "Booting...");

    // Load connection info
    execute_connection_info_loading_process(services.lock().await.deref_mut()).await
}

