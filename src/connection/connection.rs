use std::sync::Arc;

use futures::FutureExt;
use rumqttc::{AsyncClient, MqttOptions};
use tokio::sync::Mutex;

use crate::platform::PlatformTaskResult;
use crate::platform::TaskPoolLoader;

use crate::link::Manager as LinkManager;
use crate::link::AmManager as AmLinkManager;
use crate::subscription;

/// Connection object
///
pub struct Connection {

    // Name of the connection
    name: String,

    /// Event loop
    eventloop: Arc<Mutex<rumqttc::EventLoop>>,

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
            name: mqtt_options.client_id(),
            eventloop: Arc::new(Mutex::new(eventloop)),
            link_manager: Arc::new(Mutex::new(
                LinkManager::new(client.clone())
            ))
        }

    }

    /// Start the connection
    ///
    pub async fn start(&mut self, task_loader: &mut TaskPoolLoader) {

        //
        let ev: Arc<Mutex<rumqttc::EventLoop>> = self.eventloop.clone();
        let lm: Arc<Mutex<LinkManager>> = self.link_manager.clone();

        let cname = self.name.clone();

        // Start connection process in a task
        task_loader.load(async move {
            Connection::run(cname, ev, lm).await
        }.boxed()).unwrap();

        // Info log
        tracing::info!(class="Connection", cname=self.name,
            "Connection started");
    }

    /// Run the connection
    ///
    async fn run(
        conneciton_name: String,
        ev: Arc<Mutex<rumqttc::EventLoop>>,
        lm: Arc<Mutex<LinkManager>>)
        -> PlatformTaskResult {

        // Event loop mangement
        // Poll the reception in a loop
        loop {
            while let Ok(notification) = ev.lock().await.poll().await {
                // Debug log
                tracing::trace!(class="Connection", cname=conneciton_name,
                    "{:?}", notification);

                // Check notification
                match notification {
                    rumqttc::Event::Incoming(incoming) => {
                        Connection::process_incoming_packet(lm.clone(), &incoming).await;
                        
                    }
                    rumqttc::Event::Outgoing(outgoing) => {
                        match outgoing {
                            
                            rumqttc::Outgoing::Subscribe(subscribe) => {
                                println!("Subscribe = {:?}", subscribe);
                            },
                            _ => {
                                // println!("Outgoing = {:?}", outgoing);
                            }
                            // rumqttc::Outgoing::Publish(_) => todo!(),
                            // rumqttc::Outgoing::Unsubscribe(_) => todo!(),
                            // rumqttc::Outgoing::PubAck(_) => todo!(),
                            // rumqttc::Outgoing::PubRec(_) => todo!(),
                            // rumqttc::Outgoing::PubRel(_) => todo!(),
                            // rumqttc::Outgoing::PubComp(_) => todo!(),
                            // rumqttc::Outgoing::PingReq => todo!(),
                            // rumqttc::Outgoing::PingResp => todo!(),
                            // rumqttc::Outgoing::Disconnect => todo!(),
                            // rumqttc::Outgoing::AwaitAck(_) => todo!(),
                        }
                        // println!("Outgoing = {:?}", outgoing);
                    }
                    _ => {
                        // println!("Received = {:?}", notification);
                    }
                }
            }

            // Here the broker is disconnected
            tracing::warn!(class="Connection", cname=conneciton_name,
                "Broker disconnected, trying to reconnect");

            let message = 
                                subscription::Message::new_connection_status(false);
            // let r = link.tx.send(message).await;
        }

    }

    /// Process incoming packets
    /// 
    async fn process_incoming_packet(lm: Arc<Mutex<LinkManager>>, packet: &rumqttc::Packet) {
    
        match packet {
            rumqttc::Incoming::ConnAck(ack) => {
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

                            tracing::trace!(
                                "Sending message to interface {}", message);


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


}


