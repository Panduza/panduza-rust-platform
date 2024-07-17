use std::sync::Arc;
use tokio::sync::Mutex;

use super::SerialDriver;
use crate::SerialSettings;

use panduza_core::platform_error;
use panduza_core::FunctionResult;
use panduza_core::Error as PlatformError;

#[derive(Clone)]
pub struct Connector {
    driver: Option< Arc< Mutex< SerialDriver > > >
}

impl Connector {
    
    /// Create a new instance of the connector
    /// 
    pub fn new() -> Self {
        Connector {
            driver: None
        }
    }

    /// Create a new instance of the connector from the settings
    /// 
    pub fn from_settings(settings: &SerialSettings) -> Self {
        Connector {
            driver: Some( Arc::new(Mutex::new( SerialDriver::new(settings) )) )
        }
    }
    
    /// Check if the connector is initialized
    /// 
    pub fn is_initialized(&self) -> bool {
        self.driver.is_some()
    }

    /// Clear the connector
    /// 
    pub fn clear(&mut self) {
        self.driver = None;
    }

    /// Count the number of references to the driver
    /// 
    pub fn count_refs(&self) -> usize {
        match self.driver.as_ref() {
            Some(obj) => Arc::strong_count(obj),
            None => 0
        }
    }

    /// Initialize the connector
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

    /// Write a command to the serial port
    /// 
    pub async fn write_then_read_until(&mut self, command: &[u8], response: &mut [u8], end: u8)
            -> Result<usize, PlatformError> {
        self.driver
            .as_ref()
            .ok_or(platform_error!("Connector is not initialized"))?
            .lock().await
            .write_then_read_until(command, response, end).await
    } 

}
