use panduza_core::FunctionResult;
use panduza_core::Error as PlatformError;

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

    /// Encode command with SLIP (serial line internet protocol)
    /// and write it to the serial port.
    /// Then wait for the response and decode it before returning it to the user.
    /// 
    /// # Arguments
    /// 
    /// - `command` - Command to be sent to the serial port.
    /// - `response` - Buffer to store the response.
    /// 
    pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8])
            -> Result<usize, PlatformError> {
        

        let mut encoded_command = [0u8; 15];
        let mut slip_encoder = serial_line_ip::Encoder::new();
        
        let res = slip_encoder.encode(command, &mut encoded_command);

        if res.is_ok() {
            let rrrrr = res.unwrap();
            println!("Encoding command: r{:?} w{:?}", rrrrr.read, rrrrr.written);
        }
        println!("Encoded command: {:?}", encoded_command);


        let slip_decoder = serial_line_ip::Decoder::new();

        
        // self.parent_connector
        //     .write_then_read(command, response, None)
        //     .await


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

