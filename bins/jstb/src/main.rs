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

#[derive(clap::Subcommand)]
enum Command {
    /// Evaluate an expression and print the result.
    Eval(Eval),
}

#[derive(clap::Args)]
struct Eval {
    /// Print LLVM-IR.
    #[arg(short, long)]
    debug: bool,

    /// The expression to evaluate.
    #[arg()]
    expr: Option<String>,
}

fn main() -> Result<()> {
    logging::init();
    Runtime::initialize();
    let cl = CommandLine::parse();
    let mut runtime = Runtime::new().with_host_function("print", print);
    match cl.command {
        Command::Eval(eval) => {
            let expr = match eval.expr {
                Some(expr) => expr,
                None => read_from_stdin()?,
            };
            let module = runtime.compile_script(&expr).unwrap();
            if eval.debug {
                module.dump();
            }
            runtime.eval(module);
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
