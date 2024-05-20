use std::fs::File;
use std::io;

use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;


use std::io::stdout;
use std::io::Write;
use tracing_subscriber::fmt::Layer;

use super::formatter_csv::FormatterCSV;

struct LogIssueMultiWriter {
    filea: tracing_appender::rolling::RollingFileAppender
}

impl LogIssueMultiWriter {
    pub fn new() -> Self {
        LogIssueMultiWriter{
            filea: tracing_appender::rolling::never(".", "log.csv")
        }
    }
}

impl std::io::Write for LogIssueMultiWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();


        self.filea.write_all(buf).unwrap();

        print!("{}", String::from_utf8_lossy(buf));
        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.filea.flush().unwrap();
        Ok(())
    }
}

/// Configuration for Github/Gitlab issue logger
///
pub fn init_fmt_subscriber_for_log_issue()
{

 
    let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    // Display source code line numbers
    .with_line_number(false)
    // Display the thread ID an event was recorded on
    .with_thread_ids(true)
    // Don't display the event's target (module path)
    .with_target(false)
    // No span
    .with_span_events(FmtSpan::NONE)
    // 
    .event_format(FormatterCSV{})
    //
    .with_writer(||LogIssueMultiWriter::new())
    // .and(stdout_writer)
    //
    .finish();


    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
