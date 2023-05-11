mod cursor;
mod dfa;
mod goals;
mod lexer;
mod tokens;

pub use goals::Goal as JsLexerGoal;
pub use lexer::Lexer as JsLexer;
pub use tokens::Token as JsToken;
pub use tokens::TokenKind as JsTokenKind;
