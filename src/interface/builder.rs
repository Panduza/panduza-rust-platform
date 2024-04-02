use super::{fsm, subscriber::Subscriber, traits::IdentityProvider};

/// The builder allow the device to create a new interface
/// 
pub struct Builder {
    /// The name of the interface
    name: String,
    /// The identity provider
    idn: Box<dyn IdentityProvider>,
    /// The states of the interface state machine
    states: Box<dyn fsm::States>,
    /// The subscriber to manage network events
    subscriber: Box<dyn Subscriber>
}

impl Builder {
    
    /// Create a new instance of the Builder
    /// 
    fn new<A: Into<String>>(
        name: A,
        idn: Box<dyn IdentityProvider>,
        states: Box<dyn fsm::States>,
        subscriber: Box<dyn Subscriber>) -> Self {
        return Builder {
            name: name.into(),
            idn: idn,
            states: states,
            subscriber: subscriber
        };
    }

}


