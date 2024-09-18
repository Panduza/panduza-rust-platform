use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Error, MessageCodec};

#[derive(Clone, PartialEq, Debug)]
pub struct NumberCodec {
    value: serde_json::Value,
}

///
/// Allow implicit convertion
///
impl Into<NumberCodec> for u64 {
    fn into(self) -> NumberCodec {
        return NumberCodec {
            value: serde_json::json!(self),
        };
    }
}

///
/// Allow implicit convertion
///
impl Into<NumberCodec> for u16 {
    fn into(self) -> NumberCodec {
        return NumberCodec {
            value: serde_json::json!(self),
        };
    }
}

///
/// Allow implicit convertion
///
impl Into<NumberCodec> for i32 {
    fn into(self) -> NumberCodec {
        return NumberCodec {
            value: serde_json::json!(self),
        };
    }
}

///
/// Do not use derive because we do not want { "value": true }
/// But only true or false on the payload
///
impl Serialize for NumberCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

///
/// See Serialize
///
impl<'de> Deserialize<'de> for NumberCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(NumberCodec { value })
    }
}

impl MessageCodec for NumberCodec {
    ///
    ///
    ///
    fn from_message_payload(data: &bytes::Bytes) -> Result<Self, Error> {
        let p: Self =
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
        "number".to_string()
    }
}
