// Tests contained in this file are not exhaustive.
// We use tc39/test262 for that purpose eventually.

use assert_matches::assert_matches;
use jsruntime::Runtime;
use jsruntime::Value;

logging::init!();

macro_rules! eval {
    ($src:expr, $expected:expr) => {
        Runtime::initialize();
        let mut runtime = Runtime::new().with_host_function("print", |_, args| {
            // Some cases including `f64::NAN` fail in `assert_eq!()`.
            let actual = format!("{:?}", args[0]);
            let expected = format!("{:?}", Value::from($expected));
            assert_eq!(actual, expected);
        });
        let program = runtime.parse_script($src.as_ref()).unwrap();
        let module = runtime.compile(&program, true).unwrap();
        assert_matches!(runtime.evaluate(module), Ok(_));
    };
    (file: $filename:literal, $expected:expr) => {
        let src = include_str!($filename);
        eval!(src, $expected);
    };
    ($src:expr, throws: $expected:expr) => {
        Runtime::initialize();
        let mut runtime = Runtime::new();
        let program = runtime.parse_script($src.as_ref()).unwrap();
        let module = runtime.compile(&program, true).unwrap();
        assert_matches!(runtime.evaluate(module), Err(v) => {
            // Some cases including `f64::NAN` fail in `assert_eq!()`.
            let actual = format!("{:?}", v);
            let expected = format!("{:?}", Value::from($expected));
            assert_eq!(actual, expected);
        });
    };
    (file: $filename:literal, throws: $expected:expr) => {
        let src = include_str!($filename);
        eval!(src, throws: $expected);
    };
}

#[test]
fn eval_undefined() {
    eval!("print(undefined)", Value::UNDEFINED);
}

#[test]
fn eval_null() {
    eval!("print(null)", Value::NULL);
}

#[test]
fn eval_boolean() {
    eval!("print(true)", true);
    eval!("print(false)", false);
}

#[test]
fn eval_number() {
    eval!("print(1)", 1.);
    eval!("print(NaN)", f64::NAN);
    eval!("print(Infinity)", f64::INFINITY);
}

#[test]
fn eval_addition_expression() {
    eval!("print(1 + 2)", 3.);
}

#[test]
fn eval_subtraction_expression() {
    eval!("print(2 - 1)", 1.);
    eval!("print(1 - 2)", -1.);
}

#[test]
fn eval_multiplication_expression() {
    eval!("print(2 * 3)", 6.);
}

#[test]
fn eval_division_expression() {
    eval!("print(4 / 2)", 2.);
    eval!("print(1 / 3)", 1. / 3.);
}

#[test]
fn eval_remainder_expression() {
    eval!("print(1 % 3)", 1.);
    eval!("print(1.2 % 3.4)", 1.2 % 3.4);
}

#[test]
fn eval_group_expression() {
    eval!("print(2 * (3 + 4))", 14.);
}

#[test]
fn eval_left_shift() {
    eval!("print(NaN << 1)", 0);
    eval!("print(Infinity << 1)", 0);
    eval!("print(-Infinity << 0)", 0);
    eval!("print(0 << 1)", 0);
    eval!("print(1 << 1)", 2);
    eval!("print(0.9 << 1)", 0);
    eval!("print(1.1 << 1)", 2);
    eval!("print(-1 << 1)", -2);
    eval!("print(1 << 32)", 1);
}

#[test]
fn eval_signed_right_shift() {
    eval!("print(NaN >> 1)", 0);
    eval!("print(Infinity >> 1)", 0);
    eval!("print(-Infinity >> 0)", 0);
    eval!("print(0 >> 1)", 0);
    eval!("print(4 >> 1)", 2);
    eval!("print(3.9 >> 1)", 1);
    eval!("print(4.1 >> 1)", 2);
    eval!("print(-4 >> 1)", -2);
    eval!("print(1 >> 32)", 1);
}

#[test]
fn eval_unsigned_right_shift() {
    eval!("print(NaN >>> 1)", 0);
    eval!("print(Infinity >>> 1)", 0);
    eval!("print(-Infinity >>> 0)", 0);
    eval!("print(0 >>> 1)", 0);
    eval!("print(4 >>> 1)", 2);
    eval!("print(3.9 >>> 1)", 1);
    eval!("print(4.1 >>> 1)", 2);
    eval!("print(-4 >>> 1)", 2147483646);
    eval!("print(1 >> 32)", 1);
}

#[test]
fn eval_postfix_increment() {
    eval!("let a = 0; print(a++)", 0);
    eval!("let a = 0; a++; print(a)", 1);
    //TODO: eval!("print(0++)", reference_error!());
}

#[test]
fn eval_postfix_decrement() {
    eval!("let a = 0; print(a--)", 0);
    eval!("let a = 0; a--; print(a)", -1);
    //TODO: eval!("print(0--)", reference_error!());
}

#[test]
fn eval_prefix_increment() {
    eval!("let a = 0; print(++a)", 1);
    eval!("let a = 0; ++a; print(a)", 1);
    //TODO: eval!("print(++0)", reference_error!());
}

#[test]
fn eval_prefix_decrement() {
    eval!("let a = 0; print(--a)", -1);
    eval!("let a = 0; --a; print(a)", -1);
    //TODO: eval!("print(--0)", reference_error!());
}

#[test]
fn eval_void() {
    eval!("print(void undefined)", Value::UNDEFINED);
    eval!("print(void null)", Value::UNDEFINED);
    eval!("print(void true)", Value::UNDEFINED);
    eval!("print(void false)", Value::UNDEFINED);
    eval!("print(void 0)", Value::UNDEFINED);
    eval!("print(void NaN)", Value::UNDEFINED);
    eval!("print(void Infinity)", Value::UNDEFINED);
    eval!("print(void void 0)", Value::UNDEFINED);
    eval!("const a = 1; print(void a)", Value::UNDEFINED);
}

#[test]
fn eval_unary_plus() {
    eval!("print(+undefined)", f64::NAN);
    eval!("print(+null)", 0);
    eval!("print(+true)", 1);
    eval!("print(+false)", 0);
    eval!("print(+0)", 0);
    eval!("print(+1)", 1);
    eval!("print(+(+1))", 1);
    eval!("print(+NaN)", f64::NAN);
    eval!("print(+Infinity)", f64::INFINITY);
}

#[test]
fn eval_unary_minus() {
    eval!("print(-undefined)", f64::NAN);
    eval!("print(-null)", -0.);
    eval!("print(-true)", -1);
    eval!("print(-false)", -0.);
    eval!("print(-0)", -0.);
    eval!("print(-1)", -1);
    eval!("print(-(-1))", 1);
    eval!("print(-NaN)", f64::NAN);
    eval!("print(-Infinity)", f64::NEG_INFINITY);
}

#[test]
fn eval_bitwise_not() {
    eval!("print(~undefined)", -1);
    eval!("print(~null)", -1);
    eval!("print(~true)", -2);
    eval!("print(~false)", -1);
    eval!("print(~0)", -1);
    eval!("print(~1)", -2);
    eval!("print(~-3)", 2);
    eval!("print(~NaN)", -1);
    eval!("print(~Infinity)", -1);
}

#[test]
fn eval_logical_not() {
    eval!("print(!undefined)", true);
    eval!("print(!null)", true);
    eval!("print(!true)", false);
    eval!("print(!false)", true);
    eval!("print(!0)", true);
    eval!("print(!1)", false);
    eval!("print(!-1)", false);
    eval!("print(!NaN)", true);
    eval!("print(!Infinity)", false);
}

#[test]
fn eval_equality() {
    eval_equality_inequality("==");
}

#[test]
fn eval_inequality() {
    eval_equality_inequality("!=");
}

fn eval_equality_inequality(op: &str) {
    macro_rules! eval_equality {
        ($lhs:literal, $rhs:literal) => {
            if op == "==" {
                eval!(format!("print({} == {})", $lhs, $rhs), true);
            } else {
                eval!(format!("print({} != {})", $lhs, $rhs), false);
            }
        };
    }

    macro_rules! eval_inequality {
        ($lhs:literal, $rhs:literal) => {
            if op == "==" {
                eval!(format!("print({} == {})", $lhs, $rhs), false);
            } else {
                eval!(format!("print({} != {})", $lhs, $rhs), true);
            }
        };
    }

    eval_equality!("undefined", "undefined");
    eval_equality!("undefined", "null");
    eval_equality!("true", "true");
    eval_equality!("false", "false");
    eval_equality!("0", "0");
    eval_equality!("+0", "-0");
    eval_equality!("1", "1");
    eval_equality!("Infinity", "Infinity");

    eval_inequality!("undefined", "true");
    eval_inequality!("null", "true");
    eval_inequality!("true", "false");
    eval_inequality!("0", "1");
    eval_inequality!("0", "Infinity");
    eval_inequality!("0", "NaN");
    eval_inequality!("1", "Infinity");
    eval_inequality!("1", "NaN");
    eval_inequality!("Infinity", "NaN");
    eval_inequality!("NaN", "NaN");
}

#[test]
fn eval_strict_equality() {
    eval_strict_equality_inequality("===");
}

#[test]
fn eval_strict_inequality() {
    eval_strict_equality_inequality("!==");
}

fn eval_strict_equality_inequality(op: &str) {
    macro_rules! eval_strict_equality {
        ($lhs:literal, $rhs:literal) => {
            if op == "===" {
                eval!(format!("print({} === {})", $lhs, $rhs), true);
                eval!(format!("let a = {}; print(a === {})", $lhs, $rhs), true);
                eval!(
                    format!("let a = {}, b = {}; print(a === b)", $lhs, $rhs),
                    true
                );
            } else {
                eval!(format!("print({} !== {})", $lhs, $rhs), false);
                eval!(format!("let a = {}; print(a !== {})", $lhs, $rhs), false);
                eval!(
                    format!("let a = {}, b = {}; print(a !== b)", $lhs, $rhs),
                    false
                );
            }
        };
    }

    macro_rules! eval_strict_inequality {
        ($lhs:literal, $rhs:literal) => {
            if op == "===" {
                eval!(format!("print({} === {})", $lhs, $rhs), false);
                eval!(format!("let a = {}; print(a === {})", $lhs, $rhs), false);
                eval!(
                    format!("let a = {}, b = {}; print(a === b)", $lhs, $rhs),
                    false
                );
            } else {
                eval!(format!("print({} !== {})", $lhs, $rhs), true);
                eval!(format!("let a = {}; print(a !== {})", $lhs, $rhs), true);
                eval!(
                    format!("let a = {}, b = {}; print(a !== b)", $lhs, $rhs),
                    true
                );
            }
        };
    }

    eval_strict_equality!("undefined", "undefined");
    eval_strict_equality!("null", "null");
    eval_strict_equality!("true", "true");
    eval_strict_equality!("false", "false");
    eval_strict_equality!("0", "0");
    eval_strict_equality!("+0", "-0");
    eval_strict_equality!("1", "1");
    eval_strict_equality!("Infinity", "Infinity");

    eval_strict_inequality!("undefined", "null");
    eval_strict_inequality!("undefined", "true");
    eval_strict_inequality!("undefined", "false");
    eval_strict_inequality!("undefined", "0");
    eval_strict_inequality!("undefined", "1");
    eval_strict_inequality!("undefined", "Infinity");
    eval_strict_inequality!("undefined", "NaN");
    eval_strict_inequality!("null", "true");
    eval_strict_inequality!("null", "false");
    eval_strict_inequality!("null", "0");
    eval_strict_inequality!("null", "1");
    eval_strict_inequality!("null", "Infinity");
    eval_strict_inequality!("null", "NaN");
    eval_strict_inequality!("true", "false");
    eval_strict_inequality!("true", "0");
    eval_strict_inequality!("true", "1");
    eval_strict_inequality!("true", "Infinity");
    eval_strict_inequality!("true", "NaN");
    eval_strict_inequality!("false", "0");
    eval_strict_inequality!("false", "1");
    eval_strict_inequality!("false", "Infinity");
    eval_strict_inequality!("false", "NaN");
    eval_strict_inequality!("0", "1");
    eval_strict_inequality!("0", "Infinity");
    eval_strict_inequality!("0", "NaN");
    eval_strict_inequality!("1", "Infinity");
    eval_strict_inequality!("1", "NaN");
    eval_strict_inequality!("Infinity", "NaN");
    eval_strict_inequality!("NaN", "NaN");
}

#[test]
fn eval_bitewise_and() {
    eval!("print(0 & 0)", 0);
    eval!("print(0 & 1)", 0);
    eval!("print(1 & 1)", 1);
}

#[test]
fn eval_bitewise_xor() {
    eval!("print(0 ^ 0)", 0);
    eval!("print(0 ^ 1)", 1);
    eval!("print(1 ^ 1)", 0);
}

#[test]
fn eval_bitewise_or() {
    eval!("print(0 | 0)", 0);
    eval!("print(0 | 1)", 1);
    eval!("print(1 | 1)", 1);
}

#[test]
fn eval_logical_and() {
    eval!("print(true && true)", true);
    eval!("print(true && false)", false);
    eval!("print(false && true)", false);
    eval!("print(false && false)", false);
    eval!("let a = 0; true && (a = 1); print(a)", 1);
    eval!("let a = 0; false && (a = 1); print(a)", 0);
    eval!("print(0 && 1)", 0);
    eval!("print(1 && 0)", 0);
    eval!("print(1 && 2)", 2);
}

#[test]
fn eval_logical_or() {
    eval!("print(true || true)", true);
    eval!("print(true || false)", true);
    eval!("print(false || true)", true);
    eval!("print(false || false)", false);
    eval!("let a = 0; true || (a = 1); print(a)", 0);
    eval!("let a = 0; false || (a = 1); print(a)", 1);
    eval!("print(0 || 0)", 0);
    eval!("print(0 || 1)", 1);
    eval!("print(1 || 0)", 1);
}

#[test]
fn eval_nullish_coalescing() {
    eval!("print(undefined ?? 1)", 1);
    eval!("print(null ?? 1)", 1);
    eval!("print(0 ?? 1)", 0);
    eval!("let a = 0; null ?? (a = 1); print(a)", 1);
    eval!("let a = 0; 0 ?? (a = 1); print(a)", 0);
}

#[test]
fn eval_to_numeric() {
    eval!("print(undefined + 0)", f64::NAN);
    eval!("print(null + 0)", 0.);
    eval!("print(false + 0)", 0.);
    eval!("print(true + 0)", 1.);
}

#[test]
fn eval_call_with_no_argument() {
    eval!("function a() { return 1 } print(a())", 1);
}

#[test]
fn eval_call_with_no_argument_hoistable_declaration() {
    eval!("print(a()); function a() { return 1 }", 1.);
}

#[test]
fn eval_const_declaration() {
    eval!("const a = 1, b = 2; print(a);", 1.);
    eval!("const a = 1, b = 2; print(b);", 2.);
}

#[test]
fn eval_let_declaration() {
    eval!("let a; print(a);", Value::UNDEFINED);
    eval!("let a, b = 2; a = 1; print(a);", 1.);
    eval!("let a, b = 2; a = 1; print(b);", 2.);
}

#[test]
fn eval_arithmetic_operations_with_variables() {
    eval!("const a = 1, b = 2, c = 3; print(a + b * c);", 7.);
}

#[test]
fn eval_conditional_expression() {
    eval!("print(1 > 0 ? 2 : 3)", 2.);
    eval!("print(1 < 0 ? 2 : 3)", 3.);
    eval!("print(1 > 0 ? true : false)", true);
    eval!("print(1 < 0 ? true : false)", false);
    eval!("print(1 > 0 ? undefined : undefined)", Value::UNDEFINED);
    eval!("print(1 < 0 ? undefined : undefined)", Value::UNDEFINED);
}

#[test]
fn eval_multiplication_assignment() {
    eval!("let a = 2; print(a *= 2)", 4);
    eval!("let a = 2; a *= 2; print(a)", 4);
}

#[test]
fn eval_division_assignment() {
    eval!("let a = 2; print(a /= 2)", 1);
    eval!("let a = 2; a /= 2; print(a)", 1);
}

#[test]
fn eval_remainder_assignment() {
    eval!("let a = 2; print(a %= 7)", 2);
    eval!("let a = 2; a %= 7; print(a)", 2);
}

#[test]
fn eval_addition_assignment() {
    eval!("let a = 2; print(a += 2)", 4);
    eval!("let a = 2; a += 2; print(a)", 4);
}

#[test]
fn eval_subtraction_assignment() {
    eval!("let a = 2; print(a -= 2)", 0);
    eval!("let a = 2; a -= 2; print(a)", 0);
}

#[test]
fn eval_left_shift_assignment() {
    eval!("let a = 2; print(a <<= 2)", 8);
    eval!("let a = 2; a <<= 2; print(a)", 8);
}

#[test]
fn eval_signed_right_shift_assignment() {
    eval!("let a = 4; print(a >>= 1)", 2);
    eval!("let a = 4; a >>= 1; print(a)", 2);
}

#[test]
fn eval_unsigned_right_shift_assignment() {
    eval!("let a = 4; print(a >>>= 1)", 2);
    eval!("let a = 4; a >>>= 1; print(a)", 2);
}

#[test]
fn eval_bitwise_and_assignment() {
    eval!("let a = 4; print(a &= 1)", 0);
    eval!("let a = 4; a &= 1; print(a)", 0);
}

#[test]
fn eval_bitwise_xor_assignment() {
    eval!("let a = 4; print(a ^= 5)", 1);
    eval!("let a = 4; a ^= 5; print(a)", 1);
}

#[test]
fn eval_bitwise_or_assignment() {
    eval!("let a = 4; print(a |= 1)", 5);
    eval!("let a = 4; a |= 1; print(a)", 5);
}

#[test]
fn eval_logical_and_assignment() {
    eval!("let a = 0; print(a &&= 1)", 0);
    eval!("let a = 0; a &&= 1; print(a)", 0);
    eval!("let a = 4; print(a &&= 1)", 1);
    eval!("let a = 4; a &&= 1; print(a)", 1);
}

#[test]
fn eval_logical_or_assignment() {
    eval!("let a = 0; print(a ||= 1)", 1);
    eval!("let a = 0; a ||= 1; print(a)", 1);
    eval!("let a = 4; print(a ||= 1)", 4);
    eval!("let a = 4; a ||= 1; print(a)", 4);
}

#[test]
fn eval_nullish_coalescing_assignment() {
    eval!("let a = null; print(a ??= 1)", 1);
    eval!("let a = null; a ??= 1; print(a)", 1);
    eval!("let a = 0; print(a ??= 1)", 0);
    eval!("let a = 0; a ??= 1; print(a)", 0);
}

#[test]
fn eval_nested_conditional_expression() {
    eval!("print(1 > 0 ? 1 > 0 ? 2 : 3 : 1 > 0 ? 4 : 5)", 2.);
}

#[test]
fn eval_conditional_expression_mixed_types() {
    eval!("print(true ? 2.0 : false)", 2.);
    eval!("print(false ? 2.0 : false)", false);
    eval!("print(true ? 2.0 : undefined)", 2.);
    eval!("print(false ? 2.0 : undefined)", Value::UNDEFINED);
}

#[test]
fn eval_comma_operator() {
    eval!(file: "comma_operator.js", 2);
}

#[test]
fn eval_if_statement() {
    eval!("let a = 1; if (true) { a = 2; } print(a);", 2.);
    eval!("let a = 1; if (false) { a = 2; } print(a);", 1.);
}

#[test]
fn eval_if_else_statement() {
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
fn eval_block_statement() {
    eval!("let a = 1; { let a = 2; } print(a);", 1.);
    eval!("let a = 1; { a = 2; } print(a);", 2.);
}

#[test]
fn eval_return_statement_in_block() {
    eval!("print(a()); function a() { let a = 1; { return a; } }", 1.);
}

#[test]
fn eval_terminated_basic_block() {
    eval!(
        "print(a()); function a() { if (1) { return 1; } return 2; }",
        1.
    );
}

#[test]
fn eval_function_single_name_binding() {
    eval!("print(a(1)); function a(x) { return x; }", 1.);
}

#[test]
fn eval_call_other_function() {
    eval!(
        "print(a()); function a() { return b() } function b() { return 1 }",
        1.
    );
}

#[test]
fn eval_nested_function() {
    eval!(
        "print(a()); function a() { return b(); function b() { return 1 } }",
        1.
    );
}

#[test]
fn eval_argument_in_outer_function() {
    eval!(
        "print(a(1)); function a(x) { return b(); function b() { return x } }",
        1.
    );
}

#[test]
fn eval_fibonacci() {
    eval!(
        "print(fib(10)); \
         function fib(n) { if (n < 2) return n; return fib(n - 1) + fib(n - 2); }",
        55.
    );
}

#[test]
fn eval_function_expression() {
    eval!("const a = function x() { return 1 }; print(a())", 1);
}

#[test]
fn eval_anonymous_function_expression() {
    eval!("const a = function() { return 1 }; print(a())", 1);
}

#[test]
fn eval_iife() {
    // IIFE: Immediately Invoked Function Expression
    eval!("print((function() { return 1 })())", 1);
    eval!("print((function x() { return 1 })())", 1);
}

#[test]
fn eval_arrow_function_binding_identifier_expression_body() {
    eval!(file: "arrow_function_binding_identifier_expression_body.js", 1);
}

#[test]
fn eval_arrow_function_binding_identifier_expression_body_iife() {
    eval!(file: "arrow_function_binding_identifier_expression_body_iife.js", 1);
}

#[test]
fn eval_arrow_function_binding_identifier_function_body() {
    eval!(file: "arrow_function_binding_identifier_function_body.js", 1);
}

#[test]
fn eval_arrow_function_binding_identifier_function_body_iife() {
    eval!(file: "arrow_function_binding_identifier_function_body_iife.js", 1);
}

#[test]
fn eval_arrow_function_expression_body() {
    eval!(file: "arrow_function_expression_body.js", 1);
}

#[test]
fn eval_arrow_function_expression_body_iife() {
    eval!(file: "arrow_function_expression_body_iife.js", 1);
}

#[test]
fn eval_arrow_function_function_body() {
    eval!(file: "arrow_function_function_body.js", 1);
}

#[test]
fn eval_arrow_function_function_body_iife() {
    eval!(file: "arrow_function_function_body_iife.js", 1);
}

#[test]
fn eval_arrow_function_multiple_parameters() {
    eval!(file: "arrow_function_multiple_parameters.js", 3);
}

#[test]
fn eval_arrow_function_trailing_comma() {
    eval!(file: "arrow_function_trailing_comma.js", 3);
}

#[test]
fn eval_arrow_function_empty_parameter_list() {
    eval!(file: "arrow_function_empty_parameter_list.js", 1);
}

#[test]
fn eval_do_while_statement() {
    eval!("let i = 0; do { i++ } while (i < 2); print(i)", 2);
}

#[test]
fn eval_while_statement() {
    eval!("let i = 0; while (i < 2) { i++ } print(i)", 2);
}

#[test]
fn eval_for_statement() {
    eval!(
        "let i = 0; for (let j = 0; j < 2; ++j) { i = j } print(i)",
        1
    );
}

#[test]
fn eval_for_statement_no_init() {
    eval!("let i = 0; for (; i < 2; ++i) {} print(i)", 2);
}

#[test]
fn eval_for_statement_no_test() {
    eval!("let i; for (i = 0; ; ++i) { if (i > 2) break } print(i)", 3);
}

#[test]
fn eval_for_statement_no_init_next() {
    eval!("let i = 0; for (; i < 2; ) { ++i } print(i)", 2);
}

#[test]
fn eval_for_statement_no_init_test() {
    eval!("let i = 0; for (; ; ++i) { if (i > 2) break } print(i)", 3);
}

#[test]
fn eval_for_statement_no_test_next() {
    eval!(
        "let i; for (i = 0; ; ) { if (i > 2) break; ++i } print(i)",
        3
    );
}

#[test]
fn eval_for_statement_no_init_test_next() {
    eval!("let i = 0; for (;;) { if (i > 2) break; ++i } print(i)", 3);
}

#[test]
fn eval_continue() {
    eval!("let i = 0; for (; i < 2; ++i) { continue } print(i)", 2);
}

#[test]
fn eval_deadcode_after_continue() {
    eval!(
        "let i = 0; for (; i < 2; ++i) { continue; i = 1 } print(i)",
        2
    );
}

#[test]
fn eval_break() {
    eval!("let i = 0; for (;;) { break } print(0)", 0);
}

#[test]
fn eval_deadcode_after_break() {
    eval!("let i = 0; for (;;) { break; i = 1 } print(i)", 0);
}

#[test]
fn eval_switch_statement_empty() {
    eval!(file: "switch_statement_empty.js", 0);
}

#[test]
fn eval_switch_statement_single_case_fall_through() {
    eval!(file: "switch_statement_single_case_fall_through.js", 1);
}

#[test]
fn eval_switch_statement_single_case_break() {
    eval!(file: "switch_statement_single_case_break.js", 1);
}

#[test]
fn eval_switch_statement_cases_fall_through() {
    eval!(file: "switch_statement_cases_fall_through.js", 2);
}

#[test]
fn eval_switch_statement_cases_break() {
    eval!(file: "switch_statement_cases_break.js", 1);
}

#[test]
fn eval_switch_statement_default_fall_through() {
    eval!(file: "switch_statement_default_fall_through.js", 1);
}

#[test]
fn eval_switch_statement_default_break() {
    eval!(file: "switch_statement_default_break.js", 1);
}

#[test]
fn eval_switch_statement_cases_default_fall_through() {
    eval!(file: "switch_statement_cases_default_fall_through.js", 3);
}

#[test]
fn eval_switch_statement_cases_default_break() {
    eval!(file: "switch_statement_cases_default_break.js", 2);
}

#[test]
fn eval_switch_statement_default_cases_fall_through() {
    eval!(file: "switch_statement_default_cases_fall_through.js", 3);
}

#[test]
fn eval_switch_statement_default_cases_break() {
    eval!(file: "switch_statement_default_cases_break.js", 1);
}

#[test]
fn eval_switch_statement_cases_default_cases_fall_through() {
    eval!(file: "switch_statement_cases_default_cases_fall_through.js", 3);
}

#[test]
fn eval_switch_statement_cases_default_cases_break() {
    eval!(file: "switch_statement_cases_default_cases_break.js", 2);
}

#[test]
fn eval_labelled_statement_continue_do_while() {
    eval!(file: "labelled_statement_continue_do_while.js", 2);
}

#[test]
fn eval_labelled_statement_continue_while() {
    eval!(file: "labelled_statement_continue_while.js", 2);
}

#[test]
fn eval_labelled_statement_continue_for() {
    eval!(file: "labelled_statement_continue_for.js", 2);
}

#[test]
fn eval_labelled_statement_continue_nested() {
    eval!(file: "labelled_statement_continue_nested.js", 2);
}

#[test]
fn eval_labelled_statement_break_iteration() {
    eval!(file: "labelled_statement_break_iteration.js", 0);
}

#[test]
fn eval_labelled_statement_break_switch() {
    eval!(file: "labelled_statement_break_switch.js", 1);
}

#[test]
fn eval_labelled_statement_break_nested() {
    eval!(file: "labelled_statement_break_nested.js", 0);
}

#[test]
fn eval_throw_undefined() {
    eval!(file: "throw_undefined.js", throws: Value::UNDEFINED);
}

#[test]
fn eval_throw_null() {
    eval!(file: "throw_null.js", throws: Value::NULL);
}

#[test]
fn eval_throw_true() {
    eval!(file: "throw_true.js", throws: true);
}

#[test]
fn eval_throw_false() {
    eval!(file: "throw_false.js", throws: false);
}

#[test]
fn eval_throw_number() {
    eval!(file: "throw_number.js", throws: 1);
}

#[test]
fn eval_try_catch_no_parameter() {
    eval!(file: "try_catch_no_parameter.js", 2);
}

#[test]
fn eval_try_catch() {
    eval!(file: "try_catch.js", 2);
}

#[test]
fn eval_try_catch_throw() {
    eval!(file: "try_catch_throw.js", throws: 2);
}

#[test]
fn eval_try_finally() {
    eval!(file: "try_finally.js", throws: 1);
}

#[test]
fn eval_try_finally_throw() {
    eval!(file: "try_finally_throw.js", throws: 2);
}

#[test]
fn eval_try_catch_finally() {
    eval!(file: "try_catch_finally.js", 3);
}

#[test]
fn eval_try_catch_throw_finally() {
    eval!(file: "try_catch_throw_finally.js", throws: 2);
}

#[test]
fn eval_try_catch_finally_throw() {
    eval!(file: "try_catch_finally_throw.js", throws: 3);
}

#[test]
fn eval_try_catch_throw_finally_throw() {
    eval!(file: "try_catch_throw_finally_throw.js", throws: 3);
}

#[test]
fn eval_try_nested() {
    eval!(file: "try_nested.js", 11);
}

#[test]
fn eval_try_call_throw() {
    eval!(file: "try_call_throw.js", 1);
}
