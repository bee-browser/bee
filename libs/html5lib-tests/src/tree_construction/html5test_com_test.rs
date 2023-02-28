//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<div<div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div<div>"),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<div foo<bar=\'\'>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "foo<bar=\"\""),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<div foo=`bar`>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "foo=\"`bar`\""),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<div \\\"foo=\'\'>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\\\"foo=\"\""),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<a href=\'\\nbar\'></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "href=\"\\nbar\""),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "&lang;&rang;",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"⟨⟩\"")],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "&apos;",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"\'\"")],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "&ImaginaryI;",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"ⅈ\"")],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "&Kopf;",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"𝕂\"")],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "&notinva;",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"∉\"")],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<?import namespace=\"foo\" implementation=\"#bar\">",
        document: vec![
            (
                0,
                "<!-- ?import namespace=\"foo\" implementation=\"#bar\" -->",
            ),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!--foo--bar-->",
        document: vec![
            (0, "<!-- foo--bar -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<![CDATA[x]]>",
        document: vec![
            (0, "<!-- [CDATA[x]] -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<textarea><!--</textarea>--></textarea>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"<!--\""),
            (2, "\"-->\""),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<textarea><!--</textarea>-->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"<!--\""),
            (2, "\"-->\""),
        ],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<style><!--</style>--></style>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<style><!--</style>-->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<ul><li>A </li> <li>B</li></ul>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ul>"),
            (3, "<li>"),
            (4, "\"A \""),
            (3, "\" \""),
            (3, "<li>"),
            (4, "\"B\""),
        ],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<table><form><input type=hidden><input></form><div></div></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<input>"),
            (2, "<div>"),
            (2, "<table>"),
            (3, "<form>"),
            (3, "<input>"),
            (4, "type=\"hidden\""),
        ],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<i>A<b>B<p></i>C</b>D",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<i>"),
            (3, "\"A\""),
            (3, "<b>"),
            (4, "\"B\""),
            (2, "<b>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "<i>"),
            (4, "\"C\""),
            (3, "\"D\""),
        ],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<div>")],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<svg></svg>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
        ],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<math></math>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
        ],
    });
}
//</coverage:exclude>
