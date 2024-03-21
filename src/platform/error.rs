use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub file: &'static str,
    pub line: u32,
    pub message: String,
    pub parent: Option<Box<Error>>
}

impl Error {
    pub fn new(file: &'static str, line: u32, message: String, parent: Option<Box<Error>>) -> Self {
        Self { file, line, message, parent }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{} - {}", self.file, self.line, self.message)
    }
}

impl std::error::Error for Error {}


