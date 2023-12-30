#[macro_use]
mod macros;

mod localnames;
mod parser;
mod treebuilder;

pub use crate::parser::Parser;
pub use crate::treebuilder::DomTreeBuilder;
pub use crate::treebuilder::Namespace;
pub use crate::treebuilder::QuirksMode;

pub use htmltokenizer::token::Comment;
pub use htmltokenizer::token::Doctype;
pub use htmltokenizer::token::Tag;
pub use htmltokenizer::token::Text;
