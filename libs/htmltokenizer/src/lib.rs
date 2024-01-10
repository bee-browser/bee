mod charref;
mod error;
mod inputstream;
mod logger;
pub mod token;
mod tokenizer;

use std::fmt;

pub use crate::error::Error;
pub use crate::error::ErrorCode;
pub use crate::tokenizer::InitialState;
pub use crate::tokenizer::Tokenizer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn incr(&mut self) {
        self.column += 1;
    }

    pub fn incr_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    pub fn offset(&self, offset: i32) -> Location {
        Location {
            line: self.line,
            column: if offset < 0 {
                self.column - (-offset) as usize
            } else {
                self.column + offset as usize
            },
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location { line: 1, column: 1 }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line#{} Column#{}", self.line, self.column)
    }
}
