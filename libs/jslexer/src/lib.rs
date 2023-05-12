mod cursor;
mod dfa;
mod goals;
mod lexer;
mod tokens;

/// A goal symbol that a JavaScript lexer recognizes.
pub use goals::Goal as JsLexerGoal;

/// A JavaScript lexer.
pub use lexer::Lexer as JsLexer;

/// A token recognized by a JavaScript lever.
pub use tokens::Token as JsToken;

/// Token types.
pub use tokens::TokenKind as JsTokenKind;
