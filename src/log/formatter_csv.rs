use std::{fmt, thread};
use regex::Regex;
use tracing_core::{Event, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext
};
use tracing_subscriber::registry::LookupSpan;

use crate::log::hash_visitor::HashVisitor;


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

        // Format the event, if it has at least one message
        let res = visitor.entries().get("message");
        if res.is_some() {

            // Format values from the event's metadata:
            let metadata = event.metadata();

            // // Print thread id
            // let thread_id = thread::current().id();
            // let thread_id_string = format!("{:?}", thread_id);
            // // println!("{}", thread_id_string);
            // let re = Regex::new(r"ThreadId\((\d+)\)").unwrap();
            // let caps = re.captures(&thread_id_string).unwrap();
            // let id_number = &caps[1];
            // write!(&mut writer, "-{}-", id_number )?;

            // Get class name
            let class_name = visitor.entries()
                .get("class")
                .or(Some(&"".to_string()))
                .and_then(|s| Some(String::from(s)))
                .unwrap();

            // match class_opt {
            //     Some(class_name) => {
            //         match class_name.trim_matches('"') {
            //             "Platform" => {
            //                 write!(&mut writer, "{}", "[P] ".to_string() )?;
            //             },
            //             "Factory" => {
            //                 write!(&mut writer, "{}", "[F] ".to_string() )?;
            //             },
            //             "Connection" => {
            //                 let f = format!("[{}] ", visitor.entries().get("cname").unwrap().trim_matches('"'));
            //                 write!(&mut writer, "{}", f )?;
            //             },
            //             "Device" => {
            //                 let f = format!("[{}/{}] ", 
            //                     visitor.entries().get("bname").unwrap().trim_matches('"'),
            //                     visitor.entries().get("dname").unwrap().trim_matches('"')
            //                 );
            //                 write!(&mut writer, "{}", f )?;
            //             },
            //             "Interface" => {
            //                 let f = format!("[{}/{}/{}] ",
            //                     visitor.entries().get("bname").unwrap().trim_matches('"'),
            //                     visitor.entries().get("dname").unwrap().trim_matches('"'),
            //                     visitor.entries().get("iname").unwrap().trim_matches('"')
            //                 );
            //                 write!(&mut writer, "{}", f )?;
            //             },
            //             _ => {}
            //         }
            //     },
            //     None => {}
            // }

            // timestamp
            // Level (debug/info/warning…)
            // class
            // i1
            // i2
            // i3
            // message
            // thread/task
            let message = res.unwrap();
            write!(&mut writer, "{},{},{}", metadata.level().as_str(), class_name, message)?;
        }

        // Return the formatted event
        writeln!(writer)
    }
}