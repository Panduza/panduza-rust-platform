pub mod pmacro;

// Main error crate for Panduza Platform
mod error;
pub use error::Error;

/// Loggers
mod logger;
pub use logger::DeviceLogger;
pub use logger::FactoryLogger;
pub use logger::GenericLogger;
pub use logger::PlatformLogger;

///
mod factory;
pub use factory::production_order::DeviceSettings;
pub use factory::production_order::ProductionOrder;
pub use factory::Factory;

// The heavy machine
mod platform;
pub use platform::Platform;

//
mod device;
pub use device::monitor::DeviceMonitor;
pub use device::Device;
pub use device::DeviceInner;
//
mod interface;
pub use interface::builder::InterfaceBuilder;
pub use interface::Interface;

//
mod attribute;
pub use attribute::att_only_msg_att::AttOnlyMsgAtt;
pub use attribute::bidir_msg_att::BidirMsgAtt;
pub use attribute::builder::AttributeBuilder;
pub use attribute::cmd_only_msg_att::CmdOnlyMsgAtt;

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
mod info;
pub use info::pack::InfoPack;

//
mod codec;
pub use codec::boolean::BooleanCodec;
pub use codec::json::JsonCodec;
pub use codec::memory_command::MemoryCommandCodec;
pub use codec::memory_command::MemoryCommandMode;
pub use codec::number::NumberCodec;
pub use codec::string::StringCodec;
pub use codec::string_list::StringListCodec;

mod task_channel;
pub use task_channel::TaskReceiver;
pub use task_channel::TaskSender;

/// Return type for spawned task
pub type TaskResult = Result<(), Error>;
