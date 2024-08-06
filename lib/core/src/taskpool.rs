use futures::future::BoxFuture;
use futures::future::Join;
use futures::Future;
use std::pin::Pin;
use tokio::task::JoinSet;

use crate::Error;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

pub struct TaskPool<O> {
    pub requests_receiver: Receiver<BoxFuture<'static, O>>,
}

impl<O> TaskPool<O> {
    /// Create the channel
    ///
    pub fn create() -> (TaskPool<O>, TaskPoolSpawner<O>) {
        let (tx, rx) = channel::<BoxFuture<'static, O>>(100);
        return (
            TaskPool::<O> {
                requests_receiver: rx,
            },
            TaskPoolSpawner::<O> { sender: tx },
        );
    }
}

#[derive(Clone)]
pub struct TaskPoolSpawner<O> {
    sender: Sender<BoxFuture<'static, O>>,
}

impl<O> TaskPoolSpawner<O> {
    /// Load a future into the task pool
    ///
    pub fn spawn(&mut self, future: BoxFuture<'static, O>) -> Result<(), crate::Error> {
        let r = self.sender.try_send(future);
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

// impl From<Sender<MainTask>> for TaskPoolSpawner {
//     fn from(tx: Sender<MainTask>) -> Self {
//         TaskPoolSpawner { tx: tx }
//     }
// }
