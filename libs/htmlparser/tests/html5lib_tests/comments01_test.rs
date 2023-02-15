//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0() {
    parse(Test {
        data: r#"FOO<!-- BAR -->BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  BAR  -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_1() {
    parse(Test {
        data: r#"FOO<!-- BAR --!>BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  BAR  -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_2() {
    parse(Test {
        data: r#"FOO<!-- BAR --! >BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  BAR --! >BAZ -->"#),
        ],
    });
}

#[test]
fn test_3() {
    parse(Test {
        data: r#"FOO<!-- BAR --!
>BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (
                2,
                r#"<!--  BAR --!
>BAZ -->"#,
            ),
        ],
    });
}

#[test]
fn test_4() {
    parse(Test {
        data: r#"FOO<!-- BAR --   >BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  BAR --   >BAZ -->"#),
        ],
    });
}

#[test]
fn test_5() {
    parse(Test {
        data: r#"FOO<!-- BAR -- <QUX> -- MUX -->BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  BAR -- <QUX> -- MUX  -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_6() {
    parse(Test {
        data: r#"FOO<!-- BAR -- <QUX> -- MUX --!>BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  BAR -- <QUX> -- MUX  -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_7() {
    parse(Test {
        data: r#"FOO<!-- BAR -- <QUX> -- MUX -- >BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  BAR -- <QUX> -- MUX -- >BAZ -->"#),
        ],
    });
}

#[test]
fn test_8() {
    parse(Test {
        data: r#"FOO<!---->BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_9() {
    parse(Test {
        data: r#"FOO<!--->BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_10() {
    parse(Test {
        data: r#"FOO<!-->BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!--  -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_11() {
    parse(Test {
        data: r#"<?xml version="1.0">Hi"#,
        document: vec![
            (0, r#"<!-- ?xml version="1.0" -->"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hi""#),
        ],
    });
}

#[test]
fn test_12() {
    parse(Test {
        data: r#"<?xml version="1.0">"#,
        document: vec![
            (0, r#"<!-- ?xml version="1.0" -->"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_13() {
    parse(Test {
        data: r#"<?xml version"#,
        document: vec![
            (0, r#"<!-- ?xml version -->"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_14() {
    parse(Test {
        data: r#"FOO<!----->BAZ"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO""#),
            (2, r#"<!-- - -->"#),
            (2, r#""BAZ""#),
        ],
    });
}

#[test]
fn test_15() {
    parse(Test {
        data: r#"<html><!-- comment --><title>Comment before head</title>"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<!--  comment  -->"#),
            (1, r#"<head>"#),
            (2, r#"<title>"#),
            (3, r#""Comment before head""#),
            (1, r#"<body>"#),
        ],
    });
}
//</coverage:exclude>
