use std::sync::Arc;

use panduza_platform_core::Error as PlatformError;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio_serial::SerialStream;

use crate::serial::time_lock::TimeLock;
use crate::ConnectorLogger;
use crate::SerialSettings;

/// Serial GENERIC driver
///
pub struct Driver {
    logger: ConnectorLogger,
    settings: SerialSettings,
    // builder: Option< SerialPortBuilder >,
    serial_stream: Option<SerialStream>,

    time_lock: Option<TimeLock>,
}

/// Connector is just a mutex protected driver
///
pub type Connector = Arc<Mutex<Driver>>;

impl Driver {
    /// Create a new instance of the driver
    ///
    pub fn new(settings: &SerialSettings) -> Self {
        // Get the port name safely
        let port_name = settings
            .port_name
            .as_ref()
            .map(|val| val.clone())
            .unwrap_or("undefined".to_string())
            .clone();

        // Create instance
        Driver {
            logger: ConnectorLogger::new("serial", port_name, ""),
            settings: settings.clone(),
            serial_stream: None,
            time_lock: None,
        }
    }

    /// Convert the driver into a connector
    ///
    pub fn into_connector(self) -> Connector {
        Arc::new(Mutex::new(self))
    }

    /// Initialize the driver
    ///
    pub async fn init(&mut self) -> Result<(), PlatformError> {
        // Internal driver already initialized by an other entity => OK
        if self.serial_stream.is_some() {
            return Ok(());
        }

        // Get the port name
        let port_name = self.settings.port_name.as_ref().ok_or_else(|| {
            PlatformError::BadSettings("Port name is not set in settings".to_string())
        })?;

        // Setup builder
        let serial_builder = tokio_serial::new(port_name, self.settings.baudrate)
            .data_bits(self.settings.data_bits)
            .stop_bits(self.settings.stop_bits)
            .parity(self.settings.parity)
            .flow_control(self.settings.flow_control);

        // Build the stream
        self.serial_stream = Some(SerialStream::open(&serial_builder).map_err(|e| {
            PlatformError::BadSettings(format!("Unable to open serial stream: {}", e))
        })?);

        Ok(())
    }

    /// Write a command on the serial stream
    ///
    pub async fn write_time_locked(&mut self, command: &[u8]) -> Result<usize, PlatformError> {
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
        let write_result = self
            .serial_stream
            .as_mut()
            .ok_or_else(|| PlatformError::BadSettings(format!("No serial stream")))?
            .write(command)
            .await
            .map_err(|e| {
                PlatformError::BadSettings(format!("Unable to write on serial stream: {}", e))
            });

        // Set the time lock
        if let Some(duration) = self.settings.time_lock_duration {
            self.time_lock = Some(TimeLock {
                duration: duration,
                t0: tokio::time::Instant::now(),
            });
        }

        return write_result;
    }

    /// Lock the connector to write a command then wait for the answers
    ///
    pub async fn write_then_read(
        &mut self,
        command: &[u8],
        response: &mut [u8],
    ) -> Result<usize, PlatformError> {
        // Write
        self.write_time_locked(command).await?;

        // Read the response
        self.serial_stream
            .as_mut()
            .ok_or_else(|| PlatformError::BadSettings("No serial stream".to_string()))?
            .read(response)
            .await
            .map_err(|e| {
                PlatformError::BadSettings(format!("Unable to read on serial stream {:?}", e))
            })
    }

    ///
    ///
    pub async fn write_then_read_until(
        &mut self,
        command: &[u8],
        response: &mut [u8],
        end: u8,
    ) -> Result<usize, PlatformError> {
        // Write
        self.write_time_locked(command).await?;

        // Read the response until "end"
        let mut n = 0;
        loop {
            let mut single_buf = [0u8; 1];
            self.serial_stream
                .as_mut()
                .ok_or_else(|| PlatformError::BadSettings("No serial stream".to_string()))?
                .read_exact(&mut single_buf)
                .await
                .map_err(|e| {
                    PlatformError::BadSettings(format!("Unable to read on serial stream {:?}", e))
                })?;
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
        self.logger.warn("Closing serial stream");
        self.serial_stream = None;
    }
}
