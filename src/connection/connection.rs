use std::sync::Arc;

use futures::FutureExt;
use rumqttc::{AsyncClient, MqttOptions};
use tokio::sync::Mutex;

use crate::platform::PlatformTaskResult;
use crate::platform::TaskPoolLoader;

use crate::link::Manager as LinkManager;
use crate::link::AmManager as AmLinkManager;
use crate::subscription;

use super::logger::Logger;

use super::task::task as ConnectionTask;

/// Event loop for a single owner (the connection)
/// But this ownership is took by the task that runs the connection and released when the task ends
type ThreadSafeEventLoop = std::sync::Arc<
                                    std::sync::Mutex<
                                        rumqttc::EventLoop
                                    >    
                                >;
fn new_thread_safe_event_loop(event_loop: rumqttc::EventLoop) -> ThreadSafeEventLoop {
    std::sync::Arc::new(std::sync::Mutex::new(event_loop))
}


/// Connection object
///
#[derive(Clone)]
pub struct Connection {

    // Name of the connection
    name: String,

    // \todo: append connection status

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
            AsyncClient::new(mqtt_options.clone(), 10);

        // Create Connection Object
        return Connection {
            name: mqtt_options.client_id().clone(),
            logger: Logger::new( mqtt_options.client_id().clone() ),
            eventloop: new_thread_safe_event_loop(eventloop),
            link_manager: Arc::new(Mutex::new(
                LinkManager::new(client.clone())
            ))
        }

    }

    // /// Start the connection
    // ///
    // pub async fn start(&mut self, task_loader: &mut TaskPoolLoader) {

    //     // //
    //     // let ev: Arc<Mutex<rumqttc::EventLoop>> = self.eventloop.clone();
    //     // let lm: Arc<Mutex<LinkManager>> = self.link_manager.clone();

    //     // let cname = self.name.clone();


    //     let pp = self.clone();

    //     // Start connection process in a task
    //     task_loader.load(async move {
    //         ConnectionTask(pp).await
    //     }.boxed()).unwrap();

    //     // Info log
    //     tracing::info!(class="Connection", cname=self.name,
    //         "Connection started");
    // }

    /// Run the connection
    /// 
    /// \todo: rename connection_task and move it outisde of the connection impl block
    /// \todo: pass as parameter the connection object inside of all its components (connection will be clonable)
    /// 
    // async fn run(
    //     conneciton_name: String,
    //     ev: Arc<Mutex<rumqttc::EventLoop>>,
    //     lm: Arc<Mutex<LinkManager>>)
    //     -> PlatformTaskResult {


    //     }

    // }

    /// Process incoming packets
    /// 
    async fn process_incoming_packet(lm: Arc<Mutex<LinkManager>>, packet: &rumqttc::Packet) {
    
        match packet {
            rumqttc::Incoming::ConnAck(_ack) => {
                lm.lock().await.send_to_all(subscription::Message::new_connection_status(true)).await;
            },
            // rumqttc::Packet::SubAck(ack) => {
            //     println!("SubAck = {:?}", ack);
            // },
            rumqttc::Incoming::Publish(publish) => {
                // For each link with interfaces, check if the topic matches a filter
                // then send the message to the interface
                for link in lm.lock().await.links_as_mut().iter_mut() {
                    for filter in link.filters().iter() {
                        if filter.match_topic(&publish.topic) {
                            let message = 
                                subscription::Message::from_filter_and_publish_packet(filter, publish);

                            // tracing::trace!(
                            //     "Sending message to interface {}", message);


                            let r = link.tx().send(message).await;
                            if r.is_err() {
                                println!("Error sending message to interface {}",
                                    r.err().unwrap());
                            }
                        }
                    }
                }
            }
            _ => {
                // println!("? = {:?}", packet);
            }
        }
    }

    /// Get the link manager, to share it with the devices
    /// 
    pub fn link_manager(&self) -> AmLinkManager {
        return self.link_manager.clone();
    }


    pub fn logger(&self) -> Logger {
        return self.logger.clone();
    }

    pub fn event_loop(&self) -> ThreadSafeEventLoop {
        return self.eventloop;
    }

}


