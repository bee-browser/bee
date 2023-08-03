use std::io::Read;

use anyhow::Result;
use clap::Parser;

use bee_jsparser::JsParser;

#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine;

/// `bee-jslexer-demo` reads a JavaScript source text from STDIN and prints
/// recognized tokens to STDOUT.
///
/// `bee-jslexer-demo` cannot recognize tokens in real-world JavaScript files.
/// Because ES2022 requires that we have to switch the goal symbol recognized by
/// the lexer while parsing the source file depending on a parsing context, but
/// `bee-jslexer-demo` doesn't implement such a function.
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let _cl = CommandLine::parse();

    let mut script = String::new();
    std::io::stdin().read_to_string(&mut script)?;

    let mut parser = JsParser::new(&script);
    parser.parse();

    Ok(())
}
