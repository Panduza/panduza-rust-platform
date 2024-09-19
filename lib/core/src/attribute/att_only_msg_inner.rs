use crate::AttributeBuilder;
use crate::Error;
use crate::MessageClient;
use crate::MessageCodec;
use rumqttc::QoS;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Read Only Inner implementation of the message attribute
/// This inner implementation allow the public part to be cloneable easly
pub struct AttOnlyMsgAttInner<TYPE: MessageCodec> {
    /// The message client (MQTT)
    pub message_client: MessageClient,

    /// The topic of the attribute
    topic: String,

    /// The topic
    topic_att: String,

    /// Requested value of the attribute (set by the user)
    requested_value: Option<TYPE>,
}

impl<TYPE: MessageCodec> AttOnlyMsgAttInner<TYPE> {
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
        self.message_client
            .publish(&self.topic_att, QoS::AtMostOnce, true, value)
            .await
            .map_err(|e| Error::MessageAttributePublishError(e.to_string()))
    }
}

/// Allow creation from the builder
impl<TYPE: MessageCodec> From<AttributeBuilder> for AttOnlyMsgAttInner<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        let topic_att = format!("{}/att", builder.topic.as_ref().unwrap());
        AttOnlyMsgAttInner {
            message_client: builder.message_client,
            topic: builder.topic.as_ref().unwrap().clone(),
            topic_att: topic_att,
            requested_value: None,
        }
    }
}

/// Allow mutation into Arc pointer
impl<TYPE: MessageCodec> Into<Arc<Mutex<AttOnlyMsgAttInner<TYPE>>>> for AttOnlyMsgAttInner<TYPE> {
    fn into(self) -> Arc<Mutex<AttOnlyMsgAttInner<TYPE>>> {
        Arc::new(Mutex::new(self))
    }
}
