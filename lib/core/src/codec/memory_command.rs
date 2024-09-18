use crate::{Error, MessageCodec};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///
/// Available actions for a memory command
///
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MemoryCommandMode {
    Read,
    Write,
    Erase,
}

///
///
///
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AccessSize {
    _8Bits,
    _16Bits,
    _32Bits,
    _64Bits,
}

///
/// Memory Command
/// Standardized command to request action on a memory
///
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MemoryCommandCodec {
    // Manage att return for a status ?
    ///
    /// Which action must be done
    ///
    pub mode: MemoryCommandMode,
    ///
    /// Target starting address
    ///
    pub address: u64,
    ///
    ///
    ///
    pub size: Option<u64>,
    pub values: Option<Vec<u64>>,
    pub repeat_ms: Option<u64>,
}

impl MemoryCommandCodec {
    pub fn new(mode: MemoryCommandMode, address: u64) -> Self {
        Self {
            mode: mode,
            address: address,
            size: None,
            values: None,
            repeat_ms: None,
        }
    }
}

// impl Into<MemoryCommandCodec> for u64 {
//     fn into(self) -> MemoryCommandCodec {
//         return MemoryCommandCodec::new();
//     }
// }

impl Display for MemoryCommandCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.mode))
    }
}

impl MessageCodec for MemoryCommandCodec {
    ///
    ///
    ///
    fn from_message_payload(data: &bytes::Bytes) -> Result<MemoryCommandCodec, Error> {
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
        "memory_command".to_string()
    }
}
