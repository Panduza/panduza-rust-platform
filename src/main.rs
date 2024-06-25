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

use tokio::runtime::Runtime;
use tokio::task;
use tokio::sync::oneshot;

use rumqttd::Config;
use rumqttd::Broker;

use std::thread;
use std::time::Duration;
use std::ffi::OsString;
use std::sync::{Arc, Mutex};

use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceControlHandlerResult, ServiceExitCode,
        ServiceState, ServiceStatus, ServiceType,
    },
    service_dispatcher,
};

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(_arguments: Vec<OsString>) {
    if let Err(e) = run_service() {
        eprintln!("Service error: {}", e);
    }
}

fn run_service() -> windows_service::Result<()> {

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));

    let status_handle = windows_service::service_control_handler::register(
        "panduza_platform",
        move |control_event| match control_event {
            ServiceControl::Stop => {
                if let Some(tx) = shutdown_tx.lock().unwrap().take() {
                    tx.send(()).unwrap();
                }
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        },
    ).unwrap();

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Your async code here
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

            plugin_std::import_plugin_producers(factory);
        }
    });

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;
    Ok(())
}



fn main() -> Result<(), windows_service::Error> {
    service_dispatcher::start("my_service", ffi_service_main)?;
    Ok(())
}