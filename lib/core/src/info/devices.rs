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
///
///
pub struct InfoDev {
    state: State,
    notifications: Vec<Notification>,
}

impl InfoDev {
    pub fn new() -> InfoDev {
        InfoDev {
            state: State::Booting,
            notifications: Vec::new(),
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
    devs: HashMap<String, Arc<Mutex<InfoDev>>>,

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
    pub fn get_dev_info(&self, name: &String) -> Option<Arc<Mutex<InfoDev>>> {
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

    pub fn validate_request(&mut self, request: InfoDevRequest) {
        self.devs
            .insert(request.name, Arc::new(Mutex::new(InfoDev::new())));
        self.request_validation_notifier.notify_waiters();
    }
}
