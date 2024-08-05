mod info;
mod file;
mod error;
mod serde;

use std::path::PathBuf;
use crate::FunctionResult;

// ---
// Documentation
// https://panduza.github.io/panduza-doc/docs/50_platform/architecture/connection_info/
// ---

pub type Info = info::Info;
pub type Error = error::Error;
pub type ErrorType = error::ErrorType;

#[inline(always)]
pub fn system_file_path() -> PathBuf {
    file::system_file_path()
}

#[inline(always)]
pub async fn import_file(file_path: PathBuf) -> Result<Info, Error> {
    file::import_file(file_path).await
}

#[inline(always)]
pub fn export_file(info: &Info) -> FunctionResult {
    file::export_file(info)
}
