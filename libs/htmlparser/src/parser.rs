use bee_htmltokenizer::Error;
use bee_htmltokenizer::Token;
use bee_htmltokenizer::Tokenizer;

pub struct Parser {
    tokenizer: Tokenizer,
    opened_tags: Vec<String>,
    depth: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokenizer: Tokenizer::new(),
            opened_tags: vec![],
            depth: 0,
        }
    }

    pub fn feed_data(&mut self, data: Vec<u16>) {
        self.tokenizer.feed_data(data);
    }

    pub fn feed_end(&mut self) {
        self.tokenizer.feed_end();
    }

    pub fn next_event(&mut self) -> Result<Event, Error> {
        const EMPTY_TAG_NAMES: &[&str] = &[
            "meta"
        ];
        match self.tokenizer.next_token()? {
            Token::Doctype { .. }=> {
                self.depth = self.opened_tags.len();
                Ok(Event::Doctype)
            }
            Token::StartTag { name, self_closing, .. } => {
                self.depth = self.opened_tags.len();
                if !self_closing && !EMPTY_TAG_NAMES.contains(&name) {
                    self.opened_tags.push(name.to_string());
                }
                Ok(Event::StartTag)
            }
            Token::EndTag { .. } => {
                self.opened_tags.pop();
                self.depth = self.opened_tags.len();
                Ok(Event::EndTag)
            }
            Token::Text { .. } => {
                self.depth = self.opened_tags.len();
                Ok(Event::Text)
            }
            Token::Comment { .. } => {
                self.depth = self.opened_tags.len();
                Ok(Event::Comment)
            }
            Token::End => {
                Ok(Event::End)
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

pub enum Event {
    Doctype,
    StartTag,
    EndTag,
    Text,
    Comment,
    End,
}
