use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;

use jsruntime::BasicRuntime;
use jsruntime::Value;

#[derive(clap::Parser)]
struct CommandLine {
    #[command(subcommand)]
    command: Command,

    /// Enables the scope cleanup checker.
    #[arg(global = true, long)]
    scope_cleanup_checker: bool,

    /// The source file of the JavaScript program to compile.
    ///
    /// Reads the source text from STDIN if this argument is not specified.
    #[arg(global = true)]
    source: Option<PathBuf>,
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
}

#[derive(clap::Args)]
struct Compile {
    /// Disable optimization.
    #[arg(long)]
    no_optimize: bool,
}

#[derive(clap::Args)]
struct Run {
    /// Disable optimization.
    #[arg(long)]
    no_optimize: bool,
}

fn main() -> Result<()> {
    logging::init();

    let cl = CommandLine::parse();

    jsruntime::initialize();

    let mut runtime = BasicRuntime::new();
    if cl.scope_cleanup_checker {
        runtime.enable_scope_cleanup_checker();
    }
    runtime.register_host_function("print", print);

    let source = read_source(cl.source.as_ref())?;

    match cl.command {
        Command::Parse(args) => {
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
        }
        Command::Compile(args) => {
            let program = runtime.parse_script(&source)?;
            let module = runtime.compile(&program, !args.no_optimize)?;
            module.print(false); // to STDOUT
        }
        Command::Run(args) => {
            let program = runtime.parse_script(&source)?;
            let module = runtime.compile(&program, !args.no_optimize)?;
            if let Err(v) = runtime.evaluate(module) {
                println!("Uncaught {v:?}");
            }
        }
    }

    Ok(())
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

fn print(_runtime: &mut BasicRuntime, args: &[Value]) {
    println!("{args:?}");
}
