use async_trait::async_trait;


use panduza_core::meta::blc::BlcAttributes;
use panduza_core::Error as PlatformError;
use panduza_core::platform_error_result;
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use panduza_core::meta::blc;
use panduza_connectors::usb::usb::{self, UsbConnector};
use panduza_connectors::usb::usb::Config as UsbConfig;
use panduza_core::interface::AmInterface;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

///
/// 
struct LBX488BlcActions {
    connector_usb: usb::UsbConnector,
    serial_config: UsbConfig,
    mode_value: String,
    enable_value: bool,
    power_max: f64,
    power_value: f64,
    current_value: f64,
    analog_modulation: bool
}

impl LBX488BlcActions {

    /// Wrapper to format the commands
    /// 
    async fn ask(&mut self, command: &[u8]) -> Result<String, PlatformError> {

        let mut cmd = vec![0; 32];
        cmd[..command.len()].copy_from_slice(command);

        self.connector_usb.write(cmd.as_ref()).await?;
        Ok(self.connector_usb.read().await?)
        // match self.connector_usb.read().await {
        //     Ok(val) => Ok(val),
        //     Err(e) => platform_error_result!(format!("Unable to read usb : {}", e))
        // }
    }

    /// Parse the data into f64 using 2 decimals 
    /// 
    async fn ask_float(&mut self, command: &[u8]) -> Result<f64, PlatformError> {
        match self.ask(command).await?.trim_end_matches("\0").to_string().parse::<f64>() {
            Ok(f) => {
                // Use the decimal library to have a better precision 
                let value_dec = match Decimal::from_f64(f) {
                    Some(value) => value,
                    None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as integer")
                };

                let final_value = match (value_dec * dec!(0.001)).to_f64() {
                    Some(value) => value,
                    None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as integer")
                };

                return Ok(final_value);

            },
            Err(e) => return platform_error_result!(format!("Unexpected answer from Oxxius LBX488 : could not parse as integer, error message {:?}", e))
        }
    }
}

#[async_trait]
impl blc::BlcActions for LBX488BlcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_usb = match usb::get(&self.serial_config).await {
            Some(connector) => connector,
            None => return platform_error_result!("Unable to create USB connector for Oxxius LBX488")
        };
        self.connector_usb.init().await?;

        let result = self.ask("?HID".as_bytes()).await?;

        interface.lock().await.log_info(
            format!("LBX_488 - initializing: {}", result)
        );


        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the analog modulation
    /// 
    async fn read_analog_modulation(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {
        
        let answer = self.ask("?AM".as_bytes()).await?;
        if answer == "0\x00" {
            self.analog_modulation = false;
        }

        if answer == "1\x00" {
            self.analog_modulation = true;
        }

        interface.lock().await.log_info(
            format!("read analog modulation value : {}", self.analog_modulation)
        );
        return Ok(self.analog_modulation);
    }

    /// Write the analog modulation
    /// 
    async fn write_analog_modulation(&mut self, interface: &AmInterface, v: bool) -> Result<(), PlatformError> {

        interface.lock().await.log_info(
            format!("write analog modulation value : {}", v)
        );

        let cmd;

        if v {
            cmd = 1;
        } else {
            cmd = 0;
        }

        // Analog need 1 to be activate but digital (CW) is desactivated
        // with 1, so here I activate analog and disable digital (or reverse)

        self.ask(format!("AM {}", cmd).as_bytes()).await?;
        self.ask(format!("CW {}", cmd).as_bytes()).await?;
        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the mode value
    /// 
    async fn read_mode_value(&mut self, _interface: &AmInterface) -> Result<String, PlatformError> {

        self.mode_value = "no_regulation".to_string();

        let acc = self.ask("?ACC".as_bytes()).await?;
        if acc == "1\x00" {
            self.mode_value = "constant_current".to_string();
        }
        
        let apc = self.ask("?APC".as_bytes()).await?;
        if apc == "1\x00" {
            self.mode_value = "constant_power".to_string();
        }

        return Ok(self.mode_value.clone());
    }

    /// Write the mode value
    /// 
    async fn write_mode_value(&mut self, interface: &AmInterface, v: String) -> Result<(), PlatformError> {

        interface.lock().await.log_info(
            format!("write mode value : {}", v)
        );

        let command = match v.as_str() {
            "constant_current" => format!("ACC 1"),
            "constant_power" => format!("APC 1"),
            _ => return platform_error_result!("Unexpected command for mode value")
        };

        self.ask(command.as_bytes()).await?;
        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, _interface: &AmInterface) -> Result<bool, PlatformError> {

        let emission = self.ask("?L".as_bytes()).await?;
        if emission == "1\x00" {
            self.enable_value = true;
        } else {
            self.enable_value = false;
        }

        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) -> Result<(), PlatformError> {

        let val_int = match v {
            true => 1,
            false => 0
        };

        let command = format!("L {}", val_int);

        let status = self.ask(command.as_bytes()).await?;
        
        interface.lock().await.log_info(
            format!("write enable value : {}", status)
        );
        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the power value
    /// 
    async fn read_power_max(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        self.power_max = self.ask_float(b"?MAXLP").await?;

        interface.lock().await.log_info(
            format!("read max power : {}", self.power_max)
        );

        return Ok(self.power_max);
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        self.power_value = self.ask_float(b"?SP\r").await?;

        println!("Success reading power value : {:?}", self.power_value);

        interface.lock().await.log_info(
            format!("read power : {}", self.power_value)
        );

        return Ok(self.power_value);
    }

    /// Write the power value
    /// 
    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError> {

        let value_dec = match Decimal::from_f64(v) {
            Some(value) => value,
            None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as Decimal")
        };
        
        // 3 represent the number of decimal who can manage by the device,
        // here for oxxius it is 3
        let val_mw = (value_dec * dec!(1000.0)).round_dp(2);
        
        let val_f64 = match val_mw.to_f64() {
            Some(value) => value,
            None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as f64")
        };

        let command = format!("PM {}", val_f64);

        interface.lock().await.log_info(
            format!("write power : {}", command)
        );

        self.ask(command.as_bytes()).await?;
        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read max current value
    /// 
    
    async fn read_max_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let value_max_f64 = self.ask_float(b"?MAXLC").await?;

        println!("max current : {}", value_max_f64);

        // Oxxius give 125% of max current but only 100% can be used else
        // the laser need something to be reboot
        let value_max_dec = match Decimal::from_f64(value_max_f64) {
            Some(value) => (value * dec!(100) / dec!(125)).round_dp(3),
            None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as Decimal")
        };

        let value_max = match value_max_dec.to_f64() {
            Some(value) => value,
            None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as f64")
        };

        self.current_value = value_max;

        interface.lock().await.log_info(
            format!("read max current : {}", self.current_value)
        );

        return Ok(self.current_value);
    }

    /// Read the current value
    /// 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        self.current_value = self.ask_float(b"?SC").await?;

        interface.lock().await.log_info(
            format!("read current : {}", self.current_value)
        );

        return Ok(self.current_value);
    }

    /// Write the current value
    /// 
    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError> {

        let value_dec = match Decimal::from_f64(v) {
            Some(value) => value,
            None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as Decimal")
        };
        
        // 3 represent the number of decimal who can manage by the device,
        // here for oxxius it is 3
        let val_ma = (value_dec * dec!(1000.0)).round_dp(2);
        
        let val_f64 = match val_ma.to_f64() {
            Some(value) => value,
            None => return platform_error_result!("Unexpected answer form Oxxius LBX488 : could not parse as f64")
        };

        let command = format!("CM {}", val_f64);

        interface.lock().await.log_info(
            format!("write current : {}", val_ma)
        );

        self.ask(command.as_bytes()).await?;
        return Ok(());
    }
}



/// Interface
/// 
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &UsbConfig
) -> InterfaceBuilder {

    return blc::build(
        name, 
        blc::BlcParams {
            power_min: 0.0,
            // power_max: 0.04,
            power_decimals: 5,

            current_min: 0.0,
            current_decimals: 5,
        }, 
        Box::new(LBX488BlcActions {
            connector_usb: UsbConnector::new(None),
            serial_config: serial_config.clone(),
            mode_value: "constant_power".to_string(),
            enable_value: false,
            power_max: 0.0,
            power_value: 0.0,
            current_value: 0.0,
            analog_modulation: true
        }),
        BlcAttributes::all_attributes()
    )
}

