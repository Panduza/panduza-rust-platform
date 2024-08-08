use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Error, MessageCodec};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NumberCodec {
    value: serde_json::Value,
}

///
/// Allow implicit convertion from u64
///
impl Into<NumberCodec> for u64 {
    fn into(self) -> NumberCodec {
        return NumberCodec {
            value: serde_json::json!(self),
        };
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
}
