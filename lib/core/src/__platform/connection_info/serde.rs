use super::Info;
use super::file::system_file_path;

use serde_json::json;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

use crate::connection_info_content_bad_format_error;
use crate::connection_info_mandatory_field_missing_error;

// ---
// Documentation
// https://panduza.github.io/panduza-doc/docs/50_platform/architecture/connection_info/
// ---

const DEFAULT_PLATFORM_NAME: &str = "default_name";

// ----------------------------------------------------------------------------

/// Serialize the info object into a JSON string
///
pub fn serialize(info: &Info) -> Result<String, crate::Error> {
    Ok(
        json!({
            "broker": {
                "addr": info.broker_addr,
                "port": info.broker_port,
            },
            "platform": {
                "name": info.platform_name
            },
            "credentials": {
                "user": info.credentials_user,
                "pass": info.credentials_pass
            },
            "services": {
                "retry_delay": info.services_retry_delay,
                "enable_plbd": info.services_enable_plbd
            }
        })
        .to_string()
    )
}

// ----------------------------------------------------------------------------

/// Deserialize a JSON string into a Info object
///
pub fn deserialize(json_string: &str) -> Result<Info, super::Error> {
    serde_json::from_str(json_string)
        .map_err(
            |e| connection_info_content_bad_format_error!(e.to_string().as_str())
        )
        .and_then(
            parse_json_obj
        )
}

/// Create a new Info object from a JSON value
///
fn parse_json_obj(json_obj: JsonValue) -> Result<Info, super::Error> {
    json_obj.as_object()
        .ok_or(
            connection_info_content_bad_format_error!("Except a JSON object at file root")
        )
        .and_then(
            parse_map_object
        )
}

/// Create a new Info object from a JSON map object
///
fn parse_map_object(map_obj: &JsonMap<String, JsonValue>) -> Result<Info, super::Error> {

    // ---

    // Get Broker Section
    let broker = map_obj.get("broker")
        .ok_or(
            connection_info_mandatory_field_missing_error!("[broker] section must be provided")
        )?;

    // Get Host Address
    let broker_addr = broker.get("addr")
        .ok_or(connection_info_mandatory_field_missing_error!("[broker.addr] must be provided"))?
        .as_str()
        .ok_or(connection_info_content_bad_format_error!("[broker.addr] must be a string"))?
        .to_string();

    // Get Host Port
    let broker_port = broker.get("port")
        .ok_or(connection_info_mandatory_field_missing_error!("[broker.port] must be provided"))?
        .as_u64()
        .ok_or(connection_info_content_bad_format_error!("[broker.port] must be a number"))?
        as u16;

    // ---

    // Get Platform info section, if not platform info section
    let default_platform_obj = json!(
        {
            "name": DEFAULT_PLATFORM_NAME
        }
    );
    let platform = map_obj.get("platform")
        .or(
            Some(&default_platform_obj)
        )
        .unwrap();

    // Get Platform Name
    let platform_name = platform.get("name")
        .or(Some(&json!(DEFAULT_PLATFORM_NAME)))
        .unwrap()
        .as_str()
        .or(Some(DEFAULT_PLATFORM_NAME))
        .unwrap()
        .to_string();

    // ---

    let credentials = map_obj.get("credentials");

    let credentials_user = credentials
        .and_then(
            |c| c.get("user")
                .and_then(|u| u.as_str())
                .map(|u| u.to_string())
        )
        .or(None);

    let credentials_pass = credentials
        .and_then(
            |c| c.get("pass")
                .and_then(|u| u.as_str())
                .map(|u| u.to_string())
        )
        .or(None);

    // ---

    // Get Services info section
    let default_services_obj = json!(
        {
            "retry_delay": 1,
            "enable_plbd": "false"
        }
    );
    let services = map_obj.get("services")
        .or(
            Some(&default_services_obj)
        )
        .unwrap();

    // Get Services Retry Delay
    let services_retry_delay = services.get("retry_delay")
        .or(Some(&json!(1)))
        .unwrap()
        .as_u64()
        .or(Some(1))
        .unwrap() as u32;

    let services_enable_plbd = services.get("enable_plbd")
        .or(Some(&json!(false)))
        .unwrap()
        .as_bool()
        .or(Some(false))
        .unwrap();

    // Return the Info object
    Ok(
        Info {
            file_path: system_file_path().to_str().unwrap().to_string(),
            broker_addr: broker_addr,
            broker_port: broker_port,
            credentials_user: credentials_user,
            credentials_pass: credentials_pass,
            platform_name: platform_name,
            services_retry_delay: services_retry_delay,
            services_enable_plbd: services_enable_plbd,
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
            "enable_plbd": true,
        }
    });
    let output = deserialize(input.to_string().as_str());
    assert_eq!(output.is_err(), false);
    let ci = output.unwrap();
    assert_eq!(ci.broker_addr, "192.168.1.42");
    assert_eq!(ci.broker_port, 5555);
    assert_eq!(ci.credentials_user, Some("foo".to_string()));
    assert_eq!(ci.credentials_pass, Some("xxxxxxxxxx".to_string()));
    assert_eq!(ci.platform_name, "test1");
    assert_eq!(ci.services_retry_delay, 53);
    assert_eq!(ci.services_enable_plbd, true);
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

