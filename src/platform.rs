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


        // stop
        // read config
        // create devices
        // create connections
        // create benches
        // create interfaces on connections (associations)
        // start



        self.connections.create_connection(&mut self.task_pool,"default".to_string(), "localhost".to_string(), 1883);


        self.devices.create_device("server", "panduza.server");


        self.attach_device_to_connection("server", "default");


        self.devices.mount_devices();
        

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
    fn attach_device_to_connection(&mut self, device: &str, connection: &str) {

        // get device
        self.devices.get_device(&device.to_string()).unwrap().
            attach_connection(self.connections.get_connection(connection).unwrap());


        // get connection
        // attach device to connection
        
        // self.devices.attach_connection(device, connection);

    }




}

