use std::sync::Weak;

use tokio::sync::Mutex;

use crate::{
    MessageClient, MessageCodec, MessageDispatcher, RoMessageAttribute, RwMessageAttribute,
};

/// Object that allow to build an generic attribute
///
pub struct AttributeBuilder {
    /// The mqtt client
    pub message_client: MessageClient,

    /// The Object that allow the reactor to dispatch
    /// incoming messages on attributes
    pub message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    /// Topic of the attribute
    pub topic: Option<String>,
}

impl AttributeBuilder {
    /// Create a new builder
    pub fn new(
        message_client: MessageClient,
        message_dispatcher: Weak<Mutex<MessageDispatcher>>,
    ) -> AttributeBuilder {
        AttributeBuilder {
            message_client,
            message_dispatcher,
            topic: None,
        }
    }
    /// Attach a topic
    pub fn with_topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = Some(topic.into());
        self
    }

    pub fn message(self) -> MessageAttributeBuilder {
        MessageAttributeBuilder { base: self }
    }
    pub fn stream(self) {
        todo!()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct MessageAttributeBuilder {
    base: AttributeBuilder,
}

impl MessageAttributeBuilder {
    pub fn with_ro_access(self) -> RoMessageAttributeBuilder {
        RoMessageAttributeBuilder { base: self.base }
    }

    pub fn with_rw_access(self) -> RwMessageAttributeBuilder {
        RwMessageAttributeBuilder { base: self.base }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for Ro Attribute
pub struct RoMessageAttributeBuilder {
    base: AttributeBuilder,
}

impl RoMessageAttributeBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> RoMessageAttribute<TYPE> {
        RoMessageAttribute::from(self.base).init().await.unwrap()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for Rw Attribute
pub struct RwMessageAttributeBuilder {
    base: AttributeBuilder,
}

impl RwMessageAttributeBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> RwMessageAttribute<TYPE> {
        RwMessageAttribute::from(self.base)
            .init()
            .await
            .unwrap()
            .init()
            .await
            .unwrap()
    }
}
