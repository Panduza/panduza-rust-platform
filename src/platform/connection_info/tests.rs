use super::ConnectionInfo;
use serde_json::json;
use serde_json::Value as JsonValue;

// ----------------------------------------------------------------------------
#[test]
fn from_json_value_ok() {
    let input = json!({
        "broker_host": "192.168.1.1",
        "broker_port": 5555,
    });
    let output = ConnectionInfo::from_json_value(input);
    assert_eq!(output.is_err(), false, "Error: {:?}", output);
    let ci = output.unwrap();
    assert_eq!(ci.hostname(), "192.168.1.1");
    assert_eq!(ci.port(), 5555);
}

// ----------------------------------------------------------------------------
#[test]
fn from_json_value_fail_0() {
    let input = JsonValue::Null;
    let output = ConnectionInfo::from_json_value(input);
    assert_eq!(output.is_err(), true);
}

// ----------------------------------------------------------------------------
#[test]
fn from_json_value_fail_1() {
    let input = json!({
        "hostname": "localhost",
        "port": 1883
    });
    let output = ConnectionInfo::from_json_value(input);
    assert_eq!(output.is_err(), true);
}


