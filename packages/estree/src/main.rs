mod builder;
mod nodes;

use std::io::BufRead;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser as _;
use clap::Subcommand;
use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use tracing_subscriber::filter::EnvFilter;

use bee_jsparser::Parser;

use crate::builder::Builder;
use crate::nodes::NodeRef;

/// Show the ESTree of a JavaScript program.
#[derive(clap::Parser)]
pub struct CommandLine {
    /// Logging format.
    #[arg(long, value_enum, env = "BEE_LOG_FORMAT", default_value = "text")]
    log_format: LogFormat,

    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, ValueEnum)]
enum LogFormat {
    Text,
    Json,
}

#[derive(Clone, Copy, Debug, Deserialize, ValueEnum)]
enum SourceType {
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "module")]
    Module,
}

#[derive(Subcommand)]
enum Command {
    Parser {
        /// Parse the JavaScript program as an ES module.
        #[arg()]
        source_type: SourceType,

        /// A path to the file containing the JavaScript program.
        #[arg()]
        source_file: Option<PathBuf>,
    },
    Server,
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

    match cl.command {
        Command::Parser { source_type, source_file } => parse(source_type, source_file),
        Command::Server => serve(),
    }
}

fn parse<P: AsRef<Path>>(source_type: SourceType, source_file: Option<P>) -> Result<()> {
    // The source text should be a UTF-8 character sequence, but invalid UTF-8 character may
    // appear.  So, we firstly read it as a byte sequence.
    let raw = match source_file {
        Some(ref file) => std::fs::read(file)?,
        None => {
            let mut raw = vec![];
            std::io::stdin().read_to_end(&mut raw)?;
            raw
        }
    };

    // And then convert it into a UTF-8 string loosely.
    let source = String::from_utf8_lossy(&raw);

    let node = match parse_program(source_type, &source) {
        Ok(node) => node,
        Err(_) => {
            return Err(anyhow!("Parse error"));
        }
    };

    let writer = BufWriter::new(std::io::stdout());
    serde_json::to_writer(writer, &node)?;

    Ok(())
}

fn parse_program(source_type: SourceType, source: &str) -> std::result::Result<NodeRef, ()> {
    match source_type {
        SourceType::Script => Parser::for_script(source, Builder::new()).parse(),
        SourceType::Module => Parser::for_module(source, Builder::new()).parse(),
    }
}

fn serve() -> Result<()> {
    let reader = std::io::stdin().lock();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let query: Query = match serde_json::from_str(&line) {
                    Ok(queru) => queru,
                    Err(err) => {
                        tracing::error!(%err);
                        continue;
                    }
                };
                let program = parse_program(query.source_type, &query.source).ok();
                let reply = Reply { program };
                let mut writer = BufWriter::new(std::io::stdout());
                let _ = serde_json::to_writer(&mut writer, &reply);
                let _ = writer.write_all(b"\n");
            }
            Err(_) => break,
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Query {
    #[serde(rename = "sourceType")]
    source_type: SourceType,
    source: String,
}

#[derive(Debug, Serialize)]
struct Reply {
    program: Option<NodeRef>,
}
