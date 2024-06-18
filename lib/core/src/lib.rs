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


/// Macro to create a platform error
///
#[macro_export]
macro_rules! platform_error_result {
    ($msg:expr, $parent:expr) => {
        Err(crate::error::Error::new(file!(), line!(), $msg.to_string()))
    };
    ($msg:expr) => {
        Err(crate::error::Error::new(file!(), line!(), $msg.to_string()))
    };
}
#[macro_export]
macro_rules! platform_error {
    ($msg:expr) => {
        crate::error::Error::new(file!(), line!(), $msg.to_string())
    };
}
