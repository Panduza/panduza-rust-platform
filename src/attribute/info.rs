
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
        attr.update_field_with_string("type", &itype.into());
        attr.update_field_with_string("version", version.into());
        attr.update_field_with_string("state", "init".to_string());
        attr.update_field_with_string("error", "".to_string());
        return InfoAttribute { attr };
    }

    pub fn change_state<A: Into<String>>(&mut self, state: A) {
        self.attr.update_field_with_string("state", state.into());
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

    fn from_mqtt_payload(&mut self, payload: &str) {
        self.attr.from_mqtt_payload(payload);
    }

    fn update_field_with_string<F: Into<String>, V: Into<String>>(&mut self, field: F, value: V) {
        self.attr.update_field_with_string(field, value);
    }
}


