mod log;
mod link;
mod device;
mod platform;
mod interface;
mod connection;
mod subscription;
mod builtin_devices;

use crate::platform::Platform;

#[tokio::main]
async fn main() {
    // Init tracing subscribers
    log::init();

    // Create platform runner
    let _ = Platform::new("test-platform").work().await;
}
