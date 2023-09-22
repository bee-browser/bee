use std::io::Read;
use std::path::PathBuf;

use anyhow::anyhow;
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

    // The source text should be a UTF-8 character sequence, but invalid UTF-8 character may
    // appear.  So, we firstly read it as a byte sequence.
    let raw = match cl.script_file {
        Some(ref file) => std::fs::read(file)?,
        None => {
            let mut raw = vec![];
            std::io::stdin().read_to_end(&mut raw)?;
            raw
        }
    };

    // And then convert it into a UTF-8 string loosely.
    let script = String::from_utf8_lossy(&raw);

    let now = std::time::Instant::now();
    let mut parser = JsParser::new(&script);
    if parser.parse() {
        let elapsed = now.elapsed().as_micros();
        let bytes = script.len();
        let depth = parser.max_stack_depth();
        println!("time={elapsed} size={bytes} max-stack-depth={depth}");
        Ok(())
    } else {
        Err(anyhow!("Parse error"))
    }
}
