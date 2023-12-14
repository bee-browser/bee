//<coverage:exclude>
mod helper;

use test_log::test;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<a><p></a></p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (2, "<p>"),
            (3, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<a>1<p>2</a>3</p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "\"1\""),
            (2, "<p>"),
            (3, "<a>"),
            (4, "\"2\""),
            (3, "\"3\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<a>1<button>2</a>3</button>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "\"1\""),
            (2, "<button>"),
            (3, "<a>"),
            (4, "\"2\""),
            (3, "\"3\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<a>1<b>2</a>3</b>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "\"1\""),
            (3, "<b>"),
            (4, "\"2\""),
            (2, "<b>"),
            (3, "\"3\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<a>1<div>2<div>3</a>4</div>5</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "\"1\""),
            (2, "<div>"),
            (3, "<a>"),
            (4, "\"2\""),
            (3, "<div>"),
            (4, "<a>"),
            (5, "\"3\""),
            (4, "\"4\""),
            (3, "\"5\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<table><a>1<p>2</a>3</p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "\"1\""),
            (2, "<p>"),
            (3, "<a>"),
            (4, "\"2\""),
            (3, "\"3\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<b><b><a><p></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<b>"),
            (4, "<a>"),
            (4, "<p>"),
            (5, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<b><a><b><p></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<a>"),
            (4, "<b>"),
            (3, "<b>"),
            (4, "<p>"),
            (5, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<a><b><b><p></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<b>"),
            (4, "<b>"),
            (2, "<b>"),
            (3, "<b>"),
            (4, "<p>"),
            (5, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<p>1<s id=\"A\">2<b id=\"B\">3</p>4</s>5</b>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"1\""),
            (3, "<s>"),
            (4, "id=\"A\""),
            (4, "\"2\""),
            (4, "<b>"),
            (5, "id=\"B\""),
            (5, "\"3\""),
            (2, "<s>"),
            (3, "id=\"A\""),
            (3, "<b>"),
            (4, "id=\"B\""),
            (4, "\"4\""),
            (2, "<b>"),
            (3, "id=\"B\""),
            (3, "\"5\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<table><a>1<td>2</td>3</table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "\"1\""),
            (2, "<a>"),
            (3, "\"3\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"2\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<table>A<td>B</td>C</table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"AC\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"B\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<a><svg><tr><input></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<svg svg>"),
            (4, "<svg tr>"),
            (5, "<svg input>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<div><a><b><div><div><div><div><div><div><div><div><div><div></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<a>"),
            (4, "<b>"),
            (3, "<b>"),
            (4, "<div>"),
            (5, "<a>"),
            (5, "<div>"),
            (6, "<a>"),
            (6, "<div>"),
            (7, "<a>"),
            (7, "<div>"),
            (8, "<a>"),
            (8, "<div>"),
            (9, "<a>"),
            (9, "<div>"),
            (10, "<a>"),
            (10, "<div>"),
            (11, "<a>"),
            (11, "<div>"),
            (12, "<a>"),
            (13, "<div>"),
            (14, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<div><a><b><u><i><code><div></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<a>"),
            (4, "<b>"),
            (5, "<u>"),
            (6, "<i>"),
            (7, "<code>"),
            (3, "<u>"),
            (4, "<i>"),
            (5, "<code>"),
            (6, "<div>"),
            (7, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<b><b><b><b>x</b></b></b></b>y",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<b>"),
            (4, "<b>"),
            (5, "<b>"),
            (6, "\"x\""),
            (2, "\"y\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<p><b><b><b><b><p>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "<b>"),
            (5, "<b>"),
            (6, "<b>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "<b>"),
            (5, "<b>"),
            (6, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<b><em><foo><foob><fooc><aside></b></em>",
        document: vec![
            (0, "<b>"),
            (1, "<em>"),
            (2, "<foo>"),
            (3, "<foob>"),
            (4, "<fooc>"),
            (0, "<aside>"),
            (1, "<b>"),
        ],
        context_element: Some(("html", "div")),
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
