use serde_json::json;
use panduza_core::platform_error;
use panduza_core::Error as PlatformError;

use tokio_serial::SerialPortInfo;
use tokio_serial::UsbPortInfo;
use tokio_serial::available_ports as available_serial_ports;

use crate::GateLogger;
use crate::UsbSettings;

/// Key for the usb serial in the json settings
static SERIAL_PORT_NAME_KEY: &str = "usb_serial";

/// Settings for the serial connector
/// 
pub struct Settings {
    /// Local logger
    pub logger: GateLogger,

    /// The serial port name
    pub port_name: Option<String>,

    /// The serial port baudrate
    pub baudrate: Option<u32>
}

impl Settings {

    /// Creates a new Settings instance
    /// 
    pub fn new() -> Settings {
        Settings {
            logger: GateLogger::new("serial-settings"),
            port_name: None,
            baudrate: None,
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
    pub fn set_port_name_from_json_or_usb_settings(mut self, json_settings: &serde_json::Value, usb_settings: &UsbSettings)
        -> Result<Self, PlatformError>
    {
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
    pub fn set_port_name_from_json_settings(mut self, json_settings: &serde_json::Value)
        -> Result<Self, PlatformError>
    {
        self.port_name = Some(
            Self::extract_port_name_from_json_settings(json_settings)?
        );
        Ok(self)
    }

    /// Extracts the serial port name from the json settings
    /// 
    pub fn extract_port_name_from_json_settings(json_settings: &serde_json::Value)
        -> Result<String, PlatformError>
    {
        Ok(
            json_settings.get(SERIAL_PORT_NAME_KEY)
            .ok_or(platform_error!("Unable to get \"{}\"", SERIAL_PORT_NAME_KEY))?
            .as_str()
            .ok_or(platform_error!("\"{}\" not a string", SERIAL_PORT_NAME_KEY))?
            .to_string()
        )
    }

    /// Try to set the port name from usb_settings
    /// 
    pub fn set_port_name_from_usb_settings(mut self, usb_settings: &UsbSettings)
        -> Result<Self, PlatformError>
    {
        self.port_name = Some(Self::find_port_name_from_usb_settings(usb_settings)?);
        Ok(self)
    }

    /// Set the baudrate
    /// 
    pub fn set_baudrate(mut self, baudrate: u32) -> Self {
        self.baudrate = Some(baudrate);
        self
    }

    /// Try to find a serial port name that match usb settings
    /// 
    pub fn find_port_name_from_usb_settings(usb_settings: &UsbSettings) 
        -> Result<String, PlatformError>
    {
        Self::find_serial_port_info_from_usb_settings(usb_settings)
            .map(|info| info.port_name)
    }

    /// To try find a serial port that match usb settings
    ///
    pub fn find_serial_port_info_from_usb_settings(usb_settings: &UsbSettings) 
        -> Result<SerialPortInfo, PlatformError>
    {
        available_serial_ports()
            .map_err(
                |e| platform_error!("Enable to get serial ports {:?}", e)
            )
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
                Err(platform_error!("No matching usb device"))
            })
    }

    /// Check if the provided info port match the usb settings
    /// 
    fn usb_info_port_match_usb_settings(usb_info_port: &UsbPortInfo, usb_settings: &UsbSettings)
        -> bool
    {
        // Match VID
        let match_vid = usb_settings.vendor
            .and_then(
                |vid| Some(vid == usb_info_port.vid)
            )
            // If here, it means that the user did not provided the VID so pass the check
            .unwrap_or(true);
            
        // Match PID
        let match_pid = usb_settings.vendor
            .and_then(
                |pid| Some(pid == usb_info_port.pid)
            )
            // If here, it means that the user did not provided the PID so pass the check
            .unwrap_or(true);
        
        // Match SERIAL
        let match_serial = usb_settings.serial.as_ref()
            .and_then(
                |val| {
                    usb_info_port.serial_number.as_ref()
                        .and_then( |s| Some(*s == *val) )
                        .or(Some(false))
                }
            )
            // If here, it means that the user did not provided the SERIAL so pass the check
            .unwrap_or(true);
    
        // Ok only if all the conditions are met
        return match_vid && match_pid && match_serial as bool;
    }

}



// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;


}

