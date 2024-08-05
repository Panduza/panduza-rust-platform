pub struct ReactorSettings {
    addr: String,
    port_mqtt: u16,
}

impl ReactorSettings {
    pub fn new<A: Into<String>>(addr: A, port_mqtt: u16) -> Self {
        Self {
            addr: addr.into(),
            port_mqtt: port_mqtt,
        }
    }
}
