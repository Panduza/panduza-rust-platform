use async_trait::async_trait;
use crate::platform::PlatformError;
use crate::meta::blc;
use crate::interface::AmInterface;
use crate::interface::builder::Builder as InterfaceBuilder;


use panduza_connectors::usb::usb::{self, UsbConnector};
use panduza_connectors::usb::usb::Config as UsbConfig;

/// 
/// 
async fn ask(mut connector_usb: UsbConnector, command: &[u8]) -> String {
    // let cmd =
    connector_usb.write(command).await;
    connector_usb.read().await
}

///
/// 
struct LBX488BlcActions {
    connector_usb: usb::UsbConnector,
    serial_config: UsbConfig,
    mode_value: String,
    enable_value: bool,
    power_value: f64,
    current_value: f64,
}

#[async_trait]
impl blc::BlcActions for LBX488BlcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_usb = usb::get(&self.serial_config).await.unwrap();
        self.connector_usb.init().await;

        let result = ask(self.connector_usb.clone(), "?HID".as_bytes()).await;
        // let result = self.connector_usb.ask("?HID".to_string()).await;

        interface.lock().await.log_info(
            format!("LBX_488 - initializing: {}", result)
        );


        return Ok(());
    }

    /// Read the mode value
    /// 
    async fn read_mode_value(&mut self, _interface: &AmInterface) -> Result<String, PlatformError> {

        self.mode_value = "no_regulation".to_string();

        let acc = ask(self.connector_usb.clone(), "?ACC".as_bytes()).await;
        if acc == "1\x00" {
            self.mode_value = "constant_current".to_string();
        }
        
        let apc = ask(self.connector_usb.clone(), "?APC".as_bytes()).await;
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

        ask(self.connector_usb.clone(), command.as_bytes()).await;
    }

     /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, _interface: &AmInterface) -> Result<bool, PlatformError> {

        let emission = ask(self.connector_usb.clone(), "?L".as_bytes()).await;
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

        let status = ask(self.connector_usb.clone(), command.as_bytes()).await;
        
        interface.lock().await.log_info(
            format!("write enable value : {}", status)
        );

        let mut value_i = 5;
        while value_i != val_int {
            value_i = if ask(self.connector_usb.clone(), "?L".as_bytes()).await == "1\00" {
                1
            } else {
                0
            };
        }
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let response = ask(self.connector_usb.clone(), "?SP".as_bytes()).await;
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

        ask(self.connector_usb.clone(), command.as_bytes()).await;
    }

    /// Read the current value
    /// 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let response = ask(self.connector_usb.clone(), "?SC".as_bytes()).await;
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

        ask(self.connector_usb.clone(), command.as_bytes()).await;
    }
}



/// Interface to emulate a Bench Power Channel
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &UsbConfig
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
            connector_usb: UsbConnector::new(None),
            serial_config: serial_config.clone(),
            mode_value: "no_regulation".to_string(),
            enable_value: false,
            power_value: 0.0,
            current_value: 0.0,
        })
    )
}

