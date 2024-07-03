use async_trait::async_trait;
use panduza_core::Error as PlatformError;
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

#[async_trait]
impl bpc::BpcActions for VoxpowerInhibiterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        self.connector_tty.init().await;

        return Ok(());
    }

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {

        interface.lock().await.log_info(
            format!("VoxpowerInhibiter - read enable value: {}", self.enable_value)
        );

        let command = format!("S{}\n", self.id);
        let command_bytes = command.as_bytes();

        let mut response_buf: &mut [u8] = &mut [0; 1024];

        // Send the command to get the ON/OFF state of the channel
        let _result = self.connector_tty.write_then_read(
            command_bytes,
            &mut response_buf
        ).await
            .map(|nb_of_bytes| {
                let response_bytes = &response_buf[0..nb_of_bytes];
                let response_string = String::from_utf8(response_bytes.to_vec()).unwrap();

                // Pin state High = channel inhibited (OFF)
                // Pin state Low = channel enabled (ON)
                self.enable_value = if response_string == "H" {
                    false
                } else {
                    true
                };
            });

        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - channel_{} enable value : {}", self.id, self.enable_value)
        );

        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {
        
        let command = if v {
            // enable the channel
            format!("E{}\n", self.id)
        } else {
            // inhibit the channel
            format!("I{}\n", self.id)
        };

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await;
        
        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - write enable value; {}", self.enable_value)
        );
    }

    async fn write_voltage_value(&mut self, _interface: &AmInterface, v: f64) {
        self.voltage_value = v;
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
        })
    )
}
