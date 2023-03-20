//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html><math></math>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!DOCTYPE html><body><math></math>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!DOCTYPE html><math><mi>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!DOCTYPE html><math><annotation-xml><svg><u>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "<svg svg>"),
            (2, "<u>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!DOCTYPE html><body><select><math></math></select>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html><body><select><option><math></math></option></select>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><math></math></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (2, "<table>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><math><mi>foo</mi></math></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"foo\""),
            (2, "<table>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><math><mi>foo</mi><mi>bar</mi></math></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"foo\""),
            (3, "<math mi>"),
            (4, "\"bar\""),
            (2, "<table>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><tbody><math><mi>foo</mi><mi>bar</mi></math></tbody></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"foo\""),
            (3, "<math mi>"),
            (4, "\"bar\""),
            (2, "<table>"),
            (3, "<tbody>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><tbody><tr><math><mi>foo</mi><mi>bar</mi></math></tr></tbody></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"foo\""),
            (3, "<math mi>"),
            (4, "\"bar\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><tbody><tr><td><math><mi>foo</mi><mi>bar</mi></math></td></tr></tbody></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<math math>"),
            (7, "<math mi>"),
            (8, "\"foo\""),
            (7, "<math mi>"),
            (8, "\"bar\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><tbody><tr><td><math><mi>foo</mi><mi>bar</mi></math><p>baz</td></tr></tbody></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<math math>"),
            (7, "<math mi>"),
            (8, "\"foo\""),
            (7, "<math mi>"),
            (8, "\"bar\""),
            (6, "<p>"),
            (7, "\"baz\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><caption><math><mi>foo</mi><mi>bar</mi></math><p>baz</caption></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "<math math>"),
            (5, "<math mi>"),
            (6, "\"foo\""),
            (5, "<math mi>"),
            (6, "\"bar\""),
            (4, "<p>"),
            (5, "\"baz\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><caption><math><mi>foo</mi><mi>bar</mi><p>baz</table><p>quux",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "<math math>"),
            (5, "<math mi>"),
            (6, "\"foo\""),
            (5, "<math mi>"),
            (6, "\"bar\""),
            (4, "<p>"),
            (5, "\"baz\""),
            (2, "<p>"),
            (3, "\"quux\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data:
            "<!DOCTYPE html><body><table><caption><math><mi>foo</mi><mi>bar</mi>baz</table><p>quux",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "<math math>"),
            (5, "<math mi>"),
            (6, "\"foo\""),
            (5, "<math mi>"),
            (6, "\"bar\""),
            (5, "\"baz\""),
            (2, "<p>"),
            (3, "\"quux\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><colgroup><math><mi>foo</mi><mi>bar</mi><p>baz</table><p>quux",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"foo\""),
            (3, "<math mi>"),
            (4, "\"bar\""),
            (2, "<p>"),
            (3, "\"baz\""),
            (2, "<table>"),
            (3, "<colgroup>"),
            (2, "<p>"),
            (3, "\"quux\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><tr><td><select><math><mi>foo</mi><mi>bar</mi><p>baz</table><p>quux",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<select>"),
            (7, "\"foobarbaz\""),
            (2, "<p>"),
            (3, "\"quux\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!DOCTYPE html><body><table><select><math><mi>foo</mi><mi>bar</mi><p>baz</table><p>quux",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "\"foobarbaz\""),
            (2, "<table>"),
            (2, "<p>"),
            (3, "\"quux\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!DOCTYPE html><body></body></html><math><mi>foo</mi><mi>bar</mi><p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"foo\""),
            (3, "<math mi>"),
            (4, "\"bar\""),
            (2, "<p>"),
            (3, "\"baz\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!DOCTYPE html><body></body><math><mi>foo</mi><mi>bar</mi><p>baz",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"foo\""),
            (3, "<math mi>"),
            (4, "\"bar\""),
            (2, "<p>"),
            (3, "\"baz\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!DOCTYPE html><frameset><math><mi></mi><mi></mi><p><span>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!DOCTYPE html><frameset></frameset><math><mi></mi><mi></mi><p><span>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!DOCTYPE html><body xlink:href=foo><math xlink:href=foo></math>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "xlink:href=\"foo\""),
            (2, "<math math>"),
            (3, "xlink href=\"foo\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!DOCTYPE html><body xlink:href=foo xml:lang=en><math><mi xml:lang=en xlink:href=foo></mi></math>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "xlink:href=\"foo\""),
            (2, "xml:lang=\"en\""),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "xlink href=\"foo\""),
            (4, "xml lang=\"en\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!DOCTYPE html><body xlink:href=foo xml:lang=en><math><mi xml:lang=en xlink:href=foo /></math>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "xlink:href=\"foo\""),
            (2, "xml:lang=\"en\""),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "xlink href=\"foo\""),
            (4, "xml lang=\"en\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!DOCTYPE html><body xlink:href=foo xml:lang=en><math><mi xml:lang=en xlink:href=foo />bar</math>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "xlink:href=\"foo\""),
            (2, "xml:lang=\"en\""),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "xlink href=\"foo\""),
            (4, "xml lang=\"en\""),
            (3, "\"bar\""),
        ],
        context_element: None,
    });
}
//</coverage:exclude>
