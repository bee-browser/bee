#[macro_use]
mod macros;

mod localnames;
mod parser;
mod treebuilder;

pub use crate::parser::Parser;
pub use crate::treebuilder::DomTreeBuilder;
pub use crate::treebuilder::Namespace;
pub use crate::treebuilder::QuirksMode;

pub use bee_htmltokenizer::token::Comment;
pub use bee_htmltokenizer::token::Doctype;
pub use bee_htmltokenizer::token::Tag;
pub use bee_htmltokenizer::token::Text;
