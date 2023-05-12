use std::io::Read;

use anyhow::Result;
use clap::Parser;
use clap::ValueEnum;

use bee_jslexer::JsLexer;
use bee_jslexer::JsLexerGoal;
use bee_jslexer::JsTokenKind;

#[derive(Parser)]
#[command(author, version, about)]
struct Opt {
    /// A goal symbol that the JavaScript lexer recognizes.
    #[arg(short, long, default_value = "input-element-div")]
    goal: Goal,
}

/// `bee-jslexer-demo` reads a JavaScript source text from STDIN and prints
/// recognized tokens to STDOUT.
///
/// `bee-jslexer-demo` cannot recognize tokens in real-world JavaScript files.
/// Because ES2022 requires that we have to switch the goal symbol recognized by
/// the lexer while parsing the source file depending on a parsing context, but
/// `bee-jslexer-demo` doesn't implement such a function.
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    let mut js = String::new();
    std::io::stdin().read_to_string(&mut js)?;

    let mut lexer = JsLexer::new(&js);
    lexer.set_goal(opt.goal.into());

    let mut line = 1;
    let mut column = 1;

    loop {
        let token = lexer.next_token();
        match token.kind {
            JsTokenKind::WhiteSpaceSequence => {
                column += token.lexeme.chars().count();
            }
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

#[derive(Clone, Copy, ValueEnum)]
enum Goal {
    InputElementDiv,
    InputElementRegExp,
    InputElementRegExpOrTemplateTail,
    InputElementTemplateTail,
}

impl Into<JsLexerGoal> for Goal {
    fn into(self) -> JsLexerGoal {
        match self {
            Self::InputElementDiv => JsLexerGoal::InputElementDiv,
            Self::InputElementRegExp => JsLexerGoal::InputElementRegExp,
            Self::InputElementRegExpOrTemplateTail => JsLexerGoal::InputElementRegExpOrTemplateTail,
            Self::InputElementTemplateTail => JsLexerGoal::InputElementTemplateTail,
        }
    }
}
