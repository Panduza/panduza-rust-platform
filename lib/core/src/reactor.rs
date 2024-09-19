pub mod settings;
use async_trait::async_trait;
use bytes::Bytes;
use futures::FutureExt;
pub use settings::ReactorSettings;
mod message_engine;
use message_engine::MessageEngine;
pub mod message_dispatcher;
use crate::info::devices::ThreadSafeInfoDynamicDeviceStatus;
use crate::MessageClient;
use crate::{AttributeBuilder, Error, MessageDispatcher, MessageHandler, TaskResult, TaskSender};
use chrono::prelude::*;
use rumqttc::AsyncClient;
use rumqttc::{MqttOptions, QoS};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

struct PzaScanMessageHandler {
    message_client: MessageClient,
}

#[async_trait]
impl MessageHandler for PzaScanMessageHandler {
    async fn on_message(&mut self, _incomming_data: &Bytes) -> Result<(), Error> {
        let hostname = hostname::get().unwrap().to_string_lossy().to_string();
        let now = Utc::now();

        self.message_client
            .publish(
                format!("pza/{}", hostname),
                QoS::AtLeastOnce,
                false,
                format!("{}", now.timestamp_millis()),
            )
            .await
            .map_err(|e| Error::MessageAttributePublishError(e.to_string()))?;
        Ok(())
    }
}

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

    scan_handler: Option<Arc<Mutex<PzaScanMessageHandler>>>,
}

impl Reactor {
    /// Create a new Reactor
    ///
    /// # Arguments
    ///
    /// * `core` - The core of the reactor
    ///
    pub fn new(_settings: ReactorSettings) -> Self {
        // let data = ;

        // Server hostname
        let hostname = hostname::get().unwrap().to_string_lossy().to_string();

        Reactor {
            root_topic: format!("pza/{}", hostname),
            message_client: None,
            message_dispatcher: Arc::new(Mutex::new(MessageDispatcher::new())),
            scan_handler: None,
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

        self.message_client = Some(client.clone());

        self.scan_handler = Some(Arc::new(Mutex::new(PzaScanMessageHandler {
            message_client: client.clone(),
        })));

        let h = self.scan_handler.as_ref().unwrap().clone();
        let dispatcher = self.message_dispatcher.clone();
        let mut message_engine = MessageEngine::new(self.message_dispatcher.clone(), event_loop);
        main_task_sender.spawn(
            async move {
                dispatcher
                    .lock()
                    .await
                    .register_message_attribute("pza".to_string(), h);
                client.subscribe("pza", QoS::AtLeastOnce).await.unwrap();
                message_engine.run().await;
                println!("ReactorCore is not runiing !!!!!!!!!!!!!!!!!!!!!!");
                Ok(())
            }
            .boxed(),
        )
    }

    pub fn create_new_attribute(
        &self,
        device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
    ) -> AttributeBuilder {
        AttributeBuilder::new(
            self.message_client.as_ref().unwrap().clone(),
            Arc::downgrade(&self.message_dispatcher),
            device_dyn_info,
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
