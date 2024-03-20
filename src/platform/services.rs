use std::sync::Arc;
use tokio::sync::Mutex;
use bitflags::bitflags;
use tokio::sync::Notify;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Requests: u32 {
        const NO_REQUEST            = 0b00000000;
        const BOOTING               = 0b00000001;
    }
}

/// Services provided by the platform to all the sub objects
pub struct Services {
    /// Requests
    requests: Requests,

    /// Requests change notifier
    requests_change_notifier : Arc<Notify>
}
pub type AmServices = Arc<Mutex<Services>>;

impl Services {

    /// Create a new instance of the Services
    pub fn new() -> AmServices {
        return Arc::new(Mutex::new(Services {
            requests: Requests::BOOTING,
            requests_change_notifier: Arc::new(Notify::new())
        }));
    }

    /// Check if there are pending requests
    pub fn has_pending_requests(&self) -> bool {
        return self.requests.contains(Requests::NO_REQUEST);
    }

    /// Set the requests
    pub fn get_requests_change_notifier(&self) -> Arc<Notify> {
        return self.requests_change_notifier.clone();
    }

}
