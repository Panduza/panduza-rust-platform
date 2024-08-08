use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::MessageCodec;

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct U64Codec {
    value: u64,
}

///
/// Allow implicit convertion from u64
///
impl Into<U64Codec> for u64 {
    fn into(self) -> U64Codec {
        return U64Codec { value: self };
    }
}

///
/// Decoding data
///
impl From<Vec<u8>> for U64Codec {
    fn from(data: Vec<u8>) -> Self {
        return U64Codec {
            value: u64::from_be_bytes(data[..4].try_into().unwrap()),
        };
    }
}

impl MessageCodec for U64Codec {
    fn from_message_payload(data: &bytes::Bytes) -> Result<U64Codec, crate::Error> {
        todo!()
    }

    fn into_message_payload(&self) -> Result<Vec<u8>, crate::Error> {
        todo!()
    }
}
