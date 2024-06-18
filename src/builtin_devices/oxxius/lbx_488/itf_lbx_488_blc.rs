use async_trait::async_trait;
use crate::platform::PlatformError;
use crate::meta::blc;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


// use panduza_connectors::serial::tty::{self, TtyConnector};
// use panduza_connectors::serial::tty::Config as SerialConfig;
use panduza_connectors::usb::usbtmc::{self, UsbtmcConnector};
use panduza_connectors::usb::usbtmc::Config as UsbtmcConfig;

///
/// 
struct LBX488BlcActions {
    connector_usbtmc: usbtmc::UsbtmcConnector,
    serial_config: UsbtmcConfig,
    mode_value: String,
    enable_value: bool,
    power_value: f64,
    current_value: f64,
    // time_lock_duration: Option<tokio::time::Duration>,
}

#[async_trait]
impl blc::BlcActions for LBX488BlcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_usbtmc = usbtmc::get(&self.serial_config).await.unwrap();
        self.connector_usbtmc.init().await;

        let result = self.connector_usbtmc.ask("?HID".to_string()).await;

        interface.lock().await.log_info(
            format!("LBX_488 - initializing: {}", result)
        );

        

        // println!("yooooo!");

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
    async fn read_mode_value(&mut self, _interface: &AmInterface) -> Result<String, PlatformError> {

        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"gam?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //         let mode_b = &response[0..nb_of_bytes];
        //         println!("mode {:?}", mode_b);
        //         let mode_i = String::from_utf8(mode_b.to_vec()).unwrap().parse::<u16>().unwrap();
        //         println!("mode {}", mode_i);
        //         self.mode_value = match mode_i {
        //             0 => "constant_current".to_string(),
        //             1 => "constant_power".to_string(),
        //             _ => "no_regulation".to_string()
        //         };
        //     });
        // let mut mode_val = String::new();
        // swap(&mut mode_val, &mut self.mode_value);
        self.mode_value = "no_regulation".to_string();

        let acc = self.connector_usbtmc.ask("?ACC".to_string()).await;
        if acc == "1\x00" {
            self.mode_value = "constant_current".to_string();
        }
        
        let apc = self.connector_usbtmc.ask("?APC".to_string()).await;
        if apc == "1\x00" {
            self.mode_value = "constant_current".to_string();
        }

        // interface.lock().await.log_info(
        //     format!("PM100A - read measure value: {}", self.measure_value)
        // );

        return Ok(self.mode_value.clone());
    }

    /// Write the mode value
    /// 
    async fn write_mode_value(&mut self, interface: &AmInterface, v: String) {

        interface.lock().await.log_info(
            format!("write mode value : {}", v)
        );

        let command = match v.as_str() {
            "constant_current" => format!("ACC 1"),
            "constant_power" => format!("APC 1"),
            _ => return
        };

        self.connector_usbtmc.ask(command).await;


        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //     });
    }

     /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, _interface: &AmInterface) -> Result<bool, PlatformError> {

        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"l?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //         let value_b = &response[0..nb_of_bytes];
        //         let value_i = String::from_utf8(value_b.to_vec()).unwrap().parse::<u16>().unwrap();
        //         self.enable_value = match value_i {
        //             0 => false,
        //             _ => true
        //         };
        //         println!("read enable value : {} | {}", value_i, self.enable_value);
        //     });

        let emission = self.connector_usbtmc.ask("?L".to_string()).await;
        if emission == "1\x00" {
            self.enable_value = true;
        } else {
            self.enable_value = false;
        }

        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) {

        let val_int = match v {
            true => 1,
            false => 0
        };

        let command = format!("L {}", val_int);

        let status = self.connector_usbtmc.ask(command).await;
        
        interface.lock().await.log_info(
            format!("write enable value : {}", status)
        );

        let mut value_i = 5;
        while value_i != val_int {
            value_i = if self.connector_usbtmc.ask("?L".to_string()).await == "1\00" {
                1
            } else {
                0
            };
        }

        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //     });
        
        // let mut value_i = 5;
        // while value_i != val_int {
        //     let mut response: &mut [u8] = &mut [0; 1024];
        //     let _result = self.connector_tty.write_then_read(
        //         b"l?",
        //         &mut response,
        //         self.time_lock_duration
        //     ).await
        //         .map(|nb_of_bytes| {
        //             // println!("nb of bytes: {:?}", nb_of_bytes);
        //             let value_b = &response[0..nb_of_bytes];
        //             value_i = String::from_utf8(value_b.to_vec()).unwrap().parse::<u16>().unwrap();
        //         });
        // }
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"p?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //         let power_b = &response[0..nb_of_bytes];
        //         self.power_value = String::from_utf8(power_b.to_vec()).unwrap().parse::<f64>().unwrap();
        //         println!(" read power : {}", self.power_value);
        //     });
        let response = self.connector_usbtmc.ask("?SP".to_string()).await;
        let response_float = response.parse::<f64>().unwrap();
        self.power_value = response_float * 0.001;

        interface.lock().await.log_info(
            format!("write power : {}", self.power_value)
        );

        return Ok(self.power_value);
    }

    /// Write the power value
    /// 
    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) {
        
        interface.lock().await.log_info(
            format!("write power : {}", v)
        );

        let val_mw = v * 1000.0;
        let command = format!("PW {}", val_mw);

        self.connector_usbtmc.ask(command).await;

        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //     });
    }

    /// Read the current value
    /// 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        // let mut response: &mut [u8] = &mut [0; 1024];
        // let _result = self.connector_tty.write_then_read(
        //     b"glc?",
        //     &mut response,
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //         let current_b = &response[0..nb_of_bytes];
        //         self.current_value = String::from_utf8(current_b.to_vec()).unwrap().parse::<f64>().unwrap();
        //         println!("read current : {}", self.current_value);
        //     });
        let response = self.connector_usbtmc.ask("?SC".to_string()).await;
        let response_float = response.parse::<f64>().unwrap();
        self.current_value = response_float * 0.001;

        interface.lock().await.log_info(
            format!("write power : {}", self.current_value)
        );

        return Ok(self.current_value);
    }

    /// Write the current value
    /// 
    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) {
        interface.lock().await.log_info(
            format!("write current : {}", v)
        );

        let val_ma = v * 1000.0;
        let command = format!("CM {}", val_ma);

        self.connector_usbtmc.ask(command).await;

        // let _result = self.connector_tty.write(
        //     command.as_bytes(),
        //     self.time_lock_duration
        // ).await
        //     .map(|nb_of_bytes| {
        //         println!("nb of bytes: {:?}", nb_of_bytes);
        //     });
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &UsbtmcConfig
) -> InterfaceBuilder {

    return blc::build(
        name, 
        blc::BlcParams {
            power_min: 0.0,
            power_max: 0.3,
            power_decimals: 3,

            current_min: 0.0,
            current_max: 0.5,
            current_decimals: 3,
        }, 
        Box::new(LBX488BlcActions {
            connector_usbtmc: UsbtmcConnector::new(None),
            serial_config: serial_config.clone(),
            mode_value: "no_regulation".to_string(),
            enable_value: false,
            power_value: 0.0,
            current_value: 0.0,
            // time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}

