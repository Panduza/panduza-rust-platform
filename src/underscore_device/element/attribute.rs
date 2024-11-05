use panduza_platform_core::AttributeMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributElement {
    ///
    /// Type of the attribute
    ///
    #[serde(rename = "type")]
    typee: String,

    ///
    ///
    ///
    mode: AttributeMode,
}

impl AttributElement {
    ///
    ///
    ///
    pub fn new<T: Into<String>>(typee: T, mode: AttributeMode) -> Self {
        Self {
            typee: typee.into(),
            mode,
        }
    }
}
