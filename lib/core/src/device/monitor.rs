// pas clone, move only
// to run the fsm
// and to run the task monitoring

use crate::task_channel::create_task_channel;
use crate::{Error, InfoPack, ProductionOrder};
use std::sync::Arc;

use super::Device;
use crate::{DeviceOperations, Reactor, TaskReceiver};
use std::time::Duration;

use tokio::sync::Notify;
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

    subtask_pool_not_empty_notifier: Arc<Notify>,
}

impl DeviceMonitor {
    ///
    /// Constructor
    pub fn new(
        reactor: Reactor,
        info_pack: Option<InfoPack>,
        operations: Box<dyn DeviceOperations>,
        production_order: ProductionOrder,
    ) -> (DeviceMonitor, Device) {
        //
        // Move in data and consume production order
        let name = production_order.device_name;
        let settings = production_order.device_settings;
        //
        // Create the task channel between the device and its monitoring object
        let (task_tx, task_rx) = create_task_channel::<DeviceTaskResult>(50);
        //
        // Create the device object
        let device = Device::new(
            reactor.clone(),
            info_pack,
            task_tx,
            name,
            operations,
            settings,
        );
        //
        // Create the monitoring object
        let monitor = DeviceMonitor {
            device: device.clone(),
            subtask_pool: JoinSet::new(),
            subtask_receiver: Arc::new(Mutex::new(task_rx)),
            subtask_pool_not_empty_notifier: Arc::new(Notify::new()),
        };
        //
        // Ok
        (monitor, device)
    }

    pub async fn run(&mut self) {
        let subtask_receiver_clone = self.subtask_receiver.clone();
        let mut subtask_receiver_clone_lock = subtask_receiver_clone.lock().await;
        let subtask_pool_not_empty_notifier_clone = self.subtask_pool_not_empty_notifier.clone();
        loop {
            tokio::select! {

                task = subtask_receiver_clone_lock.rx.recv() => {
                    // Function to effectily spawn tasks requested by the system
                    let ah = self.subtask_pool.spawn(task.unwrap());
                    println!("New task created ! [{:?}]", ah );
                    subtask_pool_not_empty_notifier_clone.notify_one();
                },
                _ = self.end_of_all_tasks() => {
                    // Juste alert the user, but it can be not important
                    self.device.logger.warn("No sub task anymore");

                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// Wait for all tasks to complete
    ///
    async fn end_of_all_tasks(&mut self) {
        //
        // Wait for some task in the pool if the pool is empty
        if self.subtask_pool.is_empty() {
            self.subtask_pool_not_empty_notifier.notified().await;
        }

        while let Some(join_result) = self.subtask_pool.join_next().await {
            // self.services.lock().await.stop_requested();

            match join_result {
                Ok(task_result) => match task_result {
                    Ok(_) => {
                        println!("Task completed");
                    }
                    Err(e) => {
                        println!("Sub Task failed: {}", e.to_string());
                        self.subtask_pool.abort_all();

                        // TODO HERE we have to alert the device for a reboot
                    }
                },
                Err(e) => {
                    println!("Join failed: {}", e);
                }
            }
        }
    }
}
