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
pub struct DeviceMonitor {
    /// To allow the communication with the state machine
    ///
    device: Device,

    subtask_pool: JoinSet<DeviceTaskResult>,
    subtask_receiver: Arc<Mutex<TaskReceiver<DeviceTaskResult>>>,
}

impl DeviceMonitor {
    pub fn new(
        reactor: Reactor,
        name: String,
        operations: Box<dyn DeviceOperations>,
    ) -> (DeviceMonitor, Device) {
        let (task_tx, task_rx) = create_task_channel::<DeviceTaskResult>(50);

        let device = Device::new(reactor.clone(), task_tx, "dev".to_string(), operations);

        let runner = DeviceMonitor {
            device: device.clone(),
            subtask_pool: JoinSet::new(),
            subtask_receiver: Arc::new(Mutex::new(task_rx)),
        };

        (runner, device)
    }

    pub async fn run(&mut self) {
        let subtask_receiver_clone = self.subtask_receiver.clone();
        let mut subtask_receiver_clone_lock = subtask_receiver_clone.lock().await;
        loop {
            tokio::select! {

                task = subtask_receiver_clone_lock.rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.subtask_pool.spawn(task.unwrap());
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
        while let Some(join_result) = self.subtask_pool.join_next().await {
            // self.services.lock().await.stop_requested();

            match join_result {
                Ok(task_result) => match task_result {
                    Ok(_) => {
                        println!("Task completed");
                    }
                    Err(e) => {
                        println!("Task failed: {}", e);
                        self.subtask_pool.abort_all();
                    }
                },
                Err(e) => {
                    println!("Join failed: {}", e);
                }
            }
        }
    }
}
