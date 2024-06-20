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

use rumqttd::Config;
use rumqttd::Broker;


#[tokio::main]
async fn main() {
    // Init tracing subscribers
    log::init();


    // see docs of config crate to know more
    let config = config::Config::builder()
        .add_source(config::File::with_name("rumqttd.toml"))
        // .set_default("id", 0).unwrap()
        .build()
        .unwrap();

    // this is where we deserialize it into Config
    let rumqttd_config: Config = config.try_deserialize().unwrap();
    let mut broker = Broker::new(rumqttd_config);


    // Create platform runner
    let mut platform = Platform::new("test-platform");
    std::thread::spawn(move || {
        broker.start().unwrap();
    });


    // Load plugins section
    {
        let mut devices = platform.devices().lock().await;
        let factory = &mut devices.factory;

        builtin_devices::import_plugin_producers(factory);

        plugins_std_serial::import_plugin_producers(factory);
    }
    
    
    platform.work().await;
}

