mod info;
mod file;
mod error;
mod serde;

// use std::env;
// use std::io::Write;
// use std::path::PathBuf;


// use std::fs::File;


pub type Info = info::Info;
pub type Error = error::Error;
pub type ErrorType = error::ErrorType;

pub fn deserialize(json_string: &str) -> Result<Info, Error> {
    serde::deserialize(json_string)
}


