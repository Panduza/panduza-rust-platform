use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Notify;

///
///
///
struct StoreData {
    ///
    /// Notified when a data change
    ///
    change_notifier: Arc<Notify>,
    // data: HashMap<String, StoreEntry>,
    // instances: HashMap<String, ?>,
}
