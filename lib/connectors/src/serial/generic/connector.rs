
use std::sync::Arc;
use tokio::sync::Mutex;

use super::SerialDriver;
use crate::SerialSettings;

#[derive(Clone)]
pub struct Connector {
    // config: Config,
    // builder: Option< SerialPortBuilder >,
    // serial_stream: Option< SerialStream >,
    // time_lock: Option<TimeLock>


    driver: Arc< Mutex< SerialDriver > >
}

impl Connector {
    
    pub fn new(settings: &SerialSettings) -> Self {
        Connector {
            driver: Arc::new(Mutex::new( SerialDriver::new(settings) ))
        }
    }
    

    pub fn count_refs(&self) -> usize {
        Arc::strong_count(&self.driver)
    }

}

impl Drop for Connector {
    fn drop(&mut self) {
        
        println!("Connector is being dropped!");
        
        println!("d -----> {}", self.count_refs());
        // Perform cleanup logic here
    }
}
