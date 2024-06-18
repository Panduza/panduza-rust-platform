use std::sync::Arc;

use rumqttc::{AsyncClient, MqttOptions};
use tokio::sync::Mutex;

use crate::link::ThreadSafeLinkManager;

use crate::link::Manager as LinkManager;
use crate::link::AmManager as AmLinkManager;

use super::logger::Logger;

/// Event loop for a single owner (the connection)
/// But this ownership is took by the task that runs the connection and released when the task ends
type ThreadSafeEventLoop = std::sync::Arc<
                                tokio::sync::Mutex<
                                        rumqttc::EventLoop
                                    >    
                                >;
fn new_thread_safe_event_loop(event_loop: rumqttc::EventLoop) -> ThreadSafeEventLoop {
    std::sync::Arc::new(tokio::sync::Mutex::new(event_loop))
}


/// Connection object
///
#[derive(Clone)]
pub struct Connection {

    // Name of the connection
    // name: String,


    logger: Logger,

    /// Event loop
    eventloop: ThreadSafeEventLoop,

    /// Links
    link_manager: AmLinkManager
}
pub type AmConnection = Arc<Mutex<Connection>>;

impl Connection {

    /// Create a new connection
    /// 
    pub fn new(mqtt_options: MqttOptions) -> Connection {
        // Info log
        tracing::info!(class="Connection", cname=mqtt_options.client_id(), "Connection created");

        // Create the client and event loop
        let (client, eventloop) = 
            AsyncClient::new(mqtt_options.clone(), 100);

        // Create Connection Object
        return Connection {
            // name: mqtt_options.client_id().clone(),
            logger: Logger::new( mqtt_options.client_id().clone() ),
            eventloop: new_thread_safe_event_loop(eventloop),
            link_manager: Arc::new(Mutex::new(
                LinkManager::new(client.clone())
            ))
        }

    }



    /// Get the link manager, to share it with the devices
    /// 
    pub fn link_manager(&self) -> ThreadSafeLinkManager {
        return self.link_manager.clone();
    }


    pub fn logger(&self) -> Logger {
        return self.logger.clone();
    }

    pub fn event_loop(&self) -> ThreadSafeEventLoop {
        return self.eventloop.clone();
    }

}


