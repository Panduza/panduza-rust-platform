#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PicohaDioRequest {
    #[prost(enumeration = "RequestType", tag = "1")]
    pub r#type: i32,
    #[prost(uint32, tag = "2")]
    pub pin_num: u32,
    #[prost(enumeration = "PinValue", tag = "3")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PicohaDioAnswer {
    #[prost(enumeration = "AnswerType", tag = "1")]
    pub r#type: i32,
    #[prost(enumeration = "PinValue", optional, tag = "3")]
    pub value: ::core::option::Option<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestType {
    Ping = 0,
    SetPinDirection = 1,
    SetPinValue = 2,
    GetPinDirection = 3,
    GetPinValue = 4,
}
impl RequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RequestType::Ping => "PING",
            RequestType::SetPinDirection => "SET_PIN_DIRECTION",
            RequestType::SetPinValue => "SET_PIN_VALUE",
            RequestType::GetPinDirection => "GET_PIN_DIRECTION",
            RequestType::GetPinValue => "GET_PIN_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PING" => Some(Self::Ping),
            "SET_PIN_DIRECTION" => Some(Self::SetPinDirection),
            "SET_PIN_VALUE" => Some(Self::SetPinValue),
            "GET_PIN_DIRECTION" => Some(Self::GetPinDirection),
            "GET_PIN_VALUE" => Some(Self::GetPinValue),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PinValue {
    Low = 0,
    High = 1,
    Input = 2,
    Output = 3,
}
impl PinValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PinValue::Low => "LOW",
            PinValue::High => "HIGH",
            PinValue::Input => "INPUT",
            PinValue::Output => "OUTPUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOW" => Some(Self::Low),
            "HIGH" => Some(Self::High),
            "INPUT" => Some(Self::Input),
            "OUTPUT" => Some(Self::Output),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnswerType {
    Success = 0,
    Failure = 1,
}
impl AnswerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnswerType::Success => "SUCCESS",
            AnswerType::Failure => "FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
