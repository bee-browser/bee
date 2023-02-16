//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0() {
    parse(Test {
        data: r#"<!DOCTYPE html>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE html>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_1() {
    parse(Test {
        data: r#"<!dOctYpE HtMl>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE html>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_2() {
    parse(Test {
        data: r#"<!DOCTYPEhtml>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE html>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_3() {
    parse(Test {
        data: r#"<!DOCTYPE>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE >"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_4() {
    parse(Test {
        data: r#"<!DOCTYPE >Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE >"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_5() {
    parse(Test {
        data: r#"<!DOCTYPE potato>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_6() {
    parse(Test {
        data: r#"<!DOCTYPE potato >Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_7() {
    parse(Test {
        data: r#"<!DOCTYPE potato taco>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_8() {
    parse(Test {
        data: r#"<!DOCTYPE potato taco "ddd>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_9() {
    parse(Test {
        data: r#"<!DOCTYPE potato sYstEM>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_10() {
    parse(Test {
        data: r#"<!DOCTYPE potato sYstEM    >Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_11() {
    parse(Test {
        data: r#"<!DOCTYPE   potato       sYstEM  ggg>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_12() {
    parse(Test {
        data: r#"<!DOCTYPE potato SYSTEM taco  >Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_13() {
    parse(Test {
        data: r#"<!DOCTYPE potato SYSTEM 'taco"'>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato "" "taco"">"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_14() {
    parse(Test {
        data: r#"<!DOCTYPE potato SYSTEM "taco">Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato "" "taco">"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_15() {
    parse(Test {
        data: r#"<!DOCTYPE potato SYSTEM "tai'co">Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato "" "tai'co">"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_16() {
    parse(Test {
        data: r#"<!DOCTYPE potato SYSTEMtaco "ddd">Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_17() {
    parse(Test {
        data: r#"<!DOCTYPE potato grass SYSTEM taco>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_18() {
    parse(Test {
        data: r#"<!DOCTYPE potato pUbLIc>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_19() {
    parse(Test {
        data: r#"<!DOCTYPE potato pUbLIc >Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_20() {
    parse(Test {
        data: r#"<!DOCTYPE potato pUbLIcgoof>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_21() {
    parse(Test {
        data: r#"<!DOCTYPE potato PUBLIC goof>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_22() {
    parse(Test {
        data: r#"<!DOCTYPE potato PUBLIC "go'of">Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato "go'of" "">"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_23() {
    parse(Test {
        data: r#"<!DOCTYPE potato PUBLIC 'go'of'>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato "go" "">"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_24() {
    parse(Test {
        data: r#"<!DOCTYPE potato PUBLIC 'go:hh   of' >Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato "go:hh   of" "">"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_25() {
    parse(Test {
        data: r#"<!DOCTYPE potato PUBLIC "W3C-//dfdf" SYSTEM ggg>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE potato "W3C-//dfdf" "">"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_26() {
    parse(Test {
        data: r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN"
"http://www.w3.org/TR/html4/strict.dtd">Hello"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_27() {
    parse(Test {
        data: r#"<!DOCTYPE ...>Hello"#,
        document: vec![
            (0, r#"<!DOCTYPE ...>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""Hello""#),
        ],
    });
}

#[test]
fn test_28() {
    parse(Test {
        data: r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN"
"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_29() {
    parse(Test {
        data: r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Frameset//EN"
"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd">"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//W3C//DTD XHTML 1.0 Frameset//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_30() {
    parse(Test {
        data: r#"<!DOCTYPE root-element [SYSTEM OR PUBLIC FPI] "uri" [
<!-- internal declarations -->
]>"#,
        document: vec![
            (0, r#"<!DOCTYPE root-element>"#),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""]>""#),
        ],
    });
}

#[test]
fn test_31() {
    parse(Test {
        data: r#"<!DOCTYPE html PUBLIC
"-//WAPFORUM//DTD XHTML Mobile 1.0//EN"
"http://www.wapforum.org/DTD/xhtml-mobile10.dtd">"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//WAPFORUM//DTD XHTML Mobile 1.0//EN" "http://www.wapforum.org/DTD/xhtml-mobile10.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_32() {
    parse(Test {
        data: r#"<!DOCTYPE HTML SYSTEM "http://www.w3.org/DTD/HTML4-strict.dtd"><body><b>Mine!</b></body>"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "" "http://www.w3.org/DTD/HTML4-strict.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#"<b>"#),
            (3, r#""Mine!""#),
        ],
    });
}

#[test]
fn test_33() {
    parse(Test {
        data: r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN""http://www.w3.org/TR/html4/strict.dtd">"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_34() {
    parse(Test {
        data: r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN"'http://www.w3.org/TR/html4/strict.dtd'>"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_35() {
    parse(Test {
        data: r#"<!DOCTYPE HTML PUBLIC"-//W3C//DTD HTML 4.01//EN"'http://www.w3.org/TR/html4/strict.dtd'>"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}

#[test]
fn test_36() {
    parse(Test {
        data: r#"<!DOCTYPE HTML PUBLIC'-//W3C//DTD HTML 4.01//EN''http://www.w3.org/TR/html4/strict.dtd'>"#,
        document: vec![
            (
                0,
                r#"<!DOCTYPE html "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            ),
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
        ],
    });
}
//</coverage:exclude>
