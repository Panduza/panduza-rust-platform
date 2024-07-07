
mod connector;

use super::tty;
use connector::Connector as SlipConnector;

use panduza_core::interface::logger::Logger as InterfaceLogger;

pub type Config = tty::Config;

pub async fn get(config: &Config, logger: Option<InterfaceLogger>) -> Option<SlipConnector> {
    let gate = tty::get(config).await;
    return Some(
        SlipConnector::new(
            gate.unwrap()
            , logger
        )
    );
}




