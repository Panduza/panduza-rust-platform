mod info;
mod file;
mod error;
mod serde;

use std::env;
use std::io::Write;
use std::path::PathBuf;


use std::fs::File;


pub type Info = info::Info;
pub type Error = error::Error;


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

