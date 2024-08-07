use std::sync::Arc;

use crate::device::State;
use std::collections::HashMap;
use tokio::sync::Notify;

///
///
///
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}

///
///
///
pub struct Notification {
    level: NotificationLevel,
    message: String,
    timestamp: u64,
}

///
///
///
pub struct InfoDev {
    state: State,
    notifications: Vec<Notification>,
}

impl InfoDev {
    ///
    ///
    ///  
    pub fn change_state(&mut self, new_state: State) {
        self.state = new_state;
    }
}

pub struct InfoDevs {
    devs: HashMap<String, Arc<Mutex<InfoDev>>>,
    // requests: Vec<Request>
    notifier: Arc<Notify>,
}

impl InfoDevs {
    ///

    ///
    ///
    pub fn change_state(&mut self, device: String, new_state: State) {
        // self.devs.get_mut(device).
        self.notifier.notify_waiters();
    }
}
