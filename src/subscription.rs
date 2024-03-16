use regex::Regex;

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

}


