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

mod log;
// mod builtin_devices;

use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use panduza_platform_core::BooleanCodec;
use panduza_platform_core::Factory;
use panduza_platform_core::Platform;

use panduza_platform_core::Plugin;
use panduza_platform_core::ProductionOrder;
use rumqttd::Broker;
use rumqttd::Config;

#[tokio::main]
async fn main() {
    // Init tracing subscribers
    log::init();

    let mut router: std::collections::HashMap<String, config::Value> = config::Map::new();
    router.insert("id".to_string(), config::Value::new(None, 0));
    router.insert(
        "max_connections".to_string(),
        config::Value::new(None, 10010),
    );
    router.insert(
        "max_outgoing_packet_count".to_string(),
        config::Value::new(None, 200),
    );
    router.insert(
        "max_segment_size".to_string(),
        config::Value::new(None, 104857600),
    );
    router.insert(
        "max_segment_count".to_string(),
        config::Value::new(None, 10),
    );

    let mut server_connections: std::collections::HashMap<String, config::Value> =
        config::Map::new();
    server_connections.insert(
        "connection_timeout_ms".to_string(),
        config::Value::new(None, 60000),
    );
    server_connections.insert(
        "max_payload_size".to_string(),
        config::Value::new(None, 20480),
    );
    server_connections.insert(
        "max_inflight_count".to_string(),
        config::Value::new(None, 10000),
    );
    server_connections.insert(
        "dynamic_filters".to_string(),
        config::Value::new(None, true),
    );

    let mut server: std::collections::HashMap<String, config::Value> = config::Map::new();
    server.insert("name".to_string(), config::Value::new(None, "v4-1"));
    server.insert(
        "listen".to_string(),
        config::Value::new(None, "0.0.0.0:1883"),
    );
    server.insert(
        "next_connection_delay_ms".to_string(),
        config::Value::new(None, 1),
    );
    server.insert(
        "connections".to_string(),
        config::Value::new(None, server_connections),
    );

    // see docs of config crate to know more
    let config = config::Config::builder()
        .set_default("id", 0)
        .unwrap()
        .set_default("router", router)
        .unwrap()
        .set_default("v4.1", server)
        .unwrap()
        .build()
        .unwrap();

    // this is where we deserialize it into Config
    let rumqttd_config: Config = config.try_deserialize().unwrap();
    let mut broker = Broker::new(rumqttd_config);

    //
    let mut factory = Factory::new();
    // factory.add_producers(pza_plugin_fake::plugin_producers());
    // factory.add_producers(pza_plugin_picoha::plugin_producers());
    // factory.add_producers(pza_plugin_picoha_ssb::plugin_producers());

    let mut libs = Vec::new();
    let mut plugins: Vec<Plugin> = Vec::new();

    unsafe {
        // let lib = libloading::Library::new(
        //     "C:/Users/rodriguez.NET/Documents/workspace/50-PROJET/XX-XXXX-PZA/pza-plugin-fakes/target/release/pza_plugin_fakes.dll",
        // )
        // .unwrap();

        let lib = libloading::Library::new(
            "C:/Users/rodriguez.NET/Documents/workspace/50-PROJET/XX-XXXX-PZA/pza-plugin-fakes/target/debug/pza_plugin_fakes.dll"
        )
        .unwrap();

        let func: libloading::Symbol<extern "C" fn() -> Plugin> =
            lib.get(b"plugin_entry_point").unwrap();

        let plugin_ptr = (*func)(); // Get the pointer to the Plugin struct

        // Create a CStr from the pointer, handling potential errors
        // let cstr = CString::from_raw(plugin_ptr.name);

        println!("plugin  got {:?} ", plugin_ptr.name);
        println!("name {:?} ", CStr::from_ptr(plugin_ptr.name).to_str());
        println!("version {:?} ", CStr::from_ptr(plugin_ptr.version).to_str());

        (plugin_ptr.test)();

        // let func2: libloading::Symbol<fn() -> *mut u32> = lib.get(b"get_number_pointer").unwrap();
        // println!("get_number_pointer got {} == expect 5", *func2());

        // let func3: libloading::Symbol<fn() -> *mut simple_struct> = lib.get(b"get_simple_struct_ptr").unwrap();
        // println!("get_simple_struct_ptr got {} == expect 6", (*func3()).a);

        let mut production_order = ProductionOrder::new("panduza.fake_register_map", "memory_map");

        let order = production_order.to_c_string().unwrap();
        (plugin_ptr.produce)(order.as_c_str().as_ptr());

        libs.push(lib);
        plugins.push(plugin_ptr);
    }

    // Create platform runner
    let mut platform = Platform::new(factory);
    std::thread::spawn(move || {
        broker.start().unwrap();
    });

    // Platform loop
    platform.run().await;

    for p in plugins {
        unsafe { (p.join)() };
    }
}
