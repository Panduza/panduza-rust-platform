
use std::sync::Arc;
use panduza_core::FunctionResult;
use tokio::sync::Mutex;

use super::SerialDriver;
use crate::SerialSettings;

use panduza_core::platform_error;

#[derive(Clone)]
pub struct Connector {
    driver: Option< Arc< Mutex< SerialDriver > > >
}

impl Connector {
    
    pub fn new() -> Self {
        Connector {
            driver: None
        }
    }

    pub fn from_settings(settings: &SerialSettings) -> Self {
        Connector {
            driver: Some( Arc::new(Mutex::new( SerialDriver::new(settings) )) )
        }
    }
    
    pub fn count_refs(&self) -> usize {
        match self.driver.as_ref() {
            Some(obj) => Arc::strong_count(obj),
            None => 0
        }
    }

    pub async fn init(&self) -> FunctionResult {
        self.driver
            .as_ref()
            .ok_or(platform_error!("Connector is not initialized"))?
            .lock().await
            .init().await
    }

}
