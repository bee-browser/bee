use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;
use itertools::Itertools;

use jsruntime::Runtime;
use jsruntime::Value;

#[derive(clap::Parser)]
struct CommandLine {
    #[command(subcommand)]
    command: Command,

    /// The type of the source text.
    ///
    /// Specify `module` explicitly when the source text read from STDIN is parsed as a module.
    #[arg(
        global = true,
        long = "as",
        default_value = "auto",
        value_name = "SOURCE_TYPE"
    )]
    parse_as: SourceType,

    /// Enables the scope cleanup checker.
    #[arg(global = true, long)]
    scope_cleanup_checker: bool,

    /// Enables the scope cleanup checker.
    #[arg(global = true, long)]
    runtime_assert: bool,

    /// The JavaScript source files to compile.
    ///
    /// Reads the source text from STDIN if this argument is not specified.
    #[arg(global = true, value_parser)]
    sources: Vec<PathBuf>,
}

impl CommandLine {
    fn sources(&self) -> Sources<'_> {
        Sources::new(self)
    }
}

/// A command-line tool for testing jsruntime.
#[derive(clap::Subcommand)]
enum Command {
    /// Parses a JavaScript program and print the result.
    Parse(Parse),

    /// Compiles a JavaScript program and print the compiled module.
    ///
    /// `lli` cannot interpret the module directly.  Because it includes unresolved symbols for the
    /// runtime function calls.  At this point, there is no command-line option to output anything
    /// containing the runtime functions which can link to the module.
    ///
    /// Contextual labels for registers and basic blocks are enabled.
    Compile(Compile),

    /// Prints the CFG of a JavaScript program in the DOT format.
    PrintCfg,

    /// Runs a JavaScript program.
    Run(Run),
}

#[derive(clap::Args)]
struct Parse {
    /// Prints information.
    ///
    /// (f)unctions, (s)cope-tree
    #[arg(short, long, default_value = "")]
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
    /// List of script files to be pre-loaded
    #[arg(long, value_parser)]
    preload_scripts: Vec<PathBuf>,

    /// Disable optimization.
    #[arg(long)]
    no_optimize: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum SourceType {
    /// Parse as a script if the file extension of the input file is "js".
    /// Parse as a module if the file extension of the input file is "mjs".
    /// Otherwise, parse as a script.
    Auto,

    /// Parse as a script.
    Script,

    /// Parse as a module.
    Module,
}

fn main() -> Result<()> {
    logging::init();

    let cl = CommandLine::parse();

    jsruntime::initialize();

    let mut runtime = Runtime::with_extension(Context);
    if cl.scope_cleanup_checker {
        runtime.enable_scope_cleanup_checker();
    }
    if cl.runtime_assert {
        runtime.enable_runtime_assert();
    }
    runtime.register_host_function("print", print);

    // This is not a good practice, but we define a macro instead of a function in order to avoid
    // code clones.  By using the macro, we can avoid additional `use` directives needed for the
    // return type.
    macro_rules! parse {
        ($input:expr, $source:expr, $source_type:expr) => {{
            let result = match $source_type {
                SourceType::Auto => match $input.extension() {
                    Some(ext) if ext == "js" => runtime.parse_script(&$source),
                    Some(ext) if ext == "mjs" => runtime.parse_module(&$source),
                    _ => runtime.parse_script(&$source),
                },
                SourceType::Script => runtime.parse_script(&$source),
                SourceType::Module => runtime.parse_module(&$source),
            };
            match result {
                Ok(program_id) => program_id,
                Err(err) => {
                    println!("Failed parsing {:?}: {err:?}", $input);
                    std::process::exit(2);
                }
            }
        }};
    }

    match cl.command {
        Command::Parse(ref args) => {
            for (input, source) in cl.sources() {
                println!("## {}", input.display());
                let program_id = parse!(input, source, cl.parse_as);
                for kind in args.print.chars() {
                    match kind {
                        'f' => {
                            println!("### functions");
                            runtime.print_functions(program_id);
                        }
                        's' => {
                            println!("### scope tree");
                            runtime.print_scope_tree(program_id);
                        }
                        'g' => {
                            println!("### global symbols");
                            runtime.print_global_symbols(program_id);
                        }
                        _ => (),
                    }
                }
            }
        }
        Command::Compile(ref args) => {
            let printer = Box::new(IrPrinter);
            runtime.set_monitor(printer);
            for (input, source) in cl.sources() {
                println!("## {}", input.display());
                let program_id = parse!(input, source, cl.parse_as);
                runtime.compile(program_id, !args.no_optimize)?;
            }
        }
        Command::PrintCfg => {
            let printer = Box::new(CfgPrinter);
            runtime.set_monitor(printer);
            for (input, source) in cl.sources() {
                println!("## {}", input.display());
                let program_id = parse!(input, source, cl.parse_as);
                runtime.compile(program_id, true)?;
            }
        }
        Command::Run(ref args) => {
            for path in args.preload_scripts.iter() {
                println!("## {} (preload script)", path.display());
                let source = std::fs::read_to_string(path)?;
                let program_id = parse!(path, source, SourceType::Script);
                let result = runtime.run(program_id, !args.no_optimize);
                runtime.process_jobs();
                if let Err(v) = result {
                    anyhow::bail!("Uncaught {v:?} in {path:?}");
                }
            }
            for (input, source) in cl.sources() {
                println!("## {}", input.display());
                let program_id = parse!(input, source, cl.parse_as);
                if let Err(v) = runtime.run(program_id, !args.no_optimize) {
                    anyhow::bail!("Uncaught {v:?} in {input:?}");
                }
            }
            runtime.process_jobs();
        }
    }

    Ok(())
}

fn read_from_stdin() -> Result<String> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;
    Ok(source)
}

fn print(_runtime: &mut Runtime<Context>, args: &[Value]) {
    println!("{}", args.iter().format(" "));
}

struct IrPrinter;

impl jsruntime::Monitor for IrPrinter {
    fn print_function_ir(&mut self, id: jsruntime::LambdaId, ir: &dyn std::fmt::Display) {
        println!("### {id:?}");
        println!("{ir}");
    }
}

struct CfgPrinter;

impl jsruntime::Monitor for CfgPrinter {
    fn print_function_ir(&mut self, id: jsruntime::LambdaId, ir: &dyn std::fmt::Display) {
        println!("### {id:?}");
        let clir = format!("{ir}");
        for func in cranelift_reader::parse_functions(&clir).unwrap() {
            print!("{}", cranelift_codegen::cfg_printer::CFGPrinter::new(&func));
        }
    }
}

struct Sources<'a> {
    cl: &'a CommandLine,
    index: usize,
}

impl<'a> Sources<'a> {
    fn new(cl: &'a CommandLine) -> Self {
        Self { cl, index: 0 }
    }
}

impl<'a> Iterator for Sources<'a> {
    type Item = (&'a Path, String);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        if index == 0 && self.cl.sources.is_empty() {
            Some((Path::new("<STDIN>"), read_from_stdin().unwrap()))
        } else if index < self.cl.sources.len() {
            let path = &self.cl.sources[index];
            Some((path, std::fs::read_to_string(path).unwrap()))
        } else {
            None
        }
    }
}

struct Context;
