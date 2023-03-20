//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<svg><![CDATA[foo]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<math><![CDATA[foo]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "\"foo\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<div><![CDATA[foo]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<!-- [CDATA[foo]] -->"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<svg><![CDATA[foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<svg><![CDATA[foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<svg><![CDATA[",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<svg><![CDATA[]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<svg><![CDATA[]] >]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"]] >\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<svg><![CDATA[]] >]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"]] >\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<svg><![CDATA[]]",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"]]\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<svg><![CDATA[]",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"]\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<svg><![CDATA[]>a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"]>a\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!DOCTYPE html><svg><![CDATA[foo]]]>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo]\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!DOCTYPE html><svg><![CDATA[foo]]]]>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo]]\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!DOCTYPE html><svg><![CDATA[foo]]]]]>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo]]]\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<svg><foreignObject><div><![CDATA[foo]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg foreignObject>"),
            (4, "<div>"),
            (5, "<!-- [CDATA[foo]] -->"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<svg><![CDATA[<svg>]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"<svg>\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<svg><![CDATA[</svg>a]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"</svg>a\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<svg><![CDATA[<svg>a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"<svg>a\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<svg><![CDATA[</svg>a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"</svg>a\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<svg><![CDATA[<svg>]]><path>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"<svg>\""),
            (3, "<svg path>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<svg><![CDATA[<svg>]]></path>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"<svg>\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<svg><![CDATA[<svg>]]><!--path-->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"<svg>\""),
            (3, "<!-- path -->"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<svg><![CDATA[<svg>]]>path",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"<svg>path\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<svg><![CDATA[<!--svg-->]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"<!--svg-->\""),
        ],
        context_element: None,
    });
}
//</coverage:exclude>
