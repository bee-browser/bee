//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

logging::init!();

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html><p><b><i><u></p> <p>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "<i>"),
            (5, "<u>"),
            (2, "<b>"),
            (3, "<i>"),
            (4, "<u>"),
            (5, "\" \""),
            (5, "<p>"),
            (6, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<p><b><i><u></p>\n<p>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "<i>"),
            (5, "<u>"),
            (2, "<b>"),
            (3, "<i>"),
            (4, "<u>"),
            (5, "\"\n\""),
            (5, "<p>"),
            (6, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!doctype html></html> <head>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\" \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!doctype html></body><meta>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<meta>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<html></html><!-- foo -->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (0, "<!--  foo  -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!doctype html></body><title>X</title>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<title>"),
            (3, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!doctype html><table> X<meta></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\" X\""),
            (2, "<meta>"),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!doctype html><table> x</table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\" x\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!doctype html><table> x </table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\" x \""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!doctype html><table><tr> x</table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\" x\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!doctype html><table>X<style> <tr>x </style> </table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"X\""),
            (2, "<table>"),
            (3, "<style>"),
            (4, "\" <tr>x \""),
            (3, "\" \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!doctype html><div><table><a>foo</a> <tr><td>bar</td> </tr></table></div>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<a>"),
            (4, "\"foo\""),
            (3, "<table>"),
            (4, "\" \""),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "\"bar\""),
            (6, "\" \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<frame></frame></frame><frameset><frame><frameset><frame></frameset><noframes></frameset><noframes>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
            (2, "<frameset>"),
            (3, "<frame>"),
            (2, "<noframes>"),
            (3, "\"</frameset><noframes>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!DOCTYPE html><object></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<object>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
