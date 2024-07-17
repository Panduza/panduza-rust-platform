
mod gate;
mod driver;
mod connector;


use crate::SerialSettings;


use panduza_core::Error as PlatformError;


pub type SlipDriver = driver::Driver;
pub type SlipConnector = connector::Connector;


pub async fn get(serial_settings: &SerialSettings)
    -> Result<SlipConnector, PlatformError>
{
    gate::get(serial_settings).await
}

pub async fn garbage_collector() {
    gate::garbage_collector().await;
}




