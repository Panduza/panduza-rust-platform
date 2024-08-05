// Main error crate for Panduza Platform
mod error;
pub use error::Error;

/// Loggers
mod logger;
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

//
mod interface;
pub use interface::builder::InterfaceBuilder;

// public traits
mod traits;
pub use traits::DeviceOperations;
pub use traits::Producer;

// pub type TaskResult = Result<(), crate::error::Error>;
// pub type FunctionResult = Result<(), crate::error::Error>;

// // Public macro to create a platform Error outside of panduza core
// //
// #[macro_export]
// macro_rules! platform_error {
//     ($msg:expr) => {
//         panduza_core::Error::new(file!(), line!(), $msg.to_string())
//     };
// }

// // Public macro to create a platform Err Result outside of panduza core
// //
// #[macro_export]
// macro_rules! platform_error_result {
//     ($msg:expr) => {
//         Err(panduza_core::Error::new(file!(), line!(), $msg.to_string()))
//     };
// }
