use super::Info;
use super::error::Error;
use super::file::system_file_path;

use serde_json::json;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use crate::connection_info_error_content_bad_format;
use crate::connection_info_error_mandatory_field_missing;

// ----------------------------------------------------------------------------

// /// Serialize the info object into a JSON string
// /// 
// pub fn serialize(&self) -> Result<(), std::io::Error> {
//     // Create the JSON object
//     let json_obj = json!({
//         "broker": {
//             "addr": self.host_addr,
//             "port": self.host_port,
//             "retry": self.host_retry,
//         }
//     });

//     // //  Write new file
//     // let mut file = File::create(&self.file_path)?;
//     // let json_string = json_obj.to_string();
//     // file.write_all(json_string.as_bytes())?;
//     // Ok(())
// }

// ----------------------------------------------------------------------------

/// Deserialize a JSON string into a Info object
///
pub fn deserialize(json_string: &str) -> Result<Info, Error> {
    serde_json::from_str(json_string)
        .map_err(
            |e| connection_info_error_content_bad_format!(e.to_string().as_str())
        )
        .and_then(
            parse_json_obj
        )
}

/// Create a new Info object from a JSON value
///
fn parse_json_obj(json_obj: JsonValue) -> Result<Info, Error> {
    json_obj.as_object()
        .ok_or(
            connection_info_error_content_bad_format!("Except a JSON object at file root")
        )
        .and_then(
            parse_map_object
        )
}

/// Create a new Info object from a JSON map object
///
fn parse_map_object(map_obj: &JsonMap<String, JsonValue>) -> Result<Info, Error> {

    // Get Host Section
    let host = map_obj.get("broker")
        .ok_or(connection_info_error_mandatory_field_missing!("[broker] section must be provided"))?;

    // Get Host Address
    let host_addr = host.get("addr")
        .ok_or(connection_info_error_mandatory_field_missing!("[broker.addr] must be provided"))?
        .as_str()
        .ok_or(connection_info_error_content_bad_format!("[broker.addr] must be a string"))?
        .to_string();

    // Get Host Port
    let host_port = host.get("port")
        .ok_or(connection_info_error_mandatory_field_missing!("[broker.port] must be provided"))?
        .as_u64()
        .ok_or(connection_info_error_content_bad_format!("[broker.port] must be a number"))?
        as u16;

    // Get Host Retry
    let default_retry_value: u32 = 1;
    let host_retry = host.get("retry")
        .unwrap_or(&json!(default_retry_value))
        .as_u64()
        .ok_or(connection_info_error_content_bad_format!("[broker.retry] must be a number"))?
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
            .ok_or(connection_info_error_content_bad_format!("[platform.name] must be a string"))?
            .to_string();
        },
        None => {
            platform_name = default_platform_name
        }
    }

    // Return the Info object
    Ok(
        Info {
            file_path: system_file_path().to_str().unwrap().to_string(),
            host_addr: host_addr,
            host_port: host_port,
            credentials_user: None,
            credentials_pass: None,
            platform_name: platform_name,
            services_retry_delay: host_retry,
            services_enable_plbd: false,
        }
    )
}

// ----------------------------------------------------------------------------
#[test]
fn deserialize_ok_000() {
    let input = json!({
        "broker": {
            "addr": "192.168.1.42",
            "port": 5555
        },
        "platform": {
            "name": "test1"
        },
        "credentials": {
            "user": "foo",
            "pass": "xxxxxxxxxx"
        },
        "services": {
            "retry_delay": 53,
            "enable_plbd": false,
        }
    });
    let output = deserialize(input.to_string().as_str());
    assert_eq!(output.is_err(), false);
    let ci = output.unwrap();
    assert_eq!(ci.host_addr, "192.168.1.42");
    assert_eq!(ci.host_port, 5555);
    assert_eq!(ci.credentials_user, None);
    assert_eq!(ci.credentials_pass, None);
    assert_eq!(ci.platform_name, "test1");
    assert_eq!(ci.services_retry_delay, 53);
    assert_eq!(ci.services_enable_plbd, false);
}

// // ----------------------------------------------------------------------------
// #[test]
// fn build_from_json_value_fail_0() {
//     let input = JsonValue::Null;
//     let output = ConnectionInfo::build_from_json_value(input);
//     assert_eq!(output.is_err(), true);
// }

// // ----------------------------------------------------------------------------
// #[test]
// fn build_from_json_value_fail_1() {
//     let input = json!({
//         "hostname": "localhost",
//         "port": 1883
//     });
//     let output = ConnectionInfo::build_from_json_value(input);
//     assert_eq!(output.is_err(), true);
// }

