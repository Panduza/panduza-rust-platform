use futures::future::BoxFuture;
use futures::Future;
use std::pin::Pin;

use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use crate::Error;

pub type MainTaskResult = Result<(), Error>;

// pub type MainTask = Pin<Box<dyn Future<Output = MainTaskResult> + Send>>;
pub type MainTask = BoxFuture<'static, MainTaskResult>;

/// Platform Spawner
///
/// Allow main tasks of the system to be monitored by the platform main thread
///
#[derive(Clone)]
pub struct PlatformTaskSpawner {
    /// Task pool sender
    /// To send tasks to the task pool
    tx: Sender<MainTask>,
}

impl PlatformTaskSpawner {
    /// Create the channel
    ///
    pub fn create() -> (PlatformTaskSpawner, Receiver<MainTask>) {
        let (tx, rx) = tokio::sync::mpsc::channel::<MainTask>(100);
        return (PlatformTaskSpawner::from(tx), rx);
    }

    /// Load a future into the task pool
    ///
    pub fn spawn(
        &mut self,
        future: Pin<Box<dyn Future<Output = MainTaskResult> + Send>>,
    ) -> Result<(), crate::Error> {
        let r = self.tx.try_send(future);
        match r {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                Ok(())
                // return __platform_error_result!(format!(
                //     "Failed to send task to task pool: {}",
                //     e
                // ));
            }
        }
    }
}

impl From<Sender<MainTask>> for PlatformTaskSpawner {
    fn from(tx: Sender<MainTask>) -> Self {
        PlatformTaskSpawner { tx: tx }
    }
}
