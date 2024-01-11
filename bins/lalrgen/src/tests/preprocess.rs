use pretty_assertions::assert_eq;

use crate::grammar::Grammar;
use crate::preprocess::preprocess;

logging::init!();

macro_rules! impl_test {
    ($test:ident, $grammar:literal, $expected:literal) => {
        #[test]
        fn $test() {
            let actual = preprocess(&load_grammar($grammar));
            let expected = load_grammar($expected);
            assert_eq!(
                actual.non_terminals().count(),
                expected.non_terminals().count()
            );
            for non_terminal in actual.non_terminals() {
                let actual = actual.non_terminal_rules(non_terminal);
                let expected = expected.non_terminal_rules(non_terminal);
                assert_eq!(actual.len(), expected.len());
                for (actual, expected) in actual.iter().zip(expected) {
                    assert_eq!(actual.name, expected.name);
                    assert_eq!(actual.production, expected.production);
                }
            }
        }
    };
}

impl_test! {test_0000, "preprocess_0000.yaml", "preprocess_0000.expected.yaml"}

fn load_grammar(grammar: &str) -> Grammar {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src").join("tests").join(grammar);
    let file = std::fs::File::open(path).unwrap();
    let rules = serde_yaml::from_reader(file).unwrap();
    Grammar::new(rules)
}
