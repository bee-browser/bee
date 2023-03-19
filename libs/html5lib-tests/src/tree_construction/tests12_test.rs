//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html><body><p>foo<math><mtext><i>baz</i></mtext><annotation-xml><svg><desc><b>eggs</b></desc><g><foreignObject><P>spam<TABLE><tr><td><img></td></table></foreignObject></g><g>quux</g></svg></annotation-xml></math>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
            (3, "<math math>"),
            (4, "<math mtext>"),
            (5, "<i>"),
            (6, "\"baz\""),
            (4, "<math annotation-xml>"),
            (5, "<svg svg>"),
            (6, "<svg desc>"),
            (7, "<b>"),
            (8, "\"eggs\""),
            (6, "<svg g>"),
            (7, "<svg foreignObject>"),
            (8, "<p>"),
            (9, "\"spam\""),
            (8, "<table>"),
            (9, "<tbody>"),
            (10, "<tr>"),
            (11, "<td>"),
            (12, "<img>"),
            (6, "<svg g>"),
            (7, "\"quux\""),
            (3, "\"bar\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!DOCTYPE html><body>foo<math><mtext><i>baz</i></mtext><annotation-xml><svg><desc><b>eggs</b></desc><g><foreignObject><P>spam<TABLE><tr><td><img></td></table></foreignObject></g><g>quux</g></svg></annotation-xml></math>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"foo\""),
            (2, "<math math>"),
            (3, "<math mtext>"),
            (4, "<i>"),
            (5, "\"baz\""),
            (3, "<math annotation-xml>"),
            (4, "<svg svg>"),
            (5, "<svg desc>"),
            (6, "<b>"),
            (7, "\"eggs\""),
            (5, "<svg g>"),
            (6, "<svg foreignObject>"),
            (7, "<p>"),
            (8, "\"spam\""),
            (7, "<table>"),
            (8, "<tbody>"),
            (9, "<tr>"),
            (10, "<td>"),
            (11, "<img>"),
            (5, "<svg g>"),
            (6, "\"quux\""),
            (2, "\"bar\""),
        ],
    });
}
//</coverage:exclude>