
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
use std::sync::Mutex as StdMutex;


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

/// Allow to connect a single connection => single interface
/// one direction only
/// dispatch incomming message
/// 
/// (owned by the connection)
struct RxLink {

    // RxConnectionHandle
    // for incoming messages
    tx: mpsc::Sender<InputMessage>, // provides the tx to the connection
 

 
    // create new rx conn handle(filter, topic)
        // subscribe to topic
        // save filter
        // return handle ?


    filters: LinkedList<LinkFilterEntry>, // regex + slot id ?

}









pub struct LinkInterfaceHandle
{
    asyncClient: AsyncClient,
    rx: mpsc::Receiver<InputMessage>, // rx for the interface (it owns the Link)
    pub topic_subscriber_tx: mpsc::Sender<String>, // provides the tx to the connection
}

struct LinkConnectionHandle
{
    tx: mpsc::Sender<InputMessage>, // provides the tx to the connection
    filters: LinkedList<Regex>,
    pub topic_subscriber_rx: mpsc::Receiver<String>, // provides the tx to the connection
}

pub type MutexedConnection = Arc<Mutex<Connection>>;





impl LinkInterfaceHandle {

    fn new(client: &mut AsyncClient, tx_sub: mpsc::Sender<String>, rx_chan: mpsc::Receiver<InputMessage>) -> LinkInterfaceHandle {
        return LinkInterfaceHandle {
            asyncClient: client.clone(),
            topic_subscriber_tx: tx_sub,
            rx: rx_chan
        }
    }

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

    // //
    // links: Arc<Mutex<LinkedList<LinkConnectionHandle>>>
}


impl Connection {


    pub fn new<S: Into<String>, T: Into<String>>(id: S, host: T, port: u16) -> Connection {
/*

        let (mut client, mut eventloop) = AsyncClient::new(options.clone(), 10);

        // 
        // broadcast: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.



        // ConnectionLink
        // one channel
        // N filters
        // fn subscribe


        // let (tx, mut rx) = mpsc::channel::<String>(32);

        

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
            eventloop: Option::None
        }
    }


    pub async fn connect(&mut self) {
        let (mut client, mut eventloop) = AsyncClient::new(self.mqtt_options.clone(), 10);

        // let ev = Box::new();
        self.eventloop = Some(eventloop);
        
    }

    pub async fn run(&mut self) {

        while let Ok(notification) = self.eventloop.as_mut().unwrap().poll().await {
            println!("Received = {:?}", notification);
        }


    }

    // start

    // pub fn stop(&self) {
    //     self.task_abort.abort();
    // }

            // let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);


    // pub async fn gen_linkkkk(&mut self) -> LinkInterfaceHandle {


    //     let (tx, mut rx) = mpsc::channel::<InputMessage>(32);

    //     let (tx_sub, mut rx_sub) = mpsc::channel::<String>(32);


        // self.links.lock().await.push_back(
        //     LinkConnectionHandle::new(tx, rx_sub)
        // );
        
        // .unwrap().push_back(
        //     LinkConnectionHandle::new(tx, rx_sub)
        // );

        // let liikn = LinkInterfaceHandle::new(&mut self.client, tx_sub, rx);

        // return liikn;
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

    /// Task pool
    task_pool: tokio::task::JoinSet<()>,

    /// Map of managed connections
    connections: HashMap<String, MutexedConnection>
}

impl Manager {

    /// Create a new manager
    ///
    pub fn new(platform_name: &str) -> Manager {
        return Manager {
            platform_name: platform_name.to_string(),
            task_pool: tokio::task::JoinSet::new(),
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
        let conn = self.connections.get(name).unwrap().clone();
        task_pool.spawn(async move {
            conn.lock().await.connect().await;
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
