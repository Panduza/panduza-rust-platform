use std::sync::Arc;

use futures_lite::future::block_on;

use panduza_core::platform_error;
use panduza_core::Error as PlatformError;
use panduza_core::FunctionResult;

use nusb::list_devices;
use nusb::transfer::Direction;
use nusb::transfer::EndpointType;
use nusb::transfer::RequestBuffer;
use nusb::Interface;

use tokio::sync::Mutex;

// use crate::ConnectorLogger;
use crate::UsbSettings;

/// Serial SLIP driver
///
pub struct Driver {
    // // Logger
    // logger: ConnectorLogger,
    // Usb settings
    settings: UsbSettings,
    // Usb interface
    interface: Option<Interface>,
    // // Accumulated incoming data buffer
    // in_buf: [u8; 512],
    // // Keep track of number of data in the buffer
    // in_buf_size: usize,
}

/// Connector is just a mutex protected driver
///
pub type Connector = Arc<Mutex<Driver>>;

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn new(settings: &UsbSettings) -> Self {
        // // Get the serial number safely
        // let serial_number = settings
        //     .serial
        //     .as_ref()
        //     .map(|val| val.clone())
        //     .unwrap_or("undefined".to_string())
        //     .clone();

        // Create instance
        Driver {
            // logger: ConnectorLogger::new("serial", serial_number),
            settings: settings.clone(),
            interface: None,
            // in_buf: [0u8; 512],
            // in_buf_size: 0,
        }
    }

    /// Convert the driver into a connector
    ///
    pub fn into_connector(self) -> Connector {
        Arc::new(Mutex::new(self))
    }

    /// Initialize the driver
    ///
    pub async fn init(&mut self) -> FunctionResult {
        // Get the serial number
        let serial_number = self
            .settings
            .serial
            .as_ref()
            .ok_or(platform_error!("Serial number is not set in settings"))?;

        // Get the informations of the device
        let device_info = list_devices()
            .map_err(|e| platform_error!("Unable to list usb devices: {}", e))?
            .find(|d| d.serial_number() == Some(serial_number))
            .ok_or(platform_error!("Usb device not found"))?;

        // Open the device
        let device = device_info
            .open()
            .map_err(|e| platform_error!("Unable to open usb device: {}", e))?;

        // Create the interface with the device
        self.interface = Some(
            device
                .claim_interface(0)
                .map_err(|e| platform_error!("Unable to claim usb interface: {}", e))?,
        );

        Ok(())
    }

    /// Write a command on the usb
    ///
    pub async fn write(&mut self, command: &[u8]) -> FunctionResult {
        let itf = self
            .interface
            .as_ref()
            .ok_or(platform_error!("Unable to use usb interface"))?;

        // Find the Bulk Out endpoint
        for interface_descriptor in itf.descriptors() {
            let endpoint = interface_descriptor
                .endpoints()
                .find(|e| {
                    e.direction() == Direction::Out && e.transfer_type() == EndpointType::Bulk
                })
                .ok_or(platform_error!("Bulk Out endpoint not found"))?;

            // Send the command on the usb
            block_on(itf.bulk_out(endpoint.address(), command.to_vec()))
                .into_result()
                .map_err(|e| platform_error!("Unable to write usb data: {}", e))?;
        }

        Ok(())
    }

    /// Read the data form the usb
    ///
    pub async fn read(&mut self) -> Result<String, PlatformError> {
        let itf = self
            .interface
            .as_ref()
            .ok_or(platform_error!("Unable to use usb interface"))?;

        let mut msg = String::new();

        // find the Bulk In endpoint
        for interface_descriptor in itf.descriptors() {
            let endpoint = interface_descriptor
                .endpoints()
                .find(|e| e.direction() == Direction::In && e.transfer_type() == EndpointType::Bulk)
                .ok_or(platform_error!("Bulk Out endpoint not found"))?;

            let resp_buf = RequestBuffer::new(32 as usize);

            let data = block_on(itf.bulk_in(endpoint.address(), resp_buf))
                .into_result()
                .map_err(|e| platform_error!("Unable to read usb data: {}", e))?;

            msg = String::from_utf8(data)
                .map_err(|e| platform_error!("Unable to decode usb data: {}", e))?;
        }

        Ok(msg)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_slip_decode() {
//         const SLIP_ENCODED: [u8; 8] = [0xc0, 0x01, 0x02, 0x03, 0x04, 0x05, 0xc0, 0x04];
//         const DATA: [u8; 5] = [0x01, 0x02, 0x03, 0x04, 0x05];

//         let mut output: [u8; 32] = [0; 32];
//         let mut slip = serial_line_ip::Decoder::new();

//         let (input_bytes_processed, output_slice, is_end_of_packet) =
//             slip.decode(&SLIP_ENCODED, &mut output).unwrap();

//         assert_eq!(7, input_bytes_processed);
//         assert_eq!(&DATA, output_slice);
//         assert_eq!(true, is_end_of_packet);
//    }
// }
