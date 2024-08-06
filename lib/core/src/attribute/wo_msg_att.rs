use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AttributeBuilder, Error, MessageCodec, TaskResult};

use super::WoMessageAttributeInner;

// use super::att::Att;
// pub use super::CoreMembers;
// pub use super::OnMessageHandler;
// pub use super::ReactorData;

// pub use inner_msg_att_bool::OnChangeHandlerFunction;

/// Attribute to manage a boolean
#[derive(Clone)]
pub struct WoMessageAttribute<TYPE: MessageCodec> {
    ///
    inner: Arc<Mutex<WoMessageAttributeInner<TYPE>>>,
}

impl<TYPE: MessageCodec> WoMessageAttribute<TYPE> {
    /// Initialize the attribute
    ///
    pub async fn init(self) -> Result<Self, Error> {
        Ok(self)
    }

    /// Set the value of the attribute
    ///
    pub async fn set<I: Into<TYPE>>(&self, value: I) -> Result<(), Error> {
        self.inner.lock().await.set(value.into()).await?;
        Ok(())
    }
}

/// Allow creation from the builder
impl<TYPE: MessageCodec> From<AttributeBuilder> for WoMessageAttribute<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        WoMessageAttribute {
            inner: WoMessageAttributeInner::from(builder).into(),
        }
    }
}
