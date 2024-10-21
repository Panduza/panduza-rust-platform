// info pack will be shared accross the application
// each subsection must have a arc mutex and a notifier
// the info device will wait on each notifier to update its attributes
//
// On peut aussi faire un notifier par device state pour update qu'un topic pour chaque device
//

use super::Topic;
use std::sync::Arc;

use panduza_platform_core::Notification;
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
                    println!("state {:?}", state_notification);

                    self.inner
                        .lock()
                        .unwrap()
                        .process_state_changed(state_notification);
                }
                Notification::ElementCreated(structural_notification) => {
                    println!("create");
                }
                Notification::ElementDeleted(structural_notification) => {
                    println!("deleted");
                }
            }
        }
        println!("manage noti");
    }

    // pub fn devices(&self) -> Arc<Mutex<InfoPackInner>> {
    //     self.inner.clone()
    // }

    // ///
    // ///
    // pub async fn device_status_change_notifier(&self) -> Arc<Notify> {
    //     self.inner.lock().await.device_status_change_notifier()
    // }

    // ///
    // ///
    // pub async fn device_structure_change_notifier(&self) -> Arc<Notify> {
    //     self.inner.lock().await.device_structure_change_notifier()
    // }

    // pub async fn device_structure_as_json_value(&self) -> serde_json::Value {
    //     self.inner.lock().await.structure_into_json_value().await
    // }

    // pub async fn add_device(&mut self, name: String) -> Arc<Mutex<InfoDynamicDeviceStatus>> {
    //     let request_validated_notifier = self.devices.lock().await.request_validation_notifier();

    //     self.devices
    //         .lock()
    //         .await
    //         .push_device_creation_request(name.clone());

    //     request_validated_notifier.notified().await;

    //     self.devices.lock().await.get_dev_info(&name).unwrap()
    // }

    ///
    ///
    ///
    pub async fn check_for_status_update(&self) {
        // let ds = self.devices.lock().await;
        // for d in ds {}
    }
}
