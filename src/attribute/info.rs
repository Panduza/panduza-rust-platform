

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
        attr.update_field_with_string("version", &version.into());
        attr.update_field_with_string("state", &"init".to_string());
        attr.update_field_with_string("error", &"".to_string());
        return InfoAttribute { attr };
    }

    /// Create a new instance of the object inside a box
    ///
    pub fn new_boxed<A: Into<String>, B: Into<String>>(
        itype: A, version: B
    ) -> Box<dyn AttributeInterface> {
        return Box::new(InfoAttribute::new(itype, version));
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

    fn need_publication(&self) -> bool {
        self.attr.need_publication()
    }

    fn publication_done(&mut self) {
        self.attr.publication_done();
    }

    fn update_field_with_f64(&mut self, field: &str, value: f64) {
        self.attr.update_field_with_f64(field, value);
    }

    fn update_field_with_bool(&mut self, field: &str, value: bool) {
        self.attr.update_field_with_bool(field, value);
    }

    fn update_field_with_string(&mut self, field: &str, value: &String) {
        self.attr.update_field_with_string(field, value);
    }
}


