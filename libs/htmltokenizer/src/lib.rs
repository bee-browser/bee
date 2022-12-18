mod charref;
mod error;
mod inputstream;
mod tokenizer;

#[cfg(test)]
mod html5libtests;

use std::fmt;
use match_cfg::match_cfg;

use bee_htmltags::HtmlTag;

pub use crate::error::Error;
pub use crate::error::ErrorCode;
pub use crate::tokenizer::Attrs;
pub use crate::tokenizer::InitialState;
pub use crate::tokenizer::Token;
pub use crate::tokenizer::Tokenizer;

pub trait TokenHandler {
    #[allow(unused_variables)]
    fn handle_doctype(&mut self, name: Option<&str>, public_id: Option<&str>, system_id: Option<&str>, force_quirks: bool) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn handle_start_tag(&mut self, name: TagKind, attrs: Attrs<'_>, self_closing: bool) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn handle_end_tag(&mut self, name: TagKind) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn handle_text(&mut self, text: &str) -> bool {
        for c in text.chars() {
            if !self.handle_character(c) {
                return false;
            }
        }
        true
    }

    #[allow(unused_variables)]
    fn handle_character(&mut self, c: char) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn handle_comment(&mut self, comment: &str) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn handle_end(&mut self) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn handle_error(&mut self, err: Error) -> bool {
        true
    }
}

pub enum TagKind<'a> {
    Html(HtmlTag),
    Other(&'a str),
}

match_cfg! {
    #[cfg(test)] => {
        use serde::Deserialize;

        #[derive(Clone, Copy, Debug, PartialEq)]
        #[derive(Deserialize)]
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
                Location {
                    line: 1,
                    column: 1,
                }
            }
        }

        impl fmt::Display for Location {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Line#{} Column#{}", self.line, self.column)
            }
        }
    }
    _ => {
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Location;

        impl Location {
            pub fn incr(&mut self) {}
            pub fn incr_line(&mut self) {}
            pub fn offset(&self, _: i32) -> Location { Location }
        }

        impl fmt::Display for Location {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "(No location data)")
            }
        }
    }
}
