

use lazy_static::lazy_static;

mod gate;
mod connector;


use panduza_core::{interface::logger::Logger, Error as PlatformError};


use crate::SerialSettings;

pub type SerialConnector = connector::Connector;


pub async fn get(serial_settings: &SerialSettings, logger: Option<InterfaceLogger>)
    -> Result<SerialConnector, PlatformError>
{
    // let gate = SerialGetFunction(config).await;
    // return Some(
    //     SlipConnector::new(
    //         gate.unwrap()
    //         , logger
    //     )
    // );
}