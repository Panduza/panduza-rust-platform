use crate::{Error, MessageCodec};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

///
/// Codec for a simple Boolean
///
#[derive(Clone, PartialEq, Debug)]
pub struct StringListCodec {
    pub list: Vec<String>,
}

///
/// Implicit conversion from bool
///
impl Into<StringListCodec> for Vec<String> {
    fn into(self) -> StringListCodec {
        return StringListCodec { list: self };
    }
}

///
/// To ease display
///
impl Display for StringListCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.list.len()))
    }
}

///
/// Do not use derive because we do not want { "value": true }
/// But only true or false on the payload
///
impl Serialize for StringListCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.list.len()))?;
        for element in &self.list {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }
}

///
/// See Serialize
///
impl<'de> Deserialize<'de> for StringListCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let list = Vec::<String>::deserialize(deserializer)?;
        Ok(StringListCodec { list })
    }
}

///
/// To apply all the required trait
///
impl MessageCodec for StringListCodec {
    ///
    ///
    ///
    fn from_message_payload(data: &bytes::Bytes) -> Result<StringListCodec, Error> {
        let p: StringListCodec =
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

    ///
    fn typee() -> String {
        "string_list".to_string()
    }
}
