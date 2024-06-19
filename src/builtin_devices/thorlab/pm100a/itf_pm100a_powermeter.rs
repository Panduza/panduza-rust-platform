// use std::result;

use async_trait::async_trait;
// use rusb;
// use std::time::Duration;

// use rust_usbtmc::instrument::Instrument;
use panduza_core::Error as PlatformError;
use panduza_core::meta::powermeter;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;


// use panduza_connectors::serial::tty::Tty;
// use panduza_connectors::serial::tty::{self, TtyConnector};
// use panduza_connectors::serial::tty::Config as SerialConfig;
use panduza_connectors::usb::usbtmc::{self, Config as SerialConfig, UsbtmcConnector};
// use crate::platform_error_result;


// static VID: u16 = 0x1313;
// static PID: u16 = 0x8079;


///
/// 
struct PM100APowermeterActions {
    connector_usbtmc: usbtmc::UsbtmcConnector,
    serial_config: SerialConfig,
    measure_value: f64,
}

#[async_trait]
impl powermeter::PowermeterActions for PM100APowermeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
        self.connector_usbtmc = usbtmc::get(&self.serial_config).await.unwrap();
        self.connector_usbtmc.init().await;

        let result = self.connector_usbtmc.ask("*IDN?".to_string()).await;

        interface.lock().await.log_info(
            format!("PM100A - initializing: {}", result)
        );
       
        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        let result = self.connector_usbtmc.ask("READ?".to_string()).await;
        self.measure_value = result.parse::<f64>().expect("bad measure");

        interface.lock().await.log_info(
            format!("PM100A - read measure value: {}", self.measure_value)
        );

        return Ok(self.measure_value);
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return powermeter::build(
        name, 
        powermeter::PowermeterParams {
            measure_decimals: 5,
        }, 
        Box::new(PM100APowermeterActions {
            connector_usbtmc: UsbtmcConnector::new(None),
            serial_config: serial_config.clone(),
            measure_value: 0.0,
        })
    )
}

