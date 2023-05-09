mod esgrammar;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about)]
struct Opt {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Extracts grammar from an ECMA specification.
    Esgrammar(esgrammar::Opt),
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    match opt.command {
        Some(Command::Esgrammar(opt)) => esgrammar::main(opt),
        None => Ok(()),
    }
}
