use panduza_core::platform_error;
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
        
        // Prepare encoding
        let mut encoded_command = [0u8; 200];
        let mut slip_encoder = serial_line_ip::Encoder::new();
        
        // Encode the command
        let mut totals = slip_encoder.encode(command, &mut encoded_command)
            .map_err(|e| platform_error!("Unable to encode command: {:?}", e))?;

        // Finalise the encoding
        totals += slip_encoder.finish(&mut encoded_command[totals.written..])
            .map_err(|e| platform_error!("Unable to finsh command encoding: {:?}", e))?;

        // Write the command to the serial port
        let mut encoded_response = [0u8; 200];
        let total_read = self.serial_connector
            .write_then_read_until(&encoded_command, &mut encoded_response, 0xc0 as u8)
            .await?;

        // Prepare decoding
        let mut slip_decoder = serial_line_ip::Decoder::new();
        let total_decoded = slip_decoder.decode(&encoded_response[..total_read], response)
            .map_err(|e| platform_error!("Unable to decode response: {:?}", e))?;

        Ok(total_decoded.0)
    }


}


impl Drop for Driver {
    fn drop(&mut self) {
        // Close the serial stream
        self.logger.log_warn("Closing serial connector");
        self.serial_connector.clear();
    }
}

