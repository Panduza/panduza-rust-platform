use crate::info::devices::AttributeMode;
use crate::info::devices::StructuralElement;
use std::sync::Weak;

use tokio::sync::Mutex;

use crate::{
    info::devices::{ElementAttribute, ThreadSafeInfoDynamicDeviceStatus},
    BidirMsgAtt, MessageClient, MessageCodec, MessageDispatcher,
};

use super::{att_only_msg_att::AttOnlyMsgAtt, cmd_only_msg_att::CmdOnlyMsgAtt};

/// Object that allow to build an generic attribute
///
pub struct AttributeBuilder {
    /// The mqtt client
    pub message_client: MessageClient,

    /// The Object that allow the reactor to dispatch
    /// incoming messages on attributes
    pub message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    ///
    pub device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,

    /// Topic of the attribute
    pub topic: Option<String>,
}

impl AttributeBuilder {
    /// Create a new builder
    pub fn new(
        message_client: MessageClient,
        message_dispatcher: Weak<Mutex<MessageDispatcher>>,
        device_dyn_info: Option<ThreadSafeInfoDynamicDeviceStatus>,
    ) -> AttributeBuilder {
        AttributeBuilder {
            message_client,
            message_dispatcher,
            device_dyn_info,
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
    pub fn with_cmd_only_access(self) -> CmdOnlyMsgAttBuilder {
        CmdOnlyMsgAttBuilder { base: self.base }
    }

    pub fn with_bidir_access(self) -> BidirMsgAttBuilder {
        BidirMsgAttBuilder { base: self.base }
    }

    pub fn with_att_only_access(self) -> AttOnlyMsgBuilder {
        AttOnlyMsgBuilder { base: self.base }
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
        //
        //
        let bis1 = self.base.topic.clone().unwrap();
        let bis = self.base.topic.clone().unwrap();
        let name = bis.split('/').last().unwrap();
        if let Some(device_dyn_info) = self.base.device_dyn_info.clone() {
            device_dyn_info
                .lock()
                .await
                .structure_insert(
                    bis1.clone(),
                    StructuralElement::Attribute(ElementAttribute::new(
                        name.to_string(),
                        TYPE::typee(),
                        AttributeMode::AttOnly,
                    )),
                )
                .unwrap();
        }

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
        //
        //
        let bis1 = self.base.topic.clone().unwrap();
        let bis = self.base.topic.clone().unwrap();
        let name = bis.split('/').last().unwrap();
        if let Some(device_dyn_info) = self.base.device_dyn_info.clone() {
            device_dyn_info
                .lock()
                .await
                .structure_insert(
                    bis1.clone(),
                    StructuralElement::Attribute(ElementAttribute::new(
                        name.to_string(),
                        TYPE::typee(),
                        AttributeMode::AttOnly,
                    )),
                )
                .unwrap();
        }

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

/// Builder specialisation for att only Attribute
pub struct AttOnlyMsgBuilder {
    base: AttributeBuilder,
}

impl AttOnlyMsgBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> AttOnlyMsgAtt<TYPE> {
        //
        //
        let bis1 = self.base.topic.clone().unwrap();
        let bis = self.base.topic.clone().unwrap();
        let name = bis.split('/').last().unwrap();
        if let Some(device_dyn_info) = self.base.device_dyn_info.clone() {
            device_dyn_info
                .lock()
                .await
                .structure_insert(
                    bis1.clone(),
                    StructuralElement::Attribute(ElementAttribute::new(
                        name.to_string(),
                        TYPE::typee(),
                        AttributeMode::AttOnly,
                    )),
                )
                .unwrap();
        }

        AttOnlyMsgAtt::from(self.base)
    }
}
