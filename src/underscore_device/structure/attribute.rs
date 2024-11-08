use panduza_platform_core::{AttributeMode, AttributeNotification};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributElement {
    ///
    /// Type of the attribute
    ///
    #[serde(rename = "type")]
    // allow serde to convert this member into the good name "type" because
    // type is protected keyword in rust
    typee: String,

    ///
    ///
    ///
    mode: AttributeMode,

    ///
    /// User information about the structure
    ///
    info: String,
}

impl AttributElement {
    ///
    ///
    ///
    pub fn new<T: Into<String>>(typee: T, mode: AttributeMode, info: String) -> Self {
        Self {
            typee: typee.into(),
            mode,
            info,
        }
    }
}

///
///
///
impl From<AttributeNotification> for AttributElement {
    fn from(notif: AttributeNotification) -> Self {
        AttributElement::new(notif.typee(), notif.mode().clone(), "".to_string())
    }
}
