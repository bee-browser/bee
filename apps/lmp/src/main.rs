use std::io::{BufRead, BufReader};
use std::fs::File;

use anyhow::Result;
use structopt::StructOpt;

use bee_layout::service::{JsonSink, MessageInterpreter};

/// Layout message processor.
#[derive(Debug, StructOpt)]
#[structopt(name = "bee-lmp")]
struct Opt {
    /// Enable the debug mode.
    #[structopt(short, long)]
    debug: bool,

    /// Input file
    #[structopt(name = "FILE")]
    input: Option<String>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.input {
        Some(ref input) => interpret(&opt, BufReader::new(File::open(input)?)),
        None => interpret(&opt, std::io::stdin().lock()),
    }
}

fn interpret<T: BufRead>(opt: &Opt, read: T) -> Result<()> {
    let mut interp = MessageInterpreter::new(JsonPrinter);

    for line in read.lines() {
        match line {
            Ok(line) => interp.interpret(&line)?,
            Err(_) => break,
        }
    }

    if opt.debug {
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
