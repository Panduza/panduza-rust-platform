use std::env;
use std::path::PathBuf;

use serde_json::json;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

mod tests;


#[derive(Debug)]
pub enum CiErrorType {
    // COVER:REQ_CONN_INFO_0030_00
    ContentBadFormat,
    // COVER:REQ_CONN_INFO_0040_00
    MandatoryFieldMissing,
    // COVER:REQ_CONN_INFO_0050_00
    FileDoesNotExist,
}


#[derive(Debug)]
pub struct CiError {

    type_: CiErrorType,

    message: String,
}

impl CiError {

    fn new(type_: CiErrorType, message: &str) -> Self {
        Self {
            type_,
            message: message.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn type_(&self) -> &CiErrorType {
        &self.type_
    }
}

fn content_bad_format_error(message: &str) -> CiError {
    CiError::new(CiErrorType::ContentBadFormat, message)
}
fn mandatory_field_missing_error(message: &str) -> CiError {
    CiError::new(CiErrorType::MandatoryFieldMissing, message)
}
fn file_does_not_exist_error(message: &str) -> CiError {
    CiError::new(CiErrorType::FileDoesNotExist, message)
}



#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionInfo {
    
    // broker info
    host_addr: String,
    host_port: u16,
    host_retry: u32,

    // credential
    
}

impl ConnectionInfo {

    /// Create a new ConnectionInfo object with default values
    ///
    fn default() -> Self {
        Self {
            host_addr: "localhost".to_string(),
            host_port: 1883,
            host_retry: 1,
        }
    }

    /// Give the system path of the connection.json file
    ///
    /// COVER:REQ_CONN_INFO_0010_00
    ///
    fn system_file_path() -> PathBuf {
        // Define the paths
        let filename = "connection.json";
        let unix_path =
            PathBuf::from("/etc/panduza").join(filename);
        let windows_path = 
            PathBuf::from(dirs::home_dir().unwrap()).join("panduza").join(filename);

        // Return the file path depeding on the OS
        match env::consts::OS {
            "windows" => {
                return windows_path;
            }
            "unix" => {
                return unix_path;
            }
            _ => {
                tracing::warn!("Unsupported system bu try with unix path anyway !");
                return unix_path;
            }
        }
    }

    /// Create a new ConnectionInfo object from a JSON file
    /// 
    pub async fn build_from_file() -> Result<Self, CiError> {
        // Get the file path
        let file_path = ConnectionInfo::system_file_path();

        // Check if the file exists
        if !file_path.exists()  {
            return Err(file_does_not_exist_error(file_path.to_str().unwrap()));
        }

        // Try to read the file content
        tokio::fs::read_to_string(&file_path).await
            .map_err(|e| content_bad_format_error(e.to_string().as_str()))
            .and_then(|v| ConnectionInfo::build_from_str(v.as_str()) )
    }

    /// Create a new ConnectionInfo object from a JSON string
    /// 
    fn build_from_str(json_string: &str) -> Result<Self, CiError> {
        serde_json::from_str(json_string)
            .map_err(|e| content_bad_format_error(e.to_string().as_str()))
            .and_then(ConnectionInfo::build_from_json_value)
    }

    /// Create a new ConnectionInfo object from a JSON value
    ///
    fn build_from_json_value(json_obj: JsonValue) -> Result<Self, CiError> {
        json_obj.as_object()
            .ok_or(content_bad_format_error( "Except a JSON object at file root"))
            .and_then(ConnectionInfo::build_from_map_object)
    }

    /// Create a new ConnectionInfo object from a JSON map object
    ///
    fn build_from_map_object(map_obj: &JsonMap<String, JsonValue>) -> Result<Self, CiError> {

        // Get Host Section
        let host = map_obj.get("host")
            .ok_or(mandatory_field_missing_error("[host] section must be provided"))?;

        // Get Host Address
        let host_addr = host.get("addr")
            .ok_or(mandatory_field_missing_error("[host.addr] must be provided"))?
            .as_str()
            .ok_or(content_bad_format_error("[host.addr] must be a string"))?
            .to_string();

        // Get Host Port
        let host_port = host.get("port")
            .ok_or(mandatory_field_missing_error("[host.port] must be provided"))?
            .as_u64()
            .ok_or(content_bad_format_error("[host.port] must be a number"))?
            as u16;

        // Get Host Retry
        let default_retry_value: u32 = 1;
        let host_retry = host.get("retry")
            .unwrap_or(&json!(default_retry_value))
            .as_u64()
            .ok_or(content_bad_format_error("[host.retry] must be a number"))?
            as u32;

        Ok(
            Self {
                host_addr: host_addr,
                host_port: host_port,
                host_retry: host_retry,
            }
        )
    }

    /// Getter Hostname
    ///
    pub fn host_addr(&self) -> &String {
        &self.host_addr
    }

    /// Getter Port
    ///
    pub fn host_port(&self) -> u16 {
        self.host_port
    }

}

