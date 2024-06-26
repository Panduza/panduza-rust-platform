use std::sync::Arc;
use std::time::Duration;

use futures::FutureExt;
use tokio::sync::Mutex;
use rumqttc::MqttOptions;

use super::task::task as ConnectionTask;
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
        mqtt_options.set_max_packet_size(1000000000, 100000000);

        // Create connection Object
        self.connection = Some(Arc::new(Mutex::new(Connection::new(mqtt_options))));




        let co = self.connection.as_mut().unwrap().clone();

        // Start connection process in a task
        self.task_loader.load(async move {
                ConnectionTask( co ).await
            }.boxed()
        ).unwrap();

    }

    /// Get the connection
    /// 
    pub fn connection(&self) -> Option<AmConnection > {
        return self.connection.clone();
    }

}


