use std::fmt::Display;

use crate::MessageCodec;

#[derive(Copy, Clone, PartialEq, Debug)]
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

///
/// Encoding data
///
impl Into<Vec<u8>> for U64Codec {
    fn into(self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }
}

impl Display for U64Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl MessageCodec for U64Codec {}
