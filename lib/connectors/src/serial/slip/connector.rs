

use nusb::Error;

use super::tty::TtyConnector;

use panduza_core::interface::logger::{self, Logger as InterfaceLogger};


#[derive(Clone)]
pub struct Connector {
    parent_connector: TtyConnector,
    logger: Option<InterfaceLogger>
}

impl Connector {
    
    pub fn new(parent_connector: TtyConnector,
        logger: Option<InterfaceLogger>
        ) -> Connector {
        Connector {
            parent_connector, logger
        }
    }

    pub async fn init(&mut self) {
        self.parent_connector
            .init()
            .await;
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
            -> Result<usize, Error> {
        

        let mut encoded_command = [0u8; 1024];
        let mut slip_encoder = serial_line_ip::Encoder::new();
        
        let res = slip_encoder.encode(command, &mut encoded_command);


        let slip_decoder = serial_line_ip::Decoder::new();

        
        self.parent_connector
            .write_then_read(command, response, None)
            .await


    }

}



