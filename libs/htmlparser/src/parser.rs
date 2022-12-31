use crate::treebuilder::TreeBuilder;
use bee_htmltokenizer::Tokenizer;

pub struct Parser {
    tokenizer: Tokenizer,
    tree_builder: TreeBuilder,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokenizer: Tokenizer::new(),
            tree_builder: TreeBuilder::new(),
        }
    }

    pub fn feed_data(&mut self, data: Vec<u16>) {
        self.tokenizer.feed_data(data);
    }

    pub fn feed_end(&mut self) {
        self.tokenizer.feed_end();
    }
}
