use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct PlatformError {
    pub message: String,
    pub backtrace: Vec<String>,
}

impl PlatformError {
    pub fn new(file: &'static str, line: u32, message: String, source: Option<Box<PlatformError>>) -> Self {
        let formated_message = format!("{}:{} - {}", file, line, message);
        if let Some(source) = source {
            let mut backtrace = source.backtrace.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            backtrace.push(source.message);
            Self { message: formated_message, backtrace:backtrace }
        }
        else {
            Self { message: formated_message, backtrace: vec![] }
        }
    }
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for PlatformError {

}
