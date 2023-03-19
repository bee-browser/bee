//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<div>\n<div></div>\n</span>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"\n\""),
            (3, "<div>"),
            (3, "\"\nx\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<div>x<div></div>\n</span>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"x\""),
            (3, "<div>"),
            (3, "\"\nx\""),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<div>x<div></div>x</span>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"x\""),
            (3, "<div>"),
            (3, "\"xx\""),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<div>x<div></div>y</span>z",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"x\""),
            (3, "<div>"),
            (3, "\"yz\""),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<table><div>x<div></div>x</span>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"x\""),
            (3, "<div>"),
            (3, "\"xx\""),
            (2, "<table>"),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<table><li><li></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<li>"),
            (2, "<li>"),
            (2, "<table>"),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "x<table>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"xx\""),
            (2, "<table>"),
        ],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "x<table><table>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"x\""),
            (2, "<table>"),
            (2, "\"x\""),
            (2, "<table>"),
        ],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<b>a<div></div><div></b>y",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"a\""),
            (3, "<div>"),
            (2, "<div>"),
            (3, "<b>"),
            (3, "\"y\""),
        ],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<a><div><p></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (2, "<div>"),
            (3, "<a>"),
            (3, "<p>"),
            (4, "<a>"),
        ],
    });
}
//</coverage:exclude>
