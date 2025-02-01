pub mod attribute;
pub mod class;
pub mod instance;

use instance::{Alert, InstanceElement};
use panduza_platform_core::{instance::State, log_trace, Container, Error, Instance};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::pack::InfoPack;

/// Manage the Panduza core attributes '_/structure'
///
/// The instance structure is represented by a json object
///
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Structure {
    /// Instances managed
    ///
    driver_instances: HashMap<String, InstanceElement>,

    /// User information about the structure
    ///
    info: Option<String>,
}

impl Structure {
    ///
    ///
    ///
    pub fn insert_instance(&mut self, name: String, instance: InstanceElement) {
        self.driver_instances.insert(name, instance);
    }

    // pub fn get_

    ///
    ///
    ///
    pub fn contains_instance(&mut self, name: &String) -> bool {
        self.driver_instances.contains_key(name)
    }

    ///
    ///
    ///
    pub fn get_mut_instance(&mut self, name: &String) -> Option<&mut InstanceElement> {
        self.driver_instances.get_mut(name)
    }

    ///
    ///
    ///
    pub fn pack_instance_status(&self) -> Vec<(String, State, Vec<Alert>)> {
        let mut r = Vec::new();
        for (_key, value) in (&self.driver_instances).into_iter() {
            r.push((_key.clone(), value.state.clone(), value.alerts.clone()));
        }
        r
    }
}

/// Mount the structure attribute and manage events
///
pub async fn mount(mut instance: Instance, pack: InfoPack) -> Result<(), Error> {
    //
    // Get logger
    let logger = instance.logger.clone();

    //
    // Structure of the devices
    let structure_att = instance
        .create_attribute("structure")
        .with_ro()
        .finish_as_json()
        .await?;

    //
    // Set the initial structure
    structure_att
        .set(pack.device_structure_as_json_value().await?)
        .await?;

    //
    // Watch for structure changes
    let pack_bis = pack.clone();
    instance
        .spawn("structure/watcher", async move {
            //
            //
            let structure_change = pack_bis.instance_structure_change_notifier().await;
            // let pack_clone4 = pack_bis.clone();

            loop {
                //
                // Wait for next status change
                structure_change.notified().await;
                log_trace!(logger, "structure change notification");

                let structure = pack_bis.device_structure_as_json_value().await.unwrap();
                log_trace!(logger, "new structure {:?}", structure);

                structure_att.set(structure).await?;
            }
            // Ok(())
        })
        .await;

    //
    //
    Ok(())
}
