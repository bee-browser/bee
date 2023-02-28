//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "FOO&#x000D;ZOO",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\rZOO\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<html>\0<frameset></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<html> \0 <frameset></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<html>a\0a<frameset></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "\"aa\"")],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<html>\0\0<frameset></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<html>\0\n<frameset></frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<html><select>\0",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<select>")],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "\0",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<body>\0",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>")],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<plaintext>\0filler\0text\0",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"�filler�text�\""),
        ],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<svg><![CDATA[\0filler\0text\0]]>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"�filler�text�\""),
        ],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<body><!\0>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<!-- � -->"),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<body><!\0filler\0text>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<!-- �filler�text -->"),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<body><svg><foreignObject>\0filler\0text",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg foreignObject>"),
            (4, "\"fillertext\""),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<svg>\0filler\0text",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"�filler�text\""),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<svg>\0<frameset>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"�\""),
            (3, "<svg frameset>"),
        ],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<svg>\0 <frameset>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"� \""),
            (3, "<svg frameset>"),
        ],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<svg>\0a<frameset>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"�a\""),
            (3, "<svg frameset>"),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<svg>\0</svg><frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<svg>\0 </svg><frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<svg>\0a</svg><frameset>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "\"�a\""),
        ],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<svg><path></path></svg><frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<svg><p><frameset>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<frameset>")],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!DOCTYPE html><pre>\r\n\r\nA</pre>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"\nA\""),
        ],
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!DOCTYPE html><pre>\r\rA</pre>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"\nA\""),
        ],
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!DOCTYPE html><pre>\rA</pre>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<pre>"),
            (3, "\"A\""),
        ],
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!DOCTYPE html><table><tr><td><math><mtext>\0a",
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
            (8, "\"a\""),
        ],
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!DOCTYPE html><table><tr><td><svg><foreignObject>\0a",
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
            (8, "\"a\""),
        ],
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!DOCTYPE html><math><mi>a\0b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mi>"),
            (4, "\"ab\""),
        ],
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!DOCTYPE html><math><mo>a\0b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mo>"),
            (4, "\"ab\""),
        ],
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<!DOCTYPE html><math><mn>a\0b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mn>"),
            (4, "\"ab\""),
        ],
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<!DOCTYPE html><math><ms>a\0b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math ms>"),
            (4, "\"ab\""),
        ],
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!DOCTYPE html><math><mtext>a\0b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math mtext>"),
            (4, "\"ab\""),
        ],
    });
}
//</coverage:exclude>