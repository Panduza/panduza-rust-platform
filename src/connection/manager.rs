use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use rumqttc::MqttOptions;

use serde_json::Value;

use super::Connection;
use super::AmConnection;

use crate::platform::TaskPoolLoader;

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

    /// Create and start the broker connection
    ///
    pub async fn start_connection(&mut self, host: &str, port: u16) {

        // Create connection ID
        let id = format!("{}", self.platform_name);
        let host  = host;
        let port = port;

        // Set default options
        let mut mqtt_options = MqttOptions::new(id, host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(60));

        // Create connection Object
        self.connection = Some(Arc::new(Mutex::new(Connection::new(mqtt_options))));

        // Start the connection
        self.connection.as_mut().unwrap().lock().await.start(&mut self.task_loader).await;
    }

    /// Get the connection
    /// 
    pub fn connection(&self) -> Option<AmConnection > {
        return self.connection.clone();
    }

}


