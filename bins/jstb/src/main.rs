use std::io::Read;

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
    /// Runs a JavaScript program.
    Run(Run),
}

#[derive(clap::Args)]
struct Run {
    /// Prints the compiled module (LLVM-IR) to STDOUT.
    ///
    /// lli cannot interpret the module directly.  Because it includes unresolved symbols for the
    /// runtime function calls.  At this point, there is no command-line option to output anything
    /// containing the runtime functions which can link to the module.
    ///
    /// The module will never be evaluated when this flag is specified.
    #[arg(long)]
    print_module: bool,

    /// The source text of the JavaScript program to run.
    ///
    /// Reads the source text from STDIN if this argument is not specified.
    #[arg()]
    source: Option<String>,
}

fn main() -> Result<()> {
    logging::init();
    Runtime::initialize();
    let cl = CommandLine::parse();
    let mut runtime = Runtime::new().with_host_function("print", print);
    match cl.command {
        Command::Run(run) => {
            let source = match run.source {
                Some(source) => source,
                None => read_from_stdin()?,
            };
            let module = runtime.compile_script(&source).unwrap();
            if run.print_module {
                module.print(false); // to STDOUT
            } else {
                runtime.eval(module);
            }
        }
    }
    Ok(())
}

fn read_from_stdin() -> Result<String> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;
    Ok(source)
}

fn print(args: &[Value]) {
    println!("{:?}", args[0]);
}
