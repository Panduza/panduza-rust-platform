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
#[derive(Clone, PartialEq, Eq)]
pub struct Message {
    id: Id,
    topic: String,
    payload: Bytes
}

impl Message {

    /// Create a new message
    pub fn new(id: Id, topic: &str, payload: Bytes) -> Message {
        return Message {
            id: id,
            topic: topic.to_string(),
            payload: payload
        }
    }

    pub fn from_filter_and_publish_packet(filter: &Filter, packet: &PublishPacket) -> Message {
        return Message {
            id: filter.id,
            topic: packet.get_topic(),
            payload: packet.get_payload()
        }
    }

    /// Get the topic
    pub fn get_topic(&self) -> &String {
        return &self.topic;
    }

    /// Get the payload
    pub fn get_payload(&self) -> &Bytes {
        return &self.payload;
    }

}

