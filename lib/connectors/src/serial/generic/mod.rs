// private
mod driver;
mod gate;

// usage
use crate::SerialSettings;
use panduza_core::Error as PlatformError;

// public interface
pub type Connector = driver::Connector;
pub async fn get(serial_settings: &SerialSettings) -> Result<Connector, PlatformError> {
    gate::get(serial_settings).await
}
