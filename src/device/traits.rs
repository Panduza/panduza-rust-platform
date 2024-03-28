use crate::interface::AmInterface;
use crate::platform::PlatformError;

/// Actions that are specific for each device type
/// 
pub trait DeviceActions : Send {

    // fn hunt(&self) -> LinkedList<serde_json::Value>;

    /// Create a new instance of the Device
    /// 
    fn create_interfaces(&self, device_settings: &serde_json::Value) -> Vec<AmInterface>;
}

/// A producer is responsible for providing actions of a device type
/// 
/// This trait is used by the factory to create new instances of the device by 
/// combining the actions with device data (naem, settings...)
/// 
pub trait Producer : Send {
    /// Produce a new instance of the device actions
    /// 
    fn produce(&self) -> Result<Box<dyn DeviceActions>, PlatformError>;
}
