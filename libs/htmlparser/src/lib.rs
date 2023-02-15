mod parser;
mod treebuilder;

pub use crate::parser::Parser;
pub use crate::treebuilder::DocumentWriter;

pub use bee_htmltokenizer::token::Comment;
pub use bee_htmltokenizer::token::Doctype;
pub use bee_htmltokenizer::token::Tag;
pub use bee_htmltokenizer::token::Text;
