use panduza_platform_core::Store;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::Notify;

#[derive(Clone)]
///
///
///
pub struct SharedStore {
    ///
    /// Notified when a data change
    ///
    change_notifier: Arc<Notify>,

    ///
    ///
    ///
    store: Arc<Mutex<Store>>,
}

impl SharedStore {
    ///
    ///
    ///
    pub fn new() -> Self {
        Self {
            change_notifier: Arc::new(Notify::new()),
            store: Arc::new(Mutex::new(Store::default())),
        }
    }

    ///
    ///
    ///
    pub async fn set_stores(&mut self, store: Store) {
        self.store.lock().await.extend_by_copy(&store);
        self.change_notifier.notify_waiters();
    }
}
