use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::meta::serial;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;


// use panduza_connectors::serial::tty::Tty;
use panduza_connectors::serial::tty::{self, TtyConnector};
use panduza_connectors::serial::tty::Config as SerialConfig;
// use crate::platform_error_result;

///
/// 
struct SerialPortActions {
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig,
    // enable_value: bool,
    // voltage_value: f64,
    // current_value: f64,
    // time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl serial::SerialActions for SerialPortActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        self.connector_tty.init().await;


        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"*IDN?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|c| {
        //         let pp = &response[0..c];
        //         let sss = String::from_utf8(pp.to_vec()).unwrap();
        //         println!("SerialActions - initializating: {:?}", sss);
        //     });


        return Ok(());
    }
    
    
    async fn read_data(&mut self, interface: &AmInterface) -> Result<String, PlatformError> {

        Ok(String::from(""))
    }


    async fn write_data(&mut self, interface: &AmInterface, v: &Vec<u8>) {
        let i = interface.lock().await;
        let logger = i.clone_logger();
        logger.log_info(format!("SerialActions - write_data: {:?}", v));
        
        self.connector_tty.write(&v, None).await.unwrap();
    }

}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return serial::build(
        name, 
        // serial::BpcParams {
        //     voltage_min: 0.0,
        //     voltage_max: 30.0,
        //     voltage_decimals: 2,

        //     current_min: 0.0,
        //     current_max: 3.0,
        //     current_decimals: 3,
        // }, 
        Box::new(SerialPortActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            // enable_value: false,
            // voltage_value: 0.0,
            // current_value: 0.0,
            // time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

