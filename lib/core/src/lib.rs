// Main error crate for Panduza Platform
mod error;
pub use error::Error;

/// Loggers
mod logger;
pub use logger::DeviceLogger;
pub use logger::FactoryLogger;
pub use logger::PlatformLogger;

///
mod factory;
pub use factory::Factory;

// The heavy machine
mod platform;
pub use platform::Platform;

//
mod device;
pub use device::Device;
pub use device::DeviceInner;
//
mod interface;
pub use interface::builder::InterfaceBuilder;
pub use interface::Interface;

//
mod attribute;
pub use attribute::builder::AttributeBuilder;
pub use attribute::ro_msg_att::RoMessageAttribute;
pub use attribute::rw_msg_att::RwMessageAttribute;

// public traits
mod traits;
pub use traits::DeviceOperations;
pub use traits::MessageCodec;
pub use traits::MessageHandler;
pub use traits::Producer;

//
mod reactor;
pub use reactor::message_dispatcher::MessageDispatcher;
pub use reactor::settings::ReactorSettings;
pub use reactor::Reactor;

// This module manage the message attributes (MQTT/TCP)
// pub mod msg;
pub type MessageClient = rumqttc::AsyncClient;

//
mod codec;
pub use codec::boolean::BooleanCodec;

/// Return type for spawned task
pub type TaskResult = Result<(), Error>;
