mod log;
mod device;
mod platform;
mod connection;
mod builtin_devices;
mod interfaces;

use crate::platform::Platform;

#[tokio::main]
async fn main() {

    // Init tracing subscribers
    log::init();

    // Create platform runner
    let mut platform_runner = Platform::new("test-platform");

    // Run platform
    platform_runner.work().await;

}

