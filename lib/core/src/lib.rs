pub mod link;
pub mod meta;
pub mod device;
pub mod interface;
pub mod attribute;
pub mod connection;
pub mod subscription;
pub mod platform;

mod error;
pub type Error = crate::error::Error;

pub type TaskResult = Result<(), crate::error::Error>;

pub type FunctionResult = Result<(), crate::error::Error>;

pub use device::Device;


use crate::interface::Builder as InterfaceBuilder;
use async_trait::async_trait;

use crate::Error as PlatformError;



/// Actions that are specific for each device type
/// 
#[async_trait]
pub trait DeviceStates : Send + Sync {


    async fn mount(&self, device: &Device) -> Result<(), ()>;


    // /// The device must provides a list of interface builders
    // /// 
    // fn interface_builders(&self, device: &Device)
    //     -> Result<Vec<InterfaceBuilder>, PlatformError>;

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
    fn produce(&self) -> Result<Box<dyn DeviceStates>, PlatformError>;
}


#[async_trait]
pub trait Hunter : Send + Sync {

    async fn hunt(&self) -> Option<Vec<serde_json::Value>>;

}







/// Public macro to create a platform Error outside of panduza core
///
#[macro_export]
macro_rules! platform_error {
    ($msg:expr) => {
        panduza_core::Error::new(file!(), line!(), $msg.to_string())
    };
}

/// Public macro to create a platform Err Result outside of panduza core
///
#[macro_export]
macro_rules! platform_error_result {
    ($msg:expr) => {
        Err(panduza_core::Error::new(file!(), line!(), $msg.to_string()))
    };
}
