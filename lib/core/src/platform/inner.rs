use std::sync::Arc;

use tokio::sync::Mutex;

pub struct PlatformInner {}

impl PlatformInner {
    pub fn new() -> PlatformInner {
        PlatformInner {}
    }
}

/// Allow mutation into Arc pointer
impl Into<Arc<Mutex<PlatformInner>>> for PlatformInner {
    fn into(self) -> Arc<Mutex<PlatformInner>> {
        Arc::new(Mutex::new(self))
    }
}
