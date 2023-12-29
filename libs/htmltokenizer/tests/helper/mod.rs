//<coverage:exclude>
use std::collections::HashMap;

use serde::Deserialize;

use bee_htmltokenizer::token::*;
use bee_htmltokenizer::Error;
use bee_htmltokenizer::InitialState;
use bee_htmltokenizer::Tokenizer;

pub fn tokenize(json: &'static str) {
    let test: TestCase = serde_json::from_str(json).unwrap();
    println!("{}", test.description);
    println!("input: {}", test.input);
    let mut validator = Validator::new(&test);
    let mut tokenizer = Tokenizer::new();
    tokenizer.set_initial_state(match test.initial_state.as_str() {
        "Data" => InitialState::Data,
        "Rcdata" => InitialState::Rcdata,
        "Rawtext" => InitialState::Rawtext,
        "ScriptData" => InitialState::ScriptData,
        "CdataSection" => InitialState::CdataSection,
        "Plaintext" => InitialState::Plaintext,
        _ => unreachable!(),
    });
    if let Some(ref tag_name) = test.last_start_tag {
        tokenizer.set_last_start_tag(tag_name.as_str());
    }
    tokenizer.feed_data(&test.input_utf16);
    tokenizer.feed_end();
    loop {
        match tokenizer.next_token() {
            Token::Doctype(doctype) => validator.handle_doctype(doctype),
            Token::StartTag(tag) => validator.handle_start_tag(tag),
            Token::EndTag(tag) => validator.handle_end_tag(tag),
            Token::Comment(comment) => validator.handle_comment(comment),
            Token::Null(text) => validator.handle_text(text),
            Token::Whitespace(text) => validator.handle_text(text),
            Token::Text(text) => validator.handle_text(text),
            Token::End => {
                validator.handle_end();
                break;
            }
            Token::Error(err) => validator.handle_error(err),
        };
    }
}

struct Validator<'a> {
    test: &'a TestCase,
    output: Vec<Output>,
    errors: Vec<ExpectedError>,
}

impl<'a> Validator<'a> {
    fn new(test: &'a TestCase) -> Self {
        Validator {
            test,
            output: vec![],
            errors: vec![],
        }
    }
}

impl<'a> Validator<'a> {
    fn handle_doctype(&mut self, doctype: Doctype) {
        self.output.push(Output::Doctype {
            name: doctype.name.map(|s| s.to_string()),
            public_id: doctype.public_id.map(|s| s.to_string()),
            system_id: doctype.system_id.map(|s| s.to_string()),
            force_quirks: doctype.force_quirks,
        });
    }

    fn handle_start_tag(&mut self, tag: Tag) {
        self.output.push(Output::StartTag {
            name: tag.name.to_string(),
            attrs: HashMap::from_iter(
                tag.attrs()
                    .map(|(name, value)| (name.to_string(), value.to_string())),
            ),
            self_closing: tag.self_closing,
        });
    }

    fn handle_end_tag(&mut self, tag: Tag) {
        self.output.push(Output::EndTag {
            name: tag.name.to_string(),
        });
    }

    fn handle_text(&mut self, text: Text) {
        match self.output.last_mut() {
            Some(Output::Character { ref mut data }) => {
                data.push_str(text.data);
            }
            _ => {
                self.output.push(Output::Character {
                    data: text.data.to_string(),
                });
            }
        }
    }

    fn handle_comment(&mut self, comment: Comment) {
        match self.output.last_mut() {
            Some(Output::Comment { ref mut data }) => {
                data.push_str(comment.data);
            }
            _ => {
                self.output.push(Output::Comment {
                    data: comment.data.to_string(),
                });
            }
        }
    }

    fn handle_end(&mut self) {
        assert_eq!(self.output, self.test.output);
        assert_eq!(self.errors, self.test.errors);
    }

    fn handle_error(&mut self, err: Error) {
        self.errors.push(ExpectedError {
            code: format!("{}", err.code),
            location: Location {
                line: err.location.line,
                column: err.location.column,
            },
        });
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestCase {
    description: String,
    input: String,
    input_utf16: Vec<u16>,
    output: Vec<Output>,
    initial_state: String,
    last_start_tag: Option<String>,
    errors: Vec<ExpectedError>,
}

#[derive(Debug, PartialEq, Deserialize)]
enum Output {
    StartTag {
        name: String,
        attrs: HashMap<String, String>,
        self_closing: bool,
    },
    EndTag {
        name: String,
    },
    Character {
        data: String,
    },
    Comment {
        data: String,
    },
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        force_quirks: bool,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
struct ExpectedError {
    code: String,
    location: Location,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Location {
    line: usize,
    column: usize,
}
//</coverage:exclude>
