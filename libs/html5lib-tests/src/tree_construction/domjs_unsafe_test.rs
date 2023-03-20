//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<svg><![CDATA[foo\nbar]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo\nbar\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<svg><![CDATA[foo\rbar]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo\nbar\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<svg><![CDATA[foo\r\nbar]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"foo\nbar\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<script>a=\'\0\'</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"a=\'�\'\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<script type=\"data\"><!--\0</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--�\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<script type=\"data\"><!--foo\0</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--foo�\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<script type=\"data\"><!-- foo-\0</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!-- foo-�\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<script type=\"data\"><!-- foo--\0</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!-- foo--�\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<script type=\"data\"><!-- foo-",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!-- foo-\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<script type=\"data\"><!-- foo-<</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!-- foo-<\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<script type=\"data\"><!-- foo-<S",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!-- foo-<S\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<script type=\"data\"><!-- foo-</SCRIPT>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!-- foo-\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<script type=\"data\"><!--<p></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<p>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<script type=\"data\"><!--<script></script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script></script>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<script type=\"data\"><!--<script>\0</script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script>�</script>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<script type=\"data\"><!--<script>-\0</script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script>-�</script>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<script type=\"data\"><!--<script>--\0</script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script>--�</script>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<script type=\"data\"><!--<script>---</script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script>---</script>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<script type=\"data\"><!--<script></scrip></SCRIPT></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script></scrip></SCRIPT>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<script type=\"data\"><!--<script></scrip </SCRIPT></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script></scrip </SCRIPT>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<script type=\"data\"><!--<script></scrip/</SCRIPT></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--<script></scrip/</SCRIPT>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<script type=\"data\"></scrip/></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"</scrip/>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<script type=\"data\"></scrip ></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"</scrip >\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<script type=\"data\"><!--</scrip></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--</scrip>\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<script type=\"data\"><!--</scrip </script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--</scrip \""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<script type=\"data\"><!--</scrip/</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"data\""),
            (3, "\"<!--</scrip/\""),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!DOCTYPE html><!DOCTYPE html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<html><!DOCTYPE html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<html><head><!DOCTYPE html></head>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<html><head></head><!DOCTYPE html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<body></body><!DOCTYPE html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<table><!DOCTYPE html></table>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<table>")],
        context_element: None,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<select><!DOCTYPE html></select>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<select>")],
        context_element: None,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<table><colgroup><!DOCTYPE html></colgroup></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<table><colgroup><!--test--></colgroup></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
            (4, "<!-- test -->"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<table><colgroup><html></colgroup></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<table><colgroup> foo</colgroup></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"foo\""),
            (2, "<table>"),
            (3, "<colgroup>"),
            (4, "\" \""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<select><!--test--></select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<!-- test -->"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<select><html></select>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<select>")],
        context_element: None,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<frameset><html></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
        context_element: None,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<frameset></frameset><html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
        context_element: None,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<frameset></frameset><!DOCTYPE html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
        context_element: None,
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<html><body></body></html><!DOCTYPE html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<svg><!DOCTYPE html></svg>",
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
fn test_0044() {
    parse(Test {
        data: "<svg><font></font></svg>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg font>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<svg><font id=foo></font></svg>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg font>"),
            (4, "id=\"foo\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<svg><font size=4></font></svg>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (2, "<font>"),
            (3, "size=\"4\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<svg><font color=red></font></svg>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (2, "<font>"),
            (3, "color=\"red\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<svg><font font=sans></font></svg>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg font>"),
            (4, "font=\"sans\""),
        ],
        context_element: None,
    });
}
//</coverage:exclude>
