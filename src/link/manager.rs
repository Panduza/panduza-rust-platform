// use std::sync::atomic::{AtomicU32, Ordering};

use std::collections::LinkedList;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use rumqttc::AsyncClient;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use crate::subscription::{self};

use super::{ConnectionHandle, InterfaceHandle};

/// Link connection manager
/// 
pub struct Manager {
    /// Mqtt client
    client: AsyncClient,

    /// List of links
    links: Vec<ConnectionHandle>,

    
    new_links: Mutex<LinkedList<ConnectionHandle>>

}

impl Manager {

    /// Create a new link connection manager
    pub fn new(client: AsyncClient) -> Manager {
        return Manager {
            client: client,
            links: Vec::new(),
            new_links: Mutex::new( LinkedList::new() ),
        }
    }

    /// Create a new link
    ///
    pub async fn request_link(&mut self, requests: Vec<subscription::Request>) -> Result<InterfaceHandle, String> {

        // Trace
        tracing::trace!("Link Manager Request link with {} subscriptions", requests.len());
        for request in requests.iter() {
            tracing::trace!("  - {}", request.topic());
        }

        // Create the channel
        let (tx, rx) =
            mpsc::channel::<subscription::Message>(64);


        let mut filters = LinkedList::new();

        for request in requests {

            self.client.subscribe(request.topic(), rumqttc::QoS::AtLeastOnce).await.unwrap();

            let filter = subscription::Filter::new(request);

            filters.push_back(filter);

        }


        // 
        self.new_links.lock().await.push_back(
            ConnectionHandle::new(tx, filters)
        );

        
        return Ok(InterfaceHandle::new(self.client.clone(), rx));
    }

    /// Send to all interfaces
    /// 
    pub async fn send_to_all(&mut self, message: subscription::Message) {
        for link in self.links.iter_mut() {
            let r = link.tx().send(message.clone()).await;
            if r.is_err() {
                println!("Error sending message to interface {}",
                    r.err().unwrap());
            }
        }
    }


    /// Process new links with saved information about connection status
    /// 
    pub async fn process_new_links(&mut self, is_connected: &AtomicBool) {
        let mut new_links = self.new_links.lock().await;
        while let Some(link) = new_links.pop_front() {
            
            link.tx().send(subscription::Message::new_connection_status(is_connected.load(Ordering::Acquire)  )).await.unwrap();


            self.links.push(link);
        }
    }

    
    // pub fn links_as_mut(&mut self) -> &mut LinkedList<ConnectionHandle> {
    //     return &mut self.links;
    // }

}


