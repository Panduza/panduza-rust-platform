use std::sync::Arc;
use tokio::sync::Mutex;
use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Requests: u32 {
        const NO_REQUEST            = 0b00000000;
        const BOOTING               = 0b00000001;
    }
}

/// Services provided by the platform to all the sub objects
pub struct Services {
    requests: Requests
}
pub type AmServices = Arc<Mutex<Services>>;

impl Services {

    /// Create a new instance of the Services
    pub fn new() -> AmServices {
        return Arc::new(Mutex::new(Services {
            requests: Requests::BOOTING
        }));
    }

    /// Check if there are pending requests
    pub fn has_pending_requests(&self) -> bool {
        return self.requests.contains(Requests::NO_REQUEST);
    }

}
