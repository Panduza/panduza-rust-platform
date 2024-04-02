use std::pin::Pin;
use futures::FutureExt;

use futures::future::BoxFuture;
use futures::Future;


use super::PlatformTaskResult;

use super::platform_error;
use super::error;

#[derive(Clone)]
pub struct TaskPoolLoader {

    task_pool_tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>>

}

impl TaskPoolLoader {

    pub fn new(tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>>) -> TaskPoolLoader {
        return TaskPoolLoader {
            task_pool_tx: tx
        }
    }

    pub fn load(&mut self, future: Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>) -> Result<(), error::PlatformError>{
        let r = self.task_pool_tx.try_send(future);
        match r {
            Ok(_) => {
                return Ok(());
            },
            Err(e) => {
                return platform_error!("Failed to send task to task pool", None);
            }
        }
    }

}

