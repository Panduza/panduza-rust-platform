
mod connector;

use super::tty3::get as SerialGetFunction;
use super::tty3::Config as SerialConfig;
use connector::Connector as SlipConnector;

pub type Connector = SlipConnector;

use panduza_core::interface::logger::Logger as InterfaceLogger;

pub type Config = SerialConfig;

pub async fn get(config: &Config, logger: Option<InterfaceLogger>) -> Option<SlipConnector> {
    let gate = SerialGetFunction(config).await;
    return Some(
        SlipConnector::new(
            gate.unwrap()
            , logger
        )
    );
}




