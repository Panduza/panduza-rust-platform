
use self::{mqtt::MqttMessage, status::ConnectionStatusMessage};
use rumqttc::mqttbytes::v4::Publish as PublishPacket;

use super::Filter;
use std::fmt::Display;

pub mod mqtt;
pub mod status;



#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Message {
    Mqtt(MqttMessage),
    ConnectionStatus(ConnectionStatusMessage)
}


impl Message {

    /// Create a new message from a filter and a publish packet
    /// Usefull to create a message direclty injectable inside fifo for the interfaces
    pub fn from_filter_and_publish_packet(filter: &Filter, packet: &PublishPacket) -> Message {
        return Message::Mqtt(MqttMessage::from_filter_and_publish_packet(filter, packet));
    }

    /// Create a new connection status message
    pub fn new_connection_status(connected: bool) -> Message {
        return Message::ConnectionStatus(ConnectionStatusMessage::new(connected));
    }
    
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Mqtt(m) => write!(f, "Mqtt Message {} {}", m.id(), m.topic()),
            Message::ConnectionStatus(m) => write!(f, "connection status")
        }
    }
}

