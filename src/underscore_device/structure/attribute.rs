use panduza_platform_core::{AttributeMode, AttributeNotification};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
///
/// Attribute element in the structure representation
///
pub struct AttributElement {
    /// Type of the attribute
    ///
    #[serde(rename = "type")]
    // allow serde to convert this member into the good name "type" because
    // type is protected keyword in rust
    typee: String,

    /// True if the attribute is enable, false else
    ///
    #[serde(skip)]
    enable: bool,

    /// Mode of the attribute
    ///
    mode: AttributeMode,

    /// User information about the structure
    ///
    info: Option<String>,

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
        enable: bool,
        mode: AttributeMode,
        info: Option<String>,
        settings: Option<serde_json::Value>,
    ) -> Self {
        Self {
            typee: typee.into(),
            enable,
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
        // TODO: here notif members should be moved !
        AttributElement::new(
            notif.typee(),
            true,
            notif.mode().clone(),
            notif.info().clone(),
            notif.settings().clone(),
        )
    }
}
