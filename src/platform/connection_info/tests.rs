use super::ConnectionInfo;
use serde_json::json;
use serde_json::Value as JsonValue;

// ----------------------------------------------------------------------------
#[test]
fn build_from_json_value_ok() {
    let input = json!({
        "broker_host": "192.168.1.1",
        "broker_port": 5555,
    });
    let output = ConnectionInfo::build_from_json_value(input).unwrap();
    assert_eq!(output.hostname(), "192.168.1.1");
    assert_eq!(output.port(), 5555);
}

// ----------------------------------------------------------------------------
#[test]
fn build_from_json_value_fail_0() {
    let input = JsonValue::Null;
    let output = ConnectionInfo::build_from_json_value(input).unwrap();
    assert_eq!(output, ConnectionInfo::default());
}

// ----------------------------------------------------------------------------
#[test]
fn build_from_json_value_fail_1() {
    let input = json!({
        "hostname": "localhost",
        "port": 1883
    });
    let output = ConnectionInfo::build_from_json_value(input);
    // assert_eq!(output.is_err(), true);
}


