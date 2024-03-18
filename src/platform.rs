use tokio::signal;
use tokio::task::JoinSet;
use crate::device::Manager as DeviceManager;
use crate::connection::Manager as ConnectionManager;

// use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use rumqttc::{MqttOptions, AsyncClient, QoS};

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


        self.devices.create_device("server", "panduza.server").await;

        self.connections.start_connection("default", &mut self.task_pool).await;

        self.attach_device_to_connection("server", "default").await;



        self.devices.mount_devices(&mut self.task_pool).await;


        // attach device and connection
        // mount interfaces


        //let mut aaa = Interface::new();
        // aaa.start(&mut self.task_pool).await;


        // self.task_pool.spawn(async move {
        //         aaa.poll().await
        //     }
        // );

        
        // create connections
        // then devices
        // then attach devices to connections
        



        // self.task_pool.spawn(self.connections.join_all_connections());
        

        // Info log
        tracing::info!("Platform Started");
        
        // tracing::trace!("Trace Mode On");
        // tracing::debug!("Trace Mode On");

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

