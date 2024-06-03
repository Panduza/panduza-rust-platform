use std::mem::swap;

use async_trait::async_trait;
use bitflags::parser::from_str;
use crate::platform::PlatformError;
use crate::meta::blc;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


// use crate::connector::serial::tty::Tty;
use crate::connector::serial::tty::{self, TtyConnector};
use crate::connector::serial::tty::Config as SerialConfig;
use crate::platform_error_result;

///
/// 
struct S0501BlcActions {
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig,
    mode_value: String,
    enable_value: bool,
    power_value: f64,
    current_value: f64,
    time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl blc::BlcActions for S0501BlcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, _interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        self.connector_tty.init().await;

        println!("yooooo!");

        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"*IDN?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|c| {
        //         let pp = &response[0..c];
        //         let sss = String::from_utf8(pp.to_vec()).unwrap();
        //         println!("Ka3005BpcActions - initializating: {:?}", sss);
        //     });


        return Ok(());
    }

    /// Read the mode value
    /// 
    async fn read_mode_value(&mut self, interface: &AmInterface) -> Result<String, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"gam?",
            &mut response,
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
                let mode_b = &response[0..nb_of_bytes];
                println!("mode {:?}", mode_b);
                let mode_i = String::from_utf8(mode_b.to_vec()).unwrap().parse::<u16>().unwrap();
                println!("mode {}", mode_i);
                if mode_i == 0 {
                    self.mode_value = "constant_current".to_string();
                } else if mode_i == 1 {
                    self.mode_value =  "constant_power".to_string();
                }
                self.mode_value =  "no_regulation".to_string();
            });
        let mut mode_val = String::new();
        swap(&mut mode_val, &mut self.mode_value);

        return Ok(mode_val);
    }

    /// Write the mode value
    /// 
    async fn write_mode_value(&mut self, interface: &AmInterface, v: String) {

        interface.lock().await.log_info(
            format!("write enable : {}", v)
        );

        // let command = match v {
        //     "constant_current" => { format!("ci\n") },
        //     "constant_power" => { format!("cp\n") }
        // };
        let command = if v == "constant_current" {
            format!("ci\n")
        } else if v == "constant_power" {
            format!("cp\n")
        } else {
            return
        };

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
            });
    }

     /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"l?",
            &mut response,
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
                let value_b = &response[0..nb_of_bytes];
                let value_i = String::from_utf8(value_b.to_vec()).unwrap().parse::<u16>().unwrap();
                self.enable_value = match value_i {
                    0 => false,
                    _ => true
                };
                println!("read enable value : {} | {}", value_i, self.enable_value);
            });

        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {

        let mut val_int = 0;
        if v {
            val_int = 1;
        }

        let command = format!("l{}\n", val_int);
        interface.lock().await.log_info(
            format!("write enable value : {}", command)
        );

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
            });
        
        let mut value_i = 5;
        while value_i != val_int {
            let mut response: &mut [u8] = &mut [0; 1024];
            let _result = self.connector_tty.write_then_read(
                b"l?",
                &mut response,
                self.time_lock_duration
            ).await
                .map(|nb_of_bytes| {
                    // println!("nb of bytes: {:?}", nb_of_bytes);
                    let value_b = &response[0..nb_of_bytes];
                    value_i = String::from_utf8(value_b.to_vec()).unwrap().parse::<u16>().unwrap();
                });
        }
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"p?",
            &mut response,
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
                let power_b = &response[0..nb_of_bytes];
                self.power_value = String::from_utf8(power_b.to_vec()).unwrap().parse::<f64>().unwrap();
                println!(" read power : {}", self.power_value);
            });

        return Ok(self.power_value);
    }

    /// Write the power value
    /// 
    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) {
        
        interface.lock().await.log_info(
            format!("write power : {}", v)
        );

        let command = format!("p {}\n", v);

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
            });
    }

    /// Read the current value
    /// 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let _result = self.connector_tty.write_then_read(
            b"glc?",
            &mut response,
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
                let current_b = &response[0..nb_of_bytes];
                self.current_value = String::from_utf8(current_b.to_vec()).unwrap().parse::<f64>().unwrap();
                println!("read current : {}", self.current_value);
            });

        return Ok(self.current_value);
    }

    /// Write the current value
    /// 
    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) {
        interface.lock().await.log_info(
            format!("write current : {}", v)
        );

        let command = format!("slc {}\n", v);

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
            });
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig
) -> InterfaceBuilder {

    return blc::build(
        name, 
        blc::BlcParams {
            power_min: 0.0,
            power_max: 0.3,
            power_decimals: 3,

            current_min: 0.0,
            current_max: 0.5,
            current_decimals: 1,
        }, 
        Box::new(S0501BlcActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            mode_value: "no_regulation".to_string(),
            enable_value: false,
            power_value: 0.0,
            current_value: 0.0,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

