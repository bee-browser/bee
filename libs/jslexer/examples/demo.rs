use std::io::Read;

use anyhow::Result;

use bee_jslexer::JsLexer;
use bee_jslexer::JsToken;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut js = String::new();
    std::io::stdin().read_to_string(&mut js)?;

    let mut lexer = JsLexer::new(&js);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if let JsToken::Eof = token {
            break;
        }
    }

    Ok(())
}
