logging::define_logger! {"bee::estree"}

mod builder;
mod nodes;

use std::io::BufRead;
use std::io::BufWriter;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;
use clap::Subcommand;
use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;

use jsparser::Error;
use jsparser::Parser;

use crate::builder::Builder;
use crate::nodes::NodeRef;

// An ESTree cannot represent in JSON.  Because an ESTree may contain values such as `Infinity`
// that cannot be used in JSON.  At this point, JSON5 can handle those values.

/// Show the ESTree of a JavaScript program in JSON5.
#[derive(clap::Parser)]
pub struct CommandLine {
    #[command(subcommand)]
    command: Command,
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
    /// Parse a JavaScript program.
    Parse {
        /// Parse the JavaScript program as an ES module.
        #[arg()]
        source_type: SourceType,

        /// A path to the file containing the JavaScript program.
        #[arg()]
        source_file: Option<PathBuf>,
    },
    /// Start a server that responds to requests to parse JavaScript programs.
    Serve,
}

fn main() -> Result<()> {
    logging::init();

    let cl = CommandLine::parse();

    match cl.command {
        Command::Parse {
            source_type,
            source_file,
        } => parse(source_type, source_file),
        Command::Serve => serve(),
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

    let node = parse_program(source_type, &source)?;
    let writer = BufWriter::new(std::io::stdout());
    // It's better to dump the ESTree when json5::to_string() fails.
    // However, "{node:$?}" takes a long time if the ESTree is large...
    serde_json::to_writer(writer, &node)?;
    println!();

    Ok(())
}

fn parse_program(source_type: SourceType, source: &str) -> std::result::Result<NodeRef, Error> {
    match source_type {
        SourceType::Script => Parser::for_script(source, Builder::new()).parse(),
        SourceType::Module => Parser::for_module(source, Builder::new()).parse(),
    }
}

// In the server mode, a parsing error doesn't stop the loop and the error is reported in the
// response.  Tests take long time to complete if the server restarts every time an error happens.
fn serve() -> Result<()> {
    let reader = std::io::stdin().lock();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let req: Request = match serde_json::from_str(&line) {
                    Ok(req) => req,
                    Err(err) => {
                        logger::error!(%err, "Failed to parse JSON");
                        continue;
                    }
                };
                let now = std::time::Instant::now();
                let result = parse_program(req.source_type, &req.source);
                let elapsed = now.elapsed().as_nanos() as u64;
                let writer = BufWriter::new(std::io::stdout());
                serde_json::to_writer(
                    writer,
                    &result.map_or_else(
                        |err| Response {
                            program: None,
                            error: Some(format!("{err:?}")),
                            elapsed,
                        },
                        |program| Response {
                            program: Some(program),
                            error: None,
                            elapsed,
                        },
                    ),
                )?;
                println!();
            }
            Err(_) => break,
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Request {
    #[serde(rename = "sourceType")]
    source_type: SourceType,
    source: String,
}

#[derive(Debug, Serialize)]
struct Response {
    program: Option<NodeRef>,
    error: Option<String>,
    elapsed: u64,
}
