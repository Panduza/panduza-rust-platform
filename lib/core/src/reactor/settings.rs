/// Settings for the reactor
///
pub struct ReactorSettings {
    pub addr: String,
    pub port_mqtt: u16,

    /// Namespace on which the reactor must work
    ///
    pub namespace: Option<String>,
}

impl ReactorSettings {
    pub fn new<A: Into<String>>(addr: A, port_mqtt: u16, namespace: Option<String>) -> Self {
        Self {
            addr: addr.into(),
            port_mqtt: port_mqtt,
            namespace: namespace.into(),
        }
    }
}
