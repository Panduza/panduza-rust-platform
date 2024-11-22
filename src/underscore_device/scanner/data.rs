use panduza_platform_core::Error;
use panduza_platform_core::ProductionOrder;
use panduza_platform_core::Store;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::Notify;

#[derive(Clone)]
///
///
///
pub struct ScannerDriver {
    ///
    /// When user request a change
    ///
    pub request_notifier: Arc<Notify>,

    ///
    /// When something new happened from platform
    ///
    pub update_notifier: Arc<Notify>,

    //
    pub is_running: Arc<Mutex<bool>>,

    //
    pub found_instances: Arc<Mutex<Vec<ProductionOrder>>>,
}

impl ScannerDriver {
    ///
    ///
    ///
    pub fn new() -> Self {
        Self {
            request_notifier: Arc::new(Notify::new()),
            update_notifier: Arc::new(Notify::new()),
            is_running: Arc::new(Mutex::new(false)),
            found_instances: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn is_already_running(&self) -> bool {
        *self.is_running.lock().await
    }

    pub async fn request_scanning_start(&mut self) {
        self.request_notifier.notify_waiters();
    }

    // ///
    // ///
    // ///
    // pub async fn set_stores(&mut self, store: Store) {
    //     self.store.lock().await.extend_by_copy(&store);
    //     self.change_notifier.notify_waiters();
    // }

    // ///
    // ///
    // ///
    // pub async fn into_json_value(&self) -> Result<JsonValue, Error> {
    //     self.store.lock().await.into_json_value()
    // }
}
