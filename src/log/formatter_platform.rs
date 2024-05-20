use std::{fmt, thread};
use colored::Colorize;
use regex::Regex;
use tracing_core::{Event, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext
};
use tracing_subscriber::registry::LookupSpan;

use crate::log::hash_visitor::HashVisitor;


/// Color words in quotes
/// 
fn color_words_in_quotes(input: &str) -> String {
    let mut in_quotes = false;
    let mut result = String::new();
    let mut word = String::new();
    let mut prev_char = '\0';

    for c in input.chars() {
        match c {
            '"' if prev_char != '\\' => {
                word.push(c);
                in_quotes = !in_quotes;
                if !in_quotes {
                    result.push_str(&word.yellow().to_string());
                    word.clear();
                }
            }
            _ => {
                if in_quotes {
                    word.push(c);
                } else {
                    result.push(c);
                }
            }
        }
        prev_char = c;
    }

    result
}


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

            // Print thread id
            let thread_id = thread::current().id();
            let thread_id_string = format!("{:?}", thread_id);
            // println!("{}", thread_id_string);
            let re = Regex::new(r"ThreadId\((\d+)\)").unwrap();
            let caps = re.captures(&thread_id_string).unwrap();
            let id_number = &caps[1];
            write!(&mut writer, "-{}-", id_number )?;

            // Display class
            let class_opt = visitor.entries().get("class");
            match class_opt {
                Some(class_name) => {
                    match class_name.trim_matches('"') {
                        "Platform" => {
                            write!(&mut writer, "{}", "[P] ".to_string().red() )?;
                        },
                        "Factory" => {
                            write!(&mut writer, "{}", "[F] ".to_string().magenta() )?;
                        },
                        "Connection" => {
                            let f = format!("[{}] ", visitor.entries().get("cname").unwrap().trim_matches('"'));
                            write!(&mut writer, "{}", f.blue() )?;
                        },
                        "Device" => {
                            let f = format!("[{}/{}] ", 
                                visitor.entries().get("bname").unwrap().trim_matches('"'),
                                visitor.entries().get("dname").unwrap().trim_matches('"')
                            );
                            write!(&mut writer, "{}", f.green() )?;
                        },
                        "Interface" => {
                            let f = format!("[{}/{}/{}] ",
                                visitor.entries().get("bname").unwrap().trim_matches('"'),
                                visitor.entries().get("dname").unwrap().trim_matches('"'),
                                visitor.entries().get("iname").unwrap().trim_matches('"')
                            );
                            write!(&mut writer, "{}", f.bright_cyan() )?;
                        },
                        _ => {}
                    }
                },
                None => {}
            }

            // Level
            if metadata.level() == &tracing_core::Level::ERROR {
                write!(&mut writer, "{}: ", "ERROR".red())?;
            } else if metadata.level() == &tracing_core::Level::WARN {
                write!(&mut writer, "{}: ", "WARN".yellow())?;
            }

            // Write message
            let message = res.unwrap();
            write!(&mut writer, "{}", color_words_in_quotes(message))?;
        }

        // Return the formatted event
        writeln!(writer)
    }
}