use std::{collections::HashMap, sync::Arc};
use tokio_serial::{UsbPortInfo};

use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref GATE : Mutex<Gate> 
        = Mutex::new(Gate { instances: HashMap::new() });
}

pub fn Get(config: &Config) -> Tty {
    let gate = GATE.lock().unwrap();
    gate.get(config)
}


struct Config {

    serial_port_name: Option<String>,

    usb_vendor: Option<String>,
    usb_model: Option<String>,

    // serial_baudrate: Option<>
}


struct Gate {
    instances: HashMap<String, Tty>
}

impl Gate {


    fn get(&self, config: &Config) -> Tty {

        // First try to get the key
        let mut key = String::new();
        if config.serial_port_name.is_some() {
            key = config.serial_port_name.clone().unwrap();
        } else {
            tracing::trace!(class="Platform", "No way to identify the serial port");
        }

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

        if !(self.instances.contains_key(&key)) {
            self.instances.get(&key) = String::new();
            match (Gate{instanes: self.instances}) {
                Ok(mut new_instance) => {
                    async {
                        new_instance.connect().await;
                    };
                    
                    self.instances.get(&key) = new_instance;
                    tracing::info!(class="Platform", "connector created");
                }
                Err(e) => {
                    tracing::trace!(class="Platform", "Error during initialization");
                }
            }
        } else {
            tracing::info!(class="Platform", "connector already created, use existing instance");
        }

        // Return the instance
        self.instances.get(key.as_str()).unwrap().clone()
    }

}



#[derive(Clone)]
pub struct Tty {
//     // fd: RawFd,
//     // termios: Termios,
//     // termios_backup: Termios,
//     // baudrate: BaudRate,
//     // timeout: Duration,
}


