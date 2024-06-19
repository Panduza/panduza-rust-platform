#![deny(
    while_true,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    bad_style,
    dead_code,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
)]

mod log;
mod builtin_devices;

use panduza_core::platform::Platform;




#[tokio::main]
async fn main() {
    // Init tracing subscribers
    log::init();

    // Create platform runner
    let mut platform = Platform::new("test-platform");


    // Load plugins section
    {
        let mut devices = platform.devices().lock().await;
        let factory = &mut devices.factory;

        builtin_devices::import_plugin_producers(factory);

        plugins_std_serial::import_plugin_producers(factory);
    }
    
    
    platform.work().await;
}

