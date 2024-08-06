// pas clone, move only
// to run the fsm
// and to run the task monitoring

use crate::task_channel::create_task_channel;
use crate::Error;
use std::sync::Arc;

use super::Device;
use crate::{DeviceOperations, Reactor, TaskReceiver};
use futures::future::Join;
use std::time::Duration;

use tokio::time::sleep;
use tokio::{sync::Mutex, task::JoinSet};

/// Result for task spawned by the device subtasks
///
pub type DeviceTaskResult = Result<(), Error>;

/// Object to manage device subtasks
/// It is important to check when a task has failed
///
pub struct DeviceRunner {
    device: Device,

    pool: JoinSet<DeviceTaskResult>,
    task_rx: Arc<Mutex<TaskReceiver<DeviceTaskResult>>>,
}

impl DeviceRunner {
    pub fn new(
        reactor: Reactor,
        name: String,
        operations: Box<dyn DeviceOperations>,
    ) -> (DeviceRunner, Device) {
        let (task_tx, task_rx) = create_task_channel::<DeviceTaskResult>();

        let device = Device::new(reactor.clone(), task_tx, "dev".to_string(), operations);

        let runner = DeviceRunner {
            device: device.clone(),
            pool: JoinSet::new(),
            task_rx: Arc::new(Mutex::new(task_rx)),
        };

        (runner, device)
    }

    pub async fn run(&mut self) {
        let task_rx = self.task_rx.clone();
        let mut task_rx_lock = task_rx.lock().await;
        loop {
            tokio::select! {

                task = task_rx_lock.rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.pool.spawn(task.unwrap());
                    println!("New task created ! [{:?}]", ah );
                },
                _ = self.end_of_all_tasks() => {
                    println!("All tasks completed, stop the device");

                    sleep(Duration::from_secs(1)).await;
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
                        println!("Task completed");
                    }
                    Err(e) => {
                        println!("Task failed: {}", e);
                        self.pool.abort_all();
                    }
                },
                Err(e) => {
                    println!("Join failed: {}", e);
                }
            }
        }
    }
}
