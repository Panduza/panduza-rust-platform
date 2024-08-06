use std::{future::Future, sync::Arc};

use tokio::{sync::Mutex, task::JoinHandle};

use crate::{DeviceOperations, Reactor, TaskResult};
use tokio::task::JoinSet;

pub struct DeviceInner {
    reactor: Reactor,
    /// Monitored tasks
    task_handles: Vec<JoinHandle<TaskResult>>,
    pub operations: Box<dyn DeviceOperations>,

    joinset: JoinSet<TaskResult>,
}

impl DeviceInner {
    pub fn new(reactor: Reactor, operations: Box<dyn DeviceOperations>) -> DeviceInner {
        DeviceInner {
            reactor: reactor,
            task_handles: vec![],
            operations: operations,
            joinset: JoinSet::new(),
        }
    }

    pub fn store_handle(&mut self, h: JoinHandle<TaskResult>) {
        self.task_handles.push(h);
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = TaskResult> + Send + 'static,
    {
        self.joinset.spawn(future);
    }
}

/// Allow mutation into Arc pointer
impl Into<Arc<Mutex<DeviceInner>>> for DeviceInner {
    fn into(self) -> Arc<Mutex<DeviceInner>> {
        Arc::new(Mutex::new(self))
    }
}
