use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::attribute::Attribute;
// use crate::attribute::JsonAttribute;
use crate::interface::AmInterface;

use crate::{attribute, subscription};
use crate::interface::builder::Builder as InterfaceBuilder;

// use crate::attribute::

use crate::Error as PlatformError;



pub mod states;
pub mod settings;
pub mod interface;
pub mod subscriber;


pub type RegisterSize = settings::RegisterSize;
pub type RegistersSettings = settings::MetaSettings;

pub type RegistersStates = states::MetaStates;
pub type RegistersSubscriber = subscriber::MetaSubscriber;



#[async_trait]
pub trait RegistersActions: Send + Sync {

    /// Initialize the interface
    /// The connector initialization must be done here
    ///
    async fn initializating(&mut self, interface: &AmInterface) -> Result<(), PlatformError>;

    async fn read(&mut self, interface: &AmInterface, index:usize, size:usize) -> Result<Vec<u64>, String>;

    async fn write(&mut self, interface: &AmInterface, index:usize, v: &Vec<u64>);

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
    settings: RegistersSettings,
    actions: Box<dyn RegistersActions>
) -> InterfaceBuilder {

    // params,
    let meta_interface = interface::MetaInterface::new_thread_safe(settings, actions);

    return InterfaceBuilder::new(
        name,
        "registers",
        "0",
        Box::new(RegistersStates{meta_interface: meta_interface.clone()}),
        Box::new(RegistersSubscriber{meta_interface: meta_interface.clone()})
    );
}

