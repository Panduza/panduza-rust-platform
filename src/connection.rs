
// use tokio::sync::mpsc;
use tokio::{sync::mpsc, time::{sleep, Duration}};
use rumqttc::{MqttOptions, AsyncClient, QoS};
use tracing::Event;

use std::{collections::HashMap};
use tokio::task::AbortHandle;

use regex::Regex;

use std::collections::LinkedList;

use bytes::{Buf, Bytes};

use tokio::sync::Mutex;
use std::sync::Arc;
use futures::stream::{FuturesUnordered, StreamExt};



#[derive(Clone, PartialEq, Eq)]
pub struct InputMessage {
    // slot id ?
    topic: String,
    pub payload: Bytes
}





struct LinkFilterEntry {
    slot_id: u32,
    filter: Regex
}













// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

pub struct LinkInterfaceHandle
{
    asyncClient: AsyncClient,
    rx: mpsc::Receiver<InputMessage>, // rx for the interface (it owns the Link)
    pub topic_subscriber_tx: mpsc::Sender<String>, // provides the tx to the connection
}


impl LinkInterfaceHandle {

    fn new(client: &mut AsyncClient, tx_sub: mpsc::Sender<String>, rx_chan: mpsc::Receiver<InputMessage>) -> LinkInterfaceHandle {
        return LinkInterfaceHandle {
            asyncClient: client.clone(),
            topic_subscriber_tx: tx_sub,
            rx: rx_chan
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
    tx: mpsc::Sender<InputMessage>, // provides the tx to the connection
    filters: LinkedList<Regex>,

    topic_subscriber_rx: mpsc::Receiver<String>, // provides the tx to the connection
}


impl LinkConnectionHandle {
    fn new(tx: mpsc::Sender<InputMessage>, rx: mpsc::Receiver<String>) -> LinkConnectionHandle {
        return LinkConnectionHandle {
            tx: tx,
            filters: LinkedList::new(),
            topic_subscriber_rx: rx
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
    /// Mqtt options
    mqtt_options: MqttOptions,

    /// Mqtt client
    client: Option<AsyncClient>,

    /// Event loop
    eventloop: Option<rumqttc::EventLoop>,

    /// Links
    links: LinkedList<LinkConnectionHandle>
}
pub type MutexedConnection = Arc<Mutex<Connection>>;


impl Connection {


    pub fn new<S: Into<String>, T: Into<String>>(id: S, host: T, port: u16) -> Connection {
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
                tracing::warn!("Broker disconnected, trying to reconnect");

            }

        });
 */

        // Set default options
        let mut mqtt_options = MqttOptions::new(id, host, port);
        mqtt_options.set_keep_alive(Duration::from_secs(5));

        // Create Object
        return Connection {
            mqtt_options: mqtt_options,
            client: Option::None,
            eventloop: Option::None,
            links: LinkedList::new()
        }
    }


    /// Start the connection
    /// 
    pub async fn connect(&mut self) {
        let (client, eventloop) = AsyncClient::new(self.mqtt_options.clone(), 10);

        self.client = Some(client);
        self.eventloop = Some(eventloop);

    }

    /// Run the connection
    /// 
    pub async fn run(&mut self) {


        if self.links.len() == 0 {
            println!("No links !!");
            return;
        }

        let mut futures = FuturesUnordered::new();
        for link in self.links.iter_mut() {
            futures.push(link.topic_subscriber_rx.recv());
        }

        // If no interface linked... the connection is pointless

        tokio::select! {
            
            notification = self.eventloop.as_mut().unwrap().poll() => {
                println!("EVEEEN !!!!!!!!!!!!!!");
                // match notification {
                    // rumqttc::Event::Incoming(incoming) => {
                    //     println!("Received = {:?}", notification);
                    // },
                    // _ => {
                    //     println!("Received = {:?}", notification);
                    // }
                // }
            },
            data = futures.next() => {
                println!("pok");
                println!("{}", data.unwrap().unwrap());
            },

        }
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

    /// Create a new link
    /// 
    pub async fn create_link(&mut self) -> Result<LinkInterfaceHandle, String> {


        let (tx, mut rx) = mpsc::channel::<InputMessage>(32);

        let (tx_sub, mut rx_sub) = mpsc::channel::<String>(32);


        self.links.push_back(
            LinkConnectionHandle::new(tx, rx_sub)
        );
    

        match self.client.as_mut() {
            // The division was valid
            Some(client) => {
                let liikn = LinkInterfaceHandle::new(self.client.as_mut().unwrap(), tx_sub, rx);
                return Ok(liikn);
            }
            // The division was invalid
            None    => {
                println!("client not here !!!!");
                return Err("client not here !!!!".to_string());
            }
        }

        

        
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
    connections: HashMap<String, MutexedConnection>
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

        // Create connection Object
        self.connections.insert(name_string,
            Arc::new(Mutex::new(
                Connection::new(id, host, port))
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

        // Create the connection before starting it in a task
        // Else it can cause sync error when attaching this connection to an interface
        // Because link creation need the connection to be done
        conn.lock().await.connect().await;

        // Start connection process in a task
        task_pool.spawn(async move {
            loop {
                conn.lock().await.run().await;
            }
        });
    }


    pub fn get_connection(&mut self, name: &str) -> MutexedConnection {
        return self.connections.get(name).unwrap().clone();
    }



}




// MqttPublisher
    // publish with pza helpers

// MqttSubscriber
    // subscribe and callback filters
