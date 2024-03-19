use tracing_subscriber::fmt::format::FmtSpan;

use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,

};
use tracing_subscriber::registry::LookupSpan;

use std::fmt;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // // Format values from the event's's metadata:
        // let metadata = event.metadata();
        // write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // // Format all the spans in the event's span context.
        // if let Some(scope) = ctx.event_scope() {
        //     for span in scope.from_root() {
        //         write!(writer, "{}", span.name())?;

        //         // `FormattedFields` is a formatted representation of the span's
        //         // fields, which is stored in its extensions by the `fmt` layer's
        //         // `new_span` method. The fields will have been formatted
        //         // by the same field formatter that's provided to the event
        //         // formatter in the `FmtContext`.
        //         let ext = span.extensions();
        //         let fields = &ext
        //             .get::<FormattedFields<N>>()
        //             .expect("will never be `None`");

        //         // Skip formatting the fields if the span had no fields.
        //         if !fields.is_empty() {
        //             write!(writer, "{{{}}}", fields)?;
        //         }
        //         write!(writer, ": ")?;
        //     }
        // }

        // // Write fields on the event
        // ctx.field_format().format_fields(writer.by_ref(), event)?;


        println!("POOKKK {}", event.metadata().fields() );
        println!("POOKKK {}", event.metadata().level() );


        println!("POOKKK {:?}", event.metadata().fields().field("test") );
        println!("POOKKK {:?}", event.metadata() );

        // display field content of the event
        // Display field content of the event
    
        for field in event.metadata().fields().iter() {
            println!("Field: {:?}", field);
            
        }
    

        // let e = event.metadata().fields().field("test");
        // if e.is_some() {
        //     let iii: tracing_core::callsite::Identifier = e.unwrap().callsite();
        //     // display iii information
        //     println!("POOKKK {:?}", iii );

        // }

        writeln!(writer)
    }
}

/// Define the fmt subscriber for the platform
/// 
fn init_fmt_subscriber()
{
    let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
    .compact()
    // .pretty()
    .with_max_level(tracing::Level::TRACE)
    // Display source code file paths
    // .with_file(true)
    // Display source code line numbers
    // .with_line_number(true)
    // Display the thread ID an event was recorded on
    // .with_thread_ids(true)
    // Don't display the event's target (module path)
    // .with_target(false)
    // .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    // .with_span_events(FmtSpan::FULL)
    // Build the subscriber

    .event_format(MyFormatter)

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

    



