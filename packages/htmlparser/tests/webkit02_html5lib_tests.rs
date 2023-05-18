//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<foo bar=qux/>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<foo>"),
            (3, "bar=\"qux/\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<p id=\"status\"><noscript><strong>A</strong></noscript><span>B</span></p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "id=\"status\""),
            (3, "<noscript>"),
            (4, "\"<strong>A</strong>\""),
            (3, "<span>"),
            (4, "\"B\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<p id=\"status\"><noscript><strong>A</strong></noscript><span>B</span></p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "id=\"status\""),
            (3, "<noscript>"),
            (4, "<strong>"),
            (5, "\"A\""),
            (3, "<span>"),
            (4, "\"B\""),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<div><sarcasm><div></div></sarcasm></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<sarcasm>"),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<html><body><img src=\"\" border=\"0\" alt=\"><div>A</div></body></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<table><td></tbody>A",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<table><td></thead>A",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<table><td></tfoot>A",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<table><thead><td></tbody>A",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<thead>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<legend>test</legend>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<legend>"),
            (3, "\"test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<table><input>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<input>"),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<b><em><foo><foo><aside></b>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<em>"),
            (4, "<foo>"),
            (5, "<foo>"),
            (2, "<em>"),
            (3, "<aside>"),
            (4, "<b>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<b><em><foo><foo><aside></b></em>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<em>"),
            (4, "<foo>"),
            (5, "<foo>"),
            (2, "<em>"),
            (2, "<aside>"),
            (3, "<em>"),
            (4, "<b>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<b><em><foo><foo><foo><aside></b>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<em>"),
            (4, "<foo>"),
            (5, "<foo>"),
            (6, "<foo>"),
            (2, "<aside>"),
            (3, "<b>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<b><em><foo><foo><foo><aside></b></em>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<em>"),
            (4, "<foo>"),
            (5, "<foo>"),
            (6, "<foo>"),
            (2, "<aside>"),
            (3, "<b>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<b><em><foo><foo><foo><foo><foo><foo><foo><foo><foo><foo><aside></b></em>",
        document: vec![
            (0, "<b>"),
            (1, "<em>"),
            (2, "<foo>"),
            (3, "<foo>"),
            (4, "<foo>"),
            (5, "<foo>"),
            (6, "<foo>"),
            (7, "<foo>"),
            (8, "<foo>"),
            (9, "<foo>"),
            (10, "<foo>"),
            (11, "<foo>"),
            (0, "<aside>"),
            (1, "<b>"),
        ],
        context_element: Some(("html", "div")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<b><em><foo><foob><foob><foob><foob><fooc><fooc><fooc><fooc><food><aside></b></em>",
        document: vec![
            (0, "<b>"),
            (1, "<em>"),
            (2, "<foo>"),
            (3, "<foob>"),
            (4, "<foob>"),
            (5, "<foob>"),
            (6, "<foob>"),
            (7, "<fooc>"),
            (8, "<fooc>"),
            (9, "<fooc>"),
            (10, "<fooc>"),
            (11, "<food>"),
            (0, "<aside>"),
            (1, "<b>"),
        ],
        context_element: Some(("html", "div")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<option><XH<optgroup></optgroup>",
        document: vec![(0, "<option>")],
        context_element: Some(("html", "select")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<svg><foreignObject><div>foo</div><plaintext></foreignObject></svg><div>bar</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg foreignObject>"),
            (4, "<div>"),
            (5, "\"foo\""),
            (4, "<plaintext>"),
            (5, "\"</foreignObject></svg><div>bar</div>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<svg><foreignObject></foreignObject><title></svg>foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg foreignObject>"),
            (3, "<svg title>"),
            (2, "\"foo\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "</foreignObject><plaintext><div>foo</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"<div>foo</div>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>