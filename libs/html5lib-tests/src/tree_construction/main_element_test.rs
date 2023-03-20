//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!doctype html><p>foo<main>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<main>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!doctype html><main><p>foo</main>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<main>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!DOCTYPE html>xxx<svg><x><g><a><main><b>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"xxx\""),
            (2, "<svg svg>"),
            (3, "<svg x>"),
            (4, "<svg g>"),
            (5, "<svg a>"),
            (6, "<svg main>"),
            (2, "<b>"),
        ],
        context_element: None,
    });
}
//</coverage:exclude>
