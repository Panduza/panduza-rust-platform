use std::fmt::Display;

use crate::MessageCodec;

#[derive(Copy, Clone, PartialEq)]
pub struct BooleanCodec {
    value: bool,
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

impl MessageCodec for BooleanCodec {}
