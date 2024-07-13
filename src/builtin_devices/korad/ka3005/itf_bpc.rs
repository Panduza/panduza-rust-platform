use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::meta::bpc::{self, BpcAttributes};
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;


// use panduza_connectors::serial::tty::Tty;
use panduza_connectors::serial::tty2::{self, TtyConnector};
use panduza_connectors::serial::tty2::Config as SerialConfig;
// use crate::platform_error_result;

///
/// 
struct Ka3005pBpcActions {
    connector_tty: TtyConnector,
    serial_config: SerialConfig,
    enable_value: bool,
    voltage_value: f64,
    current_value: f64,
    time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl bpc::BpcActions for Ka3005pBpcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_tty = tty2::get(&self.serial_config).await.unwrap();
        let _ = self.connector_tty.init().await;

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"*IDN?",
            &mut response
        ).await
            .map(|c| {
                let pp = &response[0..c];
                let sss = String::from_utf8(pp.to_vec()).unwrap();
                println!("Ka3005pBpcActions - initializating: {:?}", sss);
            });


        return Ok(());
    }

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"STATUS?",
            &mut response
        ).await
            .map(|c| {
                println!("c: {:?}", c);
                let pp = &response[0..c];
                if (pp[0] & (1 << 6)) == 0 {
                    self.enable_value = false;
                } else {
                    self.enable_value = true;
                }
            });

        interface.lock().await.log_info(
            format!("KA3005 - read_enable_value: {}", self.enable_value)
        );
        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {

        let command = format!("OUT{}", if v { 1 } else { 0 });

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await;

        interface.lock().await.log_info(
            format!("KA3005 - write_enable_value: {}", self.enable_value)
        );

        self.enable_value = v;
    }

    async fn read_voltage_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"VSET1?",
            &mut response
        ).await;

        let value = String::from_utf8(response.to_vec()).unwrap().parse::<f64>().expect("bad measure");

        interface.lock().await.log_info(
            format!("KA3005 - read_voltage_value: {}", value)
        );
        return Ok(value);
    }

    async fn write_voltage_value(&mut self, interface: &AmInterface, v: f64) {
        let command = format!("VSET1:{:05.2}", v);

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await;

        interface.lock().await.log_warn(
            format!("NOT IMPLEMENTED KA3005 - write_voltage_value: {}", v)
        );

        self.voltage_value = v;
    }

    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"ISET1?",
            &mut response
        ).await;

        let value = String::from_utf8(response.to_vec()).unwrap().parse::<f64>().expect("bad measure");

        interface.lock().await.log_info(
            format!("KA3005 - read_current_value: {}", value)
        );
        return Ok(value);
    }

    async fn write_current_value(&mut self, interface: &AmInterface, c: f64) {
        let command = format!("ISET1:{:05.3}", c);

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await;

        interface.lock().await.log_warn(
            format!("NOT IMPLEMENTED KA3005 - write_current_value: {}", c)
        );
    
        self.current_value = c;
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return bpc::build(
        name, 
        bpc::BpcParams {
            voltage_min: 0.0,
            voltage_max: 30.0,
            voltage_decimals: 2,

            current_min: 0.0,
            current_max: 3.0,
            current_decimals: 3,
        }, 
        Box::new(Ka3005pBpcActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            enable_value: false,
            voltage_value: 0.0,
            current_value: 0.0,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        }),
        BpcAttributes::all_attributes()
    )
}
