
// use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use rumqttc::{MqttOptions, AsyncClient, QoS};

use std::collections::HashMap;
use tokio::task::AbortHandle;


/// Object to manage multiple one connection
struct Runner {
    mqtt_options: MqttOptions,
    task_abort: AbortHandle,
}

impl Runner {


    pub fn new(task_pool: &mut tokio::task::JoinSet<()>, host: String, port: u16) -> Runner {
        let mut options = MqttOptions::new("TEST_1", host, port);
        options.set_keep_alive(Duration::from_secs(5));

        let (mut client, mut eventloop) = AsyncClient::new(options.clone(), 10);

        let abort = task_pool.spawn(async move {
            
            loop {
                while let Ok(notification) = eventloop.poll().await {
                    println!("Received = {:?}", notification);
                }
                tracing::warn!("Broker disconnected, trying to reconnect");

            }

        });

        return Runner {
            mqtt_options: options,
            task_abort: abort
        }
    }


    // start

    pub fn stop(&self) {
        self.task_abort.abort();
    }

            // let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);


}


/// Object to manage multiple named connections
pub struct Manager {
    connections: HashMap<String, Runner>
}


impl Manager {

    pub fn new() -> Manager {
        return Manager {
            connections: HashMap::new()
        }
    }


    pub fn add_connection(&mut self, task_pool: &mut tokio::task::JoinSet<()>, name: String, host: String, port: u16) {
        self.connections.insert(name, Runner::new(task_pool, host, port));
    }
    


}




// MqttPublisher
    // publish with pza helpers

// MqttSubscriber
    // subscribe and callback filters
