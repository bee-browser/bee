//<coverage:exclude>
use super::helper::parse;
use super::helper::Scripting;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!doctype html><p>foo<address>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<address>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!doctype html><address><p>foo</address>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<address>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!doctype html><p>foo<article>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<article>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!doctype html><article><p>foo</article>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<article>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!doctype html><p>foo<aside>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<aside>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!doctype html><aside><p>foo</aside>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<aside>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!doctype html><p>foo<blockquote>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<blockquote>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!doctype html><blockquote><p>foo</blockquote>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<blockquote>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!doctype html><p>foo<center>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<center>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!doctype html><center><p>foo</center>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<center>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!doctype html><p>foo<details>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<details>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!doctype html><details><p>foo</details>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<details>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!doctype html><p>foo<dialog>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<dialog>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!doctype html><dialog><p>foo</dialog>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dialog>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!doctype html><p>foo<dir>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<dir>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<!doctype html><dir><p>foo</dir>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dir>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!doctype html><p>foo<div>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<div>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!doctype html><div><p>foo</div>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!doctype html><p>foo<dl>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<dl>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!doctype html><dl><p>foo</dl>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dl>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!doctype html><p>foo<fieldset>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<fieldset>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!doctype html><fieldset><p>foo</fieldset>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<fieldset>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!doctype html><p>foo<figcaption>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<figcaption>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!doctype html><figcaption><p>foo</figcaption>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<figcaption>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!doctype html><p>foo<figure>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<figure>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!doctype html><figure><p>foo</figure>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<figure>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!doctype html><p>foo<footer>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<footer>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!doctype html><footer><p>foo</footer>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<footer>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!doctype html><p>foo<header>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<header>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!doctype html><header><p>foo</header>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<header>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<!doctype html><p>foo<hgroup>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<hgroup>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<!doctype html><hgroup><p>foo</hgroup>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<hgroup>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!doctype html><p>foo<listing>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<listing>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!doctype html><listing><p>foo</listing>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<listing>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!doctype html><p>foo<menu>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<menu>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<!doctype html><menu><p>foo</menu>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<menu>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<!doctype html><p>foo<nav>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<nav>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<!doctype html><nav><p>foo</nav>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<nav>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<!doctype html><p>foo<ol>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<ol>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<!doctype html><ol><p>foo</ol>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ol>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<!doctype html><p>foo<pre>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<pre>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<!doctype html><pre><p>foo</pre>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<!doctype html><p>foo<section>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<section>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<!doctype html><section><p>foo</section>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<section>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "<!doctype html><p>foo<summary>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<summary>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<!doctype html><summary><p>foo</summary>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<summary>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<!doctype html><p>foo<ul>bar<p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (2, "<ul>"),
            (3, "\"bar\""),
            (3, "<p>"),
            (4, "\"baz\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<!doctype html><ul><p>foo</ul>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ul>"),
            (3, "<p>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
