use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use clap::Parser;

use bee_layout::service::JsonSink;
use bee_layout::service::MessageInterpreter;

/// Layout message processor.
#[derive(Debug, Parser)]
struct CommandLine {
    /// Enable the debug mode.
    #[arg(short, long)]
    debug: bool,

    /// Input file.
    #[arg()]
    input: Option<String>,
}

fn main() -> Result<()> {
    let cl = CommandLine::parse();

    match cl.input {
        Some(ref input) => interpret(&cl, BufReader::new(File::open(input)?)),
        None => interpret(&cl, std::io::stdin().lock()),
    }
}

fn interpret<T: BufRead>(cl: &CommandLine, read: T) -> Result<()> {
    let mut interp = MessageInterpreter::new(JsonPrinter);

    for line in read.lines() {
        match line {
            Ok(line) => interp.interpret(&line)?,
            Err(_) => break,
        }
    }

    if cl.debug {
        interp.inspect(&mut std::io::stderr())?;
    }

    Ok(())
}

struct JsonPrinter;

impl JsonSink for JsonPrinter {
    fn consume(&mut self, json: serde_json::Value) {
        println!("{}", json);
    }
}
