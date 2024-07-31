// private
mod driver;
mod gate;

// usage
use crate::UsbSettings;
use panduza_core::Error as PlatformError;

// public interface
pub type Connector = driver::Connector;
pub async fn get(usb_settings: &UsbSettings) -> Result<Connector, PlatformError> {
    gate::get(usb_settings).await
}
