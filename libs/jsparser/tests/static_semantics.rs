use assert_matches::assert_matches;
use jsparser::Error;
use jsparser::Node;
use jsparser::NodeHandler;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::Symbol;
use jsparser::SymbolRegistry;

macro_rules! parse_fail {
    (script: $js:literal) => {
        let src = include_str!($js);
        let processor = Processor::new(NullHandler::default(), false);
        let mut parser = Parser::for_script(src, processor);
        parse_fail!(parser);
    };
    (module: $js:literal) => {
        let src = include_str!($js);
        let processor = Processor::new(NullHandler::default(), true);
        let mut parser = Parser::for_module(src, processor);
        parse_fail!(parser);
    };
    ($parser:expr) => {
        assert_matches!($parser.parse(), Err(Error::SyntaxError));
    };
}

#[test]
fn test_13_2_5_1_cover_initialized_name() {
    // TODO
    parse_fail!(script: "static_semantics_13_2_5_1_cover_initialized_name.js");
}

#[test]
fn test_13_2_5_1_duplicate_proto() {
    parse_fail!(script: "static_semantics_13_2_5_1_duplicate_proto.js");
}

#[test]
fn test_13_4_1_postfix_increment() {
    parse_fail!(script: "static_semantics_13_4_1_postfix_increment.js");
}

#[test]
fn test_13_4_1_postfix_decrement() {
    parse_fail!(script: "static_semantics_13_4_1_postfix_decrement.js");
}

#[test]
fn test_13_4_1_prefix_increment() {
    parse_fail!(script: "static_semantics_13_4_1_prefix_increment.js");
}

#[test]
fn test_13_4_1_prefix_decrement() {
    parse_fail!(script: "static_semantics_13_4_1_prefix_decrement.js");
}

#[test]
fn test_14_3_1_1_missing_initializer() {
    parse_fail!(script: "static_semantics_14_3_1_1_missing_initializer.js");
}

#[test]
fn test_14_3_1_1_already_declared() {
    parse_fail!(script: "static_semantics_14_3_1_1_already_declared.js");
}

#[test]
fn test_14_3_1_1_let() {
    parse_fail!(script: "static_semantics_14_3_1_1_let.js");
}

#[test]
fn test_14_3_1_1_let_init() {
    parse_fail!(script: "static_semantics_14_3_1_1_let_init.js");
}

#[test]
fn test_14_8_1() {
    parse_fail!(script: "static_semantics_14_8_1.js");
}

#[test]
fn test_14_8_1_label() {
    parse_fail!(script: "static_semantics_14_8_1_label.js");
}

#[test]
fn test_14_9_1() {
    parse_fail!(script: "static_semantics_14_9_1.js");
}

#[test]
fn test_14_9_1_label() {
    parse_fail!(script: "static_semantics_14_9_1_label.js");
}

#[test]
fn test_15_1_1() {
    parse_fail!(script: "static_semantics_15_1_1.js");
}

#[test]
fn test_continue_statement_with_label_not_in_label_set() {
    parse_fail!(script: "static_semantics_continue_statement_with_label_not_in_label_set.js");
}

#[test]
fn test_continue_statement_with_label_of_non_iteration_statement() {
    parse_fail!(script: "static_semantics_continue_statement_with_label_of_non_iteration_statement.js");
}

#[test]
fn test_break_statement_with_label_not_in_label_set() {
    parse_fail!(script: "static_semantics_break_statement_with_label_not_in_label_set.js");
}

#[derive(Default)]
struct NullHandler(SymbolRegistry);

impl<'s> NodeHandler<'s> for NullHandler {
    type Artifact = ();

    fn start(&mut self) {}

    fn accept(&mut self) -> Result<Self::Artifact, Error> {
        Ok(())
    }

    fn handle_nodes(&mut self, _nodes: impl Iterator<Item = Node<'s>>) -> Result<(), Error> {
        Ok(())
    }

    fn make_symbol(&mut self, lexeme: &str) -> Symbol {
        self.0.intern_str(lexeme)
    }
}
