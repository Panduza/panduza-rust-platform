#![deny(
    while_true,
    improper_ctypes,
    non_shorthand_field_patterns,
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
)]

#[cfg(feature = "built-in-drivers")]
mod built_in;

mod device_tree;
mod platform;
mod plugins_manager;
mod sys_info;
mod underscore_device;

use panduza_platform_core::env::system_default_device_tree_file;
use panduza_platform_core::env::system_default_log_dir;
pub use platform::Platform;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Enable logs on stdout
    #[arg(short, long)]
    quiet_log: bool,

    /// Also display broker logs
    #[arg(short, long)]
    broker_log_enable: bool,

    /// Enable debug logs
    #[arg(short, long)]
    debug_log: bool,

    /// Enable trace logs
    #[arg(short, long)]
    trace_log: bool,
}

/// At least print arguments when the platform is started
/// If the use start without log, he can understand why there is none.
///
fn print_platform_header(args: &Args) {
    println!("----------------------------------------");
    println!("# Panduza Platform");
    println!("");
    println!(
        "- Stdout logs         : {}",
        if !args.quiet_log {
            "ENABLED"
        } else {
            "DISABLED"
        }
    );
    println!(
        "- Broker logs         : {}",
        if args.broker_log_enable {
            "ENABLED"
        } else {
            "DISABLED"
        }
    );
    println!(
        "- Debug logs          : {}",
        if args.debug_log || args.trace_log {
            "ENABLED"
        } else {
            "DISABLED"
        }
    );
    println!(
        "- Trace logs          : {}",
        if args.trace_log {
            "ENABLED"
        } else {
            "DISABLED"
        }
    );
    println!("");
    println!(
        "- Log dir             : {:?}",
        system_default_log_dir().unwrap()
    );
    println!(
        "- Tree file           : {:?}",
        system_default_device_tree_file().unwrap()
    );

    println!("----------------------------------------");
}

#[tokio::main]
async fn main() {
    //
    // Manage args
    let args = Args::parse();

    //
    // Give some information when the platform start
    print_platform_header(&args);

    //
    // Manage logs
    // Init tracing subscriber
    panduza_platform_core::tracing::init(
        !args.quiet_log,
        args.broker_log_enable,
        args.debug_log,
        args.trace_log,
    );

    // Create platform runner
    // La platform c'est l'assemblage de
    // - 1 broker
    // - 1 runtime pour les services de bases
    // - N plugins runtime
    let mut platform = Platform::new(!args.quiet_log, args.debug_log, args.trace_log);

    //
    // Log minimal set of information
    platform.log_starting_info(&args, sys_info::PLATFORM_VERSION, sys_info::RUSTC_VERSION);

    //
    // Platform loop
    platform.run().await;
}
