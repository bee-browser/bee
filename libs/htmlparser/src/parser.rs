use bee_htmltokenizer::Error;
use bee_htmltokenizer::Token;
use bee_htmltokenizer::Tokenizer;
use bee_htmltokenizer::Attrs;

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
            Token::Doctype => {
                self.depth = self.opened_tags.len();
                Ok(Event::Doctype)
            }
            Token::StartTag => {
                self.depth = self.opened_tags.len();
                if !self.is_empty_tag() && !EMPTY_TAG_NAMES.contains(&self.tag_name()) {
                    self.opened_tags.push(self.tag_name().to_string());
                }
                Ok(Event::StartTag)
            }
            Token::EndTag => {
                self.opened_tags.pop();
                self.depth = self.opened_tags.len();
                Ok(Event::EndTag)
            }
            Token::Text => {
                self.depth = self.opened_tags.len();
                Ok(Event::Text)
            }
            Token::Comment => {
                self.depth = self.opened_tags.len();
                Ok(Event::Comment)
            }
            Token::End => {
                Ok(Event::End)
            }
        }
    }

    pub fn doctype_name(&self) -> Option<&str> {
        self.tokenizer.doctype_name()
    }

    pub fn doctype_public_id(&self) -> Option<&str> {
        self.tokenizer.doctype_public_id()
    }

    pub fn doctype_system_id(&self) -> Option<&str> {
        self.tokenizer.doctype_system_id()
    }

    pub fn force_quirks(&self) -> bool {
        self.tokenizer.force_quirks()
    }

    pub fn tag_name(&self) -> &str {
        self.tokenizer.tag_name()
    }

    pub fn attrs(&self) -> Attrs {
        self.tokenizer.attrs()
    }

    pub fn is_empty_tag(&self) -> bool {
        self.tokenizer.is_empty_tag()
    }

    pub fn text(&self) -> &str {
        self.tokenizer.text()
    }

    pub fn comment(&self) -> &str {
        self.tokenizer.comment()
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
