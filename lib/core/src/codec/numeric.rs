use std::fmt::Display;

use crate::MessageCodec;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NumericCodec {
    value: u64,
}

impl Into<NumericCodec> for u64 {
    fn into(self) -> NumericCodec {
        return NumericCodec { value: 0 };
    }
}

impl From<Vec<u8>> for NumericCodec {
    fn from(value: Vec<u8>) -> Self {
        return NumericCodec { value: 0 };
    }
}
impl Into<Vec<u8>> for NumericCodec {
    fn into(self) -> Vec<u8> {
        return vec![1];
    }
}

impl Display for NumericCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl MessageCodec for NumericCodec {}
