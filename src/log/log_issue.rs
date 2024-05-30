
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use tracing_subscriber::fmt::format::FmtSpan;

use chrono::Utc;

use std::env;
use std::process::Command;
use super::formatter_csv::FormatterCSV;


// use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use serde_json;
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION"); 

// use serde::{Deserialize, Serialize};

struct Tree {

    r#ref: String,
    name: String,
    settings: String,
}


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

    // path of json tree
    let path = "/etc/panduza/tree.json";

    // get the rustc version
    let outputRust = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("failed to excecute command");
    let rustVersion = String::from_utf8_lossy(&outputRust.stdout);

    println!("|system info| Version|");
    println!("|------------|----------|");
    println!("|rust version| {version}|", version=rustVersion.trim());
    println!("|plateform version| {pzaVersion}|", pzaVersion=VERSION);
    println!("|system information|{OS}|", OS=env::consts::OS);

    // let mut tree_file_path = PathBuf::from(dirs::home_dir().unwrap()).join("panduza").join("tree.json");
    // tree_file_path = PathBuf::from("/etc/panduza/tree.json");

    // let file_content_init = tokio::fs::read_to_string(&tree_file_path);

    // let file_path = "/etc/panduza/tree.json";
    // let file = File::open(file_path).unwrap();
    // let reader = BufReader::new(file);

    //  let u = serde_json::from_str::<serde_json::Value>(&reader);
    // println!("``` \n{:#?}\n ```", u);
    match env::consts::OS {
        "linux" => {
            let path = "/etc/panduza/tree.json";
        }
        "windows" => {

        }
        _ => {
            tracing::error!("Unsupported system!");
        }
    }
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res = serde_json::to_string_pretty(&data);
    println!("``` \n{}\n ```",serde_json::to_string_pretty(&data).unwrap());
    
    println!("- [ ] issue reproduced ");
    println!("- [ ] root cause found ");
    println!("- [ ] mpacts described (documentation/code/repos...)");
    println!("- [ ] fix implemented ? ");

}

// pub fn read_json_init(filename:String){
//     let path: &Path=Path::new(&filename);
//     let mut fData: String=String::new();
//     let mut rfile :File=File::open(path).expect("file not found");
//     rfile.read_to_string(&fData).expect("file can't be read");
// }

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

