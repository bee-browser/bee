//<coverage:exclude>
mod helper;

use test_log::test;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<head><noscript><!doctype html><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<head><noscript><html class=\"foo\"><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "class=\"foo\""),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<head><noscript></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<head><noscript>   </noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"   \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<head><noscript><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<head><noscript><basefont><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<basefont>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<head><noscript><bgsound><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<bgsound>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<head><noscript><link><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<link>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<head><noscript><meta><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<meta>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<head><noscript><noframes>XXX</noscript></noframes></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<noframes>"),
            (4, "\"XXX</noscript>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<head><noscript><style>XXX</style></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<style>"),
            (4, "\"XXX\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<head><noscript></br><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
            (2, "<br>"),
            (2, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<head><noscript><head class=\"foo\"><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<head><noscript><noscript class=\"foo\"><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<head><noscript></p><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<head><noscript><p><!--foo--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<head><noscript>XXX<!--foo--></noscript></head>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
            (2, "\"XXX\""),
            (2, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<head><noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}
//</coverage:exclude>
