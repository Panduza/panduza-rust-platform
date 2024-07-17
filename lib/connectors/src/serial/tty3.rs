use std::str::FromStr;
use std::{collections::HashMap, sync::Arc};
use serde_json::json;
use tokio_serial::{self, SerialPortBuilder};
use tokio::io::{AsyncReadExt, AsyncWriteExt};//, Result};
use tokio_serial::SerialStream;
use tokio::time::{sleep, Duration};

use tokio;
use lazy_static::lazy_static;

use panduza_core::FunctionResult as PlatformFunctionResult;
use panduza_core::Error as PlatformError;
use panduza_core::platform_error_result;
use panduza_core::platform_error;
use tracing::Value;



lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate> 
        = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
}

// get should return an error message
pub async fn get(config: &Config) -> Result<TtyConnector, PlatformError> {
    let mut gate = GATE.lock().await;
    gate.get(config)
}


#[derive(Clone, Debug)]
pub struct Config {
    pub usb_vendor: Option<u16>,
    pub usb_model: Option<u16>,
    pub usb_serial: Option<String>,

    pub serial_port_name: Option<String>,
    pub serial_baudrate: Option<u32>
}

impl Config {
    pub fn new() -> Config {
        Config {
            usb_vendor: None,
            usb_model: None,
            usb_serial: None,
            serial_port_name: None,
            serial_baudrate: None,
        }
    }

    pub fn import_from_json_settings(&mut self, settings: &serde_json::Value) -> PlatformFunctionResult {


        let serial_baudrate_default = json!(9600);
        let baudrate = settings.get("serial_baudrate")
            .or(Some(&serial_baudrate_default))
            .ok_or(platform_error!("Unable to get serial baudrate"))?
            .as_u64()
            .ok_or(platform_error!("Serial baudrate not an integer"))?;
        self.serial_baudrate = Some(baudrate as u32);


        // // get VID hexadecimal value
        // self.usb_vendor = match settings.get("usb_vendor")
        // {
        //     Some(val) => match val.as_str()
        //     {
        //         Some(s) => match u16::from_str_radix(s, 16)
        //         {
        //             Ok(val) => Some(val),
        //             Err(_e) => return platform_error_result!("usb_vendor not an hexadecimal value")
        //         },
        //         None => return platform_error_result!("usb_vendor not a String")
        //     },
        //     None => return platform_error_result!("Missing usb_vendor from tree.json")
        // };

        // // get PID hexadecimal value
        // self.usb_model = match settings.get("usb_model")
        // {
        //     Some(val) => match val.as_str()
        //     {
        //         Some(s) => match u16::from_str_radix(s, 16)
        //         {
        //             Ok(val) => Some(val),
        //             Err(_e) => return platform_error_result!("usb_model not an hexadecimal value")
        //         },
        //         None => return platform_error_result!("usb_model not a String")
        //     },
        //     None => return platform_error_result!("Missing usb_model from tree.json")
        // };



        let usb_serial = settings.get("usb_serial");
        if usb_serial.is_some() {
            let usb_serial_str = usb_serial
                .ok_or(platform_error!("Unable to get usb serial"))?
                .as_str()
                .ok_or(platform_error!("Usb serial not a string"))?;
            self.usb_serial = Some(
                String::from_str(usb_serial_str)
                    .map_err(|_e| platform_error!("Unable to convert usb_serial to string"))?
            );
        }
        else {
            self.usb_serial = None;
        }



        Ok(())
    }
}



struct Gate {
    instances: HashMap<String, TtyConnector>
}

impl Gate {

    fn get(&mut self, config: &Config) -> Result<TtyConnector, PlatformError> {
        // First try to get the key
        let key_string = Gate::generate_unique_key_from_config(config)?;
        let key= key_string.as_str();

        // if the instance is not found, it means that the port is not opened yet
        if ! self.instances.contains_key(key) {

            // Create a new instance
            let new_instance = TtyConnector::new(Some(config.clone()));

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());
            tracing::info!(class="Platform", "connector created");
        }

        // Try to find the instance
        let instance = self.instances.get(key)
            .ok_or(platform_error!(
                format!("Unable to find the tty connector \"{}\"", key)
            ))?;

        // Return the instance
        Ok(instance.clone())
    }

    /// Try to generate a unique key from the config
    /// This key will be used to find back the tty connector
    ///
    fn generate_unique_key_from_config(config: &Config) -> Result<String, PlatformError> {
        // Check if the serial port name is provided
        if let Some(k) = config.serial_port_name.as_ref() {
            tracing::debug!(class="Connector", "serial port name is provided: {:?}", k);
            return Ok(k.clone());
        }
        
        tracing::debug!(class="Connector", "usb ids: {:?}", config.usb_serial);

        // Check if
        // if usb_vid only use it only

        // Check if the usb vendor and model are provided to find the key
        if let Some(k) = tokio_serial::available_ports()
            .and_then(|ports| {
                for port in ports {

                    // Some debug logs
                    tracing::debug!(class="Connector", " - serial port: {:?}", port);

                    match port.port_type {
                        tokio_serial::SerialPortType::UsbPort(info) => {
                            
                            if Gate::match_usb_info_port(info, config).is_ok() {
                                return Ok(port.port_name);
                            }
                            
                            // if info.vid == config.usb_vendor &&
                            //     info.product_id == config.usb_model.unwrap_or(0) {
                            //     return Ok(port.port_name);
                            // }
                        },
                        _ => {}
                    }
                }
                Err(tokio_serial::Error::new(tokio_serial::ErrorKind::Unknown, "no port found"))
            })
            .ok()
        {
            return Ok(k.clone());
        }

        // Finally unable to generate a key with the config
        return Err(platform_error!("Unable to generate a key from the config"));
    }



    fn match_usb_info_port(usb_info_port: tokio_serial::UsbPortInfo, config: &Config)
        -> PlatformFunctionResult {


        let valid_vid = config.usb_vendor
            .and_then(
                |val| Some(val == usb_info_port.vid)
            )
            .ok_or(platform_error!("usb_vendor is missing from config (you need at least vid)"))?;

        let valid_pid = config.usb_model
            .and_then(
                |val| Some(val == usb_info_port.pid)
            )
            .or(Some(true)) // If the pid is not provided, it's ok
            .unwrap(); // "Some" value here anyway

        let valid_serial = config.usb_serial.as_ref()
            .and_then(
                |val| {
                    usb_info_port.serial_number
                        .and_then( |s| Some(s == *val) )
                }
            )
            .or(Some(true)) // If the serial is not provided, it's ok
            .unwrap(); // "Some" value here anyway
    
        // Ok only if all the conditions are met
        if valid_vid && valid_pid && valid_serial {
            Ok(())
        } else {
            platform_error_result!("usb info port does not match the config")
        }
    }

}



#[derive(Clone)]
pub struct TtyConnector {
    core: Option<Arc<tokio::sync::Mutex<TtyCore>>>,
}
pub type Connector = TtyConnector;

impl TtyConnector {
    
    pub fn new(config: Option<Config>) -> TtyConnector {
        match config {
            Some(config)    => {
                TtyConnector {
                    core: Some(
                        Arc::new(tokio::sync::Mutex::new(
                            TtyCore::new(config)))
                    )
                }
            }
            None            => {
                TtyConnector {
                    core: None
                }
            }
        }
    }

    pub async fn init(&mut self) -> PlatformFunctionResult {
        let _ = match self.core.as_ref()
            {
                Some(val) => val.lock().await.init().await,
                None => return platform_error_result!("Unable to initialize TTY connector")
            };

        Ok(())
    }

    pub async fn write(&mut self, command: &[u8],
        time_lock: Option<Duration>) 
            -> Result<usize, PlatformError> {
        match self.core.as_ref()
        {
            Some(val) => val.lock().await.write(command, time_lock).await,
            None => return platform_error_result!("Unable to write")
        }
    }


    pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8],
        time_lock: Option<Duration>) 
            -> Result<usize, PlatformError> {
        match self.core.as_ref()
            {
                Some(val) => val.lock().await.write_then_read(command, response, time_lock).await,
                None => return platform_error_result!("Unable to write then read")
            }
    }

}




struct TimeLock {
    duration: tokio::time::Duration,
    t0: tokio::time::Instant
}


struct TtyCore {
    config: Config,
    builder: Option< SerialPortBuilder >,
    serial_stream: Option< SerialStream >,
    time_lock: Option<TimeLock>
}

impl TtyCore {

    fn new(config: Config) -> TtyCore {
        TtyCore {
            config: config,
            builder: None,
            serial_stream: None,
            time_lock: None
        }
    }

    async fn init(&mut self) -> PlatformFunctionResult {

        // dirty fix, need to be improved
        if self.serial_stream.is_some() {
            return Ok(());
        }

        if self.config.serial_port_name.is_none() && self.config.usb_serial.is_some() {

            let ports = match tokio_serial::available_ports() {
                Ok(p) => p,
                Err(_e) => return  platform_error_result!("Unable to list serial ports")
            };
            for port in ports {
                match port.port_type {
                    tokio_serial::SerialPortType::UsbPort(info) => {
                        if info.serial_number == self.config.usb_serial {
                            self.config.serial_port_name = Some(port.port_name);
                        }
                    },
                    _ => {}
                }
            }
        } else {
            tracing::trace!(class="Platform", "unknown serial_port_name and usb_vendor");
        }

        let serial_builder = tokio_serial::new(
            match self.config.serial_port_name.as_ref() {
                Some(val) => val,
                None => return platform_error_result!("Serial port name is empty")
            },
            match self.config.serial_baudrate {
                Some(val) => val,
                None => return platform_error_result!("Serial baudrate is empty")
            }

        );

        

        let pp = SerialStream::open(&serial_builder);
        let aa = pp.expect("pok");

        
        self.builder = Some(serial_builder);
        self.serial_stream = Some(aa);

        Ok(())
    }


    async fn time_locked_write(&mut self, command: &[u8], duration: Option<Duration>)-> Result<usize, PlatformError> {


        if let Some(lock) = self.time_lock.as_mut() {
            let elapsed = tokio::time::Instant::now() - lock.t0;
            if elapsed < lock.duration {
                let wait_time = lock.duration - elapsed;
                sleep(wait_time).await;
            }
            self.time_lock = None;
        }

        // Send the command
        let stream = match self.serial_stream.as_mut() {
            Some(s) => s,
            None => return platform_error_result!("No serial stream")
        };
        
        let rrr = match stream.write(command).await {
            Ok(val) => Ok(val),
            Err(_e) => return platform_error_result!("Unable to write on serial stream")
        };

        // Set the time lock
        if let Some(duration) = duration {
            self.time_lock = Some(TimeLock {
                duration: duration,
                t0: tokio::time::Instant::now()
            });
        }

        rrr
    }

    
    async fn write(&mut self, command: &[u8],
        time_lock: Option<Duration>) 
            -> Result<usize, PlatformError> {

        self.time_locked_write(command, time_lock).await
    }

    async fn write_then_read(&mut self, command: &[u8], response: &mut [u8],
        time_lock: Option<Duration>) 
            -> Result<usize, PlatformError> {


        self.time_locked_write(command, time_lock).await?;


        let stream = match self.serial_stream.as_mut() {
            Some(s) => s,
            None => return platform_error_result!("No serial stream")
        };

        match stream.read(response).await {
            Ok(val) => Ok(val),
            Err(_e) => platform_error_result!("Unable to read on serial stream")
        }

        

    }


}
