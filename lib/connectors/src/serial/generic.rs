// private
mod driver;
mod gate;

// usage
use crate::SerialSettings;
use panduza_platform_core::Error as PlatformError;

// public interface
pub use driver::Connector;
pub async fn get(serial_settings: &SerialSettings) -> Result<Connector, PlatformError> {
    gate::get(serial_settings).await
}
