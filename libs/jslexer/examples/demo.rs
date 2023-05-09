use std::io::Read;

use anyhow::Result;

use bee_jslexer::JsLexer;
use bee_jslexer::JsTokenKind;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut js = String::new();
    std::io::stdin().read_to_string(&mut js)?;

    let mut line = 1;
    let mut column = 1;

    let mut lexer = JsLexer::new(&js);
    loop {
        let token = lexer.next_token();
        match token.kind {
            JsTokenKind::LineTerminatorSequence => {
                line += 1;
                column = 1;
            }
            JsTokenKind::Eof => break,
            _ => {
                println!("Line#{line} Column#{column}: {:?}", token);
                column += token.lexeme.chars().count();
            }
        }
    }

    Ok(())
}
