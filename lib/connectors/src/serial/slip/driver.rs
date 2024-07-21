use std::time::Duration;

use panduza_core::platform_error;
use panduza_core::FunctionResult;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;
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

    time_lock: Option<TimeLock>,

    // Accumulated incoming data buffer
    in_buf: [u8; 512],
    // Keep track of number of data in the buffer
    in_buf_size: usize,
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
            time_lock: None,
            in_buf: [0u8; 512],
            in_buf_size: 0
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

        // Read the response until "end"
        loop {

            // Read a chunck
            let read_chunk = self.serial_stream.as_mut()
                .ok_or_else(|| platform_error!("No serial stream"))?
                .read(&mut self.in_buf[self.in_buf_size..]);
        
            self.in_buf_size += timeout(Duration::from_secs(5), read_chunk).await
                .map_err(|e| platform_error!("Timeout reading {:?}", e))?
                .map_err(|e| platform_error!("Unable to read on serial stream {:?}", e))?;

            // Try decoding
            let mut slip_decoder = serial_line_ip::Decoder::new();
            let (total_decoded, _out_slice, end) = slip_decoder.decode(&self.in_buf[..self.in_buf_size], response)
                .map_err(|e| platform_error!("Unable to decode response: {:?}", e))?;

            if end {
                return Ok(total_decoded);
            }
        }
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





