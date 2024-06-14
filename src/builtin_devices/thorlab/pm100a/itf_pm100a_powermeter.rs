use async_trait::async_trait;
// use rusb;
// use std::time::Duration;

// use rust_usbtmc::instrument::Instrument;
use crate::platform::PlatformError;
use crate::meta::powermeter;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


// use crate::connector::serial::tty::Tty;
// use crate::connector::serial::tty::{self, TtyConnector};
// use crate::connector::serial::tty::Config as SerialConfig;
use crate::connector::serial::usbtmc::{self, Config as SerialConfig};
// use crate::platform_error_result;


// static VID: u16 = 0x1313;
// static PID: u16 = 0x8079;


///
/// 
struct PM100APowermeterActions {
    connector_usbtmc: usbtmc::UsbtmcConnector,
    serial_config: SerialConfig,
    // instrument: Instrument,
    measure_value: f64,
    time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl powermeter::PowermeterActions for PM100APowermeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        self.connector_usbtmc = usbtmc::get(&self.serial_config).await.unwrap();
        self.connector_usbtmc.init().await;

        let result = self.connector_usbtmc.write_then_read()
       
        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, _interface: &AmInterface) -> Result<f64, PlatformError> {

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
            // connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            // instrument: Instrument::new(0, 0),
            measure_value: 0.0,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

