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
    // dead_code,
    // unused,
    // unused_allocation,
    // unused_comparisons,
    // unused_parens,
)]

mod log;
mod link;
mod meta;
mod device;
mod platform;
mod interface;
mod attribute;
mod connector;
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

