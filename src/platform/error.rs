use std::fmt;

#[derive(Debug)]
pub struct PlatformError {
    pub file: &'static str,
    pub line: u32,
    pub message: String,
    pub parent: Option<Box<PlatformError>>
}

impl PlatformError {
    pub fn new(file: &'static str, line: u32, message: String, parent: Option<Box<PlatformError>>) -> Self {
        Self { file, line, message, parent }
    }

    fn print_stack(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.parent.is_some() {
            self.parent.as_ref().unwrap().print_stack(f).unwrap();
        }
        writeln!(f, "{}:{} - {}", self.file, self.line, self.message)
    }
}

impl fmt::Display for PlatformError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        self.print_stack(f)
    }

}

impl std::error::Error for PlatformError {}


