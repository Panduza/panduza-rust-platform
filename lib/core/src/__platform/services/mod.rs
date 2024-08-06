use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;
use bitflags::bitflags;
use tokio::sync::Notify;
use std::cmp::PartialEq;


use crate::platform::connection_info::export_file;


pub mod boot;
pub mod hunt;

use super::TaskReceiverLoader;
use super::connection_info::Info as ConnectionInfo;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Requests: u32 {
        const NO_REQUEST            = 0b00000000;
        const BOOTING               = 0b00000001;
        const RELOAD_TREE           = 0b00000010;

        const HUNT                  = 0b00000100;

        /// Request a normal stop of the platform
        const STOP                  = 0b01000000;

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


    hunt_in_progress: bool,

    device_store: serde_json::Value,

    /// Panic cause, try to keep ip empty :)
    _panic_cause: String,


    connection_info: Option<ConnectionInfo>,

    pub task_loader: TaskReceiverLoader,

}
pub type AmServices = Arc<Mutex<Services>>;
pub type ThreadSafeServices = Arc<Mutex<Services>>;

impl Services {

    /// Create a new instance of the Services
    pub fn new(task_loader: TaskReceiverLoader) -> AmServices {
        // create the requests_change_notifier and start a first notification
        let notify = Arc::new(Notify::new());
        notify.notify_one();

        // pack the object
        return Arc::new(Mutex::new(Services {
            requests: Requests::BOOTING,
            requests_change_notifier: notify,
            tree_content: serde_json::Value::Null,
            hunt_in_progress: false,
            device_store: serde_json::Value::Null,
            _panic_cause: String::new(),
            connection_info: None,
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

    pub fn start_hunting(&mut self) {
        self.hunt_in_progress = true;
        self.insert_request(Requests::HUNT);
    }

    /// Get the tree content
    ///
    pub fn get_tree_content(&self) -> &serde_json::Value {
        return &self.tree_content;
    }

    /// Trigger a panic mode
    /// 
    // pub fn trigger_panic(&mut self, main_cause: &str) {
    //     self._panic_cause = main_cause.to_string();
    //     self.insert_request(Requests::PANIC);
    // }

    /// Trigger a platform stop
    ///
    pub fn trigger_stop(&mut self) {
        self.insert_request(Requests::STOP);
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

    /// Check if stop is requested and remove the flag
    /// 
    pub fn stop_requested(&mut self) -> bool {
        return self.xxx_requested(Requests::STOP);
    }

    pub fn trigger_hunt(&mut self) {
        self.insert_request(Requests::HUNT);
    }

    pub fn hunt_requested(&mut self) -> bool {
        return self.xxx_requested(Requests::HUNT);
    }

    /// Get the connection info
    ///
    pub fn connection_info(&self) -> &Option<ConnectionInfo> {
        &self.connection_info
    }

    /// Set the connection info
    ///
    pub fn set_connection_info(&mut self, ci: ConnectionInfo) {
        self.connection_info = Some(ci);
    }

    /// Set the default connection info
    ///
    pub fn generate_default_connection_info(&mut self) -> Result<(), std::io::Error> {
        self.connection_info = Some(ConnectionInfo::default());
        
        export_file(self.connection_info.as_ref().unwrap()).unwrap();

        Ok(())
    }


    pub fn is_hunt_in_progress(&self) -> bool {
        self.hunt_in_progress
    }

    pub fn start_hunting_set_flag(&mut self) {
        self.hunt_in_progress = true;
    }

    // Store the devices currently connected in USB
    // to use it in the platform (without needing the tree.jsib)
    pub fn update_device_store(&mut self, store: serde_json::Value) {
        self.device_store = store;
        self.hunt_in_progress = false;
    }

    pub fn get_device_store(&self) -> &serde_json::Value {
        &self.device_store
    }

}
