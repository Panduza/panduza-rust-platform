

use lazy_static::lazy_static;

mod gate;
mod driver;
mod connector;




use panduza_core::{interface::logger::Logger, Error as PlatformError};

use panduza_core::interface::logger::Logger as InterfaceLogger;

use crate::SerialSettings;

pub type SerialDriver = driver::Driver;
pub type SerialConnector = connector::Connector;


pub async fn get(serial_settings: &SerialSettings, logger: Option<InterfaceLogger>)
    -> Result<SerialConnector, PlatformError>
{
    gate::get(serial_settings).await
}

