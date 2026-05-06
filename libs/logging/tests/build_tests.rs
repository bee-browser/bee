#[test]
fn test_logger() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/logger.rs");
}

#[test]
fn test_define_logger() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/define_logger_without_target.rs");
    t.pass("tests/trybuild/define_logger_with_target.rs");
    t.compile_fail("tests/trybuild/define_logger_with_invalid_target.rs");
}
