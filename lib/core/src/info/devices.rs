mod structure;

pub use structure::AttributeMode;
pub use structure::DeviceStructure;
pub use structure::ElementAttribute;
pub use structure::ElementInterface;
pub use structure::StructuralElement;

use std::sync::Arc;

use crate::{device::State, Error};
use std::collections::HashMap;
use tokio::sync::{Mutex, Notify};

///
///
///
// pub enum NotificationLevel {
//     Info,
//     Warning,
//     Error,
// }

///
///
///
// pub struct Notification {
//     level: NotificationLevel,
//     message: String,
//     timestamp: u64,
// }

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
    // notifications: Vec<Notification>,

    ///
    /// True if the object has been updated
    /// The info device set it back to false when changes are published
    has_been_updated: bool,

    ///
    /// This allow this object to notify its parent
    /// This will trigger actions to manage a status notification
    /// The excepted action is a new publication on device "_"
    device_status_change_notifier: Arc<Notify>,

    device_structure_change_notifier: Arc<Notify>,

    ///
    ///
    ///
    structure: DeviceStructure,
}

///
/// Thread safe wrapper
///
pub type ThreadSafeInfoDynamicDeviceStatus = Arc<Mutex<InfoDynamicDeviceStatus>>;

impl InfoDynamicDeviceStatus {
    pub fn new(
        device_status_change_notifier: Arc<Notify>,
        device_structure_change_notifier: Arc<Notify>,
    ) -> InfoDynamicDeviceStatus {
        let new_instance = InfoDynamicDeviceStatus {
            state: State::Booting,
            // notifications: Vec::new(),
            has_been_updated: true,
            device_status_change_notifier: device_status_change_notifier,
            device_structure_change_notifier: device_structure_change_notifier,
            structure: DeviceStructure::new(),
        };
        new_instance.device_status_change_notifier.notify_waiters();
        new_instance
    }

    pub fn state_as_string(&self) -> String {
        format!("{}", self.state)
    }

    pub fn structure_into_json_value(&self) -> serde_json::Value {
        self.structure.into_json_value()
    }

    ///
    ///
    ///  
    pub fn change_state(&mut self, new_state: State) {
        self.state = new_state;
        self.has_been_updated = true;
        self.device_status_change_notifier.notify_waiters();
    }

    pub fn has_been_updated(&mut self) -> bool {
        if self.has_been_updated {
            self.has_been_updated = false;
            return true;
        }
        return false;
    }

    pub fn structure_insert(
        &mut self,
        topic: String,
        element: StructuralElement,
    ) -> Result<(), Error> {
        // println!("{:?}", self.structure.into_json_value());

        let res = self.structure.insert(topic, element);
        self.device_structure_change_notifier.notify_waiters();
        res
    }

    // pub fn structure_remove()
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

    ///
    /// Notified when a device status change
    ///
    device_status_change_notifier: Arc<Notify>,

    device_structure_change_notifier: Arc<Notify>,
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
            device_status_change_notifier: Arc::new(Notify::new()),
            device_structure_change_notifier: Arc::new(Notify::new()),
        }
    }

    pub async fn structure_into_json_value(&self) -> serde_json::Value {
        let mut p = serde_json::Map::new();

        for e in &self.devs {
            p.insert(e.0.clone(), e.1.lock().await.structure_into_json_value());
        }

        p.into()
    }

    ///
    ///
    pub fn devs(&mut self) -> &mut HashMap<String, Arc<Mutex<InfoDynamicDeviceStatus>>> {
        &mut self.devs
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
    ///
    pub fn device_status_change_notifier(&self) -> Arc<Notify> {
        self.device_status_change_notifier.clone()
    }

    ///
    ///
    pub fn device_structure_change_notifier(&self) -> Arc<Notify> {
        self.device_structure_change_notifier.clone()
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
        let new_obj = Arc::new(Mutex::new(InfoDynamicDeviceStatus::new(
            self.device_status_change_notifier.clone(),
            self.device_structure_change_notifier.clone(),
        )));
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

    ///
    /// Go trough status and check for update
    ///
    /// WARNING
    /// Maybe not the best way of doing this feature, it will force a thread to run useless
    /// periodic actions
    ///
    pub async fn check_for_status_update(&self) -> Vec<ThreadSafeInfoDynamicDeviceStatus> {
        let mut updated_status = Vec::new();
        for d in &self.devs {
            if d.1.lock().await.has_been_updated {
                updated_status.push(d.1.clone())
            }
        }
        updated_status
    }
}
