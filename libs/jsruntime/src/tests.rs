use super::*;

macro_rules! eval {
    ($src:expr, $expected:expr) => {
        Runtime::initialize();
        let mut runtime = Runtime::new().with_host_function("print", |_, args| {
            // Some cases including `f64::NAN` fail in `assert_eq!()`.
            let actual = format!("{:?}", args[0]);
            let expected = format!("{:?}", Value::from($expected));
            assert_eq!(actual, expected);
        });
        let module = runtime.compile_script($src.as_ref(), true).unwrap();
        runtime.eval(module);
    };
}

#[test]
fn test_eval_undefined() {
    eval!("print(undefined)", Value::UNDEFINED);
}

#[test]
fn test_eval_null() {
    eval!("print(null)", Value::NULL);
}

#[test]
fn test_eval_boolean() {
    eval!("print(true)", true);
    eval!("print(false)", false);
}

#[test]
fn test_eval_number() {
    eval!("print(1)", 1.);
    eval!("print(NaN)", f64::NAN);
    eval!("print(Infinity)", f64::INFINITY);
}

#[test]
fn test_eval_addition_expression() {
    eval!("print(1 + 2)", 3.);
}

#[test]
fn test_eval_subtraction_expression() {
    eval!("print(2 - 1)", 1.);
    eval!("print(1 - 2)", -1.);
}

#[test]
fn test_eval_multiplication_expression() {
    eval!("print(2 * 3)", 6.);
}

#[test]
fn test_eval_division_expression() {
    eval!("print(4 / 2)", 2.);
    eval!("print(1 / 3)", 1. / 3.);
}

#[test]
fn test_eval_remainder_expression() {
    eval!("print(1 % 3)", 1.);
    eval!("print(1.2 % 3.4)", 1.2 % 3.4);
}

#[test]
fn test_eval_group_expression() {
    eval!("print(2 * (3 + 4))", 14.);
}

#[test]
fn test_eval_left_shift() {
    eval!("print(NaN << 1)", 0);
    eval!("print(Infinity << 1)", 0);
    //eval!("print(-Infinity << 0)", 0); TODO: unary -
    eval!("print(0 << 1)", 0);
    eval!("print(1 << 1)", 2);
    eval!("print(0.9 << 1)", 0);
    eval!("print(1.1 << 1)", 2);
    //eval!("print(-1 << 1)", -2); TODO: unary -
    eval!("print(1 << 32)", 1);
}

#[test]
fn test_eval_signed_right_shift() {
    eval!("print(NaN >> 1)", 0);
    eval!("print(Infinity >> 1)", 0);
    //eval!("print(-Infinity >> 0)", 0); TODO: unary -
    eval!("print(0 >> 1)", 0);
    eval!("print(4 >> 1)", 2);
    eval!("print(3.9 >> 1)", 1);
    eval!("print(4.1 >> 1)", 2);
    //eval!("print(-4 >> 1)", -2); TODO: unary -
    eval!("print(1 >> 32)", 1);
}

#[test]
fn test_eval_unsigned_right_shift() {
    eval!("print(NaN >>> 1)", 0);
    eval!("print(Infinity >>> 1)", 0);
    //eval!("print(-Infinity >>> 0)", 0); TODO: unary -
    eval!("print(0 >>> 1)", 0);
    eval!("print(4 >>> 1)", 2);
    eval!("print(3.9 >>> 1)", 1);
    eval!("print(4.1 >>> 1)", 2);
    //eval!("print(-4 >>> 1)", 2147483646); TODO: unary -
    eval!("print(1 >> 32)", 1);
}

#[test]
fn test_eval_to_numeric() {
    eval!("print(undefined + 0)", f64::NAN);
    eval!("print(null + 0)", 0.);
    eval!("print(false + 0)", 0.);
    eval!("print(true + 0)", 1.);
}

#[test]
fn test_eval_call_with_no_argument() {
    eval!("function a() { return 1 } print(a())", 1.);
}

#[test]
fn test_eval_call_with_no_argument_hoistable_declaration() {
    eval!("print(a()); function a() { return 1 }", 1.);
}

#[test]
fn test_eval_const_declaration() {
    eval!("const a = 1, b = 2; print(a);", 1.);
    eval!("const a = 1, b = 2; print(b);", 2.);
}

#[test]
fn test_eval_let_declaration() {
    eval!("let a; print(a);", Value::UNDEFINED);
    eval!("let a, b = 2; a = 1; print(a);", 1.);
    eval!("let a, b = 2; a = 1; print(b);", 2.);
}

#[test]
fn test_eval_arithmetic_operations_with_variables() {
    eval!("const a = 1, b = 2, c = 3; print(a + b * c);", 7.);
}

#[test]
fn test_eval_conditional_expression() {
    eval!("print(1 > 0 ? 2 : 3)", 2.);
    eval!("print(1 < 0 ? 2 : 3)", 3.);
    eval!("print(1 > 0 ? true : false)", true);
    eval!("print(1 < 0 ? true : false)", false);
    eval!("print(1 > 0 ? undefined : undefined)", Value::UNDEFINED);
    eval!("print(1 < 0 ? undefined : undefined)", Value::UNDEFINED);
}

#[test]
fn test_eval_nested_conditional_expression() {
    eval!("print(1 > 0 ? 1 > 0 ? 2 : 3 : 1 > 0 ? 4 : 5)", 2.);
}

#[test]
fn test_eval_conditional_expression_mixed_types() {
    eval!("print(true ? 2.0 : false)", 2.);
    eval!("print(false ? 2.0 : false)", false);
    eval!("print(true ? 2.0 : undefined)", 2.);
    eval!("print(false ? 2.0 : undefined)", Value::UNDEFINED);
}

#[test]
fn test_eval_if_statement() {
    eval!("let a = 1; if (true) { a = 2; } print(a);", 2.);
    eval!("let a = 1; if (false) { a = 2; } print(a);", 1.);
}

#[test]
fn test_eval_if_else_statement() {
    eval!(
        "let a = 1; if (true) { a = 2; } else { a = 3; } print(a);",
        2.
    );
    eval!(
        "let a = 1; if (false) { a = 2; } else { a = 3; } print(a);",
        3.
    );
}

#[test]
fn test_eval_block_statement() {
    eval!("let a = 1; { let a = 2; } print(a);", 1.);
    eval!("let a = 1; { a = 2; } print(a);", 2.);
}

#[test]
fn test_eval_return_statement_in_block() {
    eval!("print(a()); function a() { let a = 1; { return a; } }", 1.);
}

#[test]
fn test_eval_terminated_basic_block() {
    eval!(
        "print(a()); function a() { if (1) { return 1; } return 2; }",
        1.
    );
}

#[test]
fn test_eval_function_single_name_binding() {
    eval!("print(a(1)); function a(x) { return x; }", 1.);
}

#[test]
fn test_eval_call_other_function() {
    eval!(
        "print(a()); function a() { return b() } function b() { return 1 }",
        1.
    );
}

#[test]
fn test_eval_nested_function() {
    eval!(
        "print(a()); function a() { return b(); function b() { return 1 } }",
        1.
    );
}

#[test]
fn test_eval_argument_in_outer_function() {
    eval!(
        "print(a(1)); function a(x) { return b(); function b() { return x } }",
        1.
    );
}

#[test]
fn test_eval_fibonacci() {
    eval!(
        "print(fib(10)); \
         function fib(n) { if (n < 2) return n; return fib(n - 1) + fib(n - 2); }",
        55.
    );
}
