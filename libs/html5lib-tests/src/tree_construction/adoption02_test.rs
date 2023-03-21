//<coverage:exclude>
use super::helper::parse;
use super::helper::Scripting;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<b>1<i>2<p>3</b>4",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"1\""),
            (3, "<i>"),
            (4, "\"2\""),
            (2, "<i>"),
            (3, "<p>"),
            (4, "<b>"),
            (5, "\"3\""),
            (4, "\"4\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<a><div><style></style><address><a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (2, "<div>"),
            (3, "<a>"),
            (4, "<style>"),
            (3, "<address>"),
            (4, "<a>"),
            (4, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
