use bytes::Bytes;
use rumqttc::mqttbytes::v4::Publish as PublishPacket;

use crate::subscription;

///
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MqttMessage {
    id: subscription::Id,
    topic: String,
    payload: Bytes
}

impl MqttMessage {

    /// Create a new message from a filter and a publish packet
    /// Usefull to create a message direclty injectable inside fifo for the interfaces
    pub fn from_filter_and_publish_packet(filter: &subscription::Filter, packet: &PublishPacket) -> MqttMessage {
        return MqttMessage {
            id: filter.id(),
            topic: packet.topic.clone(),
            payload: packet.payload.clone()
        }
    }

    pub fn id(&self) -> subscription::Id {
        return self.id;
    }

    pub fn topic(&self) -> &String {
        return &self.topic;
    }

    pub fn payload(&self) -> &Bytes {
        return &self.payload;
    }

}