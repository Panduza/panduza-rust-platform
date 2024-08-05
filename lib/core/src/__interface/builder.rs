use crate::link::ThreadSafeLinkManager;
use crate::platform::services::ThreadSafeServices;

use crate::FunctionResult;

use super::fsm;
use super::subscriber::Subscriber;

/// The builder allow the device to create a new interface
/// 
pub struct Builder {

    // -- DATA FROM INTERFACE DEFINITION --

    /// The name of the interface
    pub name: String,
    /// Type of the interface
    pub itype: String,
    /// The name of the interface
    pub version: String,
    /// The states of the interface state machine
    pub states: Box<dyn fsm::States>,
    /// The subscriber to manage network events
    pub subscriber: Box<dyn Subscriber>,

    // -- DATA FROM DEVICE INSTANCE --

    // /// The name of the device that build this interface
    // device_name: Option<String>,

    // /// The name of the device that build this interface
    // bench_name: Option<String>,
    // connection_link_manager: Option<ThreadSafeLinkManager>,
    // platform_services: Option<ThreadSafeServices>
}

impl Builder {
    
    /// Create a new instance of the Builder
    /// 
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>(
        name: A,
        itype: B,
        version: C,
        states: Box<dyn fsm::States>,
        subscriber: Box<dyn Subscriber>
    ) -> Self {
        return Builder {
            name: name.into(),
            itype: itype.into(),
            version: version.into(),
            states: states,
            subscriber: subscriber,
            // device_name: None,
            // bench_name: None,
            // connection_link_manager: None,
            // platform_services: None
        };
    }


    // /// Set the device name
    // pub fn with_device_name<A: Into<String>>(mut self, name: A) -> Self {
    //     self.device_name = Some(name.into());
    //     self
    // }





}
