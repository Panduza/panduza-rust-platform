use serde_json::Value;
use async_trait::async_trait;

/// An identity provider is responsible for providing information about the device
/// 
pub struct Identity {

    /// Type of the interface
    itype: String,

    /// The name of the interface
    version: String

}

