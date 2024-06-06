use super::Info;

use serde_json::json;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

/// Serialize the info object into a JSON string
/// 
pub fn serialize(&self) -> Result<(), std::io::Error> {
    // Create the JSON object
    let json_obj = json!({
        "broker": {
            "addr": self.host_addr,
            "port": self.host_port,
            "retry": self.host_retry,
        }
    });

    // //  Write new file
    // let mut file = File::create(&self.file_path)?;
    // let json_string = json_obj.to_string();
    // file.write_all(json_string.as_bytes())?;
    // Ok(())
}

///
///  
pub fn deserialize(json_string: &str) -> Result<Self, CiError> {
    serde_json::from_str(json_string)
        .map_err(|e| content_bad_format_error(e.to_string().as_str()))
        .and_then(Info::build_from_json_value)
}

/// Create a new Info object from a JSON value
///
fn build_from_json_value(json_obj: JsonValue) -> Result<Self, CiError> {
    json_obj.as_object()
        .ok_or(content_bad_format_error( "Except a JSON object at file root"))
        .and_then(Info::build_from_map_object)
}

/// Create a new Info object from a JSON map object
///
fn build_from_map_object(map_obj: &JsonMap<String, JsonValue>) -> Result<Self, CiError> {

    // Get Host Section
    let host = map_obj.get("broker")
        .ok_or(mandatory_field_missing_error("[broker] section must be provided"))?;

    // Get Host Address
    let host_addr = host.get("addr")
        .ok_or(mandatory_field_missing_error("[broker.addr] must be provided"))?
        .as_str()
        .ok_or(content_bad_format_error("[broker.addr] must be a string"))?
        .to_string();

    // Get Host Port
    let host_port = host.get("port")
        .ok_or(mandatory_field_missing_error("[broker.port] must be provided"))?
        .as_u64()
        .ok_or(content_bad_format_error("[broker.port] must be a number"))?
        as u16;

    // Get Host Retry
    let default_retry_value: u32 = 1;
    let host_retry = host.get("retry")
        .unwrap_or(&json!(default_retry_value))
        .as_u64()
        .ok_or(content_bad_format_error("[broker.retry] must be a number"))?
        as u32;

    // Get Platform info section, if not platform info section 
    let platform_info = map_obj.get("platform");

    let platform_name: String;
    let default_platform_name: String = "panduza_platform".to_string();

    match platform_info {
        Some(value) => {
            platform_name = value.get("name")
            .unwrap_or(&json!(default_platform_name))
            .as_str()
            .ok_or(content_bad_format_error("[platform.name] must be a string"))?
            .to_string();
        },
        None => {
            platform_name = default_platform_name
        }
    }

    Ok(
        Self {
            file_path: Info::system_file_path().to_str().unwrap().to_string(),
            host_addr: host_addr,
            host_port: host_port,
            host_retry: host_retry,
            platform_name: platform_name
        }
    )
}

