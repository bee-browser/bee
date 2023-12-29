mod extract;

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
    /// Extracts grammar from an ECMA specification written in HTML.
    Extract(extract::Opt),
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    match opt.command {
        Some(Command::Extract(opt)) => extract::main(opt),
        None => Ok(()),
    }
}
