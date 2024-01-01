//<coverage:exclude>
mod helper;

use test_log::test;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<nobr>X",
        document: vec![(0, "<nobr>"), (1, "\"X\"")],
        context_element: Some(("svg", "path")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<font color></font>X",
        document: vec![(0, "<font>"), (1, "color=\"\""), (0, "\"X\"")],
        context_element: Some(("svg", "path")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<font></font>X",
        document: vec![(0, "<svg font>"), (0, "\"X\"")],
        context_element: Some(("svg", "path")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<g></path>X",
        document: vec![(0, "<svg g>"), (1, "\"X\"")],
        context_element: Some(("svg", "path")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "</path>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "path")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "</foreignObject>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "foreignObject")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "</desc>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "</title>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "title")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "</svg>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "</mfenced>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "mfenced")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "</malignmark>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "malignmark")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "</math>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "math")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "</annotation-xml>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "annotation-xml")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "</mtext>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "mtext")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "</mi>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "mi")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "</mo>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "mo")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "</mn>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "mn")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "</ms>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("math", "ms")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<b></b><mglyph/><i></i><malignmark/><u></u><ms/>X",
        document: vec![
            (0, "<b>"),
            (0, "<math mglyph>"),
            (0, "<i>"),
            (0, "<math malignmark>"),
            (0, "<u>"),
            (0, "<ms>"),
            (1, "\"X\""),
        ],
        context_element: Some(("math", "ms")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<malignmark></malignmark>",
        document: vec![(0, "<math malignmark>")],
        context_element: Some(("math", "ms")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("math", "ms")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("math", "ms")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<b></b><mglyph/><i></i><malignmark/><u></u><mn/>X",
        document: vec![
            (0, "<b>"),
            (0, "<math mglyph>"),
            (0, "<i>"),
            (0, "<math malignmark>"),
            (0, "<u>"),
            (0, "<mn>"),
            (1, "\"X\""),
        ],
        context_element: Some(("math", "mn")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<malignmark></malignmark>",
        document: vec![(0, "<math malignmark>")],
        context_element: Some(("math", "mn")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("math", "mn")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("math", "mn")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<b></b><mglyph/><i></i><malignmark/><u></u><mo/>X",
        document: vec![
            (0, "<b>"),
            (0, "<math mglyph>"),
            (0, "<i>"),
            (0, "<math malignmark>"),
            (0, "<u>"),
            (0, "<mo>"),
            (1, "\"X\""),
        ],
        context_element: Some(("math", "mo")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<malignmark></malignmark>",
        document: vec![(0, "<math malignmark>")],
        context_element: Some(("math", "mo")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("math", "mo")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("math", "mo")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<b></b><mglyph/><i></i><malignmark/><u></u><mi/>X",
        document: vec![
            (0, "<b>"),
            (0, "<math mglyph>"),
            (0, "<i>"),
            (0, "<math malignmark>"),
            (0, "<u>"),
            (0, "<mi>"),
            (1, "\"X\""),
        ],
        context_element: Some(("math", "mi")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<malignmark></malignmark>",
        document: vec![(0, "<math malignmark>")],
        context_element: Some(("math", "mi")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("math", "mi")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("math", "mi")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<b></b><mglyph/><i></i><malignmark/><u></u><mtext/>X",
        document: vec![
            (0, "<b>"),
            (0, "<math mglyph>"),
            (0, "<i>"),
            (0, "<math malignmark>"),
            (0, "<u>"),
            (0, "<mtext>"),
            (1, "\"X\""),
        ],
        context_element: Some(("math", "mtext")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<malignmark></malignmark>",
        document: vec![(0, "<math malignmark>")],
        context_element: Some(("math", "mtext")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("math", "mtext")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("math", "mtext")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("math", "annotation-xml")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<math figure>")],
        context_element: Some(("math", "annotation-xml")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("math", "math")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<math figure>")],
        context_element: Some(("math", "math")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("svg", "foreignObject")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("svg", "foreignObject")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("svg", "title")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("svg", "title")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<div><h1>X</h1></div>",
        document: vec![(0, "<div>"), (1, "<h1>"), (2, "\"X\"")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<div></div>",
        document: vec![(0, "<div>")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "<figure></figure>",
        document: vec![(0, "<figure>")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<plaintext><foo>",
        document: vec![(0, "<plaintext>"), (1, "\"<foo>\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<frameset>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<head>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<body>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<html>X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<html class=\"foo\">X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<body class=\"foo\">X",
        document: vec![(0, "\"X\"")],
        context_element: Some(("svg", "desc")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<svg><p>",
        document: vec![(0, "<svg svg>"), (0, "<p>")],
        context_element: Some(("html", "div")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "<p>",
        document: vec![(0, "<p>")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<svg></p><foo>",
        document: vec![(0, "<svg svg>"), (0, "<p>"), (0, "<foo>")],
        context_element: Some(("html", "div")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "<svg></br><foo>",
        document: vec![(0, "<svg svg>"), (0, "<br>"), (0, "<foo>")],
        context_element: Some(("html", "div")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0062() {
    parse(Test {
        data: "</p><foo>",
        document: vec![(0, "<p>"), (0, "<svg foo>")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0063() {
    parse(Test {
        data: "</br><foo>",
        document: vec![(0, "<br>"), (0, "<svg foo>")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0064() {
    parse(Test {
        data: "<body><foo>",
        document: vec![(0, "<svg foo>")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0065() {
    parse(Test {
        data: "<p><foo>",
        document: vec![(0, "<p>"), (1, "<foo>")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0066() {
    parse(Test {
        data: "<p></p><foo>",
        document: vec![(0, "<p>"), (0, "<svg foo>")],
        context_element: Some(("svg", "svg")),
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>