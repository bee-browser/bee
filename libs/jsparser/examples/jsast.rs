use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;

use jsparser::Error;
use jsparser::Node;
use jsparser::NodeHandler;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::SymbolRegistry;

/// Parse a JavaScript script.
#[derive(clap::Parser)]
#[command(author, version, about)]
struct CommandLine {
    /// Parse as an ES module.
    #[arg(short, long)]
    module: bool,

    /// Print nodes.
    #[arg(short, long)]
    print: bool,

    /// A path to a JavaScript file.
    #[arg()]
    script_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    logging::init();

    let cl = CommandLine::parse();

    // The source text should be a UTF-8 character sequence, but invalid UTF-8 character may
    // appear.  So, we firstly read it as a byte sequence.
    let raw = match cl.script_file {
        Some(ref file) => std::fs::read(file)?,
        None => {
            let mut raw = vec![];
            std::io::stdin().read_to_end(&mut raw)?;
            raw
        }
    };

    // And then convert it into a UTF-8 string loosely.
    let script = String::from_utf8_lossy(&raw);
    let printer = NodePrinter::new(cl.print);

    let now = std::time::Instant::now();
    let mut parser = if cl.module {
        Parser::for_module(&script, Processor::new(printer, true))
    } else {
        Parser::for_script(&script, Processor::new(printer, false))
    };
    parser.parse()?;

    let elapsed = now.elapsed().as_micros();
    let bytes = script.len();
    let stack_depth = parser.max_stack_depth();
    let template_literal_depth = parser.max_template_literal_depth();
    println!(
        "time={elapsed} size={bytes} max-stack-depth={stack_depth} \
         max-template-literal-depth={template_literal_depth}");
    Ok(())
}

struct NodePrinter {
    symbol_registry: SymbolRegistry,
    print: bool,
}

impl NodePrinter {
    fn new(print: bool) -> Self {
        Self {
            symbol_registry: Default::default(),
            print,
        }
    }
}

impl<'s> NodeHandler<'s> for NodePrinter {
    type Artifact = ();

    fn start(&mut self) {}

    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        Ok(())
    }

    fn handle_node(&mut self, node: Node<'s>) -> Result<(), Error> {
        if self.print {
            println!("{node:?}");
        }
        Ok(())
    }

    fn make_symbol(&mut self, lexeme: &str) -> jsparser::Symbol {
        self.symbol_registry.intern_str(lexeme)
    }
}
