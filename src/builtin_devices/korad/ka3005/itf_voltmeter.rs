use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use panduza_core::meta::voltmeter;

// use panduza_connectors::serial::tty::Tty;
use panduza_connectors::serial::tty::{self, TtyConnector};
use panduza_connectors::serial::tty::Config as SerialConfig;
// use crate::platform_error_result;

///
/// 
struct Ka3005pVoltmeterActions {
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig
}

#[async_trait]
impl voltmeter::VoltmeterActions for Ka3005pVoltmeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {
        self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        self.connector_tty.init().await;

        return Ok(());
    }

    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"VOUT1?",
            &mut response,
        ).await;

        let value = String::from_utf8(response.to_vec()).unwrap().parse::<f64>().expect("bad measure");

        interface.lock().await.log_info(
            format!("KA3005 - read_voltage_value: {}", value)
        );
        return Ok(value);
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return voltmeter::build(
        name, 
        voltmeter::VoltmeterParams {
            measure_decimals: 2
        }, 
        Box::new(Ka3005pVoltmeterActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone()
        })
    )
}
