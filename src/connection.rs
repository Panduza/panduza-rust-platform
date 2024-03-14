
// use tokio::sync::mpsc;
use tokio::{sync::mpsc, time::{sleep, Duration}};
use rumqttc::{MqttOptions, AsyncClient, QoS};

use std::{collections::HashMap};
use tokio::task::AbortHandle;

use regex::Regex;

use std::collections::LinkedList;

use bytes::{Buf, Bytes};


#[derive(Clone, PartialEq, Eq)]
struct InputMessage {
    // slot id ?
    topic: String,
    pub payload: Bytes
}



#[derive(Clone)]
struct RxConnectionHandle
{
    tx: mpsc::Sender<InputMessage>, // provides the tx to the connection
    filters: LinkedList<Regex>,
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
 

    topic_subscriber_rx: mpsc::Receiver<String>, // provides the tx to the connection
 
    // create new rx conn handle(filter, topic)
        // subscribe to topic
        // save filter
        // return handle ?


    filters: LinkedList<LinkFilterEntry>, // regex + slot id ?

}




///
/// (owned by the interface)
/// allow to publish data
/// subscribe
/// 
pub struct toto {
    asyncClient: AsyncClient,

    
    topic_subscriber_tx: mpsc::Sender<String>, 
 
    // RxInterfaceHandle
    rx: mpsc::Receiver<InputMessage>, // rx for the interface (it owns the Link)
}

impl toto {

    fn new(client: &mut AsyncClient, tx_sub: mpsc::Sender<String>, rx_chan: mpsc::Receiver<InputMessage>) -> toto {
        return toto {
            asyncClient: client.clone(),
            topic_subscriber_tx: tx_sub,
            rx: rx_chan
        }
    }

}



/// Object to manage multiple one connection
pub struct Runner {
    mqtt_options: MqttOptions,
    task_abort: AbortHandle,


    client: AsyncClient

    // List de RxLink


}

impl Runner {


    pub fn new(task_pool: &mut tokio::task::JoinSet<()>, host: String, port: u16) -> Runner {
        let mut options = MqttOptions::new("TEST_1", host, port);
        options.set_keep_alive(Duration::from_secs(5));

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


        let abort = task_pool.spawn(async move {

            // client.subscribe("pza", QoS::AtMostOnce).await.unwrap();

            loop {
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

        return Runner {
            mqtt_options: options,
            task_abort: abort,
            client: client
        }
    }


    // start

    pub fn stop(&self) {
        self.task_abort.abort();
    }

            // let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);


    pub fn gen_linkkkk(&mut self) -> toto {


        let (tx, mut rx) = mpsc::channel::<InputMessage>(32);

        let (tx_sub, mut rx_sub) = mpsc::channel::<String>(32);

        let liikn = toto::new(&mut self.client, tx_sub, rx);

        
        return liikn;
    }

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


    pub fn create_connection(&mut self, task_pool: &mut tokio::task::JoinSet<()>, name: String, host: String, port: u16) {
        self.connections.insert(name, Runner::new(task_pool, host, port));
    }
    

    pub fn get_connection(&mut self, name: &str) -> Option<&mut Runner> {
        return self.connections.get_mut(name);
    }

}




// MqttPublisher
    // publish with pza helpers

// MqttSubscriber
    // subscribe and callback filters
