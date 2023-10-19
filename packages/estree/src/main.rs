mod builder;
mod nodes;

use std::io::BufWriter;
use std::io::Read;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser as _;
use clap::ValueEnum;
use tracing_subscriber::filter::EnvFilter;

use bee_jsparser::Parser;

use crate::builder::Builder;

/// Show the ESTree of a JavaScript program.
#[derive(clap::Parser)]
pub struct CommandLine {
    /// Parse the JavaScript program as an ES module.
    #[arg(short, long)]
    module: bool,

    /// Logging format.
    #[arg(long, value_enum, env = "BEE_LOG_FORMAT", default_value = "text")]
    log_format: LogFormat,

    /// A path to the file containing the JavaScript program.
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

    let mut parser = if cl.module {
        Parser::for_module(&script, Builder::new())
    } else {
        Parser::for_script(&script, Builder::new())
    };

    let program = match parser.parse() {
        Ok(program) => program,
        Err(_) => {
            return Err(anyhow!("Parse error"));
        }
    };

    let writer = BufWriter::new(std::io::stdout());
    serde_json::to_writer(writer, &program)?;

    Ok(())
}
