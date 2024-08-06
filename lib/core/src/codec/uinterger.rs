use std::fmt::Display;

use crate::MessageCodec;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct UIntergerCodec {
    value: u64,
}

impl Into<UIntergerCodec> for u64 {
    fn into(self) -> UIntergerCodec {
        return UIntergerCodec { value: 0 };
    }
}

impl From<Vec<u8>> for UIntergerCodec {
    fn from(value: Vec<u8>) -> Self {
        return UIntergerCodec { value: 0 };
    }
}
impl Into<Vec<u8>> for UIntergerCodec {
    fn into(self) -> Vec<u8> {
        return vec![1];
    }
}

impl Display for UIntergerCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl MessageCodec for UIntergerCodec {}
