use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::MessageCodec;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BooleanCodec {
    pub value: bool,
}

impl Into<BooleanCodec> for bool {
    fn into(self) -> BooleanCodec {
        return BooleanCodec { value: true };
    }
}

impl From<Vec<u8>> for BooleanCodec {
    fn from(value: Vec<u8>) -> Self {
        return BooleanCodec { value: true };
    }
}
impl Into<Vec<u8>> for BooleanCodec {
    fn into(self) -> Vec<u8> {
        return vec![1];
    }
}

impl Display for BooleanCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl Serialize for BooleanCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.value)
    }
}

impl<'de> Deserialize<'de> for BooleanCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = bool::deserialize(deserializer)?;
        Ok(BooleanCodec { value })
    }
}

impl MessageCodec for BooleanCodec {}
