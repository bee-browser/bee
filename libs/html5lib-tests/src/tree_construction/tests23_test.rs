//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<p><font size=4><font color=red><font size=4><font size=4><font size=4><font size=4><font size=4><font color=red><p>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<font>"),
            (4, "size=\"4\""),
            (4, "<font>"),
            (5, "color=\"red\""),
            (5, "<font>"),
            (6, "size=\"4\""),
            (6, "<font>"),
            (7, "size=\"4\""),
            (7, "<font>"),
            (8, "size=\"4\""),
            (8, "<font>"),
            (9, "size=\"4\""),
            (9, "<font>"),
            (10, "size=\"4\""),
            (10, "<font>"),
            (11, "color=\"red\""),
            (2, "<p>"),
            (3, "<font>"),
            (4, "color=\"red\""),
            (4, "<font>"),
            (5, "size=\"4\""),
            (5, "<font>"),
            (6, "size=\"4\""),
            (6, "<font>"),
            (7, "size=\"4\""),
            (7, "<font>"),
            (8, "color=\"red\""),
            (8, "\"X\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<p><font size=4><font size=4><font size=4><font size=4><p>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<font>"),
            (4, "size=\"4\""),
            (4, "<font>"),
            (5, "size=\"4\""),
            (5, "<font>"),
            (6, "size=\"4\""),
            (6, "<font>"),
            (7, "size=\"4\""),
            (2, "<p>"),
            (3, "<font>"),
            (4, "size=\"4\""),
            (4, "<font>"),
            (5, "size=\"4\""),
            (5, "<font>"),
            (6, "size=\"4\""),
            (6, "\"X\""),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<p><font size=4><font size=4><font size=4><font size=\"5\"><font size=4><p>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<font>"),
            (4, "size=\"4\""),
            (4, "<font>"),
            (5, "size=\"4\""),
            (5, "<font>"),
            (6, "size=\"4\""),
            (6, "<font>"),
            (7, "size=\"5\""),
            (7, "<font>"),
            (8, "size=\"4\""),
            (2, "<p>"),
            (3, "<font>"),
            (4, "size=\"4\""),
            (4, "<font>"),
            (5, "size=\"4\""),
            (5, "<font>"),
            (6, "size=\"5\""),
            (6, "<font>"),
            (7, "size=\"4\""),
            (7, "\"X\""),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<p><font size=4 id=a><font size=4 id=b><font size=4><font size=4><p>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<font>"),
            (4, "id=\"a\""),
            (4, "size=\"4\""),
            (4, "<font>"),
            (5, "id=\"b\""),
            (5, "size=\"4\""),
            (5, "<font>"),
            (6, "size=\"4\""),
            (6, "<font>"),
            (7, "size=\"4\""),
            (2, "<p>"),
            (3, "<font>"),
            (4, "id=\"a\""),
            (4, "size=\"4\""),
            (4, "<font>"),
            (5, "id=\"b\""),
            (5, "size=\"4\""),
            (5, "<font>"),
            (6, "size=\"4\""),
            (6, "<font>"),
            (7, "size=\"4\""),
            (7, "\"X\""),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<p><b id=a><b id=a><b id=a><b><object><b id=a><b id=a>X</object><p>Y",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "id=\"a\""),
            (4, "<b>"),
            (5, "id=\"a\""),
            (5, "<b>"),
            (6, "id=\"a\""),
            (6, "<b>"),
            (7, "<object>"),
            (8, "<b>"),
            (9, "id=\"a\""),
            (9, "<b>"),
            (10, "id=\"a\""),
            (10, "\"X\""),
            (2, "<p>"),
            (3, "<b>"),
            (4, "id=\"a\""),
            (4, "<b>"),
            (5, "id=\"a\""),
            (5, "<b>"),
            (6, "id=\"a\""),
            (6, "<b>"),
            (7, "\"Y\""),
        ],
    });
}
//</coverage:exclude>
