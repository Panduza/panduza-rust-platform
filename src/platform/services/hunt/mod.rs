// use std::ops::DerefMut;

use super::AmServices;


/// 
/// 
pub async fn execute_service_hunt(
    services: AmServices
)
    -> Result<(),  &'static str >
{
    
    tracing::info!(class="Platform", "Hunting...");


    tracing::info!(class="Platform", "Hunting Success!");
    Ok(())
}


