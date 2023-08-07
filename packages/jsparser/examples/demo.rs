use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;
use tracing_subscriber::filter::EnvFilter;

use bee_jsparser::JsParser;

/// Parse a JavaScript script.
#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine {
    /// Logging format.
    #[arg(long, value_enum, env = "BEE_LOG_FORMAT", default_value = "text")]
    log_format: LogFormat,

    /// A path to a JavaScript file.
    #[arg()]
    script_file: Option<PathBuf>,
}

#[derive(Clone, ValueEnum)]
enum LogFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let cl = CommandLine::parse();

    match cl.log_format {
        LogFormat::Text => {
            tracing_subscriber::fmt()
                .with_writer(std::io::stderr)
                .with_env_filter(EnvFilter::from_default_env())
                .init();
        }
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .json()
                .with_writer(std::io::stderr)
                .with_env_filter(EnvFilter::from_default_env())
                .init();
        }
    }

    let script = match cl.script_file {
        Some(ref file) => std::fs::read_to_string(file)?,
        None => {
            let mut script = String::new();
            std::io::stdin().read_to_string(&mut script)?;
            script
        }
    };

    let now = std::time::Instant::now();
    let mut parser = JsParser::new(&script);
    if parser.parse() {
        println!("Parsed successfully: bytes={} elapsed={}", script.len(), humantime::format_duration(now.elapsed()));
    } else {
        println!("Failed parsing: bytes={} elapsed={}", script.len(), humantime::format_duration(now.elapsed()));
    }

    Ok(())
}
