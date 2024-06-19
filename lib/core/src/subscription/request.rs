
use crate::subscription;


/// Subscription request, to help the connection to prepare a filter
///
pub struct Request {
    id: subscription::Id,
    topic: String
}

impl Request {

    /// Create a new subscription request
    pub fn new(id: subscription::Id, topic: &str) -> Request {
        return Request {
            id: id,
            topic: topic.to_string()
        }
    }

    pub fn id(&self) -> subscription::Id {
        return self.id;
    }

    pub fn topic(&self) -> &String {
        return &self.topic;
    }

}
