use async_trait::async_trait;


use panduza_core::meta::blc::BlcAttributes;
use panduza_core::Error as PlatformError;
use panduza_core::platform_error_result;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;
use panduza_core::meta::blc;
use panduza_connectors::usb::usb::{self, UsbConnector};
use panduza_connectors::usb::usb::Config as UsbConfig;

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
}

impl LBX488BlcActions {

    /// Wrapper to format the commands
    /// 
    async fn ask(&mut self, command: &[u8]) -> String {

        let mut cmd = vec![0; 32];
        cmd[..command.len()].copy_from_slice(command);

        self.connector_usb.write(cmd.as_ref()).await;
        self.connector_usb.read().await
    }

    /// Parse the data into f64
    /// 
    async fn ask_float(&mut self, command: &[u8]) -> Result<f64, PlatformError> {

        match self.ask(command).await.trim_end_matches("\0").to_string().parse::<f64>() {
            Ok(f) => Ok(f),
            Err(_e) => return platform_error_result!("Unexpected answer form Cobolt S0501 : could not parse as integer")
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
        self.connector_usb.init().await;

        let result = self.ask("?HID".as_bytes()).await;

        interface.lock().await.log_info(
            format!("LBX_488 - initializing: {}", result)
        );


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

        let acc = self.ask("?ACC".as_bytes()).await;
        if acc == "1\x00" {
            self.mode_value = "constant_current".to_string();
        }
        
        let apc = self.ask("?APC".as_bytes()).await;
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

        self.ask(command.as_bytes()).await;
        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, _interface: &AmInterface) -> Result<bool, PlatformError> {

        let emission = self.ask("?L".as_bytes()).await;
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

        let status = self.ask(command.as_bytes()).await;
        
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

        let response_float = self.ask_float(b"?MAXLP").await?;
        
        self.power_max = response_float * 0.001;

        interface.lock().await.log_info(
            format!("read power : {}", response_float)
        );

        return Ok(self.power_max);
    }

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let response_float = self.ask_float(b"?SP\r").await?;
        self.power_value = response_float * 0.001;
        println!("Success reading power value : {:?}", self.power_value);

        interface.lock().await.log_info(
            format!("read power : {}", self.power_value)
        );

        return Ok(self.power_value);
    }

    /// Write the power value
    /// 
    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError> {

        let val_mw = ((v * 1000.0) * 100.0).round() / 100.0;
        let command = format!("PM {}", val_mw);

        interface.lock().await.log_info(
            format!("write power : {}", val_mw)
        );

        self.ask(command.as_bytes()).await;
        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the current value
    /// 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        let response_float = self.ask_float(b"?SC").await?;
        self.current_value = response_float * 0.001;

        

        interface.lock().await.log_info(
            format!("read current : {}", response_float)
        );

        return Ok(self.current_value);
    }

    /// Write the current value
    /// 
    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError> {

        let val_ma = ((v * 1000.0) * 100.0).round() / 100.0;
        let command = format!("CM {}", val_ma);

        interface.lock().await.log_info(
            format!("write current : {}", val_ma)
        );

        self.ask(command.as_bytes()).await;
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
            power_decimals: 3,

            current_min: 0.0,
            current_max: 0.0978,
            current_decimals: 3,
        }, 
        Box::new(LBX488BlcActions {
            connector_usb: UsbConnector::new(None),
            serial_config: serial_config.clone(),
            mode_value: "constant_power".to_string(),
            enable_value: false,
            power_max: 0.0,
            power_value: 0.0,
            current_value: 0.0,
        }),
        BlcAttributes::all_attributes()
    )
}

