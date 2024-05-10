use std::{collections::HashMap, sync::Arc};
use tokio_serial::{self, SerialPort, SerialPortBuilder};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio_serial::{SerialStream};
use tokio::time::{sleep, Duration};

use tokio;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref GATE : Mutex<Gate> 
        = Mutex::new(Gate { instances: HashMap::new() });
}

pub fn get(config: &Config) -> Option<TtyConnector> {
    let mut gate = GATE.lock().unwrap();
    gate.get(config)
}


#[derive(Clone, Debug)]
pub struct Config {
    pub usb_vendor: Option<String>,
    pub usb_model: Option<String>,

    pub serial_port_name: Option<String>,
    pub serial_baudrate: Option<u32>
}

impl Config {
    pub fn new() -> Config {
        Config {
            usb_vendor: None,
            usb_model: None,
            serial_port_name: None,
            serial_baudrate: None,
        }
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

        // # Get the serial port name
        // serial_port_name = None
        // if "serial_port_name" in kwargs:
        //     serial_port_name = kwargs["serial_port_name"]
        // elif "usb_vendor" in kwargs:
        //     # Get the serial port name using "usb_vendor"
        //     serial_port_name = SerialPortFromUsbSetting(**kwargs)
        //     kwargs["serial_port_name"] = serial_port_name
    
        // else:
        //     raise Exception("no way to identify the serial port")

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

    /// The on this connector is the serial port name
    ///
    fn generate_unique_key_from_config(config: &Config) -> Option<String> {
        // Check if the serial port name is provided
        if let Some(k) = config.serial_port_name.as_ref() {
            return Some(k.clone());
        }
        // Finally unable to generate a key with the config
        None
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


}




struct TtyCore {
    config: Config,
    builder: Option< SerialPortBuilder >,
    serial_stream: Option< SerialStream >,
}

impl TtyCore {

    fn new(config: Config) -> TtyCore {
        TtyCore {
            config: config,
            builder: None,
            serial_stream: None,
        }
    }

    async fn init(&mut self) {

        let serial_builder = tokio_serial::new(
            self.config.serial_port_name.as_ref().unwrap()   ,
            self.config.serial_baudrate.unwrap()
        );


        let pp = SerialStream::open(&serial_builder);
        let aa = pp.expect("pok");

        
        self.builder = Some(serial_builder);
        self.serial_stream = Some(aa);
       
        // aa.read(buf).await;
    }


    async fn time_locked_write(&mut self, command: &[u8]) {
        self.serial_stream.as_mut().unwrap().write(command).await.unwrap();
    
        // Duration
    }


    async fn write_then_read_during(&mut self, command: &[u8]) {


        // self.serial_stream.as_mut().unwrap().write(command).await.unwrap();

        // self.serial_stream.as_mut().unwrap().read(&mut [0; 10]).await;
    }

    // async def write_and_read_during(self, message, time_lock_s=0, read_duration_s=0.5):
    //     """Write command then read data for specified duration
    //     """
    //     async with self._mutex:
    //         await self.__write(message, time_lock_s)
    //         return await self.__read_during(read_duration_s)


}

