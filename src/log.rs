use tracing::field::Visit;
use tracing_subscriber::fmt::format::FmtSpan;

use tracing_core::{Event, Field, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,

};
use tracing_subscriber::registry::LookupSpan;

use std::{collections::HashMap, fmt};


use colored::Colorize;

#[derive(Debug)]
struct MyVisitor {
    values: HashMap<String, String>,
}

impl MyVisitor {
    fn new() -> Self {
        MyVisitor {
            values: HashMap::new(),
        }
    }
}

impl Visit for MyVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.values.insert(field.name().to_string(), format!("{:?}", value));
    }
}

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
        // Format values from the event's metadata:
        let metadata = event.metadata();
        // write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // if let Some(span_ref) = ctx.lookup_current() {
        //     if let Some(builder) = span_ref.extensions().get::<SpanBuilder>() {
        //         if let Some(trace_id) = builder.trace_id {
        //             serializer.serialize_entry("trace_id", &trace_id.to_hex())?;
        //         }
        //     }
        // }

        

        let mut visitor = MyVisitor::new();
        event.record(&mut visitor);

        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!! {:?}", visitor);



        // .format_fields(writer.by_ref(),  ppp)?;


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

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;


        
        writeln!(writer, "{}", event.metadata().target().yellow())
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

    



