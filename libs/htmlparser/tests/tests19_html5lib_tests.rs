//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!doctype html><math><mn DefinitionUrl=\"foo\">",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mn>"),
            (4, "definitionURL=\"foo\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!doctype html><html></p><!--foo-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<!-- foo -->"),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!doctype html><head></head></p><!--foo-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<!-- foo -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!doctype html><body><p><pre>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<pre>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!doctype html><body><p><listing>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<listing>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!doctype html><p><plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<plaintext>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!doctype html><p><h1>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<h1>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!doctype html><isindex type=\"hidden\">",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<isindex>"),
            (3, "type=\"hidden\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!doctype html><ruby><p><rp>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<p>"),
            (3, "<rp>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!doctype html><ruby><div><span><rp>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<div>"),
            (4, "<span>"),
            (5, "<rp>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!doctype html><ruby><div><p><rp>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<div>"),
            (4, "<p>"),
            (4, "<rp>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!doctype html><ruby><p><rt>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<p>"),
            (3, "<rt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!doctype html><ruby><div><span><rt>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<div>"),
            (4, "<span>"),
            (5, "<rt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!doctype html><ruby><div><p><rt>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<div>"),
            (4, "<p>"),
            (4, "<rt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<html><ruby>a<rb>b<rt></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rb>"),
            (4, "\"b\""),
            (3, "<rt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<html><ruby>a<rp>b<rt></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rp>"),
            (4, "\"b\""),
            (3, "<rt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<html><ruby>a<rt>b<rt></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rt>"),
            (4, "\"b\""),
            (3, "<rt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<html><ruby>a<rtc>b<rt>c<rb>d</ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rtc>"),
            (4, "\"b\""),
            (4, "<rt>"),
            (5, "\"c\""),
            (3, "<rb>"),
            (4, "\"d\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!doctype html><math/><foo>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (2, "<foo>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!doctype html><svg/><foo>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (2, "<foo>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!doctype html><div></body><!--foo-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (1, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!doctype html><h1><div><h3><span></h1>foo",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<h1>"),
            (3, "<div>"),
            (4, "<h3>"),
            (5, "<span>"),
            (4, "\"foo\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!doctype html><p></h3>foo",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"foo\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!doctype html><h3><li>abc</h2>foo",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<h3>"),
            (3, "<li>"),
            (4, "\"abc\""),
            (2, "\"foo\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!doctype html><table>abc<!--foo-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"abc\""),
            (2, "<table>"),
            (3, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!doctype html><table>  <!--foo-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "\"  \""),
            (3, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!doctype html><table> b <!--foo-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\" b \""),
            (2, "<table>"),
            (3, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!doctype html><select><option><option>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (3, "<option>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!doctype html><select><option></optgroup>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!doctype html><select><option></optgroup>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<!doctype html><dd><optgroup><dd>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dd>"),
            (3, "<optgroup>"),
            (2, "<dd>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<!doctype html><p><math><mi><p><h1>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<math math>"),
            (4, "<math mi>"),
            (5, "<p>"),
            (5, "<h1>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!doctype html><p><math><mo><p><h1>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<math math>"),
            (4, "<math mo>"),
            (5, "<p>"),
            (5, "<h1>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!doctype html><p><math><mn><p><h1>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<math math>"),
            (4, "<math mn>"),
            (5, "<p>"),
            (5, "<h1>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!doctype html><p><math><ms><p><h1>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<math math>"),
            (4, "<math ms>"),
            (5, "<p>"),
            (5, "<h1>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<!doctype html><p><math><mtext><p><h1>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<math math>"),
            (4, "<math mtext>"),
            (5, "<p>"),
            (5, "<h1>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<!doctype html><frameset></noframes>",
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
fn test_0037() {
    parse(Test {
        data: "<!doctype html><html c=d><body></html><html a=b>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "a=\"b\""),
            (1, "c=\"d\""),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<!doctype html><html c=d><frameset></frameset></html><html a=b>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "a=\"b\""),
            (1, "c=\"d\""),
            (1, "<head>"),
            (1, "<frameset>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<!doctype html><html><frameset></frameset></html><!--foo-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (0, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<!doctype html><html><frameset></frameset></html>  ",
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
fn test_0041() {
    parse(Test {
        data: "<!doctype html><html><frameset></frameset></html>abc",
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
fn test_0042() {
    parse(Test {
        data: "<!doctype html><html><frameset></frameset></html><p>",
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
fn test_0043() {
    parse(Test {
        data: "<!doctype html><html><frameset></frameset></html></p>",
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
fn test_0044() {
    parse(Test {
        data: "<html><frameset></frameset></html><!doctype html>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<!doctype html><body><frameset>",
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
fn test_0046() {
    parse(Test {
        data: "<!doctype html><p><frameset><frame>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<!doctype html><p>a<frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<!doctype html><p> <frameset><frame>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<!doctype html><pre><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "<!doctype html><listing><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<listing>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<!doctype html><li><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<li>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<!doctype html><dd><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dd>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<!doctype html><dt><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<!doctype html><button><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<button>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<!doctype html><applet><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<applet>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<!doctype html><marquee><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<marquee>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<!doctype html><object><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<object>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<!doctype html><table><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "<!doctype html><area><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<area>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<!doctype html><basefont><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<basefont>"),
            (1, "<frameset>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "<!doctype html><bgsound><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<bgsound>"),
            (1, "<frameset>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0062() {
    parse(Test {
        data: "<!doctype html><br><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<br>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0063() {
    parse(Test {
        data: "<!doctype html><embed><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<embed>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0064() {
    parse(Test {
        data: "<!doctype html><img><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<img>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0065() {
    parse(Test {
        data: "<!doctype html><input><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<input>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0066() {
    parse(Test {
        data: "<!doctype html><keygen><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<keygen>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0067() {
    parse(Test {
        data: "<!doctype html><wbr><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<wbr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0068() {
    parse(Test {
        data: "<!doctype html><hr><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<hr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0069() {
    parse(Test {
        data: "<!doctype html><textarea></textarea><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0070() {
    parse(Test {
        data: "<!doctype html><xmp></xmp><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<xmp>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0071() {
    parse(Test {
        data: "<!doctype html><iframe></iframe><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<iframe>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0072() {
    parse(Test {
        data: "<!doctype html><select></select><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0073() {
    parse(Test {
        data: "<!doctype html><svg></svg><frameset><frame>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0074() {
    parse(Test {
        data: "<!doctype html><math></math><frameset><frame>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0075() {
    parse(Test {
        data: "<!doctype html><svg><foreignObject><div> <frameset><frame>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0076() {
    parse(Test {
        data: "<!doctype html><svg>a</svg><frameset><frame>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0077() {
    parse(Test {
        data: "<!doctype html><svg> </svg><frameset><frame>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0078() {
    parse(Test {
        data: "<html>aaa<frameset></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"aaa\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0079() {
    parse(Test {
        data: "<html> a <frameset></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"a \"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0080() {
    parse(Test {
        data: "<!doctype html><div><frameset>",
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
fn test_0081() {
    parse(Test {
        data: "<!doctype html><div><body><frameset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0082() {
    parse(Test {
        data: "<!doctype html><p><math></p>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<math math>"),
            (2, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0083() {
    parse(Test {
        data: "<!doctype html><p><math><mn><span></p>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<math math>"),
            (4, "<math mn>"),
            (5, "<span>"),
            (6, "<p>"),
            (6, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0084() {
    parse(Test {
        data: "<!doctype html><math></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0085() {
    parse(Test {
        data: "<!doctype html><meta charset=\"ascii\">",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<meta>"),
            (3, "charset=\"ascii\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0086() {
    parse(Test {
        data:
            "<!doctype html><meta http-equiv=\"content-type\" content=\"text/html;charset=ascii\">",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<meta>"),
            (3, "content=\"text/html;charset=ascii\""),
            (3, "http-equiv=\"content-type\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0087() {
    parse(Test {
        data: "<!doctype html><head><!--aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa--><meta charset=\"utf8\">",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<!-- aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa -->"),
            (2, "<meta>"),
            (3, "charset=\"utf8\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0088() {
    parse(Test {
        data: "<!doctype html><html a=b><head></head><html c=d>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "a=\"b\""),
            (1, "c=\"d\""),
            (1, "<head>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0089() {
    parse(Test {
        data: "<!doctype html><image/>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<img>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0090() {
    parse(Test {
        data: "<!doctype html>a<i>b<table>c<b>d</i>e</b>f",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"a\""),
            (2, "<i>"),
            (3, "\"bc\""),
            (3, "<b>"),
            (4, "\"de\""),
            (3, "\"f\""),
            (3, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0091() {
    parse(Test {
        data: "<!doctype html><table><i>a<b>b<div>c<a>d</i>e</b>f",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<i>"),
            (3, "\"a\""),
            (3, "<b>"),
            (4, "\"b\""),
            (2, "<b>"),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<i>"),
            (5, "\"c\""),
            (5, "<a>"),
            (6, "\"d\""),
            (4, "<a>"),
            (5, "\"e\""),
            (3, "<a>"),
            (4, "\"f\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0092() {
    parse(Test {
        data: "<!doctype html><i>a<b>b<div>c<a>d</i>e</b>f",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<i>"),
            (3, "\"a\""),
            (3, "<b>"),
            (4, "\"b\""),
            (2, "<b>"),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<i>"),
            (5, "\"c\""),
            (5, "<a>"),
            (6, "\"d\""),
            (4, "<a>"),
            (5, "\"e\""),
            (3, "<a>"),
            (4, "\"f\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0093() {
    parse(Test {
        data: "<!doctype html><table><i>a<b>b<div>c</i>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<i>"),
            (3, "\"a\""),
            (3, "<b>"),
            (4, "\"b\""),
            (2, "<b>"),
            (3, "<div>"),
            (4, "<i>"),
            (5, "\"c\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0094() {
    parse(Test {
        data: "<!doctype html><table><i>a<b>b<div>c<a>d</i>e</b>f",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<i>"),
            (3, "\"a\""),
            (3, "<b>"),
            (4, "\"b\""),
            (2, "<b>"),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<i>"),
            (5, "\"c\""),
            (5, "<a>"),
            (6, "\"d\""),
            (4, "<a>"),
            (5, "\"e\""),
            (3, "<a>"),
            (4, "\"f\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0095() {
    parse(Test {
        data: "<!doctype html><table><i>a<div>b<tr>c<b>d</i>e",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<i>"),
            (3, "\"a\""),
            (3, "<div>"),
            (4, "\"b\""),
            (2, "<i>"),
            (3, "\"c\""),
            (3, "<b>"),
            (4, "\"d\""),
            (2, "<b>"),
            (3, "\"e\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0096() {
    parse(Test {
        data: "<!doctype html><table><td><table><i>a<div>b<b>c</i>d",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<i>"),
            (7, "\"a\""),
            (6, "<div>"),
            (7, "<i>"),
            (8, "\"b\""),
            (8, "<b>"),
            (9, "\"c\""),
            (7, "<b>"),
            (8, "\"d\""),
            (6, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0097() {
    parse(Test {
        data: "<!doctype html><body><bgsound>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<bgsound>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0098() {
    parse(Test {
        data: "<!doctype html><body><basefont>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<basefont>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0099() {
    parse(Test {
        data: "<!doctype html><a><b></a><basefont>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<b>"),
            (2, "<basefont>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0100() {
    parse(Test {
        data: "<!doctype html><a><b></a><bgsound>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "<b>"),
            (2, "<bgsound>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0101() {
    parse(Test {
        data: "<!doctype html><figcaption><article></figcaption>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<figcaption>"),
            (3, "<article>"),
            (2, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0102() {
    parse(Test {
        data: "<!doctype html><summary><article></summary>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<summary>"),
            (3, "<article>"),
            (2, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0103() {
    parse(Test {
        data: "<!doctype html><p><a><plaintext>b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<a>"),
            (2, "<plaintext>"),
            (3, "<a>"),
            (4, "\"b\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0104() {
    parse(Test {
        data: "<!DOCTYPE html><div>a<a></div>b<p>c</p>d",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"a\""),
            (3, "<a>"),
            (2, "<a>"),
            (3, "\"b\""),
            (3, "<p>"),
            (4, "\"c\""),
            (3, "\"d\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
