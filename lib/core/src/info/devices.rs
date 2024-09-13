use std::sync::Arc;

use crate::device::State;
use std::collections::HashMap;
use tokio::sync::{Mutex, Notify};

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
/// Dynamic information that must be provided by the device to maintain a status inside
/// the main platform device "_".
///
/// The device "_" will always be up and will be able to display state and notification
/// if the device crash and is no longer able to communicate.
///
pub struct InfoDynamicDeviceStatus {
    ///
    /// Current state of the device
    state: State,

    ///
    /// Main Notifications from the device to the user
    notifications: Vec<Notification>,

    ///
    /// True if the object has been updated
    /// The info device set it back to false when changes are published
    has_been_updated: bool,
}

///
/// Thread safe wrapper
///
pub type ThreadSafeInfoDynamicDeviceStatus = Arc<Mutex<InfoDynamicDeviceStatus>>;

impl InfoDynamicDeviceStatus {
    pub fn new() -> InfoDynamicDeviceStatus {
        InfoDynamicDeviceStatus {
            state: State::Booting,
            notifications: Vec::new(),
            has_been_updated: true,
        }
    }

    ///
    ///
    ///  
    pub fn change_state(&mut self, new_state: State) {
        self.state = new_state;
    }
}

#[derive(Debug)]
pub enum RequestType {
    Create,
    Delete,
}

#[derive(Debug)]
pub struct InfoDevRequest {
    pub rtype: RequestType,
    pub name: String,
}

impl InfoDevRequest {
    pub fn new(rtype: RequestType, name: String) -> InfoDevRequest {
        InfoDevRequest {
            rtype: rtype,
            name: name,
        }
    }
}

pub struct InfoDevs {
    ///
    ///
    devs: HashMap<String, Arc<Mutex<InfoDynamicDeviceStatus>>>,

    ///
    ///
    requests: Vec<InfoDevRequest>,

    ///
    /// Notified when a new request is pending
    ///
    new_request_notifier: Arc<Notify>,

    ///
    /// Notified when a request has been managed by the InfoDevice
    ///
    request_validation_notifier: Arc<Notify>,
}

impl InfoDevs {
    ///
    ///
    pub fn new() -> InfoDevs {
        InfoDevs {
            devs: HashMap::new(),
            requests: Vec::new(),
            new_request_notifier: Arc::new(Notify::new()),
            request_validation_notifier: Arc::new(Notify::new()),
        }
    }

    ///
    ///
    pub fn new_request_notifier(&self) -> Arc<Notify> {
        self.new_request_notifier.clone()
    }

    ///
    ///
    pub fn request_validation_notifier(&self) -> Arc<Notify> {
        self.request_validation_notifier.clone()
    }

    ///
    pub fn push_device_creation_request(&mut self, name: String) {
        self.requests
            .push(InfoDevRequest::new(RequestType::Create, name));
        self.new_request_notifier.notify_waiters();
    }

    ///
    ///
    pub fn get_dev_info(&self, name: &String) -> Option<Arc<Mutex<InfoDynamicDeviceStatus>>> {
        match self.devs.get(name) {
            Some(o) => Some(o.clone()),
            None => None,
        }
    }

    ///
    ///
    ///
    pub fn pop_next_request(&mut self) -> Option<InfoDevRequest> {
        self.requests.pop()
    }

    ///
    /// Validate the creation request on managed devices
    ///
    pub fn validate_creation_request(
        &mut self,
        request: InfoDevRequest,
    ) -> ThreadSafeInfoDynamicDeviceStatus {
        //
        // Create the new object for the new device
        let new_obj = Arc::new(Mutex::new(InfoDynamicDeviceStatus::new()));
        //
        // Insert the object in the management list for InfoDynamicDeviceStatus
        self.devs.insert(request.name, new_obj.clone());
        //
        // Then notify waiting thread that the request is accepted
        self.request_validation_notifier.notify_waiters();
        //
        // If it is a creation request, return the InfoDev created
        new_obj
    }
}
