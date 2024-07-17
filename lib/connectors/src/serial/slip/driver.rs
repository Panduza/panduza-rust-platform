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


}
