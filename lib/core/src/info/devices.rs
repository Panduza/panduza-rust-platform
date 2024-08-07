use crate::device::State;

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
    state: State, // notifications
}

///
///
///
struct InfoDevices {
    // map<string, Arc<Mutex<InfoDevice>>
}
