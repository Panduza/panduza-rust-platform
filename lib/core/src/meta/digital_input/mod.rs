
use async_trait::async_trait;

// use crate::attribute::JsonAttribute;
use crate::interface::AmInterface;

use crate::interface::builder::Builder as InterfaceBuilder;

// use crate::attribute::

use crate::Error as PlatformError;



pub mod states;
pub mod interface;
pub mod subscriber;


pub type RegistersStates = states::MetaStates;
pub type RegistersSubscriber = subscriber::MetaSubscriber;



#[async_trait]
pub trait MetaActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;

    async fn read(&mut self, interface: &AmInterface) -> Result<u8, String>;


}


// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Build the meta interface
///
pub fn build<A: Into<String>>(
    name: A,
    actions: Box<dyn MetaActions>
) -> InterfaceBuilder {

    // params,
    let meta_interface = interface::MetaInterface::new_thread_safe(actions);

    return InterfaceBuilder::new(
        name,
        "digital_input",
        "0",
        Box::new(RegistersStates{meta_interface: meta_interface.clone()}),
        Box::new(RegistersSubscriber{meta_interface: meta_interface.clone()})
    );
}

