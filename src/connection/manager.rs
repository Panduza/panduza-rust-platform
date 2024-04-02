use std::{sync::Arc, time::Duration};

use rumqttc::MqttOptions;
use tokio::sync::Mutex;

use crate::platform::TaskPoolLoader;

use super::{AmConnection, Connection};

/// Object to manage and run multiple named connections
///
pub struct Manager {
    /// Platform name
    platform_name: String,

    /// Only one connection managed
    connection: Option<AmConnection>,

    /// Task pool loader
    task_loader: TaskPoolLoader
}
pub type AmManager = Arc<Mutex<Manager>>;

impl Manager {

    /// Create a new manager
    ///
    pub fn new(task_loader: TaskPoolLoader, platform_name: &str) -> AmManager {
        return Arc::new(Mutex::new(Manager {
            platform_name: platform_name.to_string(),
            connection: None,
            task_loader: task_loader
        }));
    }

    /// Create a new inactive connection
    ///
    pub async fn load_connection<S: Into<String>, T: Into<String>>(&mut self, name: S, host: T, port: u16) {
        // Get name with the correct type
        let name_string = name.into();

        // Create connection ID
        let id = format!("{}::{}", self.platform_name, name_string);

        // Set default options
        let mut mqtt_options = MqttOptions::new(id, host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(60));

        // Create connection Object
        self.connection = Some(Arc::new(Mutex::new(Connection::new(mqtt_options))));
    }


}


