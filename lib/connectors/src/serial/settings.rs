

use panduza_core::Error as PlatformError;
use panduza_core::platform_error;




#[derive(Clone, Debug)]
pub struct Settings {
    pub usb_vendor: Option<u16>,
    pub usb_model: Option<u16>,
    pub usb_serial: Option<String>,

    pub serial_port_name: Option<String>,
    pub serial_baudrate: Option<u32>
}

impl Settings {

    pub fn new() -> Settings {
        Settings {
            usb_vendor: None,
            usb_model: None,
            usb_serial: None,
            serial_port_name: None,
            serial_baudrate: None,
        }
    }



    /// Extracts the serial port name from the json settings
    /// This function fails if the settings is not present or ill-formed
    pub fn usb_serial_from_json_settings(mut self, settings: &serde_json::Value)
        -> Result<Self, PlatformError>
    {
        self.usb_serial = Some(
            settings.get("usb_serial")
            .ok_or(platform_error!("Unable to get usb serial"))?
            .as_str()
            .ok_or(platform_error!("Usb serial not a string"))?
            .to_string()
        );
        Ok(self)
    }


    pub fn usb_serial_from_json_settings_or(mut self, settings: &serde_json::Value, default: &str)
        -> Self
    {
        match self.usb_serial_from_json_settings(settings) {
            Ok(s) => s,
            Err(_) => {
                // self.usb_serial = Some(default.to_string());
                self
            }
        }
    }


    // pub fn import_from_json_settings(&mut self, settings: &serde_json::Value) -> PlatformFunctionResult {


    //     let serial_baudrate_default = json!(9600);
    //     let baudrate = settings.get("serial_baudrate")
    //         .or(Some(&serial_baudrate_default))
    //         .ok_or(platform_error!("Unable to get serial baudrate"))?
    //         .as_u64()
    //         .ok_or(platform_error!("Serial baudrate not an integer"))?;
    //     self.serial_baudrate = Some(baudrate as u32);


    //     // // get VID hexadecimal value
    //     // self.usb_vendor = match settings.get("usb_vendor")
    //     // {
    //     //     Some(val) => match val.as_str()
    //     //     {
    //     //         Some(s) => match u16::from_str_radix(s, 16)
    //     //         {
    //     //             Ok(val) => Some(val),
    //     //             Err(_e) => return platform_error_result!("usb_vendor not an hexadecimal value")
    //     //         },
    //     //         None => return platform_error_result!("usb_vendor not a String")
    //     //     },
    //     //     None => return platform_error_result!("Missing usb_vendor from tree.json")
    //     // };

    //     // // get PID hexadecimal value
    //     // self.usb_model = match settings.get("usb_model")
    //     // {
    //     //     Some(val) => match val.as_str()
    //     //     {
    //     //         Some(s) => match u16::from_str_radix(s, 16)
    //     //         {
    //     //             Ok(val) => Some(val),
    //     //             Err(_e) => return platform_error_result!("usb_model not an hexadecimal value")
    //     //         },
    //     //         None => return platform_error_result!("usb_model not a String")
    //     //     },
    //     //     None => return platform_error_result!("Missing usb_model from tree.json")
    //     // };



    //     let usb_serial = settings.get("usb_serial");
    //     if usb_serial.is_some() {
    //         let usb_serial_str = usb_serial
    //             .ok_or(platform_error!("Unable to get usb serial"))?
    //             .as_str()
    //             .ok_or(platform_error!("Usb serial not a string"))?;
    //         self.usb_serial = Some(
    //             String::from_str(usb_serial_str)
    //                 .map_err(|_e| platform_error!("Unable to convert usb_serial to string"))?
    //         );
    //     }
    //     else {
    //         self.usb_serial = None;
    //     }



    //     Ok(())
    // }
}

