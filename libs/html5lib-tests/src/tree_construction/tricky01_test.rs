//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<b><p>Bold </b> Not bold</p>\nAlso not bold.",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "\"Bold \""),
            (3, "\" Not bold\""),
            (2, "\"\nAlso not bold.\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<html>\n<font color=red><i>Italic and Red<p>Italic and Red </font> Just italic.</p> Italic only.</i> Plain\n<p>I should not be red. <font color=red>Red. <i>Italic and red.</p>\n<p>Italic and red. </i> Red.</font> I should not be red.</p>\n<b>Bold <i>Bold and italic</b> Only Italic </i> Plain",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<font>"),
            (3, "color=\"red\""),
            (3, "<i>"),
            (4, "\"Italic and Red\""),
            (2, "<i>"),
            (3, "<p>"),
            (4, "<font>"),
            (5, "color=\"red\""),
            (5, "\"Italic and Red \""),
            (4, "\" Just italic.\""),
            (3, "\" Italic only.\""),
            (2, "\" Plain\n\""),
            (2, "<p>"),
            (3, "\"I should not be red. \""),
            (3, "<font>"),
            (4, "color=\"red\""),
            (4, "\"Red. \""),
            (4, "<i>"),
            (5, "\"Italic and red.\""),
            (2, "<font>"),
            (3, "color=\"red\""),
            (3, "<i>"),
            (4, "\"\n\""),
            (2, "<p>"),
            (3, "<font>"),
            (4, "color=\"red\""),
            (4, "<i>"),
            (5, "\"Italic and red. \""),
            (4, "\" Red.\""),
            (3, "\" I should not be red.\""),
            (2, "\"\n\""),
            (2, "<b>"),
            (3, "\"Bold \""),
            (3, "<i>"),
            (4, "\"Bold and italic\""),
            (2, "<i>"),
            (3, "\" Only Italic \""),
            (2, "\" Plain\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<html><body>\n<p><font size=\"7\">First paragraph.</p>\n<p>Second paragraph.</p></font>\n<b><p><i>Bold and Italic</b> Italic</p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"\n\""),
            (2, "<p>"),
            (3, "<font>"),
            (4, "size=\"7\""),
            (4, "\"First paragraph.\""),
            (2, "<font>"),
            (3, "size=\"7\""),
            (3, "\"\n\""),
            (3, "<p>"),
            (4, "\"Second paragraph.\""),
            (2, "\"\n\""),
            (2, "<b>"),
            (2, "<p>"),
            (3, "<b>"),
            (4, "<i>"),
            (5, "\"Bold and Italic\""),
            (3, "<i>"),
            (4, "\" Italic\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<html>\n<dl>\n<dt><b>Boo\n<dd>Goo?\n</dl>\n</html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dl>"),
            (3, "\"\n\""),
            (3, "<dt>"),
            (4, "<b>"),
            (5, "\"Boo\n\""),
            (3, "<dd>"),
            (4, "<b>"),
            (5, "\"Goo?\n\""),
            (2, "<b>"),
            (3, "\"\n\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<html><body>\n<label><a><div>Hello<div>World</div></a></label>  \n</body></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"\n\""),
            (2, "<label>"),
            (3, "<a>"),
            (3, "<div>"),
            (4, "<a>"),
            (5, "\"Hello\""),
            (5, "<div>"),
            (6, "\"World\""),
            (4, "\"  \n\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<table><center> <font>a</center> <img> <tr><td> </td> </tr> </table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<center>"),
            (3, "\" \""),
            (3, "<font>"),
            (4, "\"a\""),
            (2, "<font>"),
            (3, "<img>"),
            (3, "\" \""),
            (2, "<table>"),
            (3, "\" \""),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\" \""),
            (5, "\" \""),
            (4, "\" \""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<table><tr><p><a><p>You should see this text.",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<a>"),
            (2, "<p>"),
            (3, "<a>"),
            (4, "\"You should see this text.\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<TABLE>\n<TR>\n<CENTER><CENTER><TD></TD></TR><TR>\n<FONT>\n<TABLE><tr></tr></TABLE>\n</P>\n<a></font><font></a>\nThis page contains an insanely badly-nested tag sequence.",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<center>"),
            (3, "<center>"),
            (2, "<font>"),
            (3, "\"\n\""),
            (2, "<table>"),
            (3, "\"\n\""),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "\"\n\""),
            (5, "<td>"),
            (4, "<tr>"),
            (5, "\"\n\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (2, "<font>"),
            (3, "\"\n\""),
            (3, "<p>"),
            (3, "\"\n\""),
            (3, "<a>"),
            (2, "<a>"),
            (3, "<font>"),
            (2, "<font>"),
            (3, "\"\nThis page contains an insanely badly-nested tag sequence.\""),
        ],
        context_element: None,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<html>\n<body>\n<b><nobr><div>This text is in a div inside a nobr</nobr>More text that should not be in the nobr, i.e., the\nnobr should have closed the div inside it implicitly. </b><pre>A pre tag outside everything else.</pre>\n</body>\n</html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"\n\""),
            (2, "<b>"),
            (3, "<nobr>"),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<nobr>"),
            (5, "\"This text is in a div inside a nobr\""),
            (4, "\"More text that should not be in the nobr, i.e., the\nnobr should have closed the div inside it implicitly. \""),
            (3, "<pre>"),
            (4, "\"A pre tag outside everything else.\""),
            (3, "\""),
        ],
        context_element: None,
    });
}
//</coverage:exclude>
