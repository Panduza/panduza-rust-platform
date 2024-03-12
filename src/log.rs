use tracing_subscriber::fmt::format::FmtSpan;

/// Define the fmt subscriber for the platform
/// 
fn init_fmt_subscriber()
{
    let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
    .compact()
    // Display source code file paths
    .with_file(true)
    // Display source code line numbers
    .with_line_number(true)
    // Display the thread ID an event was recorded on
    .with_thread_ids(true)
    // Don't display the event's target (module path)
    .with_target(false)
    // .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    .with_span_events(FmtSpan::FULL)
    // Build the subscriber
    .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

/// Function to initiliaze tracing for the application
/// 
pub fn init()
{
    if cfg!(feature = "trac-fmt") {
        init_fmt_subscriber();
    }
    else if cfg!(feature = "trac-console") {
        #[cfg(feature = "trac-console")]
        console_subscriber::init();    
    }
}

    



