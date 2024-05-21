use std::{fmt, thread};
use regex::Regex;
use tracing_core::{Event, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext
};
use tracing_subscriber::registry::LookupSpan;

use crate::log::hash_visitor::HashVisitor;
use chrono::Utc;

/// A custom event formatter that formats events in a platform-specific way.
/// 
pub struct FormatterCSV;

impl<S, N> FormatEvent<S, N> for FormatterCSV
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {

        //
        let mut visitor = HashVisitor::new();
        event.record(&mut visitor);

        // println!("{:?}", visitor.entries());

        // Format the event, if it has at least one message
        let res = visitor.entries().get("message");
        if res.is_some() {

            // Format values from the event's metadata:
            let metadata = event.metadata();

            // Print thread id
            let thread_id = thread::current().id();
            let thread_id_string = format!("{:?}", thread_id);
            // println!("{}", thread_id_string);
            let re = Regex::new(r"ThreadId\((\d+)\)").unwrap();
            let caps = re.captures(&thread_id_string).unwrap();
            let id_number = &caps[1];

            // Get class name
            let class_name = visitor.entries()
                .get("class")
                .or(Some(&"".to_string()))
                .and_then(|s| Some(String::from(s)))
                .unwrap();

            let i1 = visitor.entries()
                .get("i1")
                .or(Some(&"".to_string()))
                .and_then(|s| Some(String::from(s)))
                .unwrap();

            let i2 = visitor.entries()
                .get("i2")
                .or(Some(&"".to_string()))
                .and_then(|s| Some(String::from(s)))
                .unwrap();

            let i3 = visitor.entries()
                .get("i3")
                .or(Some(&"".to_string()))
                .and_then(|s| Some(String::from(s)))
                .unwrap();

            // timestamp
            // Level (debug/info/warning…)
            // class
            // i1
            // i2
            // i3
            // message
            // thread/task
            let message = res.unwrap();
            write!(&mut writer, "{},{},{},{},{},{},{},{}",
                Utc::now().to_rfc3339().to_string(),
                metadata.level().as_str(), 
                class_name,
                i1, i2, i3,
                message, id_number)?;
        }

        // Return the formatted event
        writeln!(writer)
    }
}