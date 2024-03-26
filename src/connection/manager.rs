use std::{collections::HashMap, sync::Arc, time::Duration};

use rumqttc::MqttOptions;
use tokio::sync::Mutex;

use crate::platform::TaskPoolLoader;

use super::{AmConnection, Connection};




/// Object to manage and run multiple named connections
///
pub struct Manager {
    /// Platform name
    platform_name: String,

    /// Map of managed connections
    connections: HashMap<String, AmConnection>,

    task_loader: TaskPoolLoader
}
pub type AmManager = Arc<Mutex<Manager>>;

impl Manager {

    /// Create a new manager
    ///
    pub fn new(    task_loader: TaskPoolLoader, platform_name: &str) -> AmManager {
        return Arc::new(Mutex::new(Manager {
            platform_name: platform_name.to_string(),
            connections: HashMap::new(),
            task_loader: task_loader
        }));
    }

    /// Create a new inactive connection
    ///
    pub async fn create_connection<S: Into<String>, T: Into<String>>(&mut self, name: S, host: T, port: u16) {
        // Get name with the correct type
        let name_string = name.into();

        // Create connection ID
        let id = format!("{}::{}", self.platform_name, name_string);


        // Set default options
        let mut mqtt_options = MqttOptions::new(id, host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        // Create connection Object
        self.connections.insert(name_string,
            Arc::new(Mutex::new(
                Connection::new(mqtt_options))
            )
        );
    }

    /// Start a connection
    ///
    /// name: name of the connection to start
    /// task_pool: main JoinSet to attach the running connection to a task
    ///
    pub async fn start_connection<A: Into<String>>(&mut self, name: A) {
        // Get the connection clone for the task
        let conn = self.connections.get(&name.into()).unwrap().clone();

        // Start the connection
        conn.lock().await.start(&mut self.task_loader).await;
    }

    /// Get a connection
    /// 
    pub fn get_connection(&mut self, name: &str) -> AmConnection {
        return self.connections.get(name).unwrap().clone();
    }

}


