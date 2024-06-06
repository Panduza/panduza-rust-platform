use crate::platform::Error as PlatformError;

#[derive(Debug)]
pub enum ErrorType {
    // COVER:REQ_CONN_INFO_0030_00
    ContentBadFormat,
    // COVER:REQ_CONN_INFO_0040_00
    MandatoryFieldMissing,
    // COVER:REQ_CONN_INFO_0050_00
    FileDoesNotExist,
}

/// Error for connection info crate
#[derive(Debug)]
pub struct Error {
    /// Type of the error
    pub err_type: ErrorType,
    /// Platform error message
    pub plt_error: PlatformError,
}

impl Error {
    pub fn new(err_type: ErrorType, plt_error: PlatformError) -> Self {
        Self {
            err_type: err_type,
            plt_error: plt_error
        }
    }
}

#[macro_export]
macro_rules! connection_info_error_content_bad_format {
    ($msg:expr) => {
        Error::new (
            crate::platform::connection_info::error::ErrorType::ContentBadFormat,
            crate::platform::Error::new(file!(), line!(), $msg.to_string())
        )
    };
}

#[macro_export]
macro_rules! connection_info_error_mandatory_field_missing {
    ($msg:expr) => {
        Error::new (
            crate::platform::connection_info::error::ErrorType::MandatoryFieldMissing,
            crate::platform::Error::new(file!(), line!(), $msg.to_string())
        )
    };
}

// fn mandatory_field_missing_error(message: &str) -> CiError {
//     CiError::new(ErrorType::MandatoryFieldMissing, message)
// }
// fn file_does_not_exist_error(message: &str) -> CiError {
//     CiError::new(ErrorType::FileDoesNotExist, message)
// }
