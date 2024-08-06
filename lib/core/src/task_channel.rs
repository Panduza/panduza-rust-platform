use crate::Error;
use futures::future::BoxFuture;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

/// Object to monitor tasks that must be spawned
///
pub struct TaskReceiver<O> {
    /// Internal receiver
    pub rx: Receiver<BoxFuture<'static, O>>,
}

impl<O> From<Receiver<BoxFuture<'static, O>>> for TaskReceiver<O> {
    fn from(rx: Receiver<BoxFuture<'static, O>>) -> Self {
        TaskReceiver { rx: rx }
    }
}

/// Object to send task to the runner
///
#[derive(Clone)]
pub struct TaskSender<O> {
    tx: Sender<BoxFuture<'static, O>>,
}

impl<O> TaskSender<O> {
    /// Load a future into the task pool
    ///
    pub fn spawn(&mut self, future: BoxFuture<'static, O>) -> Result<(), Error> {
        let r = self.tx.try_send(future);
        match r {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => Err(Error::Spawn(e.to_string())),
        }
    }
}

impl<O> From<Sender<BoxFuture<'static, O>>> for TaskSender<O> {
    fn from(tx: Sender<BoxFuture<'static, O>>) -> Self {
        TaskSender { tx: tx }
    }
}

/// Create the task channel
///
pub fn create_task_channel<O>(buffer: usize) -> (TaskSender<O>, TaskReceiver<O>) {
    let (tx, rx) = channel::<BoxFuture<'static, O>>(buffer);
    return (TaskSender::<O>::from(tx), TaskReceiver::<O>::from(rx));
}
