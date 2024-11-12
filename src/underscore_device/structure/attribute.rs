use panduza_platform_core::{AttributeMode, AttributeNotification};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
///
/// Attribute element in the structure representation
///
pub struct AttributElement {
    ///
    /// Type of the attribute
    ///
    #[serde(rename = "type")]
    // allow serde to convert this member into the good name "type" because
    // type is protected keyword in rust
    typee: String,

    ///
    /// Mode of the attribute
    ///
    mode: AttributeMode,

    ///
    /// User information about the structure
    ///
    info: Option<String>,

    ///
    /// Settings of the attribute
    ///
    settings: Option<serde_json::Value>,
}

impl AttributElement {
    ///
    ///
    ///
    pub fn new<T: Into<String>>(
        typee: T,
        mode: AttributeMode,
        info: Option<String>,
        settings: Option<serde_json::Value>,
    ) -> Self {
        Self {
            typee: typee.into(),
            mode,
            info,
            settings,
        }
    }
}

///
///
///
impl From<AttributeNotification> for AttributElement {
    fn from(notif: AttributeNotification) -> Self {
        AttributElement::new(
            notif.typee(),
            notif.mode().clone(),
            None,
            notif.settings().clone(),
        )
    }
}
