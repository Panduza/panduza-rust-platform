use std;

/// Common Error type for the platform
/// Just a simple error type that holds a message and it's location in sources
///
#[derive(Debug)]
pub struct Error {
    pub message: String
}

impl Error {
    pub fn new(file: &'static str, line: u32, message: String) -> Self {
        let formated_message = format!("{}:{} - {}", file, line, message);
        Self { message: formated_message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {

}

/// Macro to create a platform Error
///
#[macro_export]
macro_rules! __platform_error {
    ($msg:expr) => {
        crate::error::Error::new(file!(), line!(), $msg.to_string())
    };
}

/// Macro to create a platform Err Result
///
#[macro_export]
macro_rules! __platform_error_result {
    ($msg:expr) => {
        Err(crate::error::Error::new(file!(), line!(), $msg.to_string()))
    };
}
