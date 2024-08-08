use std::sync::Weak;

use tokio::sync::Mutex;

use crate::{BidirMsgAtt, MessageClient, MessageCodec, MessageDispatcher};

use super::{cmd_only_msg_att::CmdOnlyMsgAtt, wo_msg_att::WoMessageAttribute};

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
    pub fn with_ro_access(self) -> CmdOnlyMsgAttBuilder {
        CmdOnlyMsgAttBuilder { base: self.base }
    }

    pub fn with_rw_access(self) -> BidirMsgAttBuilder {
        BidirMsgAttBuilder { base: self.base }
    }

    pub fn with_wo_access(self) -> WoMessageAttributeBuilder {
        WoMessageAttributeBuilder { base: self.base }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for Ro Attribute
pub struct CmdOnlyMsgAttBuilder {
    base: AttributeBuilder,
}

impl CmdOnlyMsgAttBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> CmdOnlyMsgAtt<TYPE> {
        CmdOnlyMsgAtt::from(self.base).init().await.unwrap()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for Rw Attribute
pub struct BidirMsgAttBuilder {
    base: AttributeBuilder,
}

impl BidirMsgAttBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> BidirMsgAtt<TYPE> {
        BidirMsgAtt::from(self.base)
            .init()
            .await
            .unwrap()
            .init()
            .await
            .unwrap()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for Wo Attribute
pub struct WoMessageAttributeBuilder {
    base: AttributeBuilder,
}

impl WoMessageAttributeBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> WoMessageAttribute<TYPE> {
        WoMessageAttribute::from(self.base)
    }
}
