use std::sync::Arc;

use panduza_platform_core::{instance::State, Error, Notification};
use tokio::sync::Notify;

use super::{pack_inner::InfoPackInner, structure::instance::Alert};

#[derive(Clone)]
pub struct InfoPack {
    ///
    /// Devices infos, one for each instanciated device
    ///
    inner: Arc<std::sync::Mutex<InfoPackInner>>,
}

impl InfoPack {
    /// Constructor
    ///
    pub fn new() -> InfoPack {
        InfoPack {
            inner: Arc::new(std::sync::Mutex::new(InfoPackInner::new())),
        }
    }

    /// Process runtime notifications
    ///
    pub fn process_notifications(&mut self, notifications: Vec<Notification>) {
        for not in notifications {
            match not {
                Notification::State(state_notification) => {
                    self.inner
                        .lock()
                        .unwrap()
                        .process_state_changed(&state_notification);
                }
                Notification::Alert(alert_notification) => {
                    self.inner.lock().unwrap().process_alert(alert_notification);
                }
                Notification::Class(n) => {
                    self.inner
                        .lock()
                        .unwrap()
                        .process_class_creation(n)
                        .unwrap();
                }
                Notification::Attribute(n) => {
                    self.inner
                        .lock()
                        .unwrap()
                        .process_attribute_creation(n)
                        .unwrap();
                }
                Notification::Enablement(n) => {
                    self.inner.lock().unwrap().process_enablement(n).unwrap();
                }
            }
        }
    }

    pub fn pack_instance_status(&self) -> Vec<(String, State, Vec<Alert>)> {
        self.inner.lock().unwrap().pack_instance_status()
    }

    ///
    ///
    pub fn instance_status_change_notifier(&self) -> Arc<Notify> {
        self.inner.lock().unwrap().instance_status_change_notifier()
    }

    ///
    ///
    pub async fn instance_structure_change_notifier(&self) -> Arc<Notify> {
        self.inner
            .lock()
            .unwrap()
            .instance_structure_change_notifier()
    }

    pub async fn device_structure_as_json_value(&self) -> Result<serde_json::Value, Error> {
        self.inner.lock().unwrap().structure_into_json_value()
    }
}
