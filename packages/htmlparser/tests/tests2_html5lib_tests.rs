//<coverage:exclude>
mod helper;

use test_log::test;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html>Test",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<textarea>test</div>test",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"test</div>test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<table><td>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<table><td>test</tbody></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<frame>test",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"test\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html><frameset>test",
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
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE html><frameset> te st",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "\"  \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!DOCTYPE html><frameset></frameset> te st",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (1, "\"  \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!DOCTYPE html><frameset><!DOCTYPE html>",
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
fn test_0009() {
    parse(Test {
        data: "<!DOCTYPE html><font><p><b>test</font>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<font>"),
            (2, "<p>"),
            (3, "<font>"),
            (4, "<b>"),
            (5, "\"test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!DOCTYPE html><dt><div><dd>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dt>"),
            (3, "<div>"),
            (2, "<dd>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<script></x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</x\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<table><plaintext><td>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"<td>\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<plaintext></plaintext>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"</plaintext>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!DOCTYPE html><table><tr>TEST",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"TEST\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<!DOCTYPE html><body t1=1><body t2=2><body t3=3 t4=4>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "t1=\"1\""),
            (2, "t2=\"2\""),
            (2, "t3=\"3\""),
            (2, "t4=\"4\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "</b test",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!DOCTYPE html></b test<b &=&amp>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!doctypehtml><scrIPt type=text/x-foobar;baz>X</SCRipt",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "type=\"text/x-foobar;baz\""),
            (3, "\"X</SCRipt\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "&",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"&\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "&#",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"&#\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "&#X",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"&#X\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "&#x",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"&#x\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "&#45",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"-\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "&x-test",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"&x-test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!doctypehtml><p><li>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<li>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!doctypehtml><p><dt>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<dt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!doctypehtml><p><dd>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<dd>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!doctypehtml><p><form>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<form>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!DOCTYPE html><p></P>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "&AMP",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"&\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "&AMp;",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"&AMp;\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!DOCTYPE html><html><head></head><body><thisISasillyTESTelementNameToMakeSureCrazyTagNamesArePARSEDcorrectLY>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<thisisasillytestelementnametomakesurecrazytagnamesareparsedcorrectly>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!DOCTYPE html>X</body>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"XX\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!DOCTYPE html><!-- X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<!--  X -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<!DOCTYPE html><table><caption>test TEST</caption><td>test",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "\"test TEST\""),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"test\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<!DOCTYPE html><select><option><optgroup>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (3, "<optgroup>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<!DOCTYPE html><select><optgroup><option></optgroup><option><select><option>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<optgroup>"),
            (4, "<option>"),
            (3, "<option>"),
            (2, "<option>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<!DOCTYPE html><select><optgroup><option><optgroup>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<optgroup>"),
            (4, "<option>"),
            (3, "<optgroup>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<!DOCTYPE html><datalist><option>foo</datalist>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<datalist>"),
            (3, "<option>"),
            (4, "\"foo\""),
            (2, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<!DOCTYPE html><font><input><input></font>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<font>"),
            (3, "<input>"),
            (3, "<input>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<!DOCTYPE html><!-- XXX - XXX -->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<!--  XXX - XXX  -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<!DOCTYPE html><!-- XXX - XXX",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<!--  XXX - XXX -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<!DOCTYPE html><!-- XXX - XXX - XXX -->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<!--  XXX - XXX - XXX  -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "test\ntest",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"test\ntest\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<!DOCTYPE html><body><title>test</body></title>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<title>"),
            (3, "\"test</body>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<!DOCTYPE html><body><title>X</title><meta name=z><link rel=foo><style>\nx { content:\"</style\" } </style>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<title>"),
            (3, "\"X\""),
            (2, "<meta>"),
            (3, "name=\"z\""),
            (2, "<link>"),
            (3, "rel=\"foo\""),
            (2, "<style>"),
            (3, "\"\nx { content:\"</style\" } \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<!DOCTYPE html><select><optgroup></optgroup></select>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<optgroup>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: " \n ",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<!DOCTYPE html>  <html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "<!DOCTYPE html><script>\n</script>  <title>x</title>  </head>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"\n\""),
            (2, "\"  \""),
            (2, "<title>"),
            (3, "\"x\""),
            (2, "\"  \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<!DOCTYPE html><html><body><html id=x>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "id=\"x\""),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<!DOCTYPE html>X</body><html id=\"x\">",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "id=\"x\""),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<!DOCTYPE html><head><html id=x>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "id=\"x\""),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<!DOCTYPE html>X</html>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"XX\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<!DOCTYPE html>X</html> ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"X \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<!DOCTYPE html>X</html><p>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"X\""),
            (2, "<p>"),
            (3, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<!DOCTYPE html>X<p/x/y/z>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"X\""),
            (2, "<p>"),
            (3, "x=\"\""),
            (3, "y=\"\""),
            (3, "z=\"\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<!DOCTYPE html><!--x--",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<!-- x -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "<!DOCTYPE html><table><tr><td></p></table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<!DOCTYPE <!DOCTYPE HTML>><!--<!--x-->-->",
        document: vec![
            (0, "<!DOCTYPE <!doctype>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\">\""),
            (2, "<!-- <!--x -->"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "<!doctype html><div><form></form><div></div></div>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<form>"),
            (3, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
