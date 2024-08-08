use rumqttc::QoS;
use std::borrow::Borrow;
use std::io::Read;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;

use bytes::Bytes;

use async_trait::async_trait;

use tokio::sync::Notify;

use crate::AttributeBuilder;
use crate::Error;
use crate::MessageClient;
use crate::MessageCodec;
use crate::MessageDispatcher;
use crate::MessageHandler;

/// Read Only Inner implementation of the message attribute
/// This inner implementation allow the public part to be cloneable easly
pub struct RoMessageAttributeInner<TYPE: MessageCodec> {
    /// Reactor message dispatcher
    /// (to attach this attribute to the incoming messages)
    message_dispatcher: Weak<Mutex<MessageDispatcher>>,
    /// The message client (MQTT)
    pub message_client: MessageClient,

    /// The topic of the attribute
    topic: String,

    /// Current value of the attribute
    pub value: Option<TYPE>,

    ///
    pub change_notifier: Arc<Notify>,
}

impl<TYPE: MessageCodec> RoMessageAttributeInner<TYPE> {
    /// Initialize the attribute
    /// Register the attribute on the message dispatcher then subscribe to att topic
    pub async fn init(&self, attribute: Arc<Mutex<dyn MessageHandler>>) -> Result<(), Error> {
        self.register(attribute).await?;
        self.subscribe().await
    }

    /// Get the value of the attribute
    /// If None, the first value is not yet received
    pub fn get(&self) -> Option<TYPE> {
        return self.value.clone();
    }

    /// Subscribe to the topic
    ///
    pub async fn subscribe(&self) -> Result<(), Error> {
        // no need to store the att topic
        let topic_att = format!("{}/cmd", self.topic);
        self.message_client
            .subscribe(topic_att, QoS::AtMostOnce)
            .await
            .map_err(|e| Error::MessageAttributeSubscribeError(e.to_string()))
    }

    /// Register the attribute to the reactor
    ///
    pub async fn register(&self, attribute: Arc<Mutex<dyn MessageHandler>>) -> Result<(), Error> {
        // no need to store the att topic
        let topic_att = format!("{}/cmd", self.topic);
        self.message_dispatcher
            .upgrade()
            .ok_or(Error::InternalPointerUpgrade)?
            .lock()
            .await
            // .map_err(|e| Error::InternalMutex(e.to_string()))?
            .register_message_attribute(topic_att, attribute);
        Ok(())
    }

    /// Clone the change notifier
    ///
    pub fn clone_change_notifier(&self) -> Arc<Notify> {
        self.change_notifier.clone()
    }
}

/// Allow creation from the builder
impl<TYPE: MessageCodec> From<AttributeBuilder> for RoMessageAttributeInner<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        RoMessageAttributeInner {
            message_dispatcher: builder.message_dispatcher,
            message_client: builder.message_client,
            topic: builder.topic.as_ref().unwrap().clone(),
            value: None,
            change_notifier: Arc::new(Notify::new()),
        }
    }
}

/// Allow mutation into Arc pointer
impl<TYPE: MessageCodec> Into<Arc<Mutex<RoMessageAttributeInner<TYPE>>>>
    for RoMessageAttributeInner<TYPE>
{
    fn into(self) -> Arc<Mutex<RoMessageAttributeInner<TYPE>>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl<TYPE: MessageCodec> MessageHandler for RoMessageAttributeInner<TYPE> {
    async fn on_message(&mut self, data: &Bytes) {
        let p = String::from_utf8(data.to_vec()).unwrap();
        // let string: String =
        // let p = data.as_ref().to_string();
        let d: TYPE = serde_json::from_str(p.as_str()).unwrap();
        // let new_value = TYPE::serialize  ::from(data.to_vec());
        // self.value = Some(new_value);
        self.change_notifier.notify_waiters();
    }
}
