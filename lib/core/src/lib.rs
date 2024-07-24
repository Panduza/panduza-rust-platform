pub mod link;
pub mod meta;
pub mod device;
pub mod interface;
pub mod attribute;
pub mod connection;
pub mod subscription;
pub mod platform;

mod error;
pub type Error = crate::error::Error;

pub type TaskResult = Result<(), crate::error::Error>;

pub type FunctionResult = Result<(), crate::error::Error>;

/// Public macro to create a platform Error outside of panduza core
///
#[macro_export]
macro_rules! platform_error {
    ($msg:expr) => {
        panduza_core::Error::new(file!(), line!(), $msg.to_string())
    };
    ($msg:expr, $fmt_0:expr) => {
        panduza_core::Error::new(file!(), line!(), format!($msg, $fmt_0))
    };
}

/// Public macro to create a platform Err Result outside of panduza core
///
#[macro_export]
macro_rules! platform_error_result {
    ($msg:expr) => {
        Err(panduza_core::Error::new(file!(), line!(), $msg.to_string()))
    };
}
