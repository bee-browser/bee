//<coverage:exclude>
use super::helper::parse;
use super::helper::Scripting;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "FOO<!-- BAR -->BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR  -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "FOO<!-- BAR --!>BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR  -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "FOO<!-- BAR --! >BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR --! >BAZ -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "FOO<!-- BAR --!\n>BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR --!\n>BAZ -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "FOO<!-- BAR --   >BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR --   >BAZ -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "FOO<!-- BAR -- <QUX> -- MUX -->BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR -- <QUX> -- MUX  -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "FOO<!-- BAR -- <QUX> -- MUX --!>BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR -- <QUX> -- MUX  -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "FOO<!-- BAR -- <QUX> -- MUX -- >BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  BAR -- <QUX> -- MUX -- >BAZ -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "FOO<!---->BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "FOO<!--->BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "FOO<!-->BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!--  -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<?xml version=\"1.0\">Hi",
        document: vec![
            (0, "<!-- ?xml version=\"1.0\" -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hi\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<?xml version=\"1.0\">",
        document: vec![
            (0, "<!-- ?xml version=\"1.0\" -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<?xml version",
        document: vec![
            (0, "<!-- ?xml version -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "FOO<!----->BAZ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<!-- - -->"),
            (2, "\"BAZ\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<html><!-- comment --><title>Comment before head</title>",
        document: vec![
            (0, "<html>"),
            (1, "<!--  comment  -->"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"Comment before head\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
