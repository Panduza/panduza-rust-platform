use async_trait::async_trait;
use panduza_core::FunctionResult as PlatformFunctionResult;
use panduza_core::Error as PlatformError;
use panduza_core::meta::bpc::{self, BpcAttributes};
use panduza_core::interface::AmInterface;
use panduza_core::interface::builder::Builder as InterfaceBuilder;

use panduza_connectors::serial::tty::{self, TtyConnector};
use panduza_connectors::serial::tty::Config as SerialConfig;
use panduza_core::platform_error_result;

use regex::Regex;

/// Bugs:
/// - When spamming commands
/// - Incomplete reception ? Sometimes only a part of the response is read !
///

#[derive(Copy, Clone)]
pub enum Hm7044Channel {
    Channel1,
    Channel2,
    Channel3,
    Channel4
}

struct Hm7044BpcActions {
    connector_tty: tty::TtyConnector,
    serial_config: SerialConfig,
    time_lock_duration: Option<tokio::time::Duration>,
    channel: Hm7044Channel
}

impl Hm7044BpcActions {
    async fn send_cmd_read_answer(&mut self, command: &[u8]) -> Result<String, PlatformError> {

        let mut response: &mut [u8] = &mut [0; 1024];
        let response_len = match self.connector_tty.write_then_read(
            command,
            &mut response,
            self.time_lock_duration
        ).await {
            Ok(val) => val,
            Err(_e) => {return platform_error_result!("Failed to read and write.");}
        };

        let pp: &[u8] = &response[0..response_len];

        // Debug
        // let print_command = String::from_utf8(command.to_vec()).unwrap();
        // let print_response = String::from_utf8(pp.to_vec()).unwrap();
        // println!("HM7044 Send command:\n\t{}\n received response:\n\t{}", print_command, print_response);

        return Ok(String::from_utf8(pp.to_vec()).unwrap());
    }

    async fn send_cmd_expect_answer(&mut self, command: &[u8], expected_answer: String) -> Result<(), PlatformError> {

        let response = self.send_cmd_read_answer(command).await?;

        if response == expected_answer {
            return Ok(());
        } else {
            // Debug
            // println!("Expected \n\r{}\n\r  Reveiced \n\r{}", response, expected_answer);

            return platform_error_result!("Unexpected answer from HM7044.");
        }
    }

    async fn get_status(&mut self) -> Result<(f64, f64, bool), PlatformError> {

        let sss = self.send_cmd_read_answer(b"READ\r").await?;
        let v_a_e: Vec<&str> = sss.split(';').collect();
        if v_a_e.len() != 3 {
            return platform_error_result!("Unexpected answer from HM7044.")
        }
        let voltages: Vec<&str> = v_a_e[0][..v_a_e[0].len() - 1].split(' ').collect();
        let currents: Vec<&str> = v_a_e[1][..v_a_e[1].len() - 1].split(' ').collect();
        if voltages.len() != 4 || currents.len() != 4 {
            return platform_error_result!("Unexpected answer from HM7044.")
        }

        let voltage_str = voltages[self.channel as usize];
        let voltage = match voltage_str[..voltage_str.len() - 1].parse() {
            Ok(v) => v,
            Err(_e) => {return platform_error_result!("Unexpected answer from HM7044.") }
        };

        let current_str = currents[self.channel as usize];
        let current = match current_str[..current_str.len() - 1].parse() {
            Ok(v) => v,
            Err(_e) => {return platform_error_result!("Unexpected answer from HM7044.") }
        };

        let reg = Regex::new(r"(?i:CV|CC|OFF)");
        let extract: Vec<String> = reg
            .unwrap()
            .captures_iter(v_a_e[2])
            .map(|cap| cap[0].to_string())
            .collect();
        let enable = match extract[self.channel as usize].as_str() {
            "CV" | "CC" => true,
            "OFF" => false,
            _ => {return platform_error_result!("Unexpected answer from HM7044.");}
        };

        return Ok((voltage, current, enable));
    }

    async fn select_channel(&mut self) -> Result<(), PlatformError>{

        return self.send_cmd_expect_answer(
            format!("SEL {}\r", (self.channel as usize) + 1).as_bytes(),
            format!("channel {} selected\r", (self.channel as usize) + 1))
            .await;
    }

    async fn set_voltage(&mut self, voltage: f64) -> Result<(), PlatformError>{

        return self.send_cmd_expect_answer(
            format!("SET {:.2} V\r", voltage).as_bytes(),
            format!("channel {} set to {:.2} V\r", (self.channel as usize) + 1, voltage))
            .await;
    }

    async fn set_current(&mut self, voltage: f64) -> Result<(), PlatformError>{

        return self.send_cmd_expect_answer(
            format!("SET {:.3} A\r", voltage).as_bytes(),
            format!("channel {} set to {:.3} A\r", (self.channel as usize) + 1, voltage))
            .await;
    }

    async fn set_on_off(&mut self, on: bool) -> Result<(), PlatformError>{

        return self.send_cmd_expect_answer(
            format!("{}\r", if on { "ON" } else { "OFF" }).as_bytes(),
            format!("channel {} {}\r", (self.channel as usize) + 1, if on { "on" } else { "off" }))
            .await;
    }

    async fn set_output_enable(&mut self, enable: bool) -> Result<(), PlatformError>{

        return self.send_cmd_expect_answer(
            format!("{}\r", if enable { "EN" } else { "DIS" }).as_bytes(),
            format!("output {}\r", if enable { "enabled" } else { "disabled" }))
            .await;
    }

}


#[async_trait]
impl bpc::BpcActions for Hm7044BpcActions {

    /// Initialize the interface
    ///
    async fn initializating(&mut self, _interface: &AmInterface) -> PlatformFunctionResult {

        self.connector_tty = tty::get(&self.serial_config).await.unwrap();
        let _ = self.connector_tty.init().await;

        // Send '\r' to close any incomplete command sent previously.
        self.send_cmd_read_answer(b"\r").await?;

        // There is no such thing as "enable all outputs now" in bpc definition. Enable output by default.
        self.set_output_enable(true).await?;

        return Ok(());
    }

    /// Read the enable value
    ///
    async fn read_enable_value(&mut self, _interface: &AmInterface) -> Result<bool, PlatformError> {

        // Debug
        // println!("read_enable_value");

        return match self.get_status().await {
            Ok((_voltage, _current, enable)) => Ok(enable),
            Err(e) => Err(e)
        };
    }

    /// Write the enable value
    ///
    async fn write_enable_value(&mut self, _interface: &AmInterface, v: bool) -> PlatformFunctionResult {

        // Debug
        // println!("write_enable_value");

        let result = self.select_channel().await;

        if result.is_ok() {
            let _ = self.set_on_off(v).await;
        }

        Ok(())
    }

    // / Read the voltage value
    // /
    async fn read_voltage_value(&mut self, _interface: &AmInterface) -> Result<f64, PlatformError> {

        // Debug
        // println!("read_voltage_value");

        return match self.get_status().await {
            Ok((voltage, _current, _enable)) => Ok(voltage),
            Err(e) => Err(e)
        };
    }

    async fn write_voltage_value(&mut self, _interface: &AmInterface, v: f64) {

        // Debug
        // println!("write_voltage_value");

        let result = self.select_channel().await;

        if result.is_ok() {
            let _ = self.set_voltage(v).await;
        }
    }

    async fn read_current_value(&mut self, _interface: &AmInterface) -> Result<f64, PlatformError> {

        // Debug
        // println!("read_current_value");

        return match self.get_status().await {
            Ok((_voltage, current, _enable)) => Ok(current),
            Err(e) => Err(e)
        };
    }

    async fn write_current_value(&mut self, _interface: &AmInterface, v: f64) {

        // Debug
        // println!("write_current_value");

        let result = self.select_channel().await;

        if result.is_ok() {
            let _ = self.set_current(v).await;
        }
    }
}



/// Interface to emulate a Bench Power Channel
///
pub fn build<A: Into<String>>(
    name: A,
    serial_config: &SerialConfig,
    channel: Hm7044Channel
) -> InterfaceBuilder {

    return bpc::build(
        name,
        bpc::BpcParams {
            voltage_min: 0.0,
            voltage_max: 32.0,
            voltage_decimals: 2,

            current_min: 0.0,
            current_max: 3.0,
            current_decimals: 3,
        },
        Box::new(Hm7044BpcActions {
            connector_tty: TtyConnector::new(None),
            serial_config: serial_config.clone(),
            time_lock_duration: Some(tokio::time::Duration::from_millis(100)),
            channel: channel
        }),
        BpcAttributes::all_attributes()
    )
}

