use serde::Serialize;


#[derive(Copy, Clone, Debug)]
pub enum RegisterSize {
    _8bits,
    _16bits,
    _32bits,
    _64bits
}

impl Serialize for RegisterSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        match self {
            RegisterSize::_8bits => serializer.serialize_u32(8),
            RegisterSize::_16bits => serializer.serialize_u32(16),
            RegisterSize::_32bits => serializer.serialize_u32(32),
            RegisterSize::_64bits => serializer.serialize_u32(64)
        }
    }
}

pub struct MetaSettings {
    pub base_address: u64,
    pub register_size: RegisterSize,
    pub number_of_register: usize
}
