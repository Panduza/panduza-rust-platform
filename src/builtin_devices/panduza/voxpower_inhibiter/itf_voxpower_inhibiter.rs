use async_trait::async_trait;
use panduza_core::meta::bpc::BpcAttributes;
use panduza_core::FunctionResult as PlatformFunctionResult;
use panduza_core::Error as PlatformError;
use panduza_core::platform_error_result;
use panduza_core::meta::bpc;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use panduza_connectors::serial::tty::{self, TtyConnector};
use panduza_connectors::serial::tty::Config as SerialConfig;



/// Voxpower Inhibiter Channel Data
/// 
struct VoxpowerInhibiterActions {
    id: u16,
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig,
    enable_value: bool,
    voltage_value: f64,
    current_value: f64,
    time_lock_duration: Option<tokio::time::Duration>,
    
}

impl VoxpowerInhibiterActions {
    async fn ask(&mut self, command: &[u8]) -> Result<String, PlatformError> {

        let mut response_buf: &mut [u8] = &mut [0; 1024];

        // Send the command then receive the answer
        let response_len = self.connector_tty.write_then_read(
            command,
            &mut response_buf,
            self.time_lock_duration
        ).await?;

        let response_bytes = &response_buf[0..response_len];

        // Parse the answer
        match String::from_utf8(response_bytes.to_vec()) {
            Ok(val) => Ok(val.trim().to_string()),
            Err(e) => platform_error_result!(format!("Unexpected answer form Voxpower Inhibiter : {:?}", e))
        }
    }
}

#[async_trait]
impl bpc::BpcActions for VoxpowerInhibiterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> PlatformFunctionResult {
        
        self.connector_tty = match tty::get(&self.serial_config).await {
            Some(connector) => connector,
            None => return platform_error_result!("Unable to create TTY connector for Voxpower Inhibiter")
        };
        self.connector_tty.init().await?;

        let response = self.ask(b"?").await?;

        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - channel_{} initializating : {}", self.id, response)
        );

        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {

        let command = format!("S{}\n", self.id);

        // High (inhibition ON) state when the channel is OFF
        // Low (inhibition OFF) state when the channel is ON
        self.enable_value = match self.ask(command.as_bytes()).await?.as_str() {
            "H" => false,
            "L" => true,
            e => return platform_error_result!(format!("Unexpected answer form Voxpower Inhibiter : {:?}", e))
        };

        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - read value : {}", self.enable_value)
        );

        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) -> PlatformFunctionResult {
        
        let command = if v {
            // enable the channel
            format!("E{}\n", self.id)
        } else {
            // inhibit the channel
            format!("I{}\n", self.id)
        };

        let response = self.ask(command.as_bytes()).await?;
        
        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - write enable value {} : {}", v, response)
        );

        Ok(())
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Unused functions for the Voxpower
    /// 
    async fn read_voltage_value(&mut self, _interface: &AmInterface) -> Result<f64, PlatformError> {
        return Ok(self.voltage_value);
    }

    async fn write_voltage_value(&mut self, _interface: &AmInterface, v: f64) {
        self.voltage_value = v;
    }

    async fn read_current_value(&mut self, _interface: &AmInterface) -> Result<f64, PlatformError> {
        return Ok(self.current_value);
    }

    async fn write_current_value(&mut self, _interface: &AmInterface, v: f64) {
        self.current_value = v;
    }
}


/// 
/// 
pub fn build<A: Into<String>>(
    name: A,
    id: u16,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return bpc::build(
        name,
        bpc::BpcParams {
            voltage_min: -1.0,
            voltage_max: -1.0,
            voltage_decimals: 0,

            current_min: -1.0,
            current_max: -1.0,
            current_decimals: 0,
        }, 
        Box::new(VoxpowerInhibiterActions {
            id: id.clone(),
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            enable_value: false,
            voltage_value: -1.0,
            current_value: -1.0,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        }),
        vec![
            BpcAttributes::Enable.to_string()
        ]
    )
}