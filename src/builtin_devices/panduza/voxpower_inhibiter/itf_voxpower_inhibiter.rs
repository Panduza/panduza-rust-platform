use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::meta::relay;
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
    state_open: bool,
    time_lock_duration: Option<tokio::time::Duration>,
    
}

#[async_trait]
impl relay::RelayActions for VoxpowerInhibiterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        self.connector_tty.init().await;

        // println!("yooooo!");

        return Ok(());
    }

    /// Configuration of the interface
    /// 
    // async fn config(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
    //     return Ok(());
    // }

    /// Read the state value
    /// 
    async fn read_state_open(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {

        interface.lock().await.log_info(
            format!("VoxpowerInhibiter - read_state_open: {}", self.state_open)
        );

        let command = format!("S{}\n", self.id);
        let command_bytes = command.as_bytes();

        let mut response_buf: &mut [u8] = &mut [0; 1024];

        let _result = self.connector_tty.write_then_read(
            command_bytes,
            &mut response_buf,
            self.time_lock_duration,
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
                let response_bytes = &response_buf[0..nb_of_bytes];
                let response_string = String::from_utf8(response_bytes.to_vec()).unwrap();
                println!("VoxpowerInhibiterActions - channel {} state: {:?}", self.id, response_string);
                if response_string == "H" {
                    self.state_open = false;
                } else {
                    self.state_open = true;
                }
            });

        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - state_open: {}", self.state_open)
        );

        return Ok(self.state_open);
    }

    /// Write the state value
    /// 
    async fn write_state_open(&mut self, interface: &AmInterface, v: bool) {
        
        let command = if v {
            format!("E{}\n", self.id)
        } else {
            format!("I{}\n", self.id)
        };

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|_nb_of_bytes| {
                // println!("nb_of_bytes: {:?}", nb_of_bytes);
            });
        
        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - write_state_open; {}", self.state_open)
        );
    }
}


/// 
/// 
pub fn build<A: Into<String>>(
    name: A,
    id: u16,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return relay::build(
        name,
        Box::new(VoxpowerInhibiterActions {
            id: id.clone(),
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            state_open: true,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}