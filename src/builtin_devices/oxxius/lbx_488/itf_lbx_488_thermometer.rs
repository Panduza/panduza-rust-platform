use async_trait::async_trait;

use panduza_core::{platform_error_result, Error as PlatformError};
use panduza_core::meta::thermometer;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use panduza_connectors::usb::usb::{self, UsbConnector};
use panduza_connectors::usb::usb::Config as UsbConfig;

/// Fake Thermometer Channel Data
/// 
struct LBX488ThermometerActions {
    connector_usb: usb::UsbConnector,
    serial_config: UsbConfig,
    measure_value: f64,
}

impl LBX488ThermometerActions {

    /// Wrapper to format the commands
    /// 
    async fn ask(&mut self, command: &[u8]) -> Result<String, PlatformError> {

        let mut cmd = vec![0; 32];
        cmd[..command.len()].copy_from_slice(command);

        self.connector_usb.write(cmd.as_ref()).await?;
        match self.connector_usb.read().await {
            Ok(val) => Ok(val),
            Err(_e) => platform_error_result!("Unable to read")
        }
    }

    /// Parse the data into f64
    /// 
    async fn ask_float(&mut self, command: &[u8]) -> Result<f64, PlatformError> {

        match self.ask(command).await?.trim_end_matches("\0").to_string().parse::<f64>() {
            Ok(f) => Ok(f),
            Err(_e) => return platform_error_result!("Unexpected answer form Cobolt S0501 : could not parse as integer")
        }
    }
}

#[async_trait]
impl thermometer::ThermometerActions for LBX488ThermometerActions {
    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
        
        self.connector_usb = match usb::get(&self.serial_config).await {
            Some(connector) => connector,
            None => return platform_error_result!("Unable to create USB connector for Oxxius LBX488")
        };
        self.connector_usb.init().await?;

        let result = self.ask("?HID".as_bytes()).await?;

        interface.lock().await.log_info(
            format!("LBX_488_Thermometer - initializing: {}", result)
        );

        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        
        let response_float = self.ask_float("?DT".as_bytes()).await?;
        self.measure_value = response_float * 0.001;

        interface.lock().await.log_info(
            format!("read power : {}", response_float)
        );

        interface.lock().await.log_info(
            format!("LBX_488_Thermometer - read_measure_value: {}", self.measure_value)
        );

        return Ok(self.measure_value);
    }
}



/// Interface to emulate a Thermometer Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &UsbConfig
) -> InterfaceBuilder {

    return thermometer::build(
        name, 
        thermometer::ThermometerParams {
            measure_decimals: 3
        },
        Box::new(LBX488ThermometerActions {
            connector_usb: UsbConnector::new(None),
            serial_config: serial_config.clone(),
            measure_value: 0.0,
        })
    )
}

