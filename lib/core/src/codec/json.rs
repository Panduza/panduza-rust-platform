use crate::{Error, MessageCodec};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, PartialEq, Debug)]
pub struct JsonCodec {
    value: serde_json::Value,
}

///
///
///
impl From<serde_json::Value> for JsonCodec {
    fn from(value: serde_json::Value) -> Self {
        return JsonCodec { value: value };
    }
}

///
///
///
impl Serialize for JsonCodec {
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
impl<'de> Deserialize<'de> for JsonCodec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(JsonCodec { value })
    }
}

impl MessageCodec for JsonCodec {
    ///
    ///
    ///
    fn from_message_payload(data: &bytes::Bytes) -> Result<Self, Error> {
        // Convert incoming bytes into a str
        let data_as_string = String::from_utf8(data.to_vec())
            .map_err(|e| Error::DeserializeFailure(e.to_string()))?;

        // Deserialize the string
        let p: JsonCodec = serde_json::from_str(data_as_string.as_str()).map_err(|e| {
            Error::DeserializeFailure(format!("serde_json fail on : {}", e.to_string()))
        })?;

        // Return
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
        "json".to_string()
    }
}
