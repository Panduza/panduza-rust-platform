use bytes::Bytes;
use regex::Regex;

use rumqttc::mqttbytes::v4::Publish as PublishPacket;

/// Subscription ID
///
pub type Id = u16;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Subscription request, to help the connection to prepare a filter
///
pub struct Request {
    id: Id,
    topic: String
}

impl Request {

    /// Create a new subscription request
    pub fn new(id: Id, topic: &str) -> Request {
        return Request {
            id: id,
            topic: topic.to_string()
        }
    }

    pub fn get_topic(&self) -> &String {
        return &self.topic;
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Allow a connection to filter messages for an interface.
/// The Id helps the interface to know which message is for which callback.
///
pub struct Filter {
    id: Id,
    filter: Regex
}

impl Filter {

    /// Create a new subscription filter
    pub fn new(request: Request) -> Filter {

        let filter = Regex::new(request.topic.as_str()).unwrap();

        return Filter {
            id: request.id,
            filter: filter
        }
    }

    /// Check if the topic match the filter
    ///
    pub fn match_topic(&self, topic: &str) -> bool {
        return self.filter.is_match(topic);
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Subscription ID
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MqttMessage {
    id: Id,
    topic: String,
    payload: Bytes
}

impl MqttMessage {

    /// Create a new message from a filter and a publish packet
    /// Usefull to create a message direclty injectable inside fifo for the interfaces
    pub fn from_filter_and_publish_packet(filter: &Filter, packet: &PublishPacket) -> MqttMessage {
        return MqttMessage {
            id: filter.id,
            topic: packet.topic.clone(),
            payload: packet.payload.clone()
        }
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Subscription ID
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConnectionStatusMessage {

    /// Connection status (True if connected, False if disconnected)
    connected: bool,

}

impl ConnectionStatusMessage {

    /// Create a new connection status message
    pub fn new(connected: bool) -> ConnectionStatusMessage {
        return ConnectionStatusMessage {
            connected: connected
        }
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

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
