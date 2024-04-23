use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use rumqttc::MqttOptions;

// use std::io;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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

    // /// Load the network file from system into service data
    // ///
    // async fn load_network_file(services: AmServices) -> Result<(), error::PlatformError> {

    //     // Get the network file path
    //     let network_file_path = PathBuf::from(dirs::home_dir().unwrap()).join("panduza").join("network.json");// Try to read the file content
        
    //     let file_content = tokio::fs::read_to_string(&network_file_path).await;
    //     match file_content {
    //         Ok(content) => {
    //             return Platform::load_network_string(services.clone(), &content).await;
    //         },
    //         Err(e) => {
    //             return platform_error!(
    //                 format!("Failed to read {:?} file content: {}", network_file_path, e), None)
    //         }
    //     }
    // }

    // /// Load a network string into service data
    // ///
    // async fn load_network_string(services: AmServices, content: &String) -> Result<(), error::PlatformError> {
    //     // Parse the JSON content
    //     let json_content = serde_json::from_str::<serde_json::Value>(&content);
    //     match json_content {
    //         Ok(json) => {
    //             let host = json_value.get("BROKER_HOST").as_str();
    //             let port = json_value.get("BROKER_PORT").as_i64;
    //             println!("extracted : host={}, port={}", host, port);

    //             return Ok(());
    //         },
    //         Err(e) => {
    //             return platform_error!(
    //                 format!("Failed to parse JSON content: {}", e), None)
    //         }
    //     }
    // }

    /// Create and start the broker connection
    ///
    pub async fn start_connection(&mut self) {

        // let network_file = File::open("/home/lucas/panduza/network.txt").await;
        // let mut content = vec![];
        // network_file.expect("REASON").read_to_end(&mut content).await;
        // println!("extracted : {:?}", &content);

        // Create connection ID
        let id = format!("{}", self.platform_name);
        let host  = "localhost";
        let port = 1883;

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


