use std::io::BufRead;

use anyhow::Result;
use structopt::StructOpt;

use bee_layout::service::MessageInterpreter;

fn main() -> Result<()> {
    let _opt = Opt::from_args();

    let mut interp = MessageInterpreter::new();

    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => interp.interpret(&line)?,
            Err(_) => break,
        }
    }

    Ok(())
}

/// Layout message processor.
#[derive(Debug, StructOpt)]
#[structopt(name = "bee-lmp")]
struct Opt {
    /// Enable the debug mode.
    #[structopt(short, long)]
    debug: bool
}
