use std::fmt::Display;

use crate::MessageCodec;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MemoryCommandCodec {
    value: u64,
}

impl Into<MemoryCommandCodec> for u64 {
    fn into(self) -> MemoryCommandCodec {
        return MemoryCommandCodec { value: 0 };
    }
}

impl From<Vec<u8>> for MemoryCommandCodec {
    fn from(value: Vec<u8>) -> Self {
        return MemoryCommandCodec { value: 0 };
    }
}
impl Into<Vec<u8>> for MemoryCommandCodec {
    fn into(self) -> Vec<u8> {
        return vec![1];
    }
}

impl Display for MemoryCommandCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

impl MessageCodec for MemoryCommandCodec {}
