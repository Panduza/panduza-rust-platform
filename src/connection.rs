use tokio::sync::mpsc;
use rumqttc::MqttOptions;
use rumqttc::AsyncClient;


use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use std::collections::LinkedList;



use bytes::Bytes;


use crate::subscription::Id as SubscriptionId;
use crate::subscription::Filter as SubscriptionFilter;
use crate::subscription::Request as SubscriptionRequest;



// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
pub struct SubscriptionMessage {
    sub_id: SubscriptionId,
    topic: String,
    pub payload: Bytes
}



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
    client: AsyncClient,

    /// Channel to receive messages from the connection
    rx: mpsc::Receiver<SubscriptionMessage>
}

impl LinkInterfaceHandle {

    fn new(client: AsyncClient, rx: mpsc::Receiver<SubscriptionMessage>) -> LinkInterfaceHandle {
        return LinkInterfaceHandle {
            client: client.clone(),
            rx: rx
        }
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
    tx: mpsc::Sender<SubscriptionMessage>,

    /// List of filters
    filters: LinkedList<SubscriptionFilter>,
}

impl LinkConnectionHandle {
    fn new(tx: mpsc::Sender<SubscriptionMessage>, filters: LinkedList<SubscriptionFilter>) -> LinkConnectionHandle {
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
pub type SafeLinkConnectionManager = Arc<Mutex<LinkConnectionManager>>;

impl LinkConnectionManager {
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
        let (tx, mut rx) =
            mpsc::channel::<SubscriptionMessage>(32);


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
    link_manager: SafeLinkConnectionManager
}
pub type SafeConnection = Arc<Mutex<Connection>>;

impl Connection {

    /// Create a new connection
    /// 
    pub fn new(mqtt_options: MqttOptions) -> Connection {
/*


        let text = "Hello, world! Rust is awesome.";
        let pattern = r"Rust";
    
        let regex = Regex::new(pattern).unwrap();
        if regex.is_match(text) {
            println!("Found a match for pattern '{}'", pattern);
        } else {
            println!("No match found for pattern '{}'", pattern);
        }



        let links_obj = Arc::new(Mutex::new(LinkedList::<LinkConnectionHandle>::new()));
        let links_move = links_obj.clone();


        task_pool.spawn(async move {
            loop {
                println!("checking links");
                for link in links_move.lock().await.iter_mut() {
                    // let mut link = link.lock().unwrap();

                    let data = link.topic_subscriber_rx.recv().await;
                    println!("{}", data.unwrap());
                    // link.tx.send("hello".to_string()).await;
                }
        }});


        let abort = task_pool.spawn(async move {

            // client.subscribe("pza", QoS::AtMostOnce).await.unwrap();

            loop {


                // tokio::select! {
                //     _ = signal::ctrl_c() => {

                    
                while let Ok(notification) = eventloop.poll().await {
                    println!("Received = {:?}", notification);
                    match notification {
                        rumqttc::Event::Incoming(incoming) => {
                            println!("I = {:?}", incoming);

                            match incoming {
                                rumqttc::Incoming::Publish(publish) => {
                                    println!("P = {:?}", publish);
                                    println!("  pkid    = {:?}", publish.pkid);
                                    println!("  retain  = {:?}", publish.retain);
                                    println!("  topic   = {:?}", publish.topic);
                                    println!("  payload = {:?}", publish.payload);
                                    println!("  qos     = {:?}", publish.qos);
                                    println!("  dup     = {:?}", publish.dup);
                                    
                                }
                                _ => {
                                    println!("? = {:?}", incoming);
                                }
                            }

                            // println!("I = {:?}", incoming.read().unwrap());
                            

                        }
                        rumqttc::Event::Outgoing(outgoing) => {
                            // println!("O = {:?}", outgoing);

                            
                        }
                    }
                    // println!("pp = {:?}", notification);
                }

            }

        });
 */

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
    pub async fn start(&mut self, task_pool: &mut tokio::task::JoinSet<()>) {

        //
        let ev: Arc<Mutex<rumqttc::EventLoop>> = self.eventloop.clone();
        let lm: Arc<Mutex<LinkConnectionManager>> = self.link_manager.clone();

        // Start connection process in a task
        task_pool.spawn(async move {
            Connection::run(ev, lm).await;
        });

    }

    /// Run the connection
    ///
    async fn run(ev: Arc<Mutex<rumqttc::EventLoop>>, lm: Arc<Mutex<LinkConnectionManager>>) {

        loop {
            while let Ok(notification) = ev.lock().await.poll().await {
                println!("Received = {:?}", notification);
            }
            tracing::warn!("Broker disconnected, trying to reconnect");
        }



        // let mut futures = FuturesUnordered::new();
        // for link in self.links.iter_mut() {
        //     futures.push(link.recv());
        // }

        // If no interface linked... the connection is pointless

        // tokio::select! {
            
        //     notification = self.eventloop.as_mut().unwrap().poll() => {
        //         println!("EVEEEN !!!!!!!!!!!!!!");
                // match notification {
                    // rumqttc::Event::Incoming(incoming) => {
                    //     println!("Received = {:?}", notification);
                    // },
                    // _ => {
                    //     println!("Received = {:?}", notification);
                    // }
                // }
            // },


            // // just create a separate aobject to handle the links in a task
            // data = futures.next() => {
            //     println!("pok");
            //     let dd = data.unwrap().unwrap();
            //     println!("{}", dd.0.filters.len());
            //     println!("{}", dd.1);
            // },

        // }
        // println!("{}", data.unwrap());
        // link.tx.send("hello".to_string()).await;

        

        // match notification {

            // rumqttc::Event::Incoming(incoming) => {
        //     Event => {
        //         println!("Received = {:?}", notification);
        //     },
        //     _ => {
        //         println!("Received = {:?}", notification);
        //     }
        // }


            
        


    }


    /// Get the link manager, to share it with the devices
    /// 
    pub fn clone_link_manager(&self) -> SafeLinkConnectionManager {
        return self.link_manager.clone();
    }


    // /// Create a new link
    // /// 
    // pub async fn create_link(&mut self) -> Result<LinkInterfaceHandle, String> {




    

    //     match self.client.as_mut() {
    //         // The division was valid
    //         Some(client) => {
    //             let liikn = LinkInterfaceHandle::new(self.client.as_mut().unwrap(), tx_sub, rx);
    //             return Ok(liikn);
    //         }
    //         // The division was invalid
    //         None    => {
    //             println!("client not here !!!!");
    //             return Err("client not here !!!!".to_string());
    //         }
    //     }

        

        
    // }

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
    connections: HashMap<String, SafeConnection>
}

impl Manager {

    /// Create a new manager
    ///
    pub fn new(platform_name: &str) -> Manager {
        return Manager {
            platform_name: platform_name.to_string(),
            connections: HashMap::new()
        }
    }

    /// Create a new inactive connection
    ///
    pub async fn create_connection<S: Into<String>, T: Into<String>>(&mut self, name: S, host: T, port: u16) {
        // Get name with the correct type
        let name_string = name.into();

        // Create connection ID
        let id = format!("{}::{}", self.platform_name, name_string);

        // Info log
        tracing::info!("Create connection '{:?}'", id);

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
    pub async fn start_connection(&mut self, name: &str, task_pool: &mut tokio::task::JoinSet<()>) {
        // Get the connection clone for the task
        let conn = self.connections.get(name).unwrap().clone();

        // Start the connection
        conn.lock().await.start(task_pool).await;
    }

    /// Get a connection
    /// 
    pub fn get_connection(&mut self, name: &str) -> SafeConnection {
        return self.connections.get(name).unwrap().clone();
    }

}

