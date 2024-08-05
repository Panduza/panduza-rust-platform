use std::env;
use std::io::Write;
use std::path::PathBuf;

use super::Info;
use super::Error;
use super::serde;
use crate::FunctionResult;

use crate::__platform_error;
use crate::connection_info_content_bad_format_error;
use crate::connection_info_file_does_not_exist_error_result;

// ---
// Documentation
// https://panduza.github.io/panduza-doc/docs/50_platform/architecture/connection_info/
// ---

/// Return the system path of the connection.json file
///
/// COVER:PLATF_00001_00 - File System Paths
///
pub fn system_file_path() -> PathBuf {
    // Define the paths
    let filename = "connection.json";
    let unix_path =
        PathBuf::from("/etc/panduza").join(filename);
    let windows_path = 
        PathBuf::from(dirs::public_dir().unwrap()).join("panduza").join(filename);

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
/// COVER:PLATF_00002_00 - File is JSON
///
pub async fn import_file(file_path: PathBuf) -> Result<Info, Error> {

    // Check if the file exists
    if !file_path.exists() {
        return connection_info_file_does_not_exist_error_result!(file_path.to_str().unwrap());
    }

    // Try to read the file content
    tokio::fs::read_to_string(&file_path).await
        .map_err(|e| connection_info_content_bad_format_error!(e.to_string().as_str()) )
        .and_then(|v| serde::deserialize(v.as_str()) )
}

/// Create a new Info file from a info data
///
/// COVER:PLATF_00002_00 - File is JSON
///
pub fn export_file(info: &Info) -> FunctionResult {

    // Create the file directory
    let mut filedir_path = std::path::PathBuf::from(info.file_path.clone());
    filedir_path.pop();
    std::fs::create_dir_all(filedir_path)
        .map_err(|e| __platform_error!(e.to_string()) )?;

    //  Write new file
    let mut file = std::fs::File::create(&info.file_path)
        .map_err(|e| __platform_error!(e.to_string()) )?;

    // Create the JSON string
    let json_string = serde::serialize(info)?;

    // Write file
    file.write_all(json_string.as_bytes())
        .map_err(|e| __platform_error!(e.to_string()) )?;

    Ok(())
}
