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
