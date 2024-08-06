use crate::{Device, Error};
use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Debug;

/// Actions that are specific for each device type
///
#[async_trait]
pub trait DeviceOperations: Send + Sync {
    /// Mount device and give him its structure
    async fn mount(&mut self, mut device: Device) -> Result<(), Error>;

    // /// The device must provides a list of interface builders
    // ///
    // fn interface_builders(&self, device: &Device)
    //     -> Result<Vec<InterfaceBuilder>, PlatformError>;
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
    async fn on_message(&mut self, data: &Bytes);
}

/// Encoder Decoder for message payload
///
pub trait MessageCodec:
    Into<Vec<u8>> + From<Vec<u8>> + PartialEq + Debug + Sync + Send + Clone + 'static
{
}
