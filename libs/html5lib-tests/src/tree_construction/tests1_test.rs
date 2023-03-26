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
        data: "<p>One<p>Two",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"One\""),
            (2, "<p>"),
            (3, "\"Two\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "Line1<br>Line2<br>Line3<br>Line4",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Line1\""),
            (2, "<br>"),
            (2, "\"Line2\""),
            (2, "<br>"),
            (2, "\"Line3\""),
            (2, "<br>"),
            (2, "\"Line4\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<head>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<body>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<html><head>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<html><head></head>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<html><head></head><body>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<html><head></head><body></body>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<html><head><body></body></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<html><head></body></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<html><head><body></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<html><body></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<body></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<head></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "</head>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "</body>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "</html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<b><table><td><i></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<i>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<b><table><td></b><i></table>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<i>"),
            (3, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<h1>Hello<h2>World",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<h1>"),
            (3, "\"Hello\""),
            (2, "<h2>"),
            (3, "\"World\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<a><p>X<a>Y</a>Z</p></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (2, "<p>"),
            (3, "<a>"),
            (4, "\"X\""),
            (3, "<a>"),
            (4, "\"Y\""),
            (3, "\"Z\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<b><button>foo</b>bar",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (2, "<button>"),
            (3, "<b>"),
            (4, "\"foo\""),
            (3, "\"bar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!DOCTYPE html><span><button>foo</span>bar",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<span>"),
            (3, "<button>"),
            (4, "\"foobar\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<p><b><div><marquee></p></b></div>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<b>"),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<marquee>"),
            (5, "<p>"),
            (5, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<script><div></script></div><title><p></title><p><p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<div>\""),
            (2, "<title>"),
            (3, "\"<p>\""),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!--><div>--<!-->",
        document: vec![
            (0, "<!--  -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"--\""),
            (3, "<!--  -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<p><hr></p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<hr>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<select><b><option><select><option></b></select>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (2, "<option>"),
            (3, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<a><table><td><a><table></table><a></tr><a></table><b>X</b>C<a>Y",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<a>"),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<a>"),
            (8, "<table>"),
            (7, "<a>"),
            (2, "<a>"),
            (3, "<b>"),
            (4, "\"X\""),
            (3, "\"C\""),
            (2, "<a>"),
            (3, "\"Y\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<a X>0<b>1<a Y>2",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "x=\"\""),
            (3, "\"0\""),
            (3, "<b>"),
            (4, "\"1\""),
            (2, "<b>"),
            (3, "<a>"),
            (4, "y=\"\""),
            (4, "\"2\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!-----><font><div>hello<table>excite!<b>me!<th><i>please!</tr><!--X-->",
        document: vec![
            (0, "<!-- - -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<font>"),
            (3, "<div>"),
            (4, "\"helloexcite!\""),
            (4, "<b>"),
            (5, "\"me!\""),
            (4, "<table>"),
            (5, "<tbody>"),
            (6, "<tr>"),
            (7, "<th>"),
            (8, "<i>"),
            (9, "\"please!\""),
            (6, "<!-- X -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!DOCTYPE html><li>hello<li>world<ul>how<li>do</ul>you</body><!--do-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<li>"),
            (3, "\"hello\""),
            (2, "<li>"),
            (3, "\"world\""),
            (3, "<ul>"),
            (4, "\"how\""),
            (4, "<li>"),
            (5, "\"do\""),
            (3, "\"you\""),
            (1, "<!-- do -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!DOCTYPE html>A<option>B<optgroup>C<select>D</option>E",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
            (2, "<option>"),
            (3, "\"B\""),
            (2, "<optgroup>"),
            (3, "\"C\""),
            (3, "<select>"),
            (4, "\"DE\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"<\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<#",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"<#\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "</",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"</\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "</#",
        document: vec![
            (0, "<!-- # -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<?",
        document: vec![
            (0, "<!-- ? -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<?#",
        document: vec![
            (0, "<!-- ?# -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<!",
        document: vec![
            (0, "<!--  -->"),
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
        data: "<!#",
        document: vec![
            (0, "<!-- # -->"),
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
        data: "<?COMMENT?>",
        document: vec![
            (0, "<!-- ?COMMENT? -->"),
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
        data: "<!COMMENT>",
        document: vec![
            (0, "<!-- COMMENT -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "</ COMMENT >",
        document: vec![
            (0, "<!--  COMMENT  -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<?COM--MENT?>",
        document: vec![
            (0, "<!-- ?COM--MENT? -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<!COM--MENT>",
        document: vec![
            (0, "<!-- COM--MENT -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "</ COM--MENT >",
        document: vec![
            (0, "<!--  COM--MENT  -->"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<!DOCTYPE html><style> EOF",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\" EOF\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "<!DOCTYPE html><script> <!-- </script> --> </script> EOF",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\" <!-- \""),
            (2, "\" \""),
            (1, "<body>"),
            (2, "\"-->  EOF\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<b><p></b>TEST",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (2, "<p>"),
            (3, "<b>"),
            (3, "\"TEST\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<p id=a><b><p id=b></b>TEST",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "id=\"a\""),
            (3, "<b>"),
            (2, "<p>"),
            (3, "id=\"b\""),
            (3, "\"TEST\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<b id=a><p><b id=b></p></b>TEST",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "id=\"a\""),
            (3, "<p>"),
            (4, "<b>"),
            (5, "id=\"b\""),
            (3, "\"TEST\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<!DOCTYPE html><title>U-test</title><body><div><p>Test<u></p></div></body>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"U-test\""),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<p>"),
            (4, "\"Test\""),
            (4, "<u>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<!DOCTYPE html><font><table></font></table></font>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<font>"),
            (3, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<font><p>hello<b>cruel</font>world",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<font>"),
            (2, "<p>"),
            (3, "<font>"),
            (4, "\"hello\""),
            (4, "<b>"),
            (5, "\"cruel\""),
            (3, "<b>"),
            (4, "\"world\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<b>Test</i>Test",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"TestTest\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<b>A<cite>B<div>C",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"A\""),
            (3, "<cite>"),
            (4, "\"B\""),
            (4, "<div>"),
            (5, "\"C\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "<b>A<cite>B<div>C</cite>D",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"A\""),
            (3, "<cite>"),
            (4, "\"B\""),
            (4, "<div>"),
            (5, "\"CD\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<b>A<cite>B<div>C</b>D",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"A\""),
            (3, "<cite>"),
            (4, "\"B\""),
            (2, "<div>"),
            (3, "<b>"),
            (4, "\"C\""),
            (3, "\"D\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0062() {
    parse(Test {
        data: "<DIV>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<div>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0063() {
    parse(Test {
        data: "<DIV> abc",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0064() {
    parse(Test {
        data: "<DIV> abc <B>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0065() {
    parse(Test {
        data: "<DIV> abc <B> def",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0066() {
    parse(Test {
        data: "<DIV> abc <B> def <I>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0067() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0068() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (5, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0069() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P> jkl",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (5, "<p>"),
            (6, "\" jkl\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0070() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P> jkl </B>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (3, "<i>"),
            (4, "<p>"),
            (5, "<b>"),
            (6, "\" jkl \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0071() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P> jkl </B> mno",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (3, "<i>"),
            (4, "<p>"),
            (5, "<b>"),
            (6, "\" jkl \""),
            (5, "\" mno\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0072() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P> jkl </B> mno </I>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (3, "<i>"),
            (3, "<p>"),
            (4, "<i>"),
            (5, "<b>"),
            (6, "\" jkl \""),
            (5, "\" mno \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0073() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P> jkl </B> mno </I> pqr",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (3, "<i>"),
            (3, "<p>"),
            (4, "<i>"),
            (5, "<b>"),
            (6, "\" jkl \""),
            (5, "\" mno \""),
            (4, "\" pqr\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0074() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P> jkl </B> mno </I> pqr </P>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (3, "<i>"),
            (3, "<p>"),
            (4, "<i>"),
            (5, "<b>"),
            (6, "\" jkl \""),
            (5, "\" mno \""),
            (4, "\" pqr \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0075() {
    parse(Test {
        data: "<DIV> abc <B> def <I> ghi <P> jkl </B> mno </I> pqr </P> stu",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\" abc \""),
            (3, "<b>"),
            (4, "\" def \""),
            (4, "<i>"),
            (5, "\" ghi \""),
            (3, "<i>"),
            (3, "<p>"),
            (4, "<i>"),
            (5, "<b>"),
            (6, "\" jkl \""),
            (5, "\" mno \""),
            (4, "\" pqr \""),
            (3, "\" stu\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0076() {
    parse(Test {
        data: "<test attribute---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<test>"),
            (3, "attribute----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------=\"\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0077() {
    parse(Test {
        data: "<a href=\"blah\">aba<table><a href=\"foo\">br<tr><td></td></tr>x</table>aoe",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "href=\"blah\""),
            (3, "\"aba\""),
            (3, "<a>"),
            (4, "href=\"foo\""),
            (4, "\"br\""),
            (3, "<a>"),
            (4, "href=\"foo\""),
            (4, "\"x\""),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (2, "<a>"),
            (3, "href=\"foo\""),
            (3, "\"aoe\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0078() {
    parse(Test {
        data: "<a href=\"blah\">aba<table><tr><td><a href=\"foo\">br</td></tr>x</table>aoe",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "href=\"blah\""),
            (3, "\"abax\""),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<a>"),
            (8, "href=\"foo\""),
            (8, "\"br\""),
            (3, "\"aoe\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0079() {
    parse(Test {
        data: "<table><a href=\"blah\">aba<tr><td><a href=\"foo\">br</td></tr>x</table>aoe",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "href=\"blah\""),
            (3, "\"aba\""),
            (2, "<a>"),
            (3, "href=\"blah\""),
            (3, "\"x\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<a>"),
            (7, "href=\"foo\""),
            (7, "\"br\""),
            (2, "<a>"),
            (3, "href=\"blah\""),
            (3, "\"aoe\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0080() {
    parse(Test {
        data: "<a href=a>aa<marquee>aa<a href=b>bb</marquee>aa",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "href=\"a\""),
            (3, "\"aa\""),
            (3, "<marquee>"),
            (4, "\"aa\""),
            (4, "<a>"),
            (5, "href=\"b\""),
            (5, "\"bb\""),
            (3, "\"aa\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0081() {
    parse(Test {
        data: "<wbr><strike><code></strike><code><strike></code>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<wbr>"),
            (2, "<strike>"),
            (3, "<code>"),
            (2, "<code>"),
            (3, "<code>"),
            (4, "<strike>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0082() {
    parse(Test {
        data: "<!DOCTYPE html><spacer>foo",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<spacer>"),
            (3, "\"foo\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0083() {
    parse(Test {
        data: "<title><meta></title><link><title><meta></title>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"<meta>\""),
            (2, "<link>"),
            (2, "<title>"),
            (3, "\"<meta>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0084() {
    parse(Test {
        data: "<style><!--</style><meta><script>--><link></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--\""),
            (2, "<meta>"),
            (2, "<script>"),
            (3, "\"--><link>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0085() {
    parse(Test {
        data: "<head><meta></head><link>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<meta>"),
            (2, "<link>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0086() {
    parse(Test {
        data: "<table><tr><tr><td><td><span><th><span>X</table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (4, "<tr>"),
            (5, "<td>"),
            (5, "<td>"),
            (6, "<span>"),
            (5, "<th>"),
            (6, "<span>"),
            (7, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0087() {
    parse(Test {
        data: "<body><body><base><link><meta><title><p></title><body><p></body>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<base>"),
            (2, "<link>"),
            (2, "<meta>"),
            (2, "<title>"),
            (3, "\"<p>\""),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0088() {
    parse(Test {
        data: "<textarea><p></textarea>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"<p>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0089() {
    parse(Test {
        data: "<p><image></p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<img>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0090() {
    parse(Test {
        data: "<a><table><a></table><p><a><div><a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<a>"),
            (3, "<table>"),
            (2, "<p>"),
            (3, "<a>"),
            (2, "<div>"),
            (3, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0091() {
    parse(Test {
        data: "<head></p><meta><p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<meta>"),
            (1, "<body>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0092() {
    parse(Test {
        data: "<head></html><meta><p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<meta>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0093() {
    parse(Test {
        data: "<b><table><td><i></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<i>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0094() {
    parse(Test {
        data: "<b><table><td></b><i></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<i>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0095() {
    parse(Test {
        data: "<h1><h2>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<h1>"),
            (2, "<h2>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0096() {
    parse(Test {
        data: "<a><p><a></a></p></a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (2, "<p>"),
            (3, "<a>"),
            (3, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0097() {
    parse(Test {
        data: "<b><button></b></button></b>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (2, "<button>"),
            (3, "<b>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0098() {
    parse(Test {
        data: "<p><b><div><marquee></p></b></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<b>"),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<marquee>"),
            (5, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0099() {
    parse(Test {
        data: "<script></script></div><title></title><p><p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (2, "<title>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0100() {
    parse(Test {
        data: "<p><hr></p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<hr>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0101() {
    parse(Test {
        data: "<select><b><option><select><option></b></select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (2, "<option>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0102() {
    parse(Test {
        data: "<html><head><title></title><body></body></html>",
        document: vec![(0, "<html>"), (1, "<head>"), (2, "<title>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0103() {
    parse(Test {
        data: "<a><table><td><a><table></table><a></tr><a></table><a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<a>"),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<a>"),
            (8, "<table>"),
            (7, "<a>"),
            (2, "<a>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0104() {
    parse(Test {
        data: "<ul><li></li><div><li></div><li><li><div><li><address><li><b><em></b><li></ul>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ul>"),
            (3, "<li>"),
            (3, "<div>"),
            (4, "<li>"),
            (3, "<li>"),
            (3, "<li>"),
            (4, "<div>"),
            (3, "<li>"),
            (4, "<address>"),
            (3, "<li>"),
            (4, "<b>"),
            (5, "<em>"),
            (3, "<li>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0105() {
    parse(Test {
        data: "<ul><li><ul></li><li>a</li></ul></li></ul>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ul>"),
            (3, "<li>"),
            (4, "<ul>"),
            (5, "<li>"),
            (6, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0106() {
    parse(Test {
        data: "<frameset><frame><frameset><frame></frameset><noframes></noframes></frameset>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
            (2, "<frameset>"),
            (3, "<frame>"),
            (2, "<noframes>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0107() {
    parse(Test {
        data: "<h1><table><td><h3></table><h3></h1>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<h1>"),
            (3, "<table>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "<h3>"),
            (2, "<h3>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0108() {
    parse(Test {
        data: "<table><colgroup><col><colgroup><col><col><col><colgroup><col><col><thead><tr><td></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
            (4, "<col>"),
            (3, "<colgroup>"),
            (4, "<col>"),
            (4, "<col>"),
            (4, "<col>"),
            (3, "<colgroup>"),
            (4, "<col>"),
            (4, "<col>"),
            (3, "<thead>"),
            (4, "<tr>"),
            (5, "<td>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0109() {
    parse(Test {
        data: "<table><col><tbody><col><tr><col><td><col></table><col>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
            (4, "<col>"),
            (3, "<tbody>"),
            (3, "<colgroup>"),
            (4, "<col>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (3, "<colgroup>"),
            (4, "<col>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (3, "<colgroup>"),
            (4, "<col>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0110() {
    parse(Test {
        data: "<table><colgroup><tbody><colgroup><tr><colgroup><td><colgroup></table><colgroup>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
            (3, "<tbody>"),
            (3, "<colgroup>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (3, "<colgroup>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (3, "<colgroup>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0111() {
    parse(Test {
        data: "</strong></b></em></i></u></strike></s></blink></tt></pre></big></small></font></select></h1></h2></h3></h4></h5></h6></body></br></a></img></title></span></style></script></table></th></td></tr></frame></area></link></param></hr></input></col></base></meta></basefont></bgsound></embed></spacer></p></dd></dt></caption></colgroup></tbody></tfoot></thead></address></blockquote></center></dir></div></dl></fieldset></listing></menu></ol></ul></li></nobr></wbr></form></button></marquee></object></html></frameset></head></iframe></image></isindex></noembed></noframes></noscript></optgroup></option></plaintext></textarea>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<br>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0112() {
    parse(Test {
        data: "<table><tr></strong></b></em></i></u></strike></s></blink></tt></pre></big></small></font></select></h1></h2></h3></h4></h5></h6></body></br></a></img></title></span></style></script></table></th></td></tr></frame></area></link></param></hr></input></col></base></meta></basefont></bgsound></embed></spacer></p></dd></dt></caption></colgroup></tbody></tfoot></thead></address></blockquote></center></dir></div></dl></fieldset></listing></menu></ol></ul></li></nobr></wbr></form></button></marquee></object></html></frameset></head></iframe></image></isindex></noembed></noframes></noscript></optgroup></option></plaintext></textarea>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<br>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (2, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0113() {
    parse(Test {
        data: "<frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
