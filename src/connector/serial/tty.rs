use std::{collections::HashMap, sync::Arc};

use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref GATE : Mutex<Gate> 
        = Mutex::new(Gate { instances: HashMap::new() });
}

pub fn Get(name: &str) -> Tty {
    let gate = GATE.lock().unwrap();
    gate.get(name)
}


struct Gate {
    instances: HashMap<String, Tty>
}

impl Gate {

    fn get(&self, name: &str) -> Tty {
        self.instances.get(name).unwrap().clone()
    }

    // fn add_instance(&mut self, name: &str, tty: Tty) {
    //     self.instances.insert(name.to_string(), tty);
    // }
}



// lazy_static! {
// static mut INSTANCES: Arc<Mutex<>>
//     = Arc::new(Mutex::new(HashMap::new()));
// }

#[derive(Clone)]
struct Tty {
//     // fd: RawFd,
//     // termios: Termios,
//     // termios_backup: Termios,
//     // baudrate: BaudRate,
//     // timeout: Duration,
}


