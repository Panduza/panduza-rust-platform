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

        // Create the client and event loop
        let (client, eventloop) = 
            AsyncClient::new(mqtt_options.clone(), 10);

        // Create Connection Object
        return Connection {
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

        // Start connection process in a task
        task_loader.load(async move {
            Connection::run(ev, lm).await
        }.boxed()).unwrap();

    }

    /// Run the connection
    ///
    async fn run(ev: Arc<Mutex<rumqttc::EventLoop>>, lm: Arc<Mutex<LinkConnectionManager>>) -> PlatformTaskResult {

        loop {
            while let Ok(notification) = ev.lock().await.poll().await {
                println!("Received = {:?}", notification);
                match notification {
                    rumqttc::Event::Incoming(incoming) => {
                        Connection::process_incoming_packet(lm.clone(), &incoming).await;
                        
                    }
                    _ => {
                        println!("Received = {:?}", notification);
                    }
                }
            }
            tracing::warn!("Broker disconnected, trying to reconnect");

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

                println!("!!!!!!! !Received = {:?}", ack);
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
                println!("? = {:?}", packet);
            }
        }
    }

    /// Get the link manager, to share it with the devices
    /// 
    pub fn clone_link_manager(&self) -> AmLinkConnectionManager {
        return self.link_manager.clone();
    }

}


// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Object to manage and run multiple named connections
///
pub struct Manager {
    /// Platform name
    platform_name: String,

    /// Map of managed connections
    connections: HashMap<String, AmConnection>,

    task_loader: TaskPoolLoader
}
pub type AmManager = Arc<Mutex<Manager>>;

impl Manager {

    /// Create a new manager
    ///
    pub fn new(    task_loader: TaskPoolLoader, platform_name: &str) -> AmManager {
        return Arc::new(Mutex::new(Manager {
            platform_name: platform_name.to_string(),
            connections: HashMap::new(),
            task_loader: task_loader
        }));
    }

    /// Create a new inactive connection
    ///
    pub async fn create_connection<S: Into<String>, T: Into<String>>(&mut self, name: S, host: T, port: u16) {
        // Get name with the correct type
        let name_string = name.into();

        // Create connection ID
        let id = format!("{}::{}", self.platform_name, name_string);

        // Info log
        tracing::info!("Create connection {:?}", id);


        // Set default options
        let mut mqtt_options = MqttOptions::new(id, host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        // Create connection Object
        self.connections.insert(name_string,
            Arc::new(Mutex::new(
                Connection::new(mqtt_options))
            )
        );
    }

    /// Start a connection
    ///
    /// name: name of the connection to start
    /// task_pool: main JoinSet to attach the running connection to a task
    ///
    pub async fn start_connection<A: Into<String>>(&mut self, name: A) {
        // Get the connection clone for the task
        let conn = self.connections.get(&name.into()).unwrap().clone();

        // Start the connection
        conn.lock().await.start(&mut self.task_loader).await;
    }

    /// Get a connection
    /// 
    pub fn get_connection(&mut self, name: &str) -> AmConnection {
        return self.connections.get(name).unwrap().clone();
    }

}

