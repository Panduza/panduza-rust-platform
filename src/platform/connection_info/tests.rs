use super::ConnectionInfo;
use serde_json::json;
use serde_json::Value as JsonValue;

// ----------------------------------------------------------------------------
#[test]
fn from_json_value_0() {
    let input = JsonValue::Null;
    let ci = ConnectionInfo::from_json_value(input);
    assert_eq!(ci.is_err(), true);
}

// ----------------------------------------------------------------------------
#[test]
fn from_json_value_1() {
    let input = json!({
        "hostname": "localhost",
        "port": 1883
    });
    let ci = ConnectionInfo::from_json_value(input);
    assert_eq!(ci.is_err(), true);
}


