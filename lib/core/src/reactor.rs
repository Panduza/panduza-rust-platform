pub mod settings;
pub use settings::ReactorSettings;

//
mod message_engine;
use message_engine::MessageEngine;
use tokio::task::JoinHandle;

//
pub mod message_dispatcher;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AttributeBuilder, MessageDispatcher};

use rumqttc::AsyncClient;
use rumqttc::{Client, MqttOptions, QoS};

use std::time::Duration;

use crate::MessageClient;

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone)]
pub struct Reactor {
    /// The mqtt client
    message_client: Option<MessageClient>,

    ///
    message_dispatcher: Arc<Mutex<MessageDispatcher>>,
}

impl Reactor {
    /// Create a new Reactor
    ///
    /// # Arguments
    ///
    /// * `core` - The core of the reactor
    ///
    pub fn new(settings: ReactorSettings) -> Self {
        // let data = ;

        Reactor {
            message_client: None,
            message_dispatcher: Arc::new(Mutex::new(MessageDispatcher::new())),
        }
    }

    pub fn start(&mut self) -> JoinHandle<()> {
        println!("ReactorCore is running");
        let mut mqttoptions = MqttOptions::new("rumqtt-sync", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        self.message_client = Some(client);

        let mut message_engine = MessageEngine::new(self.message_dispatcher.clone(), event_loop);
        tokio::spawn(async move {
            message_engine.run().await;
            println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
        })
    }

    pub fn create_new_attribute(&self) -> AttributeBuilder {
        AttributeBuilder::new(
            self.message_client.as_ref().unwrap().clone(),
            Arc::downgrade(&self.message_dispatcher),
        )
    }

    // pub async fn scan_platforms(&self) {
    //     println!("publish");
    //     self.message_client
    //         .publish("pza", QoS::AtLeastOnce, true, "pok")
    //         .await
    //         .unwrap();
    // }
}
