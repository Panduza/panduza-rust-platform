use futures::FutureExt;
use tokio::sync::mpsc;
use rumqttc::MqttOptions;
use rumqttc::AsyncClient;


use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use std::collections::LinkedList;


use crate::platform::PlatformTaskResult;
use crate::platform::TaskPoolLoader;
use crate::subscription;
use crate::subscription::Filter as SubscriptionFilter;
use crate::subscription::Request as SubscriptionRequest;



mod manager;

pub type Manager = manager::Manager;
pub type AmManager = manager::AmManager;



// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Link handle for the interface
/// 
pub struct LinkInterfaceHandle
{
    /// Mqtt client
    pub client: AsyncClient,

    /// Channel to receive messages from the connection
    pub rx: mpsc::Receiver<subscription::Message>
}

impl LinkInterfaceHandle {

    fn new(client: AsyncClient, rx: mpsc::Receiver<subscription::Message>) -> LinkInterfaceHandle {
        return LinkInterfaceHandle {
            client: client.clone(),
            rx: rx
        }
    }

    pub fn get_client(&self) -> AsyncClient {
        return self.client.clone();
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Link handle for the connection
///
struct LinkConnectionHandle
{
    /// Channel to send messages to the interface
    tx: mpsc::Sender<subscription::Message>,

    /// List of filters
    filters: LinkedList<SubscriptionFilter>,
}

impl LinkConnectionHandle {
    fn new(tx: mpsc::Sender<subscription::Message>, filters: LinkedList<SubscriptionFilter>) -> LinkConnectionHandle {
        return LinkConnectionHandle {
            tx: tx,
            filters: filters,
        }
    }

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Link connection manager
/// 
pub struct LinkConnectionManager {
    /// Mqtt client
    client: AsyncClient,

    /// List of links
    links: LinkedList<LinkConnectionHandle>
}
pub type AmLinkConnectionManager = Arc<Mutex<LinkConnectionManager>>;

impl LinkConnectionManager {

    /// Create a new link connection manager
    fn new(client: AsyncClient) -> LinkConnectionManager {
        return LinkConnectionManager {
            client: client,
            links: LinkedList::new()
        }
    }

    /// Create a new link
    ///
    pub async fn request_link(&mut self, requests: Vec<SubscriptionRequest>) -> Result<LinkInterfaceHandle, String> {

        // Debug
        tracing::trace!("Request link with {} subscriptions", requests.len());

        // Create the channel
        let (tx, rx) =
            mpsc::channel::<subscription::Message>(64);


        let mut filters = LinkedList::new();

        for request in requests {

            self.client.subscribe(request.get_topic(), rumqttc::QoS::AtLeastOnce).await.unwrap();

            let filter = SubscriptionFilter::new(request);

            filters.push_back(filter);

        }


        // 
        self.links.push_back(
            LinkConnectionHandle::new(tx, filters)
        );

        
        return Ok(LinkInterfaceHandle::new(self.client.clone(), rx));
    }

    /// Send to all interfaces
    /// 
    pub async fn send_to_all(&mut self, message: subscription::Message) {
        for link in self.links.iter_mut() {
            let r = link.tx.send(message.clone()).await;
            if r.is_err() {
                println!("Error sending message to interface {}",
                    r.err().unwrap());
            }
            else {
                println!("Message sent to interface");
            }
        }
    }

}




// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Connection object
///
pub struct Connection {

    // Name of the connection
    name: String,

    /// Mqtt client
    client: AsyncClient,

    /// Event loop
    eventloop: Arc<Mutex<rumqttc::EventLoop>>,

    /// Links
    link_manager: AmLinkConnectionManager
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
            client: client.clone(),
            eventloop: Arc::new(Mutex::new(eventloop)),
            link_manager: Arc::new(Mutex::new(
                LinkConnectionManager::new(client.clone())
            ))
        }

    }

    /// Start the connection
    ///
    pub async fn start(&mut self, task_loader: &mut TaskPoolLoader) {

        //
        let ev: Arc<Mutex<rumqttc::EventLoop>> = self.eventloop.clone();
        let lm: Arc<Mutex<LinkConnectionManager>> = self.link_manager.clone();

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
        lm: Arc<Mutex<LinkConnectionManager>>)
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
    async fn process_incoming_packet(lm: Arc<Mutex<LinkConnectionManager>>, packet: &rumqttc::Packet) {
    
        match packet {
            rumqttc::Incoming::ConnAck(ack) => {

                lm.lock().await.send_to_all(subscription::Message::new_connection_status(true)).await;
                // let message = subscription::Message::new_connection_status(true);

            },
            rumqttc::Incoming::Publish(publish) => {
                // For each link with interfaces, check if the topic matches a filter
                // then send the message to the interface
                for link in lm.lock().await.links.iter_mut() {
                    for filter in link.filters.iter() {
                        if filter.match_topic(&publish.topic) {
                            let message = 
                                subscription::Message::from_filter_and_publish_packet(filter, publish);
                            let r = link.tx.send(message).await;
                            if r.is_err() {
                                println!("Error sending message to interface {}",
                                    r.err().unwrap());
                            }
                            else {
                                println!("Message sent to interface");
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
    pub fn clone_link_manager(&self) -> AmLinkConnectionManager {
        return self.link_manager.clone();
    }

}


