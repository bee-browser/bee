use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;

use jsruntime::Runtime;
use jsruntime::Value;

#[derive(clap::Parser)]
struct CommandLine {
    #[command(subcommand)]
    command: Command,
}

/// A testbed for the jsruntime module.
#[derive(clap::Subcommand)]
enum Command {
    /// Compile a JavaScript program and print the compiled module.
    ///
    /// lli cannot interpret the module directly.  Because it includes unresolved symbols for the
    /// runtime function calls.  At this point, there is no command-line option to output anything
    /// containing the runtime functions which can link to the module.
    Compile(Compile),

    /// Runs a JavaScript program.
    Run(Run),
}

#[derive(clap::Args)]
struct Run {
    /// The source file of the JavaScript program to run.
    ///
    /// Reads the source text from STDIN if this argument is not specified.
    #[arg()]
    source: Option<PathBuf>,
}

#[derive(clap::Args)]
struct Compile {
    /// Disable optimization.
    #[arg(long)]
    no_optimize: bool,

    /// The source file of the JavaScript program to run.
    ///
    /// Reads the source text from STDIN if this argument is not specified.
    #[arg()]
    source: Option<PathBuf>,
}

fn main() -> Result<()> {
    logging::init();
    Runtime::initialize();
    let cl = CommandLine::parse();
    let mut runtime = Runtime::new().with_host_function("print", print);
    match cl.command {
        Command::Compile(args) => {
            let source = read_source(args.source.as_ref())?;
            let module = match runtime.compile_script(&source, !args.no_optimize) {
                Some(module) => module,
                None => anyhow::bail!("Failed to parse"),
            };
            module.print(false); // to STDOUT
            Ok(())
        }
        Command::Run(args) => {
            let source = read_source(args.source.as_ref())?;
            // Always perform optimization.
            let module = match runtime.compile_script(&source, true) {
                Some(module) => module,
                None => anyhow::bail!("Failed to parse"),
            };
            match runtime.eval(module) {
                Ok(_) => Ok(()),
                Err(v) => anyhow::bail!("Uncaught {v:?}"),
            }
        }
    }
}

fn read_source(file: Option<&PathBuf>) -> Result<String> {
    let source = match file {
        Some(file) => std::fs::read_to_string(file)?,
        None => read_from_stdin()?,
    };
    Ok(source)
}

fn read_from_stdin() -> Result<String> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;
    Ok(source)
}

fn print(_runtime: &mut Runtime, args: &[Value]) {
    println!("{args:?}");
}
