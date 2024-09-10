use crate::{Error, MessageCodec};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

///
/// Codec for a simple Boolean
///
#[derive(Clone, PartialEq, Debug)]
pub struct StringCodec {
    pub value: String,
}

///
/// Implicit conversion from String
///
impl Into<StringCodec> for String {
    fn into(self) -> StringCodec {
        return StringCodec { value: self };
    }
}

///
/// Implicit conversion from str
///
impl Into<StringCodec> for &str {
    fn into(self) -> StringCodec {
        return StringCodec {
            value: self.to_string(),
        };
    }
}

///
/// To ease display
///
impl Display for StringCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

///
/// Do not use derive because we do not want { "value": true }
/// But only true or false on the payload
///
impl Serialize for StringCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value.as_str())
    }
}

///
/// See Serialize
///
impl<'de> Deserialize<'de> for StringCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(StringCodec { value })
    }
}

///
/// To apply all the required trait
///
impl MessageCodec for StringCodec {
    ///
    ///
    ///
    fn from_message_payload(data: &bytes::Bytes) -> Result<StringCodec, Error> {
        let p: StringCodec =
            serde_json::from_str(String::from_utf8(data.to_vec()).unwrap().as_str()).unwrap();
        Ok(p)
    }
    ///
    ///
    ///
    fn into_message_payload(&self) -> Result<Vec<u8>, Error> {
        let v = serde_json::to_string(self).map_err(|e| Error::SerializeFailure(e.to_string()))?;
        Ok(v.into_bytes())
    }
}
