use async_trait::async_trait;
use panduza_core::Error as PlatformError;
use panduza_core::platform_error_result;
use panduza_core::meta::blc;
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use panduza_connectors::serial::tty::{self, TtyConnector};
use panduza_connectors::serial::tty::Config as SerialConfig;

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

impl S0501BlcActions {
    async fn ask_string(&mut self, command: &[u8]) -> Result<String, PlatformError> {

        let mut response_buf: &mut [u8] = &mut [0; 1024];

        // Send the command then receive the answer
        let response_len = match self.connector_tty.write_then_read(
            command,
            &mut response_buf,
            self.time_lock_duration
        ).await {
            Ok(len) => len,
            Err(_e) => return platform_error_result!("Failed to read and write")
        };

        let response_bytes = &response_buf[0..response_len];

        // Parse the answer
        match String::from_utf8(response_bytes.to_vec()) {
            Ok(val) => Ok(val),
            Err(_e) => platform_error_result!("Unexpected answer form Cobolt S0501 : could not parse as String")
        }
    }

    async fn ask_int(&mut self, command: &[u8]) -> Result<u16, PlatformError> {

        match self.ask_string(command).await?.trim().to_string().parse::<u16>() {
            Ok(u) => Ok(u),
            Err(_e) => return platform_error_result!("Unexpected answer form Cobolt S0501 : could not parse as integer")
        }
    }

    async fn ask_float(&mut self, command: &[u8]) -> Result<f64, PlatformError> {

        match self.ask_string(command).await?.trim().to_string().parse::<f64>() {
            Ok(f) => Ok(f),
            Err(_e) => return platform_error_result!("Unexpected answer form Cobolt S0501 : could not parse as integer")
        }
    }

    async fn cmd_ack(&mut self, command: &[u8], expected_response: String) -> Result<(), PlatformError> {

        let response = self.ask_string(command).await?;

        for resp in response.split("\r\n") {
            if resp == expected_response.as_str() {
                return Ok(());
            } else {
                continue;
            }
        }

        return platform_error_result!("Unexpected answer from Cobolt S0501");
    }
}

#[async_trait]
impl blc::BlcActions for S0501BlcActions {

    /// Initialize the interface
    /// 
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError> {

        self.connector_tty = match tty::get(&self.serial_config).await {
            Some(connector) => connector,
            None => return platform_error_result!("Unable to create TTY connector for Cobolt laser")
        };
        self.connector_tty.init().await;

        let response_string = self.ask_string(b"?\r").await?;

        interface.lock().await.log_info(
            format!("Cobolt S0501 initializing : {}", response_string)
        );

        return Ok(());
    }

    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the mode value
    /// 
    async fn read_mode_value(&mut self, interface: &AmInterface) -> Result<String, PlatformError> {

        let response_int = self.ask_int(b"gam?\r").await?;

        self.mode_value = match response_int {
            0 => "constant_current".to_string(),
            1 => "constant_power".to_string(),
            _ => "no_regulation".to_string()
        };

        interface.lock().await.log_info(
            format!("read mode value : {}", self.mode_value.clone())
        );
        return Ok(self.mode_value.clone());
    }

    /// Write the mode value
    /// 
    async fn write_mode_value(&mut self, interface: &AmInterface, v: String) -> Result<(), PlatformError> {

        interface.lock().await.log_info(
            format!("write mode value : {}", v)
        );

        let command = match v.as_str() {
            "constant_current" => format!("ci\r"),
            "constant_power" => format!("cp\r"),
            _ => return platform_error_result!("Unexpected mode command")
        };

        // let response = match self.cmd_ack(command.as_bytes(), "OK".to_string()).await {
        //     Ok(_r) => "OK".to_string(),
        //     Err(_e) => return platform_error_result!("Unexpected response from Cobolt S0501")
        // };

        // println!("{} !!!!!!!!!!!!!!!!!!!!!!!!!!!", response);
        
        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|_nb_of_bytes| {
            });
        
        // Clean the buffer from previous values
        while self.cmd_ack(b"gam?\r", "OK".to_string()).await.is_err() {
            continue;
        }
        return Ok(());
    }
    
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the enable value
    /// 
    async fn read_enable_value(&mut self, interface: &AmInterface) -> Result<bool, PlatformError> {

        let response_int = self.ask_int(b"l?\r").await?;
        
        self.enable_value = match response_int {
            0 => false,
            1 => true,
            _ => return platform_error_result!("Unexpected enable value form Cobolt S0501")
        };

        interface.lock().await.log_info(
            format!("read enable value : {}", self.enable_value)
        );

        return Ok(self.enable_value);
    }

    /// Write the enable value
    /// 
    async fn write_enable_value(&mut self, interface: &AmInterface, v: bool) -> Result<(), PlatformError> {

        let val_int = match v {
            true => 1,
            false => 0
        };

        let command = format!("l{}\r", val_int);

        interface.lock().await.log_info(
            format!("write enable value : {}", v)
        );

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|nb_of_bytes| {
                println!("nb of bytes: {:?}", nb_of_bytes);
            });
        
        // Clean the buffer from previous values

        while self.cmd_ack(b"l?\r", "OK".to_string()).await.is_err() {
            continue;
        }

        // The laser has an intertia to change to from OFF to ON so waits until it actually change state

        while self.cmd_ack(b"l?\r", format!("{val_int}")).await.is_err() {
            continue;
        }
        return Ok(());
    }
    
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the power value
    /// 
    async fn read_power_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {

        self.power_value = self.ask_float(b"p?\r").await?;

        interface.lock().await.log_info(
            format!("read power : {}", self.power_value)
        );

        return Ok(self.power_value);
    }

    /// Write the power value
    /// 
    async fn write_power_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError> {
        
        interface.lock().await.log_info(
            format!("write power : {}", v)
        );

        let command = format!("p {}\r", v);

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|_nb_of_bytes| {
            });

        // Clean the buffer from previous values
        while self.cmd_ack(b"p?\r", "OK".to_string()).await.is_err() {
            continue;
        }
        return Ok(());
    }
    
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------
    // ----------------------------------------------------------------------------

    /// Read the current value
    /// 
    async fn read_current_value(&mut self, interface: &AmInterface) -> Result<f64, PlatformError> {
        self.current_value = self.ask_float(b"glc?\r").await?;

        interface.lock().await.log_info(
            format!("read current : {}", self.current_value)
        );

        return Ok(self.current_value);
    }

    /// Write the current value
    /// 
    async fn write_current_value(&mut self, interface: &AmInterface, v: f64) -> Result<(), PlatformError> {
        interface.lock().await.log_info(
            format!("write current : {}", v)
        );

        let command = format!("slc {}\r", v);

        let _result = self.connector_tty.write(
            command.as_bytes(),
            self.time_lock_duration
        ).await
            .map(|_nb_of_bytes| {
            });

        // Clean the buffer from previous values
        while self.cmd_ack(b"glc?\r", "OK".to_string()).await.is_err() {
            continue;
        }
        return Ok(());
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
            mode_value: "constant_power".to_string(),
            enable_value: false,
            power_value: 0.0,
            current_value: 0.0,
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
        })
    )
}