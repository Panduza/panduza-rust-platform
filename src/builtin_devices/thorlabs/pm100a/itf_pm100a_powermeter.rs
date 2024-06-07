use async_trait::async_trait;
use crate::platform::PlatformError;
use crate::meta::powermeter;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


// use crate::connector::serial::tty::Tty;
// use crate::connector::serial::tty::{self, TtyConnector};
// use crate::connector::serial::tty::Config as SerialConfig;
// use crate::platform_error_result;

///
/// 
struct PM100APowermeterActions {
    // connector_tty: tty::TtyConnector,
    // serial_config: SerialConfig,
    measure_value: f64,
    // time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl powermeter::PowermeterActions for PM100APowermeterActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        // self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        // self.connector_tty.init().await;

        println!("yooooo!");

        return Ok(());
    }

    /// Read the measure value
    /// 
    async fn read_measure_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.read(
        //     b"STATUS?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|c| {
        //         println!("c: {:?}", c);
        //         let pp = &response[0..c];
        //         if (pp[0] & (1 << 6)) == 0 {
        //             self.measure_value = false;
        //         } else {
        //             self.measure_value = true;
        //         }
        //     });

        interface.lock().await.log_warn(
            format!("NOT IMPLEMENTED PM100A - read_measure_value: {}", self.measure_value)
        );
        return Ok(self.measure_value);
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    // serial_config: &SerialConfig
) -> InterfaceBuilder {

    return powermeter::build(
        name, 
        powermeter::PowermeterParams {
            measure_decimals: 5,
        }, 
        Box::new(PM100APowermeterActions {
            // connector_tty: TtyConnector::new(None),
            // serial_config: serial_config.clone(),
            measure_value: 0.0,
            // time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

