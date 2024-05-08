use std::{collections::HashMap, sync::Arc};
use tokio_serial;

use tokio;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref GATE : Mutex<Gate> 
        = Mutex::new(Gate { instances: HashMap::new() });
}

pub fn get(config: &Config) -> Option<Tty> {
    let gate = GATE.lock().unwrap();
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
    instances: HashMap<String, Tty>
}

impl Gate {


    fn get(&self, config: &Config) -> Option<Tty> {
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
pub struct Tty {
    config: Config,
//     // fd: RawFd,
//     // termios: Termios,
//     // termios_backup: Termios,
//     // baudrate: BaudRate,
//     // timeout: Duration,
    core: Arc<Mutex<TtyCore>>,
}

impl Tty {
    
    fn new(config: Config) -> Tty {

        Tty {
            config: config,
            core: Arc::new(Mutex::new(TtyCore{})),
            // fd: 0,
            // termios: Termios::from_fd(0).unwrap(),
            // termios_backup: Termios::from_fd(0).unwrap(),
            // baudrate: BaudRate::B9600,
            // timeout: Duration::from_secs(1),

        }
    }

    pub async fn init(&mut self) {

        let c = self.core.lock();
        // let serial_builder = tokio_serial::new(
        //     self.config.serial_port_name.unwrap(),
        //     self.config.serial_baudrate.unwrap()
        // );

        // let serial = serial_builder.open().await.unwrap();
    }


}


struct TtyCore {

}


