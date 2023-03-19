//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<body><table>\0filler\0text\0",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"fillertext\""),
            (2, "<table>"),
        ],
    });
}
//</coverage:exclude>
