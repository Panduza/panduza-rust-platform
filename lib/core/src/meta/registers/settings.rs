


pub enum RegisterSize {
    _8bits,
    _16bits,
    _32bits,
    _64bits
}

pub struct MetaSettings {
    pub base_address: u64,
    pub register_size: RegisterSize,
    pub number_of_register: usize
}
