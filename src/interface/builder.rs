use super::fsm;
use super::subscriber::Subscriber;

/// The builder allow the device to create a new interface
/// 
pub struct Builder {
    /// The name of the interface
    pub name: String,
    /// Type of the interface
    pub itype: String,
    /// The name of the interface
    pub version: String,
    /// The states of the interface state machine
    pub states: Box<dyn fsm::States>,
    /// The subscriber to manage network events
    pub subscriber: Box<dyn Subscriber>
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
            subscriber: subscriber
        };
    }

}
