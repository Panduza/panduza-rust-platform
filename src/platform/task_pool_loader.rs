use std::pin::Pin;
use futures::Future;

use super::error;
use super::platform_error_result;
use super::PlatformTaskResult;

/// Task pool loader
/// 
#[derive(Clone)]
pub struct TaskPoolLoader {
    /// Task pool sender
    /// To send tasks to the task pool
    task_pool_tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>>
}

impl TaskPoolLoader {

    /// Create a new task pool loader
    /// 
    pub fn new(tx: tokio::sync::mpsc::Sender<Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>>) -> TaskPoolLoader {
        return TaskPoolLoader {
            task_pool_tx: tx
        }
    }

    /// Load a future into the task pool
    /// 
    pub fn load(&mut self, future: Pin<Box<dyn Future<Output = PlatformTaskResult> + Send>>) -> Result<(), error::PlatformError>{
        let r = self.task_pool_tx.try_send(future);
        match r {
            Ok(_) => {
                return Ok(());
            },
            Err(e) => {
                return platform_error_result!(
                    format!("Failed to send task to task pool: {}", e), None);
            }
        }
    }

}

