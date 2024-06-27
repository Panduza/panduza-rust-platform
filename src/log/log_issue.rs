
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use tracing_subscriber::fmt::format::FmtSpan;

use chrono::Utc;

use std::env;
use std::process::Command;
use super::formatter_csv::FormatterCSV;

use std::path::PathBuf;
use serde_json;
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION"); 



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


struct MyFormatTime;

impl FormatTime for MyFormatTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{} ", Utc::now().to_rfc3339())
    }
}


pub fn display_issue_body(){


    // get the rustc version
    let output_rust = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("failed to excecute command");
    let rust_version = String::from_utf8_lossy(&output_rust.stdout);

    println!("|system info| Version|");
    println!("|------------|----------|");
    println!("|rust version| {version}|", version=rust_version.trim());
    println!("|plateform version| {pzaVersion}|", pzaVersion=VERSION);
    println!("|system information|{OS}|", OS=env::consts::OS);

    let mut path = PathBuf::from(dirs::public_dir().unwrap()).join("panduza").join("tree.json");
    match env::consts::OS {
        "linux" => {
            path = PathBuf::from("/etc/panduza/tree.json");
        }
        "windows" => {
            path = PathBuf::from( "C:/Users/UF.../Panduza/");
        }
        _ => {
            tracing::error!("Unsupported system!");
        }
    }

    let data_to_parse = fs::read_to_string(path).expect("Unable to read file");
    let json_content_init = serde_json::from_str::<serde_json::Value>(&data_to_parse);
    match json_content_init{
        Ok(json_init) => {
            let res = serde_json::to_string_pretty(&json_init).unwrap();
            println!("# device tree : ");
            println!("``` \n{res}\n ```");
        },
        Err(_) => {
            println!("failed to parse");
        }
    }

    
    println!("- [ ]  issue reproduced ");
    println!("- [ ] root cause found ");
    println!("- [ ] mpacts described (documentation/code/repos...)");
    println!("- [ ] fix implemented ? ");


}

/// Configuration for Github/Gitlab issue logger
///
pub fn init_fmt_subscriber_for_log_issue()
{

 
    let subscriber = tracing_subscriber::fmt()
    //
    .with_timer(MyFormatTime{})
    //
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
    
    //
    .finish();


    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

