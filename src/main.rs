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



    let mut router: std::collections::HashMap<String, config::Value> = config::Map::new();
    router.insert("id".to_string(), config::Value::new(None, 0));
    router.insert("max_connections".to_string(), config::Value::new(None, 10010));
    router.insert("max_outgoing_packet_count".to_string(), config::Value::new(None, 200));
    router.insert("max_segment_size".to_string(), config::Value::new(None, 104857600));
    router.insert("max_segment_count".to_string(), config::Value::new(None, 10));


    let mut server_connections: std::collections::HashMap<String, config::Value> = config::Map::new();
    server_connections.insert("connection_timeout_ms".to_string(), config::Value::new(None, 60000));
    server_connections.insert("max_payload_size".to_string(), config::Value::new(None, 20480));
    server_connections.insert("max_inflight_count".to_string(), config::Value::new(None, 10000));
    server_connections.insert("dynamic_filters".to_string(), config::Value::new(None, true));

    let mut server: std::collections::HashMap<String, config::Value> = config::Map::new();
    server.insert("name".to_string(), config::Value::new(None, "v4-1"));
    server.insert("listen".to_string(), config::Value::new(None, "0.0.0.0:1883"));
    server.insert("next_connection_delay_ms".to_string(), config::Value::new(None, 1));
    server.insert("connections".to_string(), config::Value::new(None, server_connections));

    // see docs of config crate to know more
    let config = config::Config::builder()
        .set_default("id", 0).unwrap()
        .set_default("router", router).unwrap()
        .set_default("v4.1", server).unwrap()
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

        //plugin_std::import_plugin_producers(factory);
        //plugin_fake::import_plugin_producers(factory);

        // plugin_video::import_plugin_producers(factory);
        //pza_plugin_picoha::import_plugin_producers(factory);
    }
    
    
    platform.work().await;
}
