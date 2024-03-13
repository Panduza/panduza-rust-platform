mod log;
mod device;
mod platform;
mod connection;
mod builtin_devices;
mod interfaces;


#[tokio::main]
async fn main() {

    // Init tracing subscribers
    log::init();

    // Create platform runner
    let mut platform_runner = platform::Runner::new();

    // Run platform
    platform_runner.work().await;
}

