use std::path::PathBuf;
use std::sync::Arc;

use dirs;
use serde_json::json;
use tokio::signal;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use crate::device;
use crate::connection;

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Services provided by the platform to all the sub objects
pub struct Services {

}
type AmServices = Arc<Mutex<Services>>;

impl Services {
    /// Create a new instance of the Services
    pub fn new() -> AmServices {
        return Arc::new(Mutex::new(Services {

        }));
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Platform main object
/// 
pub struct Platform
{
    /// Task pool to manage all tasks
    task_pool: JoinSet<()>,

    /// Services
    services: AmServices,

    /// Device manager
    devices: device::AmManager,

    /// Connection manager
    connections: connection::AmManager
}

impl Platform {

    /// Create a new instance of the Platform
    pub fn new(name: &str) -> Platform {
        return Platform {
            task_pool: JoinSet::new(),
            services: Services::new(),
            devices: device::Manager::new(),
            connections: connection::Manager::new(name)
        }
    }

    /// Main platform run loop
    pub async fn work(&mut self) {
        // Info log
        tracing::info!("Booting Platform...");

        // tracing::debug!("{:?}", dirs::home_dir().unwrap());
        // panduza

        let p = PathBuf::from(dirs::home_dir().unwrap()).join("panduza").join("tree.json");
        println!("{:?}", p);


        // Parse tree file
        // unload all
        // load tree

        // self.connections.create_connection("default", "localhost", 1883).await;

        // self.devices.create_device( &json!({ 
        //     "name": "host",
        //     "ref": "panduza.server" 
        //     })).await.unwrap();

        // self.connections.start_connection("default", &mut self.task_pool).await;

        // self.attach_device_to_connection("host", "default").await;



        // self.devices.mount_devices(&mut self.task_pool).await;
 

        // Info log
        tracing::info!("Platform Started !");
        
        // Wait for either a signal or all tasks to complete
        tokio::select! {
            _ = signal::ctrl_c() => {
                tracing::warn!("End by user ctrl-c");
            },
            _ = self.end_of_all_tasks() => {
                tracing::warn!("End by all tasks completed");
            }
        }
    }

    /// Wait for all tasks to complete
    async fn end_of_all_tasks( &mut self) {
        while let Some(result) = self.task_pool.join_next().await {
            tracing::info!("End task with result {:?}", result);
        }
    }

    

    /// Attach a device to a connection
    /// 
    async fn attach_device_to_connection(&mut self, device: &str, connection: &str) {

        // // get device
        // let devvv = self.devices.get_device(&device.to_string()).unwrap();

        // devvv.attach_connection(self.connections.get_connection(connection)).await;


        // get connection
        // attach device to connection
        
        // self.devices.attach_connection(device, connection);

    }




}

