use std::io::Read;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser as _;
use clap::ValueEnum;
use tracing_subscriber::filter::EnvFilter;

use bee_jsparser::Parser;
use bee_jsparser::ProductionRule;
use bee_jsparser::SyntaxHandler;
use bee_jsparser::Token;

/// Parse a JavaScript script.
#[derive(clap::Parser)]
#[command(author, version, about)]
struct CommandLine {
    /// Parse as an ES module.
    #[arg(short, long)]
    module: bool,

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
    let mut parser = if cl.module {
        Parser::for_module(&script, NullHandler)
    } else {
        Parser::for_script(&script, NullHandler)
    };
    match parser.parse() {
        Ok(_) => {
            let elapsed = now.elapsed().as_micros();
            let bytes = script.len();
            let stack_depth = parser.max_stack_depth();
            let template_literal_depth = parser.max_template_literal_depth();
            println!(
                "time={} size={} max-stack-depth={} max-template-literal-depth={}",
                elapsed, bytes, stack_depth, template_literal_depth
            );
            Ok(())
        }
        Err(_) => Err(anyhow!("Parse error")),
    }
}

struct NullHandler;

impl SyntaxHandler for NullHandler {
    type Artifact = ();
    type Error = std::convert::Infallible;
    fn start(&mut self) {}
    fn accept(&mut self) -> Result<Self::Artifact, Self::Error> {
        Ok(())
    }
    fn shift<'a>(&mut self, _token: &Token<'a>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn reduce(&mut self, _rule: ProductionRule) -> Result<(), Self::Error> {
        Ok(())
    }
    fn error(&mut self) {}
}
