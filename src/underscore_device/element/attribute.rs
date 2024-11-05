use serde::{Deserialize, Serialize};
use serde_json::json;

use panduza_platform_core::AttributeMode;

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributElement {
    name: String,

    ///
    /// Type of the attribute
    ///
    #[serde(rename = "type")]
    typee: String,

    mode: AttributeMode,
}

impl AttributElement {
    ///
    ///
    ///
    pub fn new<N: Into<String>, T: Into<String>>(name: N, typee: T, mode: AttributeMode) -> Self {
        Self {
            name: name.into(),
            typee: typee.into(),
            mode,
        }
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        json!({
            // "name": self.name,
            "type": self.typee,
            "mode": self.mode
        })
    }

    ///
    pub fn name(&self) -> &String {
        &self.name
    }
    // ///
    // pub fn typee(&self) -> &String {
    //     &self.typee
    // }
    // pub fn mode(&self) -> &AttributeMode {
    //     &self.mode
    // }

    // ///
    // /// Attribute does not hold any elements
    // ///
    // pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
    //     Ok(false)
    // }
}
