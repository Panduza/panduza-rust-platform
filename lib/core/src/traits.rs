use crate::{Device, Error};
use async_trait::async_trait;

/// Actions that are specific for each device type
///
#[async_trait]
pub trait DeviceOperations: Send {
    /// Mount device and give him its structure
    async fn mount(&self, device: &mut Device) -> Result<(), Error>;

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
