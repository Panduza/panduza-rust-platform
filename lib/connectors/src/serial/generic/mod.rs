mod gate;
mod driver;
mod connector;

use crate::SerialSettings;

pub type SerialDriver = driver::Driver;
pub type SerialConnector = connector::Connector;

use panduza_core::Error as PlatformError;

pub async fn get(serial_settings: &SerialSettings)
    -> Result<SerialConnector, PlatformError>
{
    gate::get(serial_settings).await
}

pub async fn garbage_collector() {
    gate::garbage_collector().await;
}
