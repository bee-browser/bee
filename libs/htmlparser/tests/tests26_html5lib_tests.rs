//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html><body><a href=\'#1\'><nobr>1<nobr></a><br><a href=\'#2\'><nobr>2<nobr></a><br><a href=\'#3\'><nobr>3<nobr></a>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<a>"),
            (3, "href=\"#1\""),
            (3, "<nobr>"),
            (4, "\"1\""),
            (3, "<nobr>"),
            (2, "<nobr>"),
            (3, "<br>"),
            (3, "<a>"),
            (4, "href=\"#2\""),
            (2, "<a>"),
            (3, "href=\"#2\""),
            (3, "<nobr>"),
            (4, "\"2\""),
            (3, "<nobr>"),
            (2, "<nobr>"),
            (3, "<br>"),
            (3, "<a>"),
            (4, "href=\"#3\""),
            (2, "<a>"),
            (3, "href=\"#3\""),
            (3, "<nobr>"),
            (4, "\"3\""),
            (3, "<nobr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!DOCTYPE html><body><b><nobr>1<nobr></b><i><nobr>2<nobr></i>3",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<nobr>"),
            (4, "\"1\""),
            (3, "<nobr>"),
            (2, "<nobr>"),
            (3, "<i>"),
            (2, "<i>"),
            (3, "<nobr>"),
            (4, "\"2\""),
            (3, "<nobr>"),
            (2, "<nobr>"),
            (3, "\"3\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!DOCTYPE html><body><b><nobr>1<table><nobr></b><i><nobr>2<nobr></i>3",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<nobr>"),
            (4, "\"1\""),
            (4, "<nobr>"),
            (5, "<i>"),
            (4, "<i>"),
            (5, "<nobr>"),
            (6, "\"2\""),
            (5, "<nobr>"),
            (4, "<nobr>"),
            (5, "\"3\""),
            (4, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!DOCTYPE html><body><b><nobr>1<table><tr><td><nobr></b><i><nobr>2<nobr></i>3",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<nobr>"),
            (4, "\"1\""),
            (4, "<table>"),
            (5, "<tbody>"),
            (6, "<tr>"),
            (7, "<td>"),
            (8, "<nobr>"),
            (9, "<i>"),
            (8, "<i>"),
            (9, "<nobr>"),
            (10, "\"2\""),
            (9, "<nobr>"),
            (8, "<nobr>"),
            (9, "\"3\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!DOCTYPE html><body><b><nobr>1<div><nobr></b><i><nobr>2<nobr></i>3",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<nobr>"),
            (4, "\"1\""),
            (2, "<div>"),
            (3, "<b>"),
            (4, "<nobr>"),
            (4, "<nobr>"),
            (3, "<nobr>"),
            (4, "<i>"),
            (3, "<i>"),
            (4, "<nobr>"),
            (5, "\"2\""),
            (4, "<nobr>"),
            (3, "<nobr>"),
            (4, "\"3\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html><body><b><nobr>1<nobr></b><div><i><nobr>2<nobr></i>3",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<nobr>"),
            (4, "\"1\""),
            (3, "<nobr>"),
            (2, "<div>"),
            (3, "<nobr>"),
            (4, "<i>"),
            (3, "<i>"),
            (4, "<nobr>"),
            (5, "\"2\""),
            (4, "<nobr>"),
            (3, "<nobr>"),
            (4, "\"3\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE html><body><b><nobr>1<nobr><ins></b><i><nobr>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<nobr>"),
            (4, "\"1\""),
            (3, "<nobr>"),
            (4, "<ins>"),
            (2, "<nobr>"),
            (3, "<i>"),
            (2, "<i>"),
            (3, "<nobr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!DOCTYPE html><body><b><nobr>1<ins><nobr></b><i>2",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "<nobr>"),
            (4, "\"1\""),
            (4, "<ins>"),
            (3, "<nobr>"),
            (2, "<nobr>"),
            (3, "<i>"),
            (4, "\"2\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!DOCTYPE html><body><b>1<nobr></b><i><nobr>2</i>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<b>"),
            (3, "\"1\""),
            (3, "<nobr>"),
            (2, "<nobr>"),
            (3, "<i>"),
            (2, "<i>"),
            (3, "<nobr>"),
            (4, "\"2\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<p><code x</code></p>\n",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<code>"),
            (4, "code=\"\""),
            (4, "x<=\"\""),
            (2, "<code>"),
            (3, "code=\"\""),
            (3, "x<=\"\""),
            (3, "\"\n\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!DOCTYPE html><svg><foreignObject><p><i></p>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg foreignObject>"),
            (4, "<p>"),
            (5, "<i>"),
            (4, "<i>"),
            (5, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!DOCTYPE html><table><tr><td><svg><foreignObject><p><i></p>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<svg svg>"),
            (7, "<svg foreignObject>"),
            (8, "<p>"),
            (9, "<i>"),
            (8, "<i>"),
            (9, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!DOCTYPE html><math><mtext><p><i></p>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mtext>"),
            (4, "<p>"),
            (5, "<i>"),
            (4, "<i>"),
            (5, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!DOCTYPE html><table><tr><td><math><mtext><p><i></p>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<math math>"),
            (7, "<math mtext>"),
            (8, "<p>"),
            (9, "<i>"),
            (8, "<i>"),
            (9, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!DOCTYPE html><body><div><!/div>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<!-- /div -->"),
            (3, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<button><p><button>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<button>"),
            (3, "<p>"),
            (2, "<button>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<svg></p><foo>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (2, "<p>"),
            (2, "<foo>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<svg></br><foo>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (2, "<br>"),
            (2, "<foo>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<math></p><foo>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (2, "<p>"),
            (2, "<foo>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<math></br><foo>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (2, "<br>"),
            (2, "<foo>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
