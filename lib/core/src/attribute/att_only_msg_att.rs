use super::AttOnlyMsgAttInner;
use crate::{AttributeBuilder, Error, MessageCodec};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Attribute to manage a boolean
#[derive(Clone)]
pub struct AttOnlyMsgAtt<TYPE: MessageCodec> {
    ///
    inner: Arc<Mutex<AttOnlyMsgAttInner<TYPE>>>,
}

impl<TYPE: MessageCodec> AttOnlyMsgAtt<TYPE> {
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
impl<TYPE: MessageCodec> From<AttributeBuilder> for AttOnlyMsgAtt<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        AttOnlyMsgAtt {
            inner: AttOnlyMsgAttInner::from(builder).into(),
        }
    }
}
