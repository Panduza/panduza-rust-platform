use crate::interface::Builder as InterfaceBuilder;
use async_trait::async_trait;

use crate::Error as PlatformError;

use super::Device;

/// Actions that are specific for each device type
/// 
pub trait DeviceActions : Send + Sync {

    /// The device must provides a list of interface builders
    /// 
    fn interface_builders(&self, device: &Device)
        -> Result<Vec<InterfaceBuilder>, PlatformError>;

}

/// A producer is responsible for providing actions of a device type
/// 
/// This trait is used by the factory to create new instances of the device by 
/// combining the actions with device data (naem, settings...)
/// 
pub trait Producer : Send {

    // fn manufacturer(&self) -> String;
    // fn model(&self) -> String;
    fn settings_props(&self) -> serde_json::Value;

    /// Produce a new instance of the device actions
    /// 
    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError>;
}


#[async_trait]
pub trait Hunter : Send + Sync {

    async fn hunt(&self) -> Option<Vec<serde_json::Value>>;

}





