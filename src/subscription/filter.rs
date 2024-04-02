
use regex::Regex;

use crate::subscription;

/// Allow a connection to filter messages for an interface.
/// The Id helps the interface to know which message is for which callback.
///
pub struct Filter {
    id: subscription::Id,
    filter: Regex
}

impl Filter {

    /// Create a new subscription filter
    pub fn new(request: subscription::Request) -> Filter {

        let filter = Regex::new(request.topic().as_str()).unwrap();

        return Filter {
            id: request.id(),
            filter: filter
        }
    }

    #[inline(always)]
    pub fn id(&self) -> subscription::Id {
        return self.id;
    }

    /// Check if the topic match the filter
    ///
    pub fn match_topic(&self, topic: &str) -> bool {
        return self.filter.is_match(topic);
    }

}
