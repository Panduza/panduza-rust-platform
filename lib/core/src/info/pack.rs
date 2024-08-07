// info pack will be shared accross the application
// each subsection must have a arc mutex and a notifier
// the info device will wait on each notifier to update its attributes
//
// On peut aussi faire un notifier par device state pour update qu'un topic pour chaque device
//

use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, Notify};

use super::devices::{self, InfoDev, InfoDevs};

#[derive(Clone)]
pub struct InfoPack {
    ///
    /// Devices infos, one for each instanciated device
    ///
    devices: Arc<Mutex<InfoDevs>>,
}

impl InfoPack {
    ///
    /// Constructor
    ///
    pub fn new() -> InfoPack {
        InfoPack {
            devices: Arc::new(Mutex::new(InfoDevs::new())),
        }
    }

    pub fn devices(&self) -> Arc<Mutex<InfoDevs>> {
        self.devices.clone()
    }

    ///
    ///
    pub async fn new_request_notifier(&self) -> Arc<Notify> {
        self.devices.lock().await.new_request_notifier()
    }

    pub async fn add_device(&mut self, name: String) -> Arc<Mutex<InfoDev>> {
        let request_validated_notifier = self.devices.lock().await.request_validation_notifier();

        self.devices
            .lock()
            .await
            .push_device_creation_request(name.clone());

        request_validated_notifier.notified().await;

        self.devices.lock().await.get_dev_info(&name).unwrap()
    }
}
