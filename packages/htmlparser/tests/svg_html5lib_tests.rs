//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<svg><tr><td><title><tr>",
        document: vec![
            (0, "<svg svg>"),
            (1, "<svg tr>"),
            (2, "<svg td>"),
            (3, "<svg title>"),
        ],
        context_element: Some(("html", "td")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<svg><tr><td><title><tr>",
        document: vec![
            (0, "<svg svg>"),
            (1, "<svg tr>"),
            (2, "<svg td>"),
            (3, "<svg title>"),
        ],
        context_element: Some(("html", "tr")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<svg><thead><title><tbody>",
        document: vec![(0, "<svg svg>"), (1, "<svg thead>"), (2, "<svg title>")],
        context_element: Some(("html", "thead")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<svg><tfoot><title><tbody>",
        document: vec![(0, "<svg svg>"), (1, "<svg tfoot>"), (2, "<svg title>")],
        context_element: Some(("html", "tfoot")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<svg><tbody><title><tfoot>",
        document: vec![(0, "<svg svg>"), (1, "<svg tbody>"), (2, "<svg title>")],
        context_element: Some(("html", "tbody")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<svg><tbody><title></table>",
        document: vec![(0, "<svg svg>"), (1, "<svg tbody>"), (2, "<svg title>")],
        context_element: Some(("html", "tbody")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<svg><thead><title></table>",
        document: vec![(0, "<svg svg>"), (1, "<svg thead>"), (2, "<svg title>")],
        context_element: Some(("html", "tbody")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<svg><tfoot><title></table>",
        document: vec![(0, "<svg svg>"), (1, "<svg tfoot>"), (2, "<svg title>")],
        context_element: Some(("html", "tbody")),
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
