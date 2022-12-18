use bee_htmltokenizer::Tokenizer;
use crate::treebuilder::TreeBuilder;

pub struct Parser {
    tokenizer: Tokenizer<TreeBuilder>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokenizer: Tokenizer::new(TreeBuilder::new()),
        }
    }

    pub fn feed_data(&mut self, data: Vec<u16>) {
        self.tokenizer.feed_data(data);
    }

    pub fn feed_end(&mut self) {
        self.tokenizer.feed_end();
    }
}
