use super::{fsm, AmInterface, Interface};
use super::subscriber::Subscriber;
use crate::link::AmManager as AmLinkManager;
use crate::subscription;
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

    /// Get the name of the interface
    /// 
    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn states(&self) -> &Box<dyn fsm::States> {
        return &self.states;
    }
    pub fn subscriber(&self) -> &Box<dyn Subscriber> {
        return &self.subscriber;
    }


    
}
