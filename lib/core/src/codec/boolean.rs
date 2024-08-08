use crate::MessageCodec;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

///
/// Codec for a simple Boolean
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BooleanCodec {
    pub value: bool,
}

///
/// Implicit conversion from bool
///
impl Into<BooleanCodec> for bool {
    fn into(self) -> BooleanCodec {
        return BooleanCodec { value: true };
    }
}

///
/// To ease display
///
impl Display for BooleanCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

///
/// Do not use derive because we do not want { "value": true }
/// But only true or false on the payload
///
impl Serialize for BooleanCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.value)
    }
}

///
/// See Serialize
///
impl<'de> Deserialize<'de> for BooleanCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = bool::deserialize(deserializer)?;
        Ok(BooleanCodec { value })
    }
}

///
/// To apply all the required trait
///
impl MessageCodec for BooleanCodec {}
