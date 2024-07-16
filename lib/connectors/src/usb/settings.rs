use serde_json::json;
use panduza_core::platform_error;
use panduza_core::Error as PlatformError;

use crate::GateLogger;

/// Key for the usb serial in the json settings
static USB_SERIAL_KEY: &str = "usb_serial";

/// Usb settings for devices
pub struct Settings {
    /// Logger
    pub logger: GateLogger,

    /// VID
    pub vendor: Option<u16>,

    /// PID
    pub model: Option<u16>,

    /// Serial String
    pub serial: Option<String>,
}

impl Settings {

    /// Creates a new Settings instance
    /// 
    pub fn new() -> Settings {
        Settings {
            logger: GateLogger::new("usb-settings"),
            vendor: None,
            model: None,
            serial: None
        }
    }

    /// Extracts the serial port name from the json settings
    /// This function fails if the settings is not present or ill-formed
    /// 
    pub fn set_serial_from_json_settings(mut self, settings: &serde_json::Value)
        -> Result<Self, PlatformError>
    {
        self.serial = Some(
            settings.get(USB_SERIAL_KEY)
            .ok_or(platform_error!("Unable to get \"{}\"", USB_SERIAL_KEY))?
            .as_str()
            .ok_or(platform_error!("\"{}\" not a string", USB_SERIAL_KEY))?
            .to_string()
        );
        Ok(self)
    }

    /// Like `set_serial_from_json_settings` but with a default value in case
    /// of error on settings extraction
    /// 
    pub fn set_serial_from_json_settings_or(mut self, settings: &serde_json::Value, default: &str)
        -> Self
    {
        let default_as_value = json!(default);
        self.serial = Some(
            settings.get(USB_SERIAL_KEY)
            .unwrap_or_else(|| {
                self.logger.log_warn(format!("Unable to get \"{}\"", USB_SERIAL_KEY));
                &default_as_value
            })
            .as_str()
            .unwrap_or_else(|| {
                self.logger.log_warn(format!("\"{}\" not a string", USB_SERIAL_KEY));
                default
            })
            .to_string()
        );
        self
    }


}



// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let settings = Settings::new();
        assert_eq!(settings.vendor, None);
        assert_eq!(settings.model, None);
        assert_eq!(settings.serial, None);
        assert_eq!(settings.serial_port_name, None);
        assert_eq!(settings.serial_baudrate, None);
    }

    #[test]
    fn test_set_serial_from_json_settings() {
        let json_settings = serde_json::json!({
            "serial": "COM1"
        });
        let mut settings = Settings::new()
            .set_serial_from_json_settings(&json_settings).unwrap();
        assert_eq!(settings.serial, Some("COM1".to_string()));
    }

    #[test]
    fn test_set_serial_from_json_settings_bad_string() {
        let json_settings = serde_json::json!({
            "serial": 5
        });
        let mut settings = Settings::new()
            .set_serial_from_json_settings(&json_settings);
        assert_eq!(settings.is_err(), true);
    }
    
    #[test]
    fn test_set_serial_from_json_settings_empty_input() {
        let json_settings = serde_json::json!({});
        let mut settings = Settings::new()
            .set_serial_from_json_settings(&json_settings);
        assert_eq!(settings.is_err(), true);
    }
    
    #[test]
    fn test_set_serial_from_json_settings_or_bad_string() {
        let json_settings = serde_json::json!({
            "serial": 5
        });
        let mut settings = Settings::new()
            .set_serial_from_json_settings_or(&json_settings, "OK_SERIAL");
        assert_eq!(settings.serial, Some("OK_SERIAL".to_string()));
    }
}

