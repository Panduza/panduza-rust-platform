use std::error::Error;
use std::{collections::HashMap, sync::Arc};
use tokio_serial::{self, SerialPortBuilder};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio_serial::SerialStream;
use tokio::time::{sleep, Duration};

use tokio;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::platform_error;


lazy_static! {
    static ref GATE : tokio::sync::Mutex<Gate> 
        = tokio::sync::Mutex::new(Gate { instances: HashMap::new() });
}

pub async fn get(config: &Config) -> Option<TtyConnector> {
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

    pub fn import_from_json_settings(&mut self, settings: &serde_json::Value) {

        self.serial_port_name =
            settings.get("serial_port_name")
                .map(|v| v.as_str().unwrap().to_string());

                // .unwrap().to_string()

        self.serial_baudrate =
            settings.get("serial_baudrate")
                .map(|v| v.as_u64().unwrap() as u32);

        self.usb_vendor =
            settings.get("usb_vendor")
                .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

        self.usb_model =
            settings.get("usb_model")
                .map(|v| v.as_str().unwrap().to_string().parse::<u16>().unwrap());

        self.usb_serial =
            settings.get("usb_serial")
                .map(|v| v.as_str().unwrap().to_string());

    }
}



struct Gate {
    instances: HashMap<String, TtyConnector>
}

impl Gate {


    fn get(&mut self, config: &Config) -> Option<TtyConnector> {
        // First try to get the key
        let key_string = Gate::generate_unique_key_from_config(config)?;
        let key= key_string.as_str();

        // if !(self.instances.contains_key(&key)) {
        //     self.instances.get(&key) = String::new();
        //     match (Gate{instanes: self.instances}) {
        //         Ok(mut new_instance) => {
        //             async {
        //                 new_instance.connect().await;
        //             };
                    
        //             self.instances.get(&key) = new_instance;
        //             tracing::info!(class="Platform", "connector created");
        //         }
        //         Err(e) => {
        //             tracing::trace!(class="Platform", "Error during initialization");
        //         }
        //     }
        // } else {
        //     tracing::info!(class="Platform", "connector already created, use existing instance");
        // }


        // if the instance is not found, it means that the port is not opened yet
        if ! self.instances.contains_key(key) {


            println!("Creating new instance !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!, {}", key);

            // Create a new instance
            let new_instance = TtyConnector::new(Some(config.clone()));

            // Save the instance
            self.instances.insert(key.to_string(), new_instance.clone());
            tracing::info!(class="Platform", "connector created");
        }

        // Try to find the instance
        let instance = self.instances.get(key)?;

        // Return the instance
        Some(instance.clone())
    }

    /// Try to generate a unique key from the config
    /// This key will be used to find back the tty connector
    ///
    fn generate_unique_key_from_config(config: &Config) -> Option<String> {
        // Check if the serial port name is provided
        if let Some(k) = config.serial_port_name.as_ref() {
            return Some(k.clone());
        }

        // Check if the usb vendor and model are provided to find the key
        if let Some(k) = tokio_serial::available_ports()
            .and_then(|ports| {
                for port in ports {
                    match port.port_type {
                        tokio_serial::SerialPortType::UsbPort(info) => {
                            if info.vid == config.usb_vendor.unwrap() && info.pid == config.usb_model.unwrap() {
                                return Ok(port.port_name);
                            }
                        },
                        _ => {}
                    }
                }
                Err(tokio_serial::Error::new(tokio_serial::ErrorKind::Unknown, "no port found"))
            })
            .ok()
        {
            return Some(k.clone());
        }

        // Finally unable to generate a key with the config
        return None;
    }

}



#[derive(Clone)]
pub struct TtyConnector {
    core: Option<Arc<tokio::sync::Mutex<TtyCore>>>,
}

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

    pub async fn init(&mut self) {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .init()
            .await;
    }

    pub async fn write(&mut self, command: &[u8],
        time_lock: Option<Duration>) 
            -> Result<usize> {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .write(command, time_lock)
            .await
    }


    pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8],
        time_lock: Option<Duration>) 
            -> Result<usize> {
        self.core
            .as_ref()
            .unwrap()
            .lock()
            .await
            .write_then_read(command, response, time_lock)
            .await
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

    async fn init(&mut self) {

        if self.config.serial_port_name.is_none() && self.config.usb_vendor.is_some() && self.config.usb_model.is_some() {

            let ports = tokio_serial::available_ports().unwrap();
            for port in ports {
                match port.port_type {
                    tokio_serial::SerialPortType::UsbPort(info) => {
                        if info.vid == self.config.usb_vendor.unwrap() && info.pid == self.config.usb_model.unwrap(){
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
            self.config.serial_port_name.as_ref().unwrap()   ,
            self.config.serial_baudrate.unwrap()

        );

        

        let pp = SerialStream::open(&serial_builder);
        let aa = pp.expect("pok");

        
        self.builder = Some(serial_builder);
        self.serial_stream = Some(aa);

    }


    async fn time_locked_write(&mut self, command: &[u8], duration: Option<Duration>)-> Result<usize> {


        if let Some(lock) = self.time_lock.as_mut() {
            let elapsed = tokio::time::Instant::now() - lock.t0;
            if elapsed < lock.duration {
                let wait_time = lock.duration - elapsed;
                sleep(wait_time).await;
            }
            self.time_lock = None;
        }

        // Send the command
        let rrr = self.serial_stream.as_mut().unwrap().write(command).await;

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
            -> Result<usize> {

        self.time_locked_write(command, time_lock).await
    }

    async fn write_then_read(&mut self, command: &[u8], response: &mut [u8],
        time_lock: Option<Duration>) 
            -> Result<usize> {


        let _ = self.time_locked_write(command, time_lock).await;


        // let mut buf: &mut [u8] = &mut [0; 1024];
        self.serial_stream.as_mut().unwrap().read(response).await
        // let n = p.unwrap();
        // println!("Read {} bytes", n);

    }


}

