#![deny(
    while_true,
    improper_ctypes,
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
)]

#[cfg(feature = "built-in-drivers")]
mod built_in;

mod device_tree;
mod platform;
mod plugins_manager;
mod sys_info;
mod underscore_device;

pub use platform::Platform;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable logs on stdout
    #[arg(short, long)]
    log_stdout_enable: bool,

    /// Also display broker logs
    #[arg(short, long)]
    broker_log_enable: bool,
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
        if args.log_stdout_enable {
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
    panduza_platform_core::tracing::init(args.log_stdout_enable, args.broker_log_enable);

    // Create platform runner
    // La platform c'est l'assemblage de
    // - 1 broker
    // - 1 runtime pour les services de bases
    // - N plugins runtime
    let mut platform = Platform::new();

    //
    // Log minimal set of information
    platform.log_starting_info(sys_info::PLATFORM_VERSION, sys_info::RUSTC_VERSION);

    //
    // Platform loop
    platform.run().await;
}
