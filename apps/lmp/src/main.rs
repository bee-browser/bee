use std::io::BufRead;

use anyhow::Result;
use structopt::StructOpt;

use bee_layout::service::{JsonSink, MessageInterpreter};

/// Layout message processor.
#[derive(Debug, StructOpt)]
#[structopt(name = "bee-lmp")]
struct Opt {
    /// Enable the debug mode.
    #[structopt(short, long)]
    debug: bool
}

fn main() -> Result<()> {
    let _opt = Opt::from_args();

    let mut interp = MessageInterpreter::new(JsonPrinter);

    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => interp.interpret(&line)?,
            Err(_) => break,
        }
    }

    Ok(())
}

struct JsonPrinter;

impl JsonSink for JsonPrinter {
    fn consume(&mut self, json: serde_json::Value) {
        println!("{}", json);
    }
}
