use super::BidirMsgAttInner;
use crate::{AttributeBuilder, Error, MessageCodec};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct BidirMsgAtt<TYPE: MessageCodec> {
    ///
    ///
    ///
    inner: Arc<Mutex<BidirMsgAttInner<TYPE>>>,
}

impl<TYPE: MessageCodec> BidirMsgAtt<TYPE> {
    ///
    /// Initialize the attribute
    ///
    pub async fn init(self) -> Result<Self, Error> {
        self.inner.lock().await.init(self.inner.clone()).await?;
        Ok(self)
    }

    ///
    /// Bloc until at least a command is received
    ///
    pub async fn wait_commands(&self) {
        let in_notifier = self.inner.lock().await.in_notifier();
        in_notifier.notified().await
    }

    ///
    /// Bloc until at least a command is received then execute the 'function'
    ///
    pub async fn wait_commands_then<F>(&self, function: F) -> Result<(), Error>
    where
        F: Future<Output = Result<(), Error>> + Send + 'static,
    {
        let in_notifier = self.inner.lock().await.in_notifier();
        in_notifier.notified().await;
        function.await
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub async fn pop_cmd(&mut self) -> Option<TYPE> {
        return self.inner.lock().await.pop_cmd();
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub async fn get_last_cmd(&self) -> Option<TYPE> {
        return self.inner.lock().await.get_last_cmd();
    }

    /// Set the value of the attribute
    ///
    pub async fn set<I: Into<TYPE>>(&self, value: I) -> Result<(), Error> {
        self.inner.lock().await.set(value.into()).await?;
        Ok(())
    }
}

/// Allow creation from the builder
impl<TYPE: MessageCodec> From<AttributeBuilder> for BidirMsgAtt<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        BidirMsgAtt {
            inner: BidirMsgAttInner::from(builder).into(),
        }
    }
}
