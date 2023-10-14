mod lexer;
mod parser;

/// A JavaScript parser.
pub use parser::Parser as JsParser;

/// A goal symbol that a JavaScript parser recognizes.
pub use parser::GoalSymbol as JsGoalSymbol;
