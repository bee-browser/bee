//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "direct div content",
        document: vec![(0, "\"direct div content\"")],
        context_element: Some(("html", "div")),
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "direct textarea content",
        document: vec![(0, "\"direct textarea content\"")],
        context_element: Some(("html", "textarea")),
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "textarea content with <em>pseudo</em> <foo>markup",
        document: vec![(0, "\"textarea content with <em>pseudo</em> <foo>markup\"")],
        context_element: Some(("html", "textarea")),
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "this is &#x0043;DATA inside a <style> element",
        document: vec![(0, "\"this is &#x0043;DATA inside a <style> element\"")],
        context_element: Some(("html", "style")),
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "</plaintext>",
        document: vec![(0, "\"</plaintext>\"")],
        context_element: Some(("html", "plaintext")),
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "setting html\'s innerHTML",
        document: vec![
            (0, "<head>"),
            (0, "<body>"),
            (1, "\"setting html\'s innerHTML\""),
        ],
        context_element: Some(("html", "html")),
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<title>setting head\'s innerHTML</title>",
        document: vec![(0, "<title>"), (1, "\"setting head\'s innerHTML\"")],
        context_element: Some(("html", "head")),
    });
}
//</coverage:exclude>
