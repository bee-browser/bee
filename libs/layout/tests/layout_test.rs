use std::fs::File;
use std::io::{BufRead, BufReader};

use assert_json_diff::assert_json_eq;
use serde_json;

use bee_layout::service::{JsonSink, MessageInterpreter};

fn layout_test(scenario_path: &str, expected_path: &str) {
    let validator = JsonValidator::new(expected_path);
    let mut interp = MessageInterpreter::new(validator);

    let file = File::open(scenario_path).unwrap_or_else(|err| {
        panic!("{}: {}", scenario_path, err);  //<coverage:exclude/>
    });

    BufReader::new(file).lines().for_each(|line| {
        let line = line.unwrap_or_else(|err| {
            panic!("{}: {}", scenario_path, err);  //<coverage:exclude/>
        });
        interp.interpret(&line).unwrap();
    });
}

struct JsonValidator {
    expected: serde_json::Value,
    actual: serde_json::Value,
}

impl JsonValidator {
    fn new(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|err| {
            panic!("{}: {}", path, err);  //<coverage:exclude/>
        });
        let expected = serde_yaml::from_reader(BufReader::new(file)).unwrap_or_else(|err| {
            panic!("{}: {}", path, err);  //<coverage:exclude/>
        });
        let actual = serde_json::Value::Array(Vec::new());
        JsonValidator { expected, actual }
    }
}

impl Drop for JsonValidator {
    fn drop(&mut self) {
        assert_json_eq!(&self.actual, &self.expected);
    }
}

impl JsonSink for JsonValidator {
    fn consume(&mut self, json: serde_json::Value) {
        if let serde_json::Value::Array(ref mut v) = self.actual {
            v.push(json);
        }
    }
}

include!(concat!(env!("BEE_CARGO_CODEGEN_DIR"), "/libs/layout/tests/layout_test.rs"));
