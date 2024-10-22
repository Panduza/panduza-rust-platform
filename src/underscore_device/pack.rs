use std::sync::Arc;

use panduza_platform_core::{device::State, Notification};
use tokio::sync::Notify;

use super::devices::InfoPackInner;

#[derive(Clone)]
pub struct InfoPack {
    ///
    /// Devices infos, one for each instanciated device
    ///
    inner: Arc<std::sync::Mutex<InfoPackInner>>,
}

impl InfoPack {
    ///
    /// Constructor
    ///
    pub fn new() -> InfoPack {
        InfoPack {
            inner: Arc::new(std::sync::Mutex::new(InfoPackInner::new())),
        }
    }

    pub fn process_notifications(&mut self, notifications: Vec<Notification>) {
        for not in &notifications {
            match not {
                Notification::StateChanged(state_notification) => {
                    // println!("state {:?}", state_notification);

                    self.inner
                        .lock()
                        .unwrap()
                        .process_state_changed(state_notification);
                }
                Notification::ElementCreated(structural_notification) => {
                    println!("create {:?}", structural_notification);

                    self.inner
                        .lock()
                        .unwrap()
                        .process_element_creation(structural_notification);
                }
                Notification::ElementDeleted(structural_notification) => {
                    println!("deleted {:?}", structural_notification);
                }
            }
        }
    }

    pub fn pack_instance_status(&self) -> Vec<(String, State)> {
        self.inner.lock().unwrap().pack_instance_status()
    }

    // pub fn devices(&self) -> Arc<Mutex<InfoPackInner>> {
    //     self.inner.clone()
    // }

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

    pub async fn device_structure_as_json_value(&self) -> serde_json::Value {
        self.inner.lock().unwrap().structure_into_json_value()
    }
}
