use assert_matches::assert_matches;

use jsruntime::Runtime;
use jsruntime::Value;

logging::init!();

#[ctor::ctor]
fn init_jsruntime() {
    jsruntime::initialize();
}

#[test]
fn test_multiple_scripts_call_func_in_other_file() {
    const SOURCES: &[(&str, bool)] = &[
        ("function a() { return 1; }", false),
        ("print(a());", false),
    ];

    let result = evaluate_multiple_programs(SOURCES, vec![Value::from(1.0)]);
    assert_matches!(result, Ok(()));
}

#[test]
fn test_multiple_scripts_call_func_in_other_file2() {
    const SOURCES: &[(&str, bool)] = &[
        ("function a() { return 1; }", false),
        ("function b() { return a(); }", false),
        ("print(b());", false),
    ];

    let result = evaluate_multiple_programs(SOURCES, vec![Value::from(1.0)]);
    assert_matches!(result, Ok(()));
}

#[test]
fn test_multiple_scripts_call_func_in_other_file3() {
    const SOURCES: &[(&str, bool)] = &[
        ("function b() { return a(); }", false),
        ("function a() { return 1; }", false),
        ("print(b());", false),
    ];

    let result = evaluate_multiple_programs(SOURCES, vec![Value::from(1.0)]);
    assert_matches!(result, Ok(()));
}

struct Validator {
    expected_values: Vec<Value>,
    actual_values: Vec<Value>,
}

impl Validator {
    fn new(expected_values: Vec<Value>) -> Self {
        Self {
            expected_values,
            actual_values: vec![],
        }
    }

    fn validate(&self) {
        assert_eq!(self.actual_values.len(), self.expected_values.len());
        for (actual, expected) in self.actual_values.iter().zip(self.expected_values.iter()) {
            match expected {
                Value::Promise(_) => assert!(matches!(actual, Value::Promise(_))),
                Value::Object(_) => assert!(matches!(actual, Value::Object(_))),
                _ => {
                    // Some cases including `f64::NAN` fail in `assert_eq!()`.
                    let actual = format!("{actual}");
                    let expected = format!("{expected}");
                    assert_eq!(actual, expected);
                }
            }
        }
    }
}

pub fn evaluate_multiple_programs(
    sources: &[(&str, bool)],
    expected_values: Vec<Value>,
) -> Result<(), Value> {
    let mut runtime = Runtime::with_extension(Validator::new(expected_values));
    runtime.enable_scope_cleanup_checker();
    runtime.register_host_function("print", |runtime, args| {
        let value = runtime.ensure_value_on_heap(&args[0]);
        runtime.extension_mut().actual_values.push(value);
    });
    for (source, module) in sources.iter().cloned() {
        let program_id = if module {
            runtime.parse_module(source).unwrap()
        } else {
            runtime.parse_script(source).unwrap()
        };
        runtime.run(program_id, true)?;
    }
    runtime.process_jobs();
    runtime.extension().validate();
    Ok(())
}
