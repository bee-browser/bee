//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html>Hello",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!dOctYpE HtMl>Hello",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!DOCTYPEhtml>Hello",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!DOCTYPE>Hello",
        document: vec![
            (0, "<!DOCTYPE >"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!DOCTYPE >Hello",
        document: vec![
            (0, "<!DOCTYPE >"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE potato>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE potato >Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!DOCTYPE potato taco>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!DOCTYPE potato taco \"ddd>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!DOCTYPE potato sYstEM>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!DOCTYPE potato sYstEM    >Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!DOCTYPE   potato       sYstEM  ggg>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!DOCTYPE potato SYSTEM taco  >Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!DOCTYPE potato SYSTEM \'taco\"\'>Hello",
        document: vec![
            (0, "<!DOCTYPE potato \"\" \"taco\"\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!DOCTYPE potato SYSTEM \"taco\">Hello",
        document: vec![
            (0, "<!DOCTYPE potato \"\" \"taco\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<!DOCTYPE potato SYSTEM \"tai\'co\">Hello",
        document: vec![
            (0, "<!DOCTYPE potato \"\" \"tai\'co\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!DOCTYPE potato SYSTEMtaco \"ddd\">Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!DOCTYPE potato grass SYSTEM taco>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!DOCTYPE potato pUbLIc>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!DOCTYPE potato pUbLIc >Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!DOCTYPE potato pUbLIcgoof>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!DOCTYPE potato PUBLIC goof>Hello",
        document: vec![
            (0, "<!DOCTYPE potato>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!DOCTYPE potato PUBLIC \"go\'of\">Hello",
        document: vec![
            (0, "<!DOCTYPE potato \"go\'of\" \"\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!DOCTYPE potato PUBLIC \'go\'of\'>Hello",
        document: vec![
            (0, "<!DOCTYPE potato \"go\" \"\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!DOCTYPE potato PUBLIC \'go:hh   of\' >Hello",
        document: vec![
            (0, "<!DOCTYPE potato \"go:hh   of\" \"\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!DOCTYPE potato PUBLIC \"W3C-//dfdf\" SYSTEM ggg>Hello",
        document: vec![
            (0, "<!DOCTYPE potato \"W3C-//dfdf\" \"\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\"\n\"http://www.w3.org/TR/html4/strict.dtd\">Hello",
        document: vec![
            (0, "<!DOCTYPE html \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!DOCTYPE ...>Hello",
        document: vec![
            (0, "<!DOCTYPE ...>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"\n\"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">",
        document: vec![
            (0, "<!DOCTYPE html \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\"\n\"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">",
        document: vec![
            (0, "<!DOCTYPE html \"-//W3C//DTD XHTML 1.0 Frameset//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<!DOCTYPE root-element [SYSTEM OR PUBLIC FPI] \"uri\" [\n<!-- internal declarations -->\n]>",
        document: vec![
            (0, "<!DOCTYPE root-element>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"]>\""),
        ],
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<!DOCTYPE html PUBLIC\n\"-//WAPFORUM//DTD XHTML Mobile 1.0//EN\"\n\"http://www.wapforum.org/DTD/xhtml-mobile10.dtd\">",
        document: vec![
            (0, "<!DOCTYPE html \"-//WAPFORUM//DTD XHTML Mobile 1.0//EN\" \"http://www.wapforum.org/DTD/xhtml-mobile10.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!DOCTYPE HTML SYSTEM \"http://www.w3.org/DTD/HTML4-strict.dtd\"><body><b>Mine!</b></body>",
        document: vec![
            (0, "<!DOCTYPE html \"\" \"http://www.w3.org/DTD/HTML4-strict.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"Mine!\""),
        ],
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\"\"http://www.w3.org/TR/html4/strict.dtd\">",
        document: vec![
            (0, "<!DOCTYPE html \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01//EN\"\'http://www.w3.org/TR/html4/strict.dtd\'>",
        document: vec![
            (0, "<!DOCTYPE html \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<!DOCTYPE HTML PUBLIC\"-//W3C//DTD HTML 4.01//EN\"\'http://www.w3.org/TR/html4/strict.dtd\'>",
        document: vec![
            (0, "<!DOCTYPE html \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<!DOCTYPE HTML PUBLIC\'-//W3C//DTD HTML 4.01//EN\'\'http://www.w3.org/TR/html4/strict.dtd\'>",
        document: vec![
            (0, "<!DOCTYPE html \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
        ],
    });
}
//</coverage:exclude>
