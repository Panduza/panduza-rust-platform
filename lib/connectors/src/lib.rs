
mod loggers;
pub mod serial;
pub mod usb;

/// Logger used for generic logging inside the connectors crate
pub type GateLogger = loggers::GateLogger;

/// Logger dedicated to connectors
pub type ConnectorLogger = loggers::ConnectorLogger;



pub type UsbSettings = usb::settings::Settings;
pub type SerialSettings = serial::settings::Settings;
