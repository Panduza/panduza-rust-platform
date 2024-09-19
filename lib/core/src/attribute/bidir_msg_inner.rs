use super::CmdOnlyMsgAttInner;
use crate::AttributeBuilder;
use crate::Error;
use crate::MessageCodec;
use crate::MessageHandler;
use async_trait::async_trait;
use bytes::Bytes;
use rumqttc::QoS;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::Notify;

///
/// Read Only Inner implementation of the message attribute
/// This inner implementation allow the public part to be cloneable easly
///
pub struct BidirMsgAttInner<TYPE: MessageCodec> {
    ///
    /// Bidir is based on CmdOnly for message reception
    ///
    pub base: CmdOnlyMsgAttInner<TYPE>,

    ///
    /// The topic for 'att' topic to send data to user
    ///
    topic_att: String,

    ///
    /// Requested value of the attribute (set by the user)
    ///
    requested_value: Option<TYPE>,
}

impl<TYPE: MessageCodec> BidirMsgAttInner<TYPE> {
    ///
    /// Initialize the attribute
    ///
    pub async fn init(&self, attribute: Arc<Mutex<dyn MessageHandler>>) -> Result<(), Error> {
        self.base.init(attribute).await
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn pop_cmd(&mut self) -> Option<TYPE> {
        let next = self.base.in_queue.pop();
        if next.is_some() {
            self.base.last_popped_value = next.clone();
        }
        return next;
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn get_last_cmd(&self) -> Option<TYPE> {
        return self.base.last_popped_value.clone();
    }

    ///
    /// Clone the change notifier
    ///
    pub fn in_notifier(&self) -> Arc<Notify> {
        self.base.in_notifier()
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
                self.publish(requested_value.into_message_payload()?)
                    .await
                    .unwrap();
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
impl<TYPE: MessageCodec> From<AttributeBuilder> for BidirMsgAttInner<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        let topic_att = format!("{}/att", builder.topic.as_ref().unwrap());
        BidirMsgAttInner {
            base: CmdOnlyMsgAttInner::from(builder),
            topic_att: topic_att,
            requested_value: None,
        }
    }
}

/// Allow mutation into Arc pointer
impl<TYPE: MessageCodec> Into<Arc<Mutex<BidirMsgAttInner<TYPE>>>> for BidirMsgAttInner<TYPE> {
    fn into(self) -> Arc<Mutex<BidirMsgAttInner<TYPE>>> {
        Arc::new(Mutex::new(self))
    }
}

///
///
///
#[async_trait]
impl<TYPE: MessageCodec> MessageHandler for BidirMsgAttInner<TYPE> {
    async fn on_message(&mut self, data: &Bytes) -> Result<(), Error> {
        let in_value = TYPE::from_message_payload(data)?;
        self.base.in_queue.push(in_value);
        self.base.in_notifier.notify_waiters();
        Ok(())
    }
}
