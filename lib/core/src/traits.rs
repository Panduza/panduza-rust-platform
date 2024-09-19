use crate::{Device, Error};
use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Debug;

/// Actions that are specific for each device type
///
#[async_trait]
pub trait DeviceOperations: Send + Sync {
    ///
    /// Mount device and give him its structure
    ///
    async fn mount(&mut self, mut device: Device) -> Result<(), Error>;

    ///
    /// This device crashed, got an error or is not available anymore
    /// This function must monitor reboot condition and await them
    /// Once this function return, the device driver will reboot
    ///
    async fn wait_reboot_event(&mut self, mut device: Device);
}

/// Trait to define a device producer
///
pub trait Producer: Send {
    /// Device Manufacturer
    ///
    fn manufacturer(&self) -> String;

    /// Device Model
    ///
    fn model(&self) -> String;

    /// Device settings properties
    ///
    // fn settings_props(&self) -> serde_json::Value; todo create a structure for properties

    /// Produce a new instance of the device actions
    ///
    fn produce(&self) -> Result<Box<dyn DeviceOperations>, Error>;
}

/// Trait to manage an message attribute (MQTT)
/// Sync version
#[async_trait]
pub trait MessageHandler: Send + Sync {
    ///
    /// Triggered on each incoming message
    ///
    async fn on_message(&mut self, data: &Bytes) -> Result<(), Error>;
}

/// Encoder Decoder for message payload
///
pub trait MessageCodec: PartialEq + Debug + Sync + Send + Clone + 'static {
    ///
    /// Decode data
    ///
    fn from_message_payload(data: &Bytes) -> Result<Self, Error>;
    ///
    /// Encode data
    ///
    fn into_message_payload(&self) -> Result<Vec<u8>, Error>;

    fn typee() -> String;
}
