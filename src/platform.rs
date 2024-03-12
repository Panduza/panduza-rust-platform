use tokio::signal;
use tokio::task::JoinSet;
use crate::device::Manager as DeviceManager;
use crate::connection::Manager as ConnectionManager;

// use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use rumqttc::{MqttOptions, AsyncClient, QoS};

pub struct Runner
{
    task_pool: JoinSet<()>,
    devices: DeviceManager,
    connections: ConnectionManager
    
    // devices  HashMap<String, Box<dyn Producer>>

}

impl Runner {

    /// Create a new instance of the Runner
    pub fn new() -> Runner {
        return Runner {
            task_pool: JoinSet::new(),
            devices: DeviceManager::new(),
            connections: ConnectionManager::new()
        }
    }

    /// Main platform run loop
    pub async fn work(&mut self) {

        // Info log
        tracing::info!("Platform Starting...");



        self.connections.add_connection(&mut self.task_pool,"default".to_string(), "localhost".to_string(), 1883);


        self.devices.create_device("server", "panduza.server");



        self.devices.mount_devices(&mut self.task_pool);
        

        // attach device and connection
        // mount interfaces



        
        // create connections
        // then devices
        // then attach devices to connections
        



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

    




}

