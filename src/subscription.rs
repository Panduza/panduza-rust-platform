
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

}
