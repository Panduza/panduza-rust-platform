use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;
use bitflags::bitflags;
use tokio::sync::Notify;
use std::cmp::PartialEq;

use super::TaskPoolLoader;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Requests: u32 {
        const NO_REQUEST            = 0b00000000;
        const BOOTING               = 0b00000001;
        const RELOAD_TREE           = 0b00000010;


        /// Critical error detected, the platform cannot work anymore even in degraded mode
        /// try to stop and give as many information as possible
        const PANIC                 = 0b10000000;
    }
}

/// Set the requests
impl PartialEq for Requests {
    fn eq(&self, other: &Self) -> bool {
        self.bits() == other.bits()
    }
}

/// Services provided by the platform to all the sub objects
pub struct Services {
    /// Requests
    requests: Requests,

    /// Requests change notifier
    requests_change_notifier : Arc<Notify>,

    /// Brut content of the currently loaded tree
    tree_content: serde_json::Value,

    /// Panic cause, try to keep ip empty :)
    panic_cause: String,

    task_loader: TaskPoolLoader
}
pub type AmServices = Arc<Mutex<Services>>;

impl Services {

    /// Create a new instance of the Services
    pub fn new(task_loader: TaskPoolLoader) -> AmServices {
        // create the requests_change_notifier and start a first notification
        let notify = Arc::new(Notify::new());
        notify.notify_one();

        // pack the object
        return Arc::new(Mutex::new(Services {
            requests: Requests::BOOTING,
            requests_change_notifier: notify,
            tree_content: serde_json::Value::Null,
            panic_cause: String::new(),
            task_loader: task_loader
        }));
    }

    /// Get the tree content
    /// 
    pub fn get_requests_change_notifier(&self) -> Arc<Notify> {
        return self.requests_change_notifier.clone();
    }

    /// Insert a request
    /// 
    fn insert_request(&mut self, request: Requests) {
        self.requests.insert(request);
        self.requests_change_notifier.notify_one();
    }

    /// Set the tree content
    /// 
    pub fn set_tree_content(&mut self, content: serde_json::Value) {
        self.tree_content = content;
        self.insert_request(Requests::RELOAD_TREE);
    }

    /// Trigger a panic mode
    /// 
    pub fn trigger_panic(&mut self, main_cause: &str) {
        self.panic_cause = main_cause.to_string();
        self.insert_request(Requests::PANIC);
    }

    /// Check if there are pending requests
    /// 
    pub fn has_pending_requests(&self) -> bool {
        return self.requests != Requests::NO_REQUEST;
    }

    /// Check if a request is pending
    /// 
    fn xxx_requested(&mut self, request: Requests) -> bool {
        let v = self.requests.contains(request);
        if v {
            self.requests.remove(request);
        }
        return v;
    }

    /// Check if booting is requested and remove the flag
    /// 
    pub fn booting_requested(&mut self) -> bool {
        return self.xxx_requested(Requests::BOOTING);
    }

    /// Check if reload tree is requested and remove the flag
    /// 
    pub fn reload_tree_requested(&mut self) -> bool {
        return self.xxx_requested(Requests::RELOAD_TREE);
    }

}
