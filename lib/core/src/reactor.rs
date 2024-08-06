pub mod settings;
use futures::FutureExt;
pub use settings::ReactorSettings;

//
mod message_engine;
use message_engine::MessageEngine;
use tokio::task::JoinHandle;

//
pub mod message_dispatcher;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AttributeBuilder, MessageDispatcher, TaskResult, TaskSender};

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
    /// Root topic (namespace/pza)
    root_topic: String,

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

        // Server hostname
        let hostname = hostname::get().unwrap().to_string_lossy().to_string();

        Reactor {
            root_topic: format!("pza/{}", hostname),
            message_client: None,
            message_dispatcher: Arc::new(Mutex::new(MessageDispatcher::new())),
        }
    }

    pub fn root_topic(&self) -> String {
        self.root_topic.clone()
    }

    pub fn start(
        &mut self,
        mut main_task_sender: TaskSender<TaskResult>,
    ) -> Result<(), crate::Error> {
        println!("ReactorCore is running");
        let mut mqttoptions = MqttOptions::new("rumqtt-sync", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(3));

        let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

        self.message_client = Some(client);

        let mut message_engine = MessageEngine::new(self.message_dispatcher.clone(), event_loop);
        main_task_sender.spawn(
            async move {
                message_engine.run().await;
                println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
                Ok(())
            }
            .boxed(),
        )
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
