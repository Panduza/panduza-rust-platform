use panduza_core::platform_error;
use panduza_core::FunctionResult;
use tokio_serial::SerialStream;



use crate::ConnectorLogger;
use crate::SerialSettings;



pub struct Driver {
    logger: ConnectorLogger,
    settings: SerialSettings,
    // builder: Option< SerialPortBuilder >,
    
    serial_stream: Option< SerialStream >,

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
            logger: ConnectorLogger::new("serial", port_name),
            settings: settings.clone(),
            serial_stream: None
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
    
}

impl Drop for Driver {
    fn drop(&mut self) {
        // Close the serial stream
        self.logger.log_warn("Closing serial stream");
        self.serial_stream = None;
    }
}





//     async fn time_locked_write(&mut self, command: &[u8], duration: Option<Duration>)-> Result<usize, PlatformError> {


//         if let Some(lock) = self.time_lock.as_mut() {
//             let elapsed = tokio::time::Instant::now() - lock.t0;
//             if elapsed < lock.duration {
//                 let wait_time = lock.duration - elapsed;
//                 sleep(wait_time).await;
//             }
//             self.time_lock = None;
//         }

//         // Send the command
//         let stream = match self.serial_stream.as_mut() {
//             Some(s) => s,
//             None => return platform_error_result!("No serial stream")
//         };
        
//         let rrr = match stream.write(command).await {
//             Ok(val) => Ok(val),
//             Err(_e) => return platform_error_result!("Unable to write on serial stream")
//         };

//         // Set the time lock
//         if let Some(duration) = duration {
//             self.time_lock = Some(TimeLock {
//                 duration: duration,
//                 t0: tokio::time::Instant::now()
//             });
//         }

//         rrr
//     }

    
//     async fn write(&mut self, command: &[u8],
//         time_lock: Option<Duration>) 
//             -> Result<usize, PlatformError> {

//         self.time_locked_write(command, time_lock).await
//     }

//     async fn write_then_read(&mut self, command: &[u8], response: &mut [u8],
//         time_lock: Option<Duration>) 
//             -> Result<usize, PlatformError> {


//         self.time_locked_write(command, time_lock).await?;


//         let stream = match self.serial_stream.as_mut() {
//             Some(s) => s,
//             None => return platform_error_result!("No serial stream")
//         };

//         match stream.read(response).await {
//             Ok(val) => Ok(val),
//             Err(_e) => platform_error_result!("Unable to read on serial stream")
//         }

        

//     }


// }

