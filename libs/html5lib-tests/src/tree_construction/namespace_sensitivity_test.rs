//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<body><table><tr><td><svg><td><foreignObject><span></td>Foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Foo\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<svg svg>"),
            (7, "<svg td>"),
            (8, "<svg foreignObject>"),
            (9, "<span>"),
        ],
    });
}
//</coverage:exclude>
