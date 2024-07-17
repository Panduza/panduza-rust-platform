
use std::sync::Arc;
use panduza_core::{platform_error, FunctionResult};
use tokio::sync::Mutex;

use crate::SerialSettings;
use panduza_core::Error as PlatformError;

use super::SlipDriver;



#[derive(Clone)]
pub struct Connector {
    driver: Option< Arc< Mutex< SlipDriver > > >
}

impl Connector {

    pub fn new() -> Self {
        Connector {
            driver: None
        }
    }

    pub fn from_settings(settings: &SerialSettings) -> Self {
        Connector {
            driver: Some( Arc::new(Mutex::new( SlipDriver::new(settings) )) )
        }
    }
    
    /// Check if the driver is initialized
    /// 
    pub fn count_refs(&self) -> usize {
        match self.driver.as_ref() {
            Some(obj) => Arc::strong_count(obj),
            None => 0
        }
    }

    /// Initialize the driver
    /// 
    pub async fn init(&self) -> FunctionResult {
        self.driver
            .as_ref()
            .ok_or(platform_error!("Connector is not initialized"))?
            .lock().await
            .init().await
    }
    
    /// Write a command to the serial port
    /// 
    pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8]) 
        -> Result<usize, PlatformError> {
        self.driver
            .as_ref()
            .ok_or(platform_error!("Connector is not initialized"))?
            .lock().await
            .write_then_read(command, response).await
    }

}



