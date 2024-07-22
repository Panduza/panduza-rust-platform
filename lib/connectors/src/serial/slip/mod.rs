// private
mod gate;
mod driver;

// usage
use panduza_core::Error as PlatformError;
use crate::SerialSettings;

// public interface
pub type Connector = driver::Connector;
pub async fn get(serial_settings: &SerialSettings) -> Result<Connector, PlatformError> {
    gate::get(serial_settings).await
}
