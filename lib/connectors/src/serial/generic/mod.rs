

mod gate;
mod driver;
mod connector;


use panduza_core::Error as PlatformError;

use crate::SerialSettings;

pub type SerialDriver = driver::Driver;
pub type SerialConnector = connector::Connector;


#[inline]
pub async fn get(serial_settings: &SerialSettings)
    -> Result<SerialConnector, PlatformError>
{
    gate::get(serial_settings).await
}

#[inline]
pub async fn garbage_collector() {
    gate::garbage_collector().await;
}
