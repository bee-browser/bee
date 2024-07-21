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
    /// Parses a JavaScript program and print the result.
    Parse(Parse),

    /// Compiles a JavaScript program and print the compiled module.
    ///
    /// `lli` cannot interpret the module directly.  Because it includes unresolved symbols for the
    /// runtime function calls.  At this point, there is no command-line option to output anything
    /// containing the runtime functions which can link to the module.
    Compile(Compile),

    /// Runs a JavaScript program.
    Run(Run),
}

#[derive(clap::Args)]
struct Parse {
    /// Prints information.
    ///
    /// (f)unctions, (s)cope-tree
    #[arg(short, long)]
    print: String,

    /// The source file of the JavaScript program to parse.
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

    /// The source file of the JavaScript program to compile.
    ///
    /// Reads the source text from STDIN if this argument is not specified.
    #[arg()]
    source: Option<PathBuf>,
}

#[derive(clap::Args)]
struct Run {
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
        Command::Parse(args) => {
            let source = read_source(args.source.as_ref())?;
            let program = runtime.parse_script(&source)?;
            for kind in args.print.chars() {
                match kind {
                    'f' => {
                        println!("### functions");
                        program.print_functions("");
                    }
                    's' => {
                        println!("### scope tree");
                        program.print_scope_tree("");
                    }
                    _ => (),
                }
            }
            Ok(())
        }
        Command::Compile(args) => {
            let source = read_source(args.source.as_ref())?;
            let program = runtime.parse_script(&source)?;
            let module = runtime.compile(&program, !args.no_optimize)?;
            module.print(false); // to STDOUT
            Ok(())
        }
        Command::Run(args) => {
            let source = read_source(args.source.as_ref())?;
            let program = runtime.parse_script(&source)?;
            let module = runtime.compile(&program, !args.no_optimize)?;
            match runtime.evaluate(module) {
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
