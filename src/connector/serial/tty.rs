use std::{collections::HashMap, sync::Arc};

use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref GATE : Mutex<Gate> 
        = Mutex::new(Gate { instances: HashMap::new() });
}

pub fn Get(name: &str) -> Tty {
    let gate = GATE.lock().unwrap();
    gate.get(Some(name.to_string()) )
}


struct Gate {
    instances: HashMap<String, Tty>
}

impl Gate {


    fn get(&self, serial_port_name: Option<String>) -> Tty {

    //     * ** (``str``) --
    //     serial port name

    // * *serial_baudrate* (``int``) --
    //     serial baudrate

    // * *usb_vendor* (``str``) --
    //     ID_VENDOR_ID
    // * *usb_model* (``str``) --
    //     ID_MODEL_ID


        self.instances.get(serial_port_name.unwrap().as_str()).unwrap().clone()
    }

    // fn add_instance(&mut self, name: &str, tty: Tty) {
    //     self.instances.insert(name.to_string(), tty);
    // }
}



#[derive(Clone)]
pub struct Tty {
//     // fd: RawFd,
//     // termios: Termios,
//     // termios_backup: Termios,
//     // baudrate: BaudRate,
//     // timeout: Duration,
}


