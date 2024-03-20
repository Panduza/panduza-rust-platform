use std::fmt;
use colored::Colorize;
use tracing_core::{Event, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext
};
use tracing_subscriber::registry::LookupSpan;

use crate::log::hash_visitor::HashVisitor;

/// A custom event formatter that formats events in a platform-specific way.
/// 
pub struct PlatformFormatter;

impl<S, N> FormatEvent<S, N> for PlatformFormatter
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

        let mut visitor = HashVisitor::new();
        event.record(&mut visitor);

        // Format values from the event's metadata:
        // let metadata = event.metadata();
        // write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // if let Some(span_ref) = ctx.lookup_current() {
        //     if let Some(builder) = span_ref.extensions().get::<SpanBuilder>() {
        //         if let Some(trace_id) = builder.trace_id {
        //             serializer.serialize_entry("trace_id", &trace_id.to_hex())?;
        //         }
        //     }
        // }

        
        // Write the event's message.
        let message = visitor.entries().get("message").unwrap();
        write!(&mut writer, "{}", message)?;



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

        // // Write fields on the event
        // ctx.field_format().format_fields(writer.by_ref(), event)?;
        
        // writeln!(writer, "{}", event.metadata().target().yellow())


        writeln!(writer)
    }
}