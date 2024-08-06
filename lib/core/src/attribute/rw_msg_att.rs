use tokio::time::sleep;

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::{AttributeBuilder, Error, MessageCodec, TaskResult};

use super::RwMessageAttributeInner;

// use super::att::Att;
// pub use super::CoreMembers;
// pub use super::OnMessageHandler;
// pub use super::ReactorData;

// pub use inner_msg_att_bool::OnChangeHandlerFunction;

/// Attribute to manage a boolean
#[derive(Clone)]
pub struct RwMessageAttribute<TYPE: MessageCodec> {
    ///
    inner: Arc<Mutex<RwMessageAttributeInner<TYPE>>>,
}

impl<TYPE: MessageCodec> RwMessageAttribute<TYPE> {
    /// Initialize the attribute
    ///
    pub async fn init(self) -> Result<Self, Error> {
        self.inner.lock().await.init(self.inner.clone()).await?;
        Ok(self)
    }

    pub async fn wait_one_command(&self) {
        let change_notifier = self.inner.lock().await.base.clone_change_notifier();
        change_notifier.notified().await
    }

    /// Wait an input command then execute the callback
    ///
    pub async fn wait_one_command_then<F>(&self, function: F) -> TaskResult
    where
        F: Future<Output = TaskResult> + Send + 'static,
    {
        let change_notifier = self.inner.lock().await.base.clone_change_notifier();
        change_notifier.notified().await;
        function.await
    }

    /// Set the value of the attribute
    ///
    pub async fn set<I: Into<TYPE>>(&self, value: I) -> Result<(), Error> {
        self.inner.lock().await.set(value.into()).await?;
        // let cv = self.inner.lock().await.set_ensure_lock_clone();
        // cv.with_lock(|mut done| {
        //     while !*done {
        //         done.wait();
        //     }
        // });
        Ok(())
    }

    /// Get the value of the attribute
    ///
    pub async fn get(&self) -> Option<TYPE> {
        self.inner.lock().await.get()
    }
}

/// Allow creation from the builder
impl<TYPE: MessageCodec> From<AttributeBuilder> for RwMessageAttribute<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        RwMessageAttribute {
            inner: RwMessageAttributeInner::from(builder).into(),
        }
    }
}