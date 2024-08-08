// use std::fmt::Display;

// use crate::MessageCodec;

// #[derive(Clone, PartialEq, Debug)]
// pub struct JsonCodec {
//     value: serde_json::Value,
// }

// impl Into<JsonCodec> for bool {
//     fn into(self) -> JsonCodec {
//         return JsonCodec {
//             value: serde_json::Value::Null,
//         };
//     }
// }

// impl From<Vec<u8>> for JsonCodec {
//     fn from(value: Vec<u8>) -> Self {
//         return JsonCodec {
//             value: serde_json::Value::Null,
//         };
//     }
// }
// impl Into<Vec<u8>> for JsonCodec {
//     fn into(self) -> Vec<u8> {
//         return vec![1];
//     }
// }

// impl Display for JsonCodec {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}", self.value))
//     }
// }

// impl MessageCodec for JsonCodec {}
