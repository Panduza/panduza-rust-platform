use rumqttc::QoS;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;

use bytes::Bytes;

use async_trait::async_trait;

use tokio::sync::Notify;

use crate::AttributeBuilder;
use crate::Error;
use crate::MessageCodec;
use crate::MessageHandler;

use super::RoMessageAttributeInner;

/// Read Only Inner implementation of the message attribute
/// This inner implementation allow the public part to be cloneable easly
pub struct RwMessageAttributeInner<TYPE: MessageCodec> {
    /// Rw is based on Ro
    pub base: RoMessageAttributeInner<TYPE>,

    /// The topic for commands
    topic_att: String,

    /// Requested value of the attribute (set by the user)
    requested_value: Option<TYPE>,
}

impl<TYPE: MessageCodec> RwMessageAttributeInner<TYPE> {
    /// Initialize the attribute
    pub async fn init(&self, attribute: Arc<Mutex<dyn MessageHandler>>) -> Result<(), Error> {
        self.base.init(attribute).await
    }

    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn get(&self) -> Option<TYPE> {
        return self.base.get();
    }

    /// Set the value of the attribute
    ///
    pub async fn set(&mut self, new_value: TYPE) -> Result<(), Error> {
        // // Do not go further if the value is already set
        // if let Some(current_value) = self.value {
        //     if current_value == new_value {
        //         return Ok(());
        //     }
        // }

        // Set the requested value and publish the request
        self.requested_value = Some(new_value);
        match self.requested_value.clone() {
            Some(requested_value) => {
                self.publish(requested_value.into()).await.unwrap();
            }
            None => {
                return Err(Error::Wtf);
            }
        }

        Ok(())
    }

    /// Publish a command
    ///
    pub async fn publish<V>(&self, value: V) -> Result<(), Error>
    where
        V: Into<Vec<u8>>,
    {
        self.base
            .message_client
            .publish(&self.topic_att, QoS::AtMostOnce, true, value)
            .await
            .map_err(|e| Error::MessageAttributePublishError(e.to_string()))
    }
}

/// Allow creation from the builder
impl<TYPE: MessageCodec> From<AttributeBuilder> for RwMessageAttributeInner<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        let topic_att = format!("{}/att", builder.topic.as_ref().unwrap());
        RwMessageAttributeInner {
            base: RoMessageAttributeInner::from(builder),
            topic_att: topic_att,
            requested_value: None,
        }
    }
}

/// Allow mutation into Arc pointer
impl<TYPE: MessageCodec> Into<Arc<Mutex<RwMessageAttributeInner<TYPE>>>>
    for RwMessageAttributeInner<TYPE>
{
    fn into(self) -> Arc<Mutex<RwMessageAttributeInner<TYPE>>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl<TYPE: MessageCodec> MessageHandler for RwMessageAttributeInner<TYPE> {
    async fn on_message(&mut self, data: &Bytes) {
        let new_value = TYPE::from(data.to_vec());
        self.base.value = Some(new_value);
        self.base.change_notifier.notify_waiters();
    }
}
