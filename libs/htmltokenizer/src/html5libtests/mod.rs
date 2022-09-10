use glob::glob;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use super::*;

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

#[derive(Debug, PartialEq)]
#[derive(Deserialize)]
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
    }
}

fn tokenize(
    input: &[u16],
    initial_state: InitialState,
    last_start_tag: Option<String>,
) -> (Vec<Output>, Vec<Error>) {
    let mut outputs = vec![];
    let mut errors = vec![];
    let mut tokenizer = Tokenizer::new();
    tokenizer.set_initial_state(initial_state);
    if let Some(tag_name) = last_start_tag {
        tokenizer.set_last_start_tag(tag_name);
    }
    tokenizer.feed_data(input.into());
    tokenizer.feed_end();
    loop {
        match tokenizer.next_token() {
            Ok(Token::Doctype) => {
                outputs.push(Output::Doctype {
                    name: tokenizer.doctype_name().map(|s| s.to_string()),
                    public_id: tokenizer.doctype_public_id().map(|s| s.to_string()),
                    system_id: tokenizer.doctype_system_id().map(|s| s.to_string()),
                    force_quirks: tokenizer.force_quirks(),
                });
            }
            Ok(Token::StartTag) => {
                let name = tokenizer.tag_name().to_string();
                let attrs = HashMap::from_iter(
                    tokenizer.attrs().map(|(name, value)| {
                        (name.to_string(), value.to_string())
                    }));
                let self_closing = tokenizer.is_empty_tag();
                outputs.push(Output::StartTag {
                    name, attrs, self_closing
                });
            }
            Ok(Token::EndTag) => {
                let name = tokenizer.tag_name().to_string();
                outputs.push(Output::EndTag { name });
            },
            Ok(Token::Text) => {
                let text = tokenizer.text();
                match outputs.last_mut() {
                    Some(Output::Character { ref mut data }) => {
                        data.push_str(text);
                    }
                    _ => {
                        outputs.push(Output::Character {
                            data: text.to_string(),
                        });
                    }
                }
            },
            Ok(Token::Comment) => {
                let text = tokenizer.comment();
                match outputs.last_mut() {
                    Some(Output::Comment { ref mut data }) => {
                        data.push_str(text);
                    }
                    _ => {
                        outputs.push(Output::Comment {
                            data: text.to_string(),
                        });
                    }
                }
            },
            Ok(Token::End) => break,
            Err(err) => {
                errors.push(err);
            }
        }
    }
    (outputs, errors)
}

// We use a single test function examining all test cases in html5lib-tests.
//
// Creating a separate test function for each test case takes a long time in
// link time.  So, we decided to use the single test function.
#[test]
fn test() {
    const PATTERN: &str =
        concat!(env!("CARGO_MANIFEST_DIR"),
                "/src/tokenizer/html5libtests/*.codegen.json");

    for json_file in glob(PATTERN).unwrap() {
        match json_file {
            Ok(path) => {
                eprintln!("Loading {:?}...", path);
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let test_suite: TestSuite =
                    serde_json::from_reader(reader).unwrap();
                for (index, test) in test_suite.tests.iter().enumerate() {
                    for initial_state in test.initial_states.iter().cloned() {
                        eprintln!("Testing {}#{} ({:?})...: {}",
                                  test_suite.name, index, initial_state,
                                  test.description);
                        let (output, errors) =
                            tokenize(&test.input, initial_state,
                                     test.last_start_tag.clone());
                        assert_eq!(output, test.output);
                        assert_eq!(errors, test.errors);
                    }
                }
            }
            Err(_err) => panic!(),
        }
    }
}
