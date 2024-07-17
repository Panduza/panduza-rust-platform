
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::SerialSettings;

use super::SlipDriver;

// use nusb::Error;

// use super::super::tty3::TtyConnector as TtyConnector;

// use panduza_core::interface::logger::{self, Logger as InterfaceLogger};
// use panduza_core::{Error as PlatformError, FunctionResult};


#[derive(Clone)]
pub struct Connector {
    driver: Option< Arc< Mutex< SlipDriver > > >
}

impl Connector {

    pub fn new() -> Self {
        Connector {
            driver: None
        }
    }

    pub fn from_settings(settings: &SerialSettings) -> Self {
        Connector {
            driver: Some( Arc::new(Mutex::new( SlipDriver::new(settings) )) )
        }
    }
    
    pub fn count_refs(&self) -> usize {
        match self.driver.as_ref() {
            Some(obj) => Arc::strong_count(obj),
            None => 0
        }
    }

//     pub fn new(parent_connector: TtyConnector,
//         logger: Option<InterfaceLogger>
//         ) -> Connector {
//         Connector {
//             parent_connector, logger
//         }
//     }

//     pub async fn init(&mut self) -> FunctionResult  {
//         self.parent_connector
//             .init()
//             .await
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
        

//         let mut encoded_command = [0u8; 15];
//         let mut slip_encoder = serial_line_ip::Encoder::new();
        
//         let res = slip_encoder.encode(command, &mut encoded_command);

//         if res.is_ok() {
//             let rrrrr = res.unwrap();
//             println!("Encoding command: r{:?} w{:?}", rrrrr.read, rrrrr.written);
//         }
//         println!("Encoded command: {:?}", encoded_command);


//         let slip_decoder = serial_line_ip::Decoder::new();

        
//         self.parent_connector
//             .write_then_read(command, response, None)
//             .await


//     }

}



