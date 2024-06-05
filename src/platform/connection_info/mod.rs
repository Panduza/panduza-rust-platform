use std::env;
use std::io::Write;
use std::path::PathBuf;

use serde_json::json;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;
use std::fs::File;


use crate::platform::Error as PlatformError;




// Serde (string -> data , data -> string)
// File (import / export file, manage the system path)
// ConnectionInfo => les data

// ----------------------------------------------------------------------------
#[test]
fn build_from_json_value_ok() {
    let input = json!({
        "broker": {
            "addr": "192.168.1.1",
            "port": 5555,
        }
    });
    let output = ConnectionInfo::build_from_json_value(input);
    assert_eq!(output.is_err(), false);
    let ci = output.unwrap();
    assert_eq!(ci.host_addr(), "192.168.1.1");
    assert_eq!(ci.host_port(), 5555);
}

// ----------------------------------------------------------------------------
#[test]
fn build_from_json_value_fail_0() {
    let input = JsonValue::Null;
    let output = ConnectionInfo::build_from_json_value(input);
    assert_eq!(output.is_err(), true);
}

// ----------------------------------------------------------------------------
#[test]
fn build_from_json_value_fail_1() {
    let input = json!({
        "hostname": "localhost",
        "port": 1883
    });
    let output = ConnectionInfo::build_from_json_value(input);
    assert_eq!(output.is_err(), true);
}

