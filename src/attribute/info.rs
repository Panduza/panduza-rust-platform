

use serde_json::json;

use super::AttributeInterface;
use super::JsonAttribute;


/// Info attribute structure
///
/// Provides interface basic information
///
/// > COVER:PLATF_REQ_CORE_0300_00
///
pub struct InfoAttribute {
    attr: JsonAttribute,
}

impl InfoAttribute {
    pub fn new<A: Into<String>, B: Into<String>>(
        itype: A, version: B
    ) -> InfoAttribute {
        let mut attr = JsonAttribute::new("info", false);
        attr.update_field("type", serde_json::Value::String(itype.into()));
        attr.update_field("version", serde_json::Value::String(version.into()));
        attr.update_field("state", serde_json::Value::String("init".to_string()));
        attr.update_field("error", serde_json::Value::String("".to_string()));
        return InfoAttribute { attr };
    }

    pub fn change_state<A: Into<String>>(&mut self, state: A) {
        self.attr.update_field("state", serde_json::Value::String(state.into()));
    }

}

impl AttributeInterface for InfoAttribute {
    fn name(&self) -> &String {
        return self.attr.name();
    }

    fn retain(&self) -> &bool {
        return self.attr.retain();
    }

    fn to_mqtt_payload(&self) -> String {
        return self.attr.to_mqtt_payload();
    }

    fn from_mqtt_payload(&self, payload: &str) {
        self.attr.from_mqtt_payload(payload);
    }
}


