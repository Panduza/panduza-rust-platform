use std::sync::Arc;

use crate::device::State;
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
struct Notification {
    level: NotificationLevel,
    message: String,
    timestamp: u64,
}

///
///
///
struct InfoDevice {
    state: State,
    notifications: Vec<Notification>,
    notifier: Arc<Notify>,
}

impl InfoDevice {
    ///
    ///
    ///  
    pub fn change_state(&mut self, new_state: State) {
        self.state = new_state;
        self.notifier.notify_waiters();
    }
}
