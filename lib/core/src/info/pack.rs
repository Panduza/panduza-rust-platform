// info pack will be shared accross the application
// each subsection must have a arc mutex and a notifier
// the info device will wait on each notifier to update its attributes
//
// On peut aussi faire un notifier par device state pour update qu'un topic pour chaque device
//

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

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
            devices: HashMap::new(),
        }
    }

    // add_device -> creation d'une interface associée
    // del_device
}
