mod lexer;
mod parser;

/// A goal symbol that a JavaScript lexer recognizes.
pub use lexer::Goal as JsLexerGoal;

/// A JavaScript lexer.
pub use lexer::Lexer as JsLexer;

/// A token recognized by a JavaScript lever.
pub use lexer::Token as JsToken;

/// Token types.
pub use lexer::TokenKind as JsTokenKind;

/// A JavaScript parser.
pub use parser::Parser as JsParser;
