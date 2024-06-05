#[derive(Debug)]
pub enum ErrorType {
    // COVER:REQ_CONN_INFO_0030_00
    ContentBadFormat,
    // COVER:REQ_CONN_INFO_0040_00
    MandatoryFieldMissing,
    // COVER:REQ_CONN_INFO_0050_00
    FileDoesNotExist,
}

#[derive(Debug)]
pub struct Error {
    /// Type of the error
    err_type: ErrorType,
    /// Platform error message
    plt_error: PlatformError,
}

impl Error {

    fn new(err_type: ErrorType, plt_error: PlatformError) -> Self {
        Self {
            err_type: err_type,
            plt_error: plt_error
        }
    }

    // pub fn message(&self) -> &str {
    //     &self.message
    // }

    pub fn type_(&self) -> &ErrorType {
        &self.type_
    }
}

fn content_bad_format_error(message: &str) -> CiError {
    CiError::new(ErrorType::ContentBadFormat, message)
}
fn mandatory_field_missing_error(message: &str) -> CiError {
    CiError::new(ErrorType::MandatoryFieldMissing, message)
}
fn file_does_not_exist_error(message: &str) -> CiError {
    CiError::new(ErrorType::FileDoesNotExist, message)
}
