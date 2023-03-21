//<coverage:exclude>
use super::helper::parse;
use super::helper::Scripting;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html><body><foo>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<foo>"),
            (3, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!DOCTYPE html><body><area>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<area>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!DOCTYPE html><body><base>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<base>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!DOCTYPE html><body><basefont>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<basefont>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!DOCTYPE html><body><bgsound>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<bgsound>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html><body><br>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<br>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE html><body><col>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!DOCTYPE html><body><command>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<command>"),
            (3, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!DOCTYPE html><body><embed>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<embed>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!DOCTYPE html><body><frame>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!DOCTYPE html><body><hr>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<hr>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!DOCTYPE html><body><img>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<img>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!DOCTYPE html><body><input>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<input>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!DOCTYPE html><body><keygen>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<keygen>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!DOCTYPE html><keygen>A</keygen>B",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<keygen>"),
            (2, "\"AB\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "</keygen>A",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"A\"")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!DOCTYPE html></keygen>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!DOCTYPE html><head></keygen>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!DOCTYPE html><head></head></keygen>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!DOCTYPE html><body></keygen>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!DOCTYPE html><body><link>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<link>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!DOCTYPE html><body><meta>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<meta>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!DOCTYPE html><body><param>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<param>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!DOCTYPE html><body><source>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<source>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!DOCTYPE html><body><track>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<track>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!DOCTYPE html><body><wbr>A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<wbr>"),
            (2, "\"A\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
