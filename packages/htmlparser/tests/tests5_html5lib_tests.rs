//<coverage:exclude>
mod helper;

use test_log::test;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<style> <!-- </style>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\" <!-- \""),
            (1, "<body>"),
            (2, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<style> <!-- </style> --> </style>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\" <!-- \""),
            (2, "\" \""),
            (1, "<body>"),
            (2, "\"--> x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<style> <!--> </style>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\" <!--> \""),
            (1, "<body>"),
            (2, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<style> <!---> </style>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\" <!---> \""),
            (1, "<body>"),
            (2, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<iframe> <!---> </iframe>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\" <!---> \""),
            (2, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<iframe> <!--- </iframe>->x</iframe> --> </iframe>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\" <!--- \""),
            (2, "\"->x --> x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<script> <!-- </script> --> </script>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\" <!-- \""),
            (2, "\" \""),
            (1, "<body>"),
            (2, "\"--> x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<title> <!-- </title> --> </title>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\" <!-- \""),
            (2, "\" \""),
            (1, "<body>"),
            (2, "\"--> x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<textarea> <!--- </textarea>->x</textarea> --> </textarea>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\" <!--- \""),
            (2, "\"->x --> x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<style> <!</-- </style>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\" <!</-- \""),
            (1, "<body>"),
            (2, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<p><xmp></xmp>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<xmp>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<xmp> <!-- > --> </xmp>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<xmp>"),
            (3, "\" <!-- > --> \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<title>&amp;</title>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"&\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<title><!--&amp;--></title>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"<!--&-->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<title><!--</title>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"<!--\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<noscript><!--</noscript>--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"<!--\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<noscript><!--</noscript>--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- </noscript> -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}
//</coverage:exclude>
