use super::*;

#[test]
fn test_eval_number() {
    const A: f64 = 1.0;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A);
    }

    eval(
        format!("{A}"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_addition_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A + B);
    }

    eval(
        format!("{A} + {B}"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_subtraction_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A - B);
    }

    eval(
        format!("{A} - {B}"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_multiplication_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A * B);
    }

    eval(
        format!("{A} * {B}"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_division_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A / B);
    }

    eval(
        format!("{A} / {B}"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_remainder_expression() {
    const A: f64 = 1.0;
    const B: f64 = 3.0;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A % B);
    }

    eval(
        format!("{A} % {B}"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_group_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    const C: f64 = 5.6;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A * (B + C));
    }

    eval(
        format!("{A} * ({B} + {C})"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_call_with_no_argument() {
    const A: f64 = 1.2;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A);
    }

    eval(
        format!("function a() {{ return {A}; }} a();"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_call_with_no_argument_hoistable_declaration() {
    const A: f64 = 1.2;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A);
    }

    eval(
        format!("a(); function a() {{ return {A}; }}"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_const_declaration() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A);
    }

    eval(
        format!("const a={A},b={B}; a;"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_let_declaration() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A);
    }

    eval(
        format!("let a,b={B}; a={A}; a;"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_arithmetic_operations_with_variables() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    const C: f64 = 5.6;

    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, A + B * C);
    }

    eval(
        format!("const a={A},b={B},c={C}; a+b*c;"),
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_conditional_expression() {
    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, 2.);
    }

    eval(
        "1 > 0 ? 2 : 3",
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_nested_conditional_expression() {
    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, 2.);
    }

    eval(
        "1 > 0 ? 1 > 0 ? 2 : 3 : 1 > 0 ? 4 : 5",
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_if_statement() {
    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, 2.);
    }

    eval(
        "let a = 1; if (1) { a = 2; } a;",
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_if_else_statement() {
    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, 3.);
    }

    eval(
        "let a = 1; if (0) { a = 2; } else { a = 3; } a;",
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_block_statement() {
    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, 1.);
    }

    eval(
        "let a = 1; { let a = 2; } a;",
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_return_statement_in_block() {
    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, 1.);
    }

    eval(
        "a(); function a() { let a = 1; { return a; } }",
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

#[test]
fn test_eval_terminated_basic_block() {
    unsafe extern "C" fn validate(value: f64) {
        assert_eq!(value, 1.);
    }

    eval(
        "a(); function a() { if (1) { return 1; } return 2; }",
        llvmir::bridge::Host {
            print_f64: Some(validate),
            ..Default::default()
        },
    );
}

fn eval<T: AsRef<str>>(source: T, host: llvmir::bridge::Host) {
    Runtime::initialize();
    let mut runtime = Runtime::with_host(host);
    let module = runtime.compile_script(source.as_ref()).unwrap();
    runtime.eval(module);
}
