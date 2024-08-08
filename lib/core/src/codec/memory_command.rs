use serde_json::json;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::MessageCodec;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MemoryCommandCodec {
    command: String,
    index: u64,
    values: Vec<u64>,
    repeat_ms: u64,
}

impl MemoryCommandCodec {
    pub fn new() -> Self {
        Self {
            command: "read".to_string(),
            index: 0,
            values: Vec::new(),
            repeat_ms: 0,
        }
    }
}

impl Into<MemoryCommandCodec> for u64 {
    fn into(self) -> MemoryCommandCodec {
        return MemoryCommandCodec::new();
    }
}

impl From<Vec<u8>> for MemoryCommandCodec {
    fn from(value: Vec<u8>) -> Self {
        return MemoryCommandCodec::new();
    }
}

impl Into<Vec<u8>> for MemoryCommandCodec {
    fn into(self) -> Vec<u8> {
        return json!({
            "command": self.command,
            "index": self.index,
            "values": self.values,
            "repeat_ms": self.repeat_ms,
        })
        .to_string()
        .as_bytes()
        .to_vec();
    }
}

impl Display for MemoryCommandCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.command))
    }
}

impl MessageCodec for MemoryCommandCodec {}
