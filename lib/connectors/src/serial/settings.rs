use std::time::Duration;

use panduza_platform_core::Error as PlatformError;

use tokio_serial::available_ports as available_serial_ports;
use tokio_serial::DataBits;
use tokio_serial::FlowControl;
use tokio_serial::Parity;
use tokio_serial::SerialPortInfo;
use tokio_serial::StopBits;
use tokio_serial::UsbPortInfo;

use crate::ConnectorLogger;
use crate::UsbSettings;

/// Key for the usb serial in the json settings
static SERIAL_PORT_NAME_KEY: &str = "usb_serial";

/// Settings for the serial connector
///
#[derive(Clone)]
pub struct SerialSettings {
    /// Local logger
    pub logger: ConnectorLogger,
    /// The serial port name
    pub port_name: Option<String>,
    /// The baud rate in symbols-per-second
    pub baudrate: u32,
    /// Number of bits used to represent a character sent on the line
    pub data_bits: DataBits,
    /// The type of signalling to use for controlling data transfer
    pub flow_control: FlowControl,
    /// The type of parity to use for error checking
    pub parity: Parity,
    /// Number of bits to use to signal the end of a character
    pub stop_bits: StopBits,

    /// Read timeout
    pub read_timeout: Option<Duration>,
    /// Time to wait between 2 operations
    pub time_lock_duration: Option<Duration>,
}

impl SerialSettings {
    /// Creates a new Settings instance
    ///
    pub fn new() -> SerialSettings {
        SerialSettings {
            logger: ConnectorLogger::new("serial", "settings", ""),
            port_name: None,
            baudrate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            read_timeout: None,
            time_lock_duration: None,
        }
    }

    /// Set the port name
    ///
    pub fn set_port_name<A: Into<String>>(mut self, port_name: A) -> Self {
        self.port_name = Some(port_name.into());
        self
    }

    /// Set the port name from the json settings or the usb settings if json settings fails
    ///
    pub fn set_port_name_from_json_or_usb_settings(
        self,
        json_settings: &serde_json::Value,
        usb_settings: &UsbSettings,
    ) -> Result<Self, PlatformError> {
        // Try to extract the port name from the json settings
        Self::extract_port_name_from_json_settings(json_settings)
            // If it fails, try to find the port name from the usb settings
            .or_else(|_| Self::find_port_name_from_usb_settings(usb_settings))
            // Finally set the portname
            .map(|port_name| self.set_port_name(port_name))
    }

    /// Extracts the serial port name from the json settings
    /// This function fails if the settings is not present or ill-formed
    ///
    pub fn set_port_name_from_json_settings(
        mut self,
        json_settings: &serde_json::Value,
    ) -> Result<Self, PlatformError> {
        self.port_name = Some(Self::extract_port_name_from_json_settings(json_settings)?);
        Ok(self)
    }

    /// Extracts the serial port name from the json settings
    ///
    pub fn extract_port_name_from_json_settings(
        json_settings: &serde_json::Value,
    ) -> Result<String, PlatformError> {
        Ok(json_settings
            .get(SERIAL_PORT_NAME_KEY)
            .ok_or(PlatformError::BadSettings(format!(
                "Unable to get \"{}\"",
                SERIAL_PORT_NAME_KEY
            )))?
            .as_str()
            .ok_or(PlatformError::BadSettings(format!(
                "\"{}\" not a string",
                SERIAL_PORT_NAME_KEY
            )))?
            .to_string())
    }

    /// Try to set the port name from usb_settings
    ///
    pub fn set_port_name_from_usb_settings(
        mut self,
        usb_settings: &UsbSettings,
    ) -> Result<Self, PlatformError> {
        self.port_name = Some(Self::find_port_name_from_usb_settings(usb_settings)?);
        Ok(self)
    }

    /// Set the baudrate
    ///
    pub fn set_baudrate(mut self, baudrate: u32) -> Self {
        self.baudrate = baudrate;
        self
    }

    /// Try to find a serial port name that match usb settings
    ///
    pub fn find_port_name_from_usb_settings(
        usb_settings: &UsbSettings,
    ) -> Result<String, PlatformError> {
        Self::find_serial_port_info_from_usb_settings(usb_settings).map(|info| info.port_name)
    }

    /// To try find a serial port that match usb settings
    ///
    pub fn find_serial_port_info_from_usb_settings(
        usb_settings: &UsbSettings,
    ) -> Result<SerialPortInfo, PlatformError> {
        available_serial_ports()
            .map_err(|e| PlatformError::BadSettings(format!("Enable to get serial ports {:?}", e)))
            .and_then(|ports| {
                for port in ports {
                    // Check only usb port type
                    // Check if the settings match
                    if let tokio_serial::SerialPortType::UsbPort(info) = &port.port_type {
                        if Self::usb_info_port_match_usb_settings(info, usb_settings) {
                            return Ok(port);
                        }
                    }
                }
                Err(PlatformError::BadSettings(format!(
                    "No matching usb device ( availables: {} )",
                    Self::list_all_serial_ports()
                )))
            })
    }

    /// List all the available serial ports for error message
    ///
    pub fn list_all_serial_ports() -> String {
        match available_serial_ports() {
            Err(e) => format!("no serial ports available {:?}", e),
            Ok(ports) => {
                let mut data = String::new();
                for port in ports {
                    if let tokio_serial::SerialPortType::UsbPort(info) = &port.port_type {
                        data.push_str(&format!(
                            "{}/{:#02x}/{:#02x} ;",
                            port.port_name, info.vid, info.pid
                        ));
                    }
                }
                data
            }
        }
    }

    /// Check if the provided info port match the usb settings
    ///
    fn usb_info_port_match_usb_settings(
        usb_info_port: &UsbPortInfo,
        usb_settings: &UsbSettings,
    ) -> bool {
        // Logger only for trace
        let trace_logger = ConnectorLogger::new("serial", "settings", "");

        // Match VID
        let match_vid = usb_settings
            .vendor
            .and_then(|vid| Some(vid == usb_info_port.vid))
            // If here, it means that the user did not provided the VID so pass the check
            .unwrap_or(true);

        // Match PID
        let match_pid = usb_settings
            .model
            .and_then(|pid| Some(pid == usb_info_port.pid))
            // If here, it means that the user did not provided the PID so pass the check
            .unwrap_or(true);

        // Match SERIAL
        let match_serial = usb_settings
            .serial
            .as_ref()
            .and_then(|val| {
                usb_info_port
                    .serial_number
                    .as_ref()
                    .and_then(|s| Some(*s == *val))
                    .or(Some(false))
            })
            // If here, it means that the user did not provided the SERIAL so pass the check
            .unwrap_or(true);

        // Compute match
        let matchhh = match_vid && match_pid && match_serial as bool;

        // Trace
        let trace_message = format!(
            "{} - match: {} vid: {} pid: {} serial: {}",
            usb_settings, matchhh, match_vid, match_pid, match_serial
        );
        // println!("{}", trace_message);
        trace_logger.trace(trace_message);

        // Ok only if all the conditions are met
        return matchhh;
    }

    /// Set the flow control
    ///
    pub fn set_data_bits(mut self, data_bits: DataBits) -> Self {
        self.data_bits = data_bits;
        self
    }

    /// Set the read timeout
    ///
    pub fn set_read_timeout(mut self, read_timeout: Duration) -> Self {
        self.read_timeout = Some(read_timeout);
        self
    }

    /// Set time lock duration
    ///
    pub fn set_time_lock_duration(mut self, time_lock_duration: Duration) -> Self {
        self.time_lock_duration = Some(time_lock_duration);
        self
    }
}
