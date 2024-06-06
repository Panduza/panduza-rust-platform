use std::env;
use std::path::PathBuf;

use super::serde;
use super::Info;
use super::Error;
use super::error::ErrorType;


use crate::platform::Error as PlfError;

use crate::connection_info_error_content_bad_format;



/// Create a new Error object for a file does not exist error
/// 
macro_rules! error_file_does_not_exist {
    ($msg:expr) => {
        Err(Error::new(
            ErrorType::FileDoesNotExist,
            PlfError::new(file!(), line!(), $msg.to_string())
            )
        )
    };
}



/// Return the system path of the connection.json file
///
/// COVER:PLATF_00001_00
///
pub fn system_file_path() -> PathBuf {
    // Define the paths
    let filename = "connection.json";
    let unix_path =
        PathBuf::from("/etc/panduza").join(filename);
    let windows_path = 
        PathBuf::from(dirs::home_dir().unwrap()).join("panduza").join(filename);

    // Return the file path depeding on the OS
    match env::consts::OS {
        "windows" => { return windows_path; }
        "unix"    => { return unix_path; }
        "linux"   => { return unix_path; }
        _ => {
            tracing::warn!("Unsupported system ({:?}) but try with unix path anyway !", env::consts::OS);
            return unix_path;
        }
    }
}

/// Create a new Info object from a JSON file
/// 
/// COVER:PLATF_00002_00
/// 
pub async fn import_file(file_path: PathBuf) -> Result<Info, Error> {

    // Check if the file exists
    if !file_path.exists() {
        return error_file_does_not_exist!(file_path.to_str().unwrap());
    }

    // Try to read the file content
    tokio::fs::read_to_string(&file_path).await
        .map_err(|e| connection_info_error_content_bad_format!(e.to_string().as_str()) )
        .and_then(|v| serde::deserialize(v.as_str()) )
}
