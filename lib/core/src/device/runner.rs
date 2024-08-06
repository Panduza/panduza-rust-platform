// pas clone, move only
// to run the fsm
// and to run the task monitoring

use std::sync::Arc;

use crate::{taskpool::TaskPool, DeviceOperations, Reactor};

use super::Device;
use futures::future::Join;
use tokio::{sync::Mutex, task::JoinSet};

pub struct DeviceRunner {
    device: Device,

    pool: JoinSet<Result<(), ()>>,
    taskpool: Arc<Mutex<TaskPool<Result<(), ()>>>>,
}

impl DeviceRunner {
    pub fn new(
        reactor: Reactor,
        name: String,
        operations: Box<dyn DeviceOperations>,
    ) -> (DeviceRunner, Device) {
        let (taskpool, spawner) = TaskPool::create();

        let device = Device::new(reactor.clone(), spawner, "dev".to_string(), operations);

        let runner = DeviceRunner {
            device: device.clone(),
            pool: JoinSet::new(),
            taskpool: Arc::new(Mutex::new(taskpool)),
        };

        (runner, device)
    }

    pub async fn run(&mut self) {
        let taskpool = self.taskpool.clone();
        let mut taskpool_lock = taskpool.lock().await;
        loop {
            tokio::select! {

                task = taskpool_lock.requests_receiver.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.pool.spawn(task.unwrap());
                    println!("New task created ! [{:?}]", ah );
                },
                _ = self.end_of_all_tasks() => {
                    println!("All tasks completed, stop the platform");
                    break;
                }
            }
        }
    }

    /// Wait for all tasks to complete
    ///
    async fn end_of_all_tasks(&mut self) {
        while let Some(join_result) = self.pool.join_next().await {
            // self.services.lock().await.stop_requested();

            match join_result {
                Ok(task_result) => match task_result {
                    Ok(_) => {
                        // self.logger.warn("Task completed");
                    }
                    Err(e) => {
                        // self.logger.error(format!("Task failed: {}", e));
                        self.pool.abort_all();
                    }
                },
                Err(e) => {
                    // self.logger.error(format!("Join failed: {}", e));
                }
            }
        }
    }
}
