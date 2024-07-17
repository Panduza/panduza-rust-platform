use panduza_core::FunctionResult;

use crate::{ConnectorLogger, SerialSettings};



use crate::serial::generic::{get as SerialGetFunction, SerialConnector};



pub struct Driver {

    logger: ConnectorLogger,
    settings: SerialSettings,

    serial_connector : SerialConnector
    // builder: Option< SerialPortBuilder >,
    
    // serial_stream: Option< SerialStream >,

    // time_lock: Option<TimeLock>
}




impl Driver {

    /// Create a new instance of the driver
    /// 
    pub fn new(settings: &SerialSettings) -> Self {
        // Get the port name safely
        let port_name = settings.port_name.as_ref()
            .map(|val| val.clone())
            .unwrap_or("undefined".to_string()).clone();

        // Create instance
        Driver {
            logger: ConnectorLogger::new("slip", port_name),
            settings: settings.clone(),
            serial_connector: SerialConnector::new()
        }
    }


    
    /// Initialize the driver
    /// 
    pub async fn init(&mut self) -> FunctionResult {

        // Internal driver already initialized by an other entity => OK
        if self.serial_connector.is_initialized() {
            return Ok(());
        }


        self.serial_connector = SerialGetFunction(&self.settings).await?;
        


        Ok(())
    }
    


}


impl Drop for Driver {
    fn drop(&mut self) {
        // Close the serial stream
        self.logger.log_warn("Closing serial connector");
        self.serial_connector.clear();
    }
}

