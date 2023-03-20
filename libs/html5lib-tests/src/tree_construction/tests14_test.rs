//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html><html><body><xyz:abc></xyz:abc>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<xyz:abc>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!DOCTYPE html><html><body><xyz:abc></xyz:abc><span></span>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<xyz:abc>"),
            (2, "<span>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!DOCTYPE html><html><html abc:def=gh><xyz:abc></xyz:abc>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "abc:def=\"gh\""),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<xyz:abc>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!DOCTYPE html><html xml:lang=bar><html xml:lang=foo>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "xml:lang=\"bar\""),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!DOCTYPE html><html 123=456>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "123=\"456\""),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html><html 123=456><html 789=012>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "123=\"456\""),
            (1, "789=\"012\""),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE html><html><body 789=012>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "789=\"012\""),
        ],
        context_element: None,
    });
}
//</coverage:exclude>
