use serde_json::json;
use tokio::signal;
use tokio::task::JoinSet;
use crate::device::Manager as DeviceManager;
use crate::connection::Manager as ConnectionManager;

pub struct Platform
{
    task_pool: JoinSet<()>,
    devices: DeviceManager,
    connections: ConnectionManager
    
    // devices  HashMap<String, Box<dyn Producer>>

}

impl Platform {

    /// Create a new instance of the Platform
    pub fn new(name: &str) -> Platform {
        return Platform {
            task_pool: JoinSet::new(),
            devices: DeviceManager::new(),
            connections: ConnectionManager::new(name)
        }
    }

    /// Main platform run loop
    pub async fn work(&mut self) {

        // Info log
        tracing::info!("Platform Starting...");


        // stop
        // read config
        // create devices
        // create connections
        // create benches
        // create interfaces on connections (associations)
        // start



        self.connections.create_connection("default", "localhost", 1883).await;


        self.devices.create_device( &json!({ 
                "name": "host",
                "ref": "panduza.server" 
            })).await.unwrap();

        self.connections.start_connection("default", &mut self.task_pool).await;

        self.attach_device_to_connection("host", "default").await;



        self.devices.mount_devices(&mut self.task_pool).await;
 

        // Info log
        tracing::info!("Platform Started");
        
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

        // get device
        let devvv = self.devices.get_device(&device.to_string()).unwrap();

        devvv.attach_connection(self.connections.get_connection(connection)).await;


        // get connection
        // attach device to connection
        
        // self.devices.attach_connection(device, connection);

    }




}

