//<coverage:exclude>
use glob::glob;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::token::*;
use super::*;

// We use a single test function examining all test cases in html5lib-tests.
//
// Creating a separate test function for each test case takes a long time in
// link time.  So, we decided to use the single test function.
#[test]
fn test() {
    const PATTERN: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/html5libtests/*.codegen.json"
    );

    tracing_subscriber::fmt::init();

    for json_file in glob(PATTERN).unwrap() {
        match json_file {
            Ok(path) => {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let test_suite: TestSuite = serde_json::from_reader(reader).unwrap();
                for (index, test) in test_suite.tests.iter().enumerate() {
                    for initial_state in test.initial_states.iter().cloned() {
                        tracing::debug!(
                            "{}#{} ({:?}): {}",
                            test_suite.name,
                            index,
                            initial_state,
                            test.description
                        );
                        let validator = Validator::new(test);
                        tokenize(
                            &test.input,
                            initial_state,
                            test.last_start_tag.clone(),
                            validator,
                        );
                    }
                }
            }
            Err(_err) => panic!(),
        }
    }
}

fn tokenize(
    input: &[u16],
    initial_state: InitialState,
    last_start_tag: Option<String>,
    mut validator: Validator,
) {
    let mut tokenizer = Tokenizer::new();
    tokenizer.set_initial_state(initial_state);
    if let Some(tag_name) = last_start_tag {
        tokenizer.set_last_start_tag(tag_name);
    }
    tokenizer.feed_data(input.into());
    tokenizer.feed_end();
    loop {
        match tokenizer.next_token() {
            Token::Doctype(doctype) => validator.handle_doctype(doctype),
            Token::StartTag(tag) => validator.handle_start_tag(tag),
            Token::EndTag(tag) => validator.handle_end_tag(tag),
            Token::Text(text) => validator.handle_text(text),
            Token::Comment(comment) => validator.handle_comment(comment),
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
    errors: Vec<Error>,
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
            name: tag.name().to_string(),
            attrs: HashMap::from_iter(
                tag.attrs()
                    .map(|(name, value)| (name.to_string(), value.to_string())),
            ),
            self_closing: tag.self_closing,
        });
    }

    fn handle_end_tag(&mut self, tag: Tag) {
        self.output.push(Output::EndTag {
            name: tag.name().to_string(),
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
        self.errors.push(err);
    }
}

#[derive(Deserialize)]
struct TestSuite {
    name: String,
    tests: Vec<TestCase>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestCase {
    description: String,
    input: Vec<u16>,
    output: Vec<Output>,
    initial_states: Vec<InitialState>,
    last_start_tag: Option<String>,
    errors: Vec<Error>,
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
//</coverage:exclude>
