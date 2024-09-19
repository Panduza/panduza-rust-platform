use super::CmdOnlyMsgAttInner;
use crate::AttributeBuilder;
use crate::Error;
use crate::MessageCodec;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

///
/// Attribute to only receive command from clients
///
#[derive(Clone)]
pub struct CmdOnlyMsgAtt<TYPE: MessageCodec> {
    ///
    /// Inner implementation
    ///
    inner: Arc<Mutex<CmdOnlyMsgAttInner<TYPE>>>,
}

impl<TYPE: MessageCodec> CmdOnlyMsgAtt<TYPE> {
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
}

///
/// Allow creation from the builder
///
impl<TYPE: MessageCodec> From<AttributeBuilder> for CmdOnlyMsgAtt<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        CmdOnlyMsgAtt {
            inner: CmdOnlyMsgAttInner::from(builder).into(),
        }
    }
}
