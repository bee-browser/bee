use std::io::Read;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser as _;

use jsparser::Error;
use jsparser::Node;
use jsparser::NodeHandler;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::SymbolTable;

/// Parse a JavaScript script.
#[derive(clap::Parser)]
#[command(author, version, about)]
struct CommandLine {
    /// Parse as an ES module.
    #[arg(short, long)]
    module: bool,

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

    let now = std::time::Instant::now();
    let mut parser = if cl.module {
        Parser::for_module(&script, Processor::new(NullHandler::default(), true))
    } else {
        Parser::for_script(&script, Processor::new(NullHandler::default(), false))
    };
    match parser.parse() {
        Ok(_) => {
            let elapsed = now.elapsed().as_micros();
            let bytes = script.len();
            let stack_depth = parser.max_stack_depth();
            let template_literal_depth = parser.max_template_literal_depth();
            println!(
                "time={} size={} max-stack-depth={} max-template-literal-depth={}",
                elapsed, bytes, stack_depth, template_literal_depth
            );
            Ok(())
        }
        Err(_) => Err(anyhow!("Parse error")),
    }
}

#[derive(Default)]
struct NullHandler(SymbolTable);

impl<'s> NodeHandler<'s> for NullHandler {
    type Artifact = ();

    #[inline(always)]
    fn start(&mut self) {}

    #[inline(always)]
    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        Ok(())
    }

    #[inline(always)]
    fn handle_nodes(&mut self, _nodes: impl Iterator<Item = Node<'s>>) -> Result<(), Error> {
        Ok(())
    }

    #[inline(always)]
    fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.0
    }
}
