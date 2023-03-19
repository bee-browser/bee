//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<head></head><style></style>",
        document: vec![(0, "<html>"), (1, "<head>"), (2, "<style>"), (1, "<body>")],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<head></head><script></script>",
        document: vec![(0, "<html>"), (1, "<head>"), (2, "<script>"), (1, "<body>")],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<head></head><!-- --><style></style><!-- --><script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (2, "<script>"),
            (1, "<!--   -->"),
            (1, "<!--   -->"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<head></head><!-- -->x<style></style><!-- --><script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<!--   -->"),
            (1, "<body>"),
            (2, "\"x\""),
            (2, "<style>"),
            (2, "<!--   -->"),
            (2, "<script>"),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><pre>\n</pre></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><pre>\nfoo</pre></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"foo\""),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><pre>",
        document: vec![],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><pre>\nfoo\n</pre></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"\nfoo\""),
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"foo\n\""),
        ],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><pre>x</pre><span>\n</span></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"x\""),
            (2, "<span>"),
            (3, "\"\n\""),
        ],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><pre>x\ny</pre></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"x\ny\""),
        ],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><pre>x<div>\ny</pre></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"x\""),
            (3, "<div>"),
            (4, "\"\ny\""),
        ],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!DOCTYPE html><pre>&#x0a;&#x0a;A</pre>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"\nA\""),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!DOCTYPE html><HTML><META><HEAD></HEAD></HTML>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<meta>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!DOCTYPE html><HTML><HEAD><head></HEAD></HTML>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<textarea>foo<span>bar</span><i>baz",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"foo<span>bar</span><i>baz\""),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<title>foo<span>bar</em><i>baz",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"foo<span>bar</em><i>baz\""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!DOCTYPE html><textarea>\n</textarea>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
        ],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!DOCTYPE html><textarea>\nfoo</textarea>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"foo\""),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!DOCTYPE html><textarea>",
        document: vec![],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><ul><li><div><p><li></ul></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"\nfoo\""),
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ul>"),
            (3, "<li>"),
            (4, "<div>"),
            (5, "<p>"),
            (3, "<li>"),
        ],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!doctype html><nobr><nobr><nobr>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<nobr>"),
            (2, "<nobr>"),
            (2, "<nobr>"),
        ],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!doctype html><nobr><nobr></nobr><nobr>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<nobr>"),
            (2, "<nobr>"),
            (2, "<nobr>"),
        ],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!doctype html><html><body><p><table></table></body></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<table>"),
        ],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<p><table></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<table>"),
        ],
    });
}
//</coverage:exclude>
