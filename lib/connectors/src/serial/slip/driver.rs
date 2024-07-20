use panduza_core::platform_error;
use panduza_core::FunctionResult;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio_serial::SerialStream;


use panduza_core::Error as PlatformError;



use crate::ConnectorLogger;
use crate::SerialSettings;



struct TimeLock {
    pub duration: tokio::time::Duration,
    pub t0: tokio::time::Instant
}




pub struct Driver {
    logger: ConnectorLogger,
    settings: SerialSettings,
    // builder: Option< SerialPortBuilder >,
    
    serial_stream: Option< SerialStream >,

    time_lock: Option<TimeLock>
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
            logger: ConnectorLogger::new("serial", port_name),
            settings: settings.clone(),
            serial_stream: None,
            time_lock: None
        }
    }

    /// Initialize the driver
    /// 
    pub async fn init(&mut self) -> FunctionResult {

        // Internal driver already initialized by an other entity => OK
        if self.serial_stream.is_some() {
            return Ok(());
        }

        // Get the port name
        let port_name = self.settings.port_name.as_ref()
            .ok_or_else(|| platform_error!("Port name is not set in settings"))?;

        // Setup builder
        let serial_builder = tokio_serial::new(
            port_name,
            self.settings.baudrate
        )
            .data_bits(self.settings.data_bits)
            .stop_bits(self.settings.stop_bits)
            .parity(self.settings.parity)
            .flow_control(self.settings.flow_control);
         
        // Build the stream
        self.serial_stream = Some(
            SerialStream::open(&serial_builder)
                .map_err(|e| platform_error!("Unable to open serial stream: {}", e))?
        );

        Ok(())
    }

    /// Write a command on the serial stream
    /// 
    pub async fn write_time_locked(&mut self, command: &[u8])-> Result<usize, PlatformError> {

        // Check if a time lock is set
        if let Some(lock) = self.time_lock.as_mut() {
            let elapsed = tokio::time::Instant::now() - lock.t0;
            if elapsed < lock.duration {
                let wait_time = lock.duration - elapsed;
                tokio::time::sleep(wait_time).await;
            }
            self.time_lock = None;
        }

        // Send the command
        let write_result = self.serial_stream.as_mut()
            .ok_or_else(|| platform_error!("No serial stream"))?
            .write(command).await
            .map_err(|e| platform_error!("Unable to write on serial stream: {}", e));

        // Set the time lock
        if let Some(duration) = self.settings.time_lock_duration {
            self.time_lock = Some(TimeLock {
                duration: duration,
                t0: tokio::time::Instant::now()
            });
        }

        return write_result;
    }

    /// Lock the connector to write a command then wait for the answers
    /// 
    pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8]) 
            -> Result<usize, PlatformError>
    {

        // Prepare encoding
        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();
        
        // Encode the command
        let mut totals = slip_encoder.encode(command, &mut encoded_command)
            .map_err(|e| platform_error!("Unable to encode command: {:?}", e))?;

        // Finalise the encoding
        totals += slip_encoder.finish(&mut encoded_command[totals.written..])
            .map_err(|e| platform_error!("Unable to finsh command encoding: {:?}", e))?;

        // Write command slip encoded
        self.write_time_locked(&encoded_command[..totals.written]).await?;

        // Prepare decoding
        // let mut slip_decoder = serial_line_ip::Decoder::new();
        // let total_decoded = slip_decoder.decode(&encoded_response[..total_read], response)
        //     .map_err(|e| platform_error!("Unable to decode response: {:?}", e))?;

        // Ok(total_decoded.0)
        Ok(3)
    }

    ///
    /// 
    pub async fn write_then_read_until(&mut self, command: &[u8], response: &mut [u8], end: u8)
            -> Result<usize, PlatformError>
    {
        // Write
        self.write_time_locked(command).await?;

        // Read the response until "end"
        let mut n = 0;
        loop {
            let mut single_buf = [0u8; 1];
            self.serial_stream.as_mut()
                .ok_or_else(|| platform_error!("No serial stream"))?
                .read_exact(&mut single_buf).await
                .map_err(|e| platform_error!("Unable to read on serial stream {:?}", e))?;
            response[n] = single_buf[0];
            n += 1;
            if single_buf[0] == end {
                break;
            }
        }
        Ok(n)
    }

}

impl Drop for Driver {
    fn drop(&mut self) {
        // Close the serial stream
        self.logger.log_warn("Closing serial stream");
        self.serial_stream = None;
    }
}



// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slip_decode() {

        const SLIP_ENCODED: [u8; 8] = [
            0xc0,
            0x01, 0x02, 0x03, 0x04, 0x05,
            0xc0 ,0x04
        ];
        const DATA: [u8; 5] = [0x01, 0x02, 0x03, 0x04, 0x05];
        
        let mut output: [u8; 32] = [0; 32];
        let mut slip = serial_line_ip::Decoder::new();
        
        let (input_bytes_processed, output_slice, is_end_of_packet) =
            slip.decode(&SLIP_ENCODED, &mut output).unwrap();
        
        assert_eq!(7, input_bytes_processed);
        assert_eq!(&DATA, output_slice);
        assert_eq!(true, is_end_of_packet);
    }

}





// use panduza_core::platform_error;
// use panduza_core::FunctionResult;
// use panduza_core::Error as PlatformError;

// use crate::{ConnectorLogger, SerialSettings};



// use crate::serial::generic::{get as SerialGetFunction, SerialConnector};



// pub struct Driver {

//     logger: ConnectorLogger,
//     settings: SerialSettings,

//     serial_connector : SerialConnector
//     // builder: Option< SerialPortBuilder >,
    
//     // serial_stream: Option< SerialStream >,

//     // time_lock: Option<TimeLock>
// }




// impl Driver {

//     /// Create a new instance of the driver
//     /// 
//     pub fn new(settings: &SerialSettings) -> Self {
//         // Get the port name safely
//         let port_name = settings.port_name.as_ref()
//             .map(|val| val.clone())
//             .unwrap_or("undefined".to_string()).clone();

//         // Create instance
//         Driver {
//             logger: ConnectorLogger::new("slip", port_name),
//             settings: settings.clone(),
//             serial_connector: SerialConnector::new()
//         }
//     }


    
//     /// Initialize the driver
//     /// 
//     pub async fn init(&mut self) -> FunctionResult {

//         // Internal driver already initialized by an other entity => OK
//         if self.serial_connector.is_initialized() {
//             return Ok(());
//         }

//         self.serial_connector = SerialGetFunction(&self.settings).await?;
        
//         Ok(())
//     }

//     /// Encode command with SLIP (serial line internet protocol)
//     /// and write it to the serial port.
//     /// Then wait for the response and decode it before returning it to the user.
//     /// 
//     /// # Arguments
//     /// 
//     /// - `command` - Command to be sent to the serial port.
//     /// - `response` - Buffer to store the response.
//     /// 
//     pub async fn write_then_read(&mut self, command: &[u8], response: &mut [u8])
//             -> Result<usize, PlatformError> {
        
//         // Prepare encoding
//         let mut encoded_command = [0u8; 200];
//         let mut slip_encoder = serial_line_ip::Encoder::new();
        
//         // Encode the command
//         let mut totals = slip_encoder.encode(command, &mut encoded_command)
//             .map_err(|e| platform_error!("Unable to encode command: {:?}", e))?;

//         // Finalise the encoding
//         totals += slip_encoder.finish(&mut encoded_command[totals.written..])
//             .map_err(|e| platform_error!("Unable to finsh command encoding: {:?}", e))?;

//         // Write the command to the serial port
//         let mut encoded_response = [0u8; 200];
//         let total_read = self.serial_connector
//             .write_then_read_until(&encoded_command, &mut encoded_response, 0xc0 as u8)
//             .await?;

//         // Prepare decoding
//         let mut slip_decoder = serial_line_ip::Decoder::new();
//         let total_decoded = slip_decoder.decode(&encoded_response[..total_read], response)
//             .map_err(|e| platform_error!("Unable to decode response: {:?}", e))?;

//         Ok(total_decoded.0)
//     }


// }


// impl Drop for Driver {
//     fn drop(&mut self) {
//         // Close the serial stream
//         self.logger.log_warn("Closing serial connector");
//         self.serial_connector.clear();
//     }
// }

