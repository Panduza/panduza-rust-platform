// ---
// Documentation
// https://panduza.github.io/panduza-doc/docs/50_platform/architecture/connection_info/
// ---

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
    pub plt_error: crate::Error,
}

impl Error {
    pub fn new(err_type: ErrorType, plt_error: crate::Error) -> Self {
        Self {
            err_type: err_type,
            plt_error: plt_error
        }
    }
}

#[macro_export]
macro_rules! connection_info_content_bad_format_error {
    ($msg:expr) => {
        super::Error::new (
            crate::platform::connection_info::error::ErrorType::ContentBadFormat,
            crate::Error::new(file!(), line!(), $msg.to_string())
        )
    };
}

#[macro_export]
macro_rules! connection_info_mandatory_field_missing_error {
    ($msg:expr) => {
        super::Error::new (
            crate::platform::connection_info::error::ErrorType::MandatoryFieldMissing,
            crate::Error::new(file!(), line!(), $msg.to_string())
        )
    };
}

#[macro_export]
macro_rules! connection_info_file_does_not_exist_error_result {
    ($msg:expr) => {
        Err(
            super::Error::new (
                crate::platform::connection_info::error::ErrorType::FileDoesNotExist,
                crate::Error::new(file!(), line!(), $msg.to_string())
            )
        )
    };
}


