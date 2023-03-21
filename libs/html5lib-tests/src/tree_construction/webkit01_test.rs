//<coverage:exclude>
use super::helper::parse;
use super::helper::Scripting;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "Test",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"Test\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<div>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<div>Test</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"Test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<di",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<div>Hello</div>\n<script>\nconsole.log(\"PASS\");\n</script>\n<div>Bye</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"Hello\""),
            (2, "\"\n\""),
            (2, "<script>"),
            (3, "\"\nconsole.log(\"PASS\");\n\""),
            (2, "\"\n\""),
            (2, "<div>"),
            (3, "\"Bye\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<div foo=\"bar\">Hello</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "foo=\"bar\""),
            (3, "\"Hello\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<div>Hello</div>\n<script>\nconsole.log(\"FOO<span>BAR</span>BAZ\");\n</script>\n<div>Bye</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"Hello\""),
            (2, "\"\n\""),
            (2, "<script>"),
            (3, "\"\nconsole.log(\"FOO<span>BAR</span>BAZ\");\n\""),
            (2, "\"\n\""),
            (2, "<div>"),
            (3, "\"Bye\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<foo bar=\"baz\"></foo><potato quack=\"duck\"></potato>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<foo>"),
            (3, "bar=\"baz\""),
            (2, "<potato>"),
            (3, "quack=\"duck\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<foo bar=\"baz\"><potato quack=\"duck\"></potato></foo>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<foo>"),
            (3, "bar=\"baz\""),
            (3, "<potato>"),
            (4, "quack=\"duck\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<foo></foo bar=\"baz\"><potato></potato quack=\"duck\">",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<foo>"),
            (2, "<potato>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "</ tttt>",
        document: vec![
            (0, "<!--  tttt -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<div FOO ><img><img></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "foo=\"\""),
            (3, "<img>"),
            (3, "<img>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<p>Test</p<p>Test2</p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"TestTest2\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<rdar://problem/6869687>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<rdar:>"),
            (3, "6869687=\"\""),
            (3, "problem=\"\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<A>test< /A>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "\"test< /A>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "&lt;",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"<\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<body foo=\'bar\'><body foo=\'baz\' yo=\'mama\'>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "foo=\"bar\""),
            (2, "yo=\"mama\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<body></br foo=\"bar\"></body>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<br>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<bdy><br foo=\"bar\"></body>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<bdy>"),
            (3, "<br>"),
            (4, "foo=\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<body></body></br foo=\"bar\">",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<br>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<bdy></body><br foo=\"bar\">",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<bdy>"),
            (3, "<br>"),
            (4, "foo=\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<html><body></body></html><!-- Hi there -->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (0, "<!--  Hi there  -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<html><body></body></html><!-- Comment A --><!-- Comment B --><!-- Comment C --><!-- Comment D --><!-- Comment E -->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (0, "<!--  Comment A  -->"),
            (0, "<!--  Comment B  -->"),
            (0, "<!--  Comment C  -->"),
            (0, "<!--  Comment D  -->"),
            (0, "<!--  Comment E  -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<html><body></body></html>x<!-- Hi there -->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"x\""),
            (2, "<!--  Hi there  -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<html><body></body></html>x<!-- Hi there --></html><!-- Again -->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"x\""),
            (2, "<!--  Hi there  -->"),
            (0, "<!--  Again  -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<html><body></body></html>x<!-- Hi there --></body></html><!-- Again -->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"x\""),
            (2, "<!--  Hi there  -->"),
            (0, "<!--  Again  -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<html><body><ruby><div><rp>xx</rp></div></ruby></body></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<div>"),
            (4, "<rp>"),
            (5, "\"xx\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<html><body><ruby><div><rt>xx</rt></div></ruby></body></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<div>"),
            (4, "<rt>"),
            (5, "\"xx\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<html><frameset><!--1--><noframes>A</noframes><!--2--></frameset><!--3--><noframes>B</noframes><!--4--></html><!--5--><noframes>C</noframes><!--6-->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<!-- 1 -->"),
            (2, "<noframes>"),
            (3, "\"A\""),
            (2, "<!-- 2 -->"),
            (1, "<!-- 3 -->"),
            (1, "<noframes>"),
            (2, "\"B\""),
            (1, "<!-- 4 -->"),
            (1, "<noframes>"),
            (2, "\"C\""),
            (0, "<!-- 5 -->"),
            (0, "<!-- 6 -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<select><option>A<select><option>B<select><option>C<select><option>D<select><option>E<select><option>F<select><option>G<select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (4, "\"A\""),
            (2, "<option>"),
            (3, "\"B\""),
            (3, "<select>"),
            (4, "<option>"),
            (5, "\"C\""),
            (2, "<option>"),
            (3, "\"D\""),
            (3, "<select>"),
            (4, "<option>"),
            (5, "\"E\""),
            (2, "<option>"),
            (3, "\"F\""),
            (3, "<select>"),
            (4, "<option>"),
            (5, "\"G\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<dd><dd><dt><dt><dd><li><li>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dd>"),
            (2, "<dd>"),
            (2, "<dt>"),
            (2, "<dt>"),
            (2, "<dd>"),
            (3, "<li>"),
            (3, "<li>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<div><b></div><div><nobr>a<nobr>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<b>"),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<nobr>"),
            (5, "\"a\""),
            (4, "<nobr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<head></head>\n<body></body>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "\"\n\""), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<head></head> <style></style>ddd",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (1, "\" \""),
            (1, "<body>"),
            (2, "\"ddd\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<kbd><table></kbd><col><select><tr>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<kbd>"),
            (3, "<select>"),
            (3, "<table>"),
            (4, "<colgroup>"),
            (5, "<col>"),
            (4, "<tbody>"),
            (5, "<tr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<kbd><table></kbd><col><select><tr></table><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<kbd>"),
            (3, "<select>"),
            (3, "<table>"),
            (4, "<colgroup>"),
            (5, "<col>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (3, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<a><li><style></style><title></title></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (2, "<li>"),
            (3, "<a>"),
            (4, "<style>"),
            (4, "<title>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<font></p><p><meta><title></title></font>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<font>"),
            (3, "<p>"),
            (2, "<p>"),
            (3, "<font>"),
            (4, "<meta>"),
            (4, "<title>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<a><center><title></title><a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (2, "<center>"),
            (3, "<a>"),
            (4, "<title>"),
            (3, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<svg><title><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg title>"),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<svg><title><rect><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg title>"),
            (4, "<rect>"),
            (5, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<svg><title><svg><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg title>"),
            (4, "<svg svg>"),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<img <=\"\" FAIL>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<img>"),
            (3, "<=\"\""),
            (3, "fail=\"\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<ul><li><div id=\'foo\'/>A</li><li>B<div>C</div></li></ul>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ul>"),
            (3, "<li>"),
            (4, "<div>"),
            (5, "id=\"foo\""),
            (5, "\"A\""),
            (3, "<li>"),
            (4, "\"B\""),
            (4, "<div>"),
            (5, "\"C\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "<svg><em><desc></em>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (2, "<em>"),
            (3, "<desc>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<table><tr><td><svg><desc><td></desc><circle>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<svg svg>"),
            (7, "<svg desc>"),
            (5, "<td>"),
            (6, "<circle>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<svg><tfoot></mi><td>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg tfoot>"),
            (4, "<svg td>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<math><mrow><mrow><mn>1</mn></mrow><mi>a</mi></mrow></math>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mrow>"),
            (4, "<math mrow>"),
            (5, "<math mn>"),
            (6, "\"1\""),
            (4, "<math mi>"),
            (5, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<!doctype html><input type=\"hidden\"><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<!doctype html><input type=\"button\"><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<input>"),
            (3, "type=\"button\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
