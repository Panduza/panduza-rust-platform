use async_trait::async_trait;
use tracing_subscriber::fmt::format;
use crate::platform::PlatformError;
use crate::meta::relay;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;

use crate::connector::serial::tty::{self, TtyConnector};
use crate::connector::serial::tty::Config as SerialConfig;



/// Voxpower Inhibiter Channel Data
/// 
struct VoxpowerInhibiterActions {
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig,
    state_open: bool,
    time_lock_duration: Option<tokio::time::Duration>,
    
}

#[async_trait]
impl relay::RelayActions for VoxpowerInhibiterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
        // let self.channel = channel;

        self.connector_tty = tty::get(&self.serial_config).unwrap();
        self.connector_tty.init().await;

        println!("yooooo!");

        // let mut response_buf: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"*IDN?",
        //     &mut response_buf,
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         let response_bytes = &response_buf[0..nb_of_bytes];
        //         let response_string = String::from_utf8(response_bytes.to_vec()).unwrap();
        //         println!("VoxpowerInhibiterActions - initializating: {:?}", response_string);
        //     });

        return Ok(());
    }

    /// Configuration of the interface
    /// 
    async fn config(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {
        return Ok(());
    }

    /// Read the state value
    /// 
    async fn read_state_open(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {
        interface.lock().await.log_info(
            format!("VoxpowerInhibiter - read_state_open: {}", self.state_open)
        );

        let mut response_buf: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"S6",
            &mut response_buf,
            self.time_lock_duration,
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
                let response_bytes = &response_buf[0..nb_of_bytes];
                let response_string = String::from_utf8(response_bytes.to_vec()).unwrap();
                println!("VoxpowerInhibiterActions - channel state: {:?}", response_string);
                if response_string == "H" {
                    self.state_open = true;
                } else {
                    self.state_open = false;
                }
            });

        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - state_open: {}", self.state_open)
        );

        return Ok(self.state_open);
        // Ok(true)

        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"S{}",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|c| {
        //         println!("c: {:?}", c);
        //         let pp = &response[0..c];
        //         if pp[0] == "H" {
        //             self.state_open = "H".to_string();
        //         } else {
        //             self.state_open = "L".to_string();
        //         }
        //     });
        
        // interface.lock().await.log_info(
        //     format!("Voxpower Inhibiter - state_open: {}", self.state_open)
        // );
        // return Ok(if self.state_open == "H" { true } else { false });

    //     if self.state_open == "H" {
    //         return Ok(true);
    //     } else {
    //         return Ok(false);
    //     }
    }

    /// Write the state value
    /// 
    async fn write_state_open(&mut self, interface: &AmInterface, v: bool) {
        println!("{}", v);
        
        let command = if v {
            format!("I6")
        } else {
            format!("E6")
        };

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb_of_bytes: {:?}", nb_of_bytes);
            });
        
        interface.lock().await.log_info(
            format!("Voxpower Inhibiter - write_state_open; {}", self.state_open)
        );

        // let command = format!("{}", if v { "I" + self.channel_id } else { "E" + self.channel_id });

        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|c| {
        //         println!("cc: {:?}", c);
        //     });

        // interface.lock().await.log_info(
        //     format!("Voxpower Inhibiter - write_state_open; {}", self.state_open)
        // );
        // self.state_open = if v { "H" } else { "L" };

        // if v {
        //     let command = "I\n"; //format!("I{}\n", self.channel);
        //     self.state_open = command.to_string();
        //     interface.lock().await.log_info(
        //         format!("VoxpowerInhibiter - inhibit channel: {}", self.state_open)
        //     );
        // } else {
        //     let command = "E\n"; //format!("E{}\n", self.channel);
        //     self.state_open = command.to_string();
        //    interface.lock().await.log_info(
        //         format!("VoxpowerInhibiter - enable channel: {}", self.state_open)
        //     );
        // }
    }
}


/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return relay::build(
        name,
        Box::new(VoxpowerInhibiterActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            state_open: false,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}