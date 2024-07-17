





pub struct Driver {
    // config: Config,
    // builder: Option< SerialPortBuilder >,
    // serial_stream: Option< SerialStream >,
    // time_lock: Option<TimeLock>
}


impl Driver {
    pub fn new() -> Self {
        println!("Driver is being created!");
        Driver {
            // driver: Some(Arc::new(Mutex::new(driver))
        }
    }
    
}

impl Drop for Driver {
    fn drop(&mut self) {
        println!("Driver is being dropped!");
        // Perform cleanup logic here
    }
}


// impl TtyCore {

//     fn new(config: Config) -> TtyCore {
//         TtyCore {
//             config: config,
//             builder: None,
//             serial_stream: None,
//             time_lock: None
//         }
//     }

//     async fn init(&mut self) -> PlatformFunctionResult {

//         // dirty fix, need to be improved
//         if self.serial_stream.is_some() {
//             return Ok(());
//         }

//         if self.config.serial_port_name.is_none() && self.config.usb_serial.is_some() {

//             let ports = match tokio_serial::available_ports() {
//                 Ok(p) => p,
//                 Err(_e) => return  platform_error_result!("Unable to list serial ports")
//             };
//             for port in ports {
//                 match port.port_type {
//                     tokio_serial::SerialPortType::UsbPort(info) => {
//                         if info.serial_number == self.config.usb_serial {
//                             self.config.serial_port_name = Some(port.port_name);
//                         }
//                     },
//                     _ => {}
//                 }
//             }
//         } else {
//             tracing::trace!(class="Platform", "unknown serial_port_name and usb_vendor");
//         }

//         let serial_builder = tokio_serial::new(
//             match self.config.serial_port_name.as_ref() {
//                 Some(val) => val,
//                 None => return platform_error_result!("Serial port name is empty")
//             },
//             match self.config.serial_baudrate {
//                 Some(val) => val,
//                 None => return platform_error_result!("Serial baudrate is empty")
//             }

//         );

        

//         let pp = SerialStream::open(&serial_builder);
//         let aa = pp.expect("pok");

        
//         self.builder = Some(serial_builder);
//         self.serial_stream = Some(aa);

//         Ok(())
//     }


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

