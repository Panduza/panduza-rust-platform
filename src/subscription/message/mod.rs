use self::{mqtt::MqttMessage, status::ConnectionStatusMessage};
use rumqttc::mqttbytes::v4::Publish as PublishPacket;

use super::Filter;

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

