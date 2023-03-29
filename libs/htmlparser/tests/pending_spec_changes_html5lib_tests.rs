//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<input type=\"hidden\"><frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!DOCTYPE html><table><caption><svg>foo</table>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "<svg svg>"),
            (5, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<table><tr><td><svg><desc><td></desc><circle>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<svg svg>"),
            (7, "<svg desc>"),
            (5, "<td>"),
            (6, "<circle>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
