//<coverage:exclude>
use super::helper::parse;
use super::helper::Scripting;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<a><b><big><em><strong><div>X</a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<b>"),
            (4, "<big>"),
            (5, "<em>"),
            (6, "<strong>"),
            (2, "<big>"),
            (3, "<em>"),
            (4, "<strong>"),
            (5, "<div>"),
            (6, "<a>"),
            (7, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<a><b><div id=1><div id=2><div id=3><div id=4><div id=5><div id=6><div id=7><div id=8>A</a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<b>"),
            (2, "<b>"),
            (3, "<div>"),
            (4, "id=\"1\""),
            (4, "<a>"),
            (4, "<div>"),
            (5, "id=\"2\""),
            (5, "<a>"),
            (5, "<div>"),
            (6, "id=\"3\""),
            (6, "<a>"),
            (6, "<div>"),
            (7, "id=\"4\""),
            (7, "<a>"),
            (7, "<div>"),
            (8, "id=\"5\""),
            (8, "<a>"),
            (8, "<div>"),
            (9, "id=\"6\""),
            (9, "<a>"),
            (9, "<div>"),
            (10, "id=\"7\""),
            (10, "<a>"),
            (10, "<div>"),
            (11, "id=\"8\""),
            (11, "<a>"),
            (12, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<a><b><div id=1><div id=2><div id=3><div id=4><div id=5><div id=6><div id=7><div id=8><div id=9>A</a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<b>"),
            (2, "<b>"),
            (3, "<div>"),
            (4, "id=\"1\""),
            (4, "<a>"),
            (4, "<div>"),
            (5, "id=\"2\""),
            (5, "<a>"),
            (5, "<div>"),
            (6, "id=\"3\""),
            (6, "<a>"),
            (6, "<div>"),
            (7, "id=\"4\""),
            (7, "<a>"),
            (7, "<div>"),
            (8, "id=\"5\""),
            (8, "<a>"),
            (8, "<div>"),
            (9, "id=\"6\""),
            (9, "<a>"),
            (9, "<div>"),
            (10, "id=\"7\""),
            (10, "<a>"),
            (10, "<div>"),
            (11, "id=\"8\""),
            (11, "<a>"),
            (12, "<div>"),
            (13, "id=\"9\""),
            (13, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<a><b><div id=1><div id=2><div id=3><div id=4><div id=5><div id=6><div id=7><div id=8><div id=9><div id=10>A</a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<b>"),
            (2, "<b>"),
            (3, "<div>"),
            (4, "id=\"1\""),
            (4, "<a>"),
            (4, "<div>"),
            (5, "id=\"2\""),
            (5, "<a>"),
            (5, "<div>"),
            (6, "id=\"3\""),
            (6, "<a>"),
            (6, "<div>"),
            (7, "id=\"4\""),
            (7, "<a>"),
            (7, "<div>"),
            (8, "id=\"5\""),
            (8, "<a>"),
            (8, "<div>"),
            (9, "id=\"6\""),
            (9, "<a>"),
            (9, "<div>"),
            (10, "id=\"7\""),
            (10, "<a>"),
            (10, "<div>"),
            (11, "id=\"8\""),
            (11, "<a>"),
            (12, "<div>"),
            (13, "id=\"9\""),
            (13, "<div>"),
            (14, "id=\"10\""),
            (14, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<cite><b><cite><i><cite><i><cite><i><div>X</b>TEST",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<cite>"),
            (3, "<b>"),
            (4, "<cite>"),
            (5, "<i>"),
            (6, "<cite>"),
            (7, "<i>"),
            (8, "<cite>"),
            (9, "<i>"),
            (3, "<i>"),
            (4, "<i>"),
            (5, "<div>"),
            (6, "<b>"),
            (7, "\"X\""),
            (6, "\"TEST\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
