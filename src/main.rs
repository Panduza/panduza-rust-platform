// #![deny(
//     while_true,
//     improper_ctypes,
//     non_shorthand_field_patterns,
//     no_mangle_generic_items,
//     overflowing_literals,
//     path_statements,
//     patterns_in_fns_without_body,
//     unconditional_recursion,
//     bad_style,
//     dead_code,
//     unused,
//     unused_allocation,
//     unused_comparisons,
//     unused_parens,
// )]

mod platform;
pub use platform::Platform;

mod device_tree;
mod plugins_manager;
mod underscore_device;

// use std::ffi::CStr;

// use panduza_platform_core::Factory;

// use panduza_platform_core::Plugin;
// use panduza_platform_core::ProductionOrder;
// use rumqttd::Broker;
// use rumqttd::Config;

#[tokio::main]
async fn main() {
    // Init tracing subscribers
    panduza_platform_core::log::init();

    // Create platform runner
    // La platform c'est l'assemblage de
    // - 1 broker
    // - 1 runtime pour les services de bases
    // - N plugins runtime
    let mut platform = Platform::new();
    // std::thread::spawn(move || {
    //     broker.start().unwrap();
    // });

    // Platform loop
    platform.run().await;

    // for p in plugins {
    //     unsafe { (p.join)() };
    // }
}
