use std::fmt;
// use std::thread;
use colored::Colorize;
use tracing::Metadata;
// use regex::Regex;
use tracing_core::{Event, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields, Writer},
    FmtContext,
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

/// Send in stdout content and level of log message
///
fn write_log_message(
    metadata: &Metadata<'static>,
    mut writer: Writer,
    res: Option<&String>,
) -> fmt::Result {
    // Level
    if metadata.level() == &tracing_core::Level::ERROR {
        write!(&mut writer, "{}: ", "ERROR".red())?;
    } else if metadata.level() == &tracing_core::Level::WARN {
        write!(&mut writer, "{}: ", "WARN".yellow())?;
    }

    // Write message
    let message = res.unwrap();
    write!(&mut writer, "{}", color_words_in_quotes(message))?;

    return writeln!(writer);
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
            // let thread_id = thread::current().id();
            // let thread_id_string = format!("{:?}", thread_id);
            // println!("{}", thread_id_string);
            // let re = Regex::new(r"ThreadId\((\d+)\)").unwrap();
            // let caps = re.captures(&thread_id_string).unwrap();
            // let id_number = &caps[1];
            // write!(&mut writer, "-{}-", id_number )?;

            // Display class
            let class_opt = visitor.entries().get("class");
            match class_opt {
                Some(class_name) => {
                    // Log platform except broker
                    if cfg!(feature = "log") {
                        match class_name.trim_matches('"') {
                            "Platform" => {
                                write!(&mut writer, "{}", "[P] ".to_string().red())?;
                            }
                            "Factory" => {
                                write!(&mut writer, "{}", "[F] ".to_string().magenta())?;
                            }
                            "Connector" => {
                                let f = format!(
                                    "[{}/{}/{}] ",
                                    visitor.entries().get("i1").unwrap().trim_matches('"'),
                                    visitor.entries().get("i2").unwrap().trim_matches('"'),
                                    visitor.entries().get("i3").unwrap().trim_matches('"')
                                );
                                write!(&mut writer, "{}", f.purple())?;
                            }
                            "Device" => {
                                let f = format!(
                                    "[{}] ",
                                    visitor.entries().get("i1").unwrap().trim_matches('"')
                                );
                                write!(&mut writer, "{}", f.green())?;
                            }
                            "Interface" => {
                                let f = format!(
                                    "[{}/{}/{}] ",
                                    visitor.entries().get("i1").unwrap().trim_matches('"'),
                                    visitor.entries().get("i2").unwrap().trim_matches('"'),
                                    visitor.entries().get("i3").unwrap().trim_matches('"')
                                );
                                write!(&mut writer, "{}", f.bright_cyan())?;
                            }
                            _ => {}
                        }
                    }
                }
                None => {
                    // Broker message
                    if cfg!(feature = "broker-log") {
                        write!(&mut writer, "{}", "[BROKER] ".to_string())?;
                    }
                }
            }

            // If broker message show it only if broker-log is activated
            if class_opt.is_none() {
                if cfg!(feature = "broker-log") {
                    return write_log_message(metadata, writer, res);
                }
            } else {
                // Level
                if cfg!(feature = "log") {
                    return write_log_message(metadata, writer, res);
                }
            }
        }

        // Return the formatted event
        // writeln!(writer)
        Ok(())
    }
}
