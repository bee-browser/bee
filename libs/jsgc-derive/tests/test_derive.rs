#[test]
fn test_derive() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
}
