//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

#[test]
fn test_0000() {
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
fn test_0001() {
    parse(Test {
        data: "<!doctype html><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
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
fn test_0002() {
    parse(Test {
        data: "<!doctype html><html><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
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
fn test_0003() {
    parse(Test {
        data: "<!doctype html><head><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
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
fn test_0004() {
    parse(Test {
        data: "<!doctype html><html><noscript><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"</plaintext>\""),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!doctype html></head><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
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
fn test_0006() {
    parse(Test {
        data: "<!doctype html><body><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
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
fn test_0007() {
    parse(Test {
        data: "<!doctype html><table><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"</plaintext>\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!doctype html><table><tbody><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"</plaintext>\""),
            (2, "<table>"),
            (3, "<tbody>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!doctype html><table><tbody><tr><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"</plaintext>\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!doctype html><table><td><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<plaintext>"),
            (7, "\"</plaintext>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!doctype html><table><caption><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "<plaintext>"),
            (5, "\"</plaintext>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!doctype html><table><colgroup><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<plaintext>"),
            (3, "\"</plaintext>\""),
            (2, "<table>"),
            (3, "<colgroup>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!doctype html><select><plaintext></plaintext>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!doctype html><table><select><plaintext>a<caption>b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "\"a\""),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "\"b\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<!doctype html><template><plaintext>a</template>b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<plaintext>"),
            (5, "\"a</template>b\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!doctype html><body></body><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
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
fn test_0017() {
    parse(Test {
        data: "<!doctype html><frameset><plaintext></plaintext>",
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
fn test_0018() {
    parse(Test {
        data: "<!doctype html><frameset></frameset><plaintext></plaintext>",
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
fn test_0019() {
    parse(Test {
        data: "<!doctype html><body></body></html><plaintext></plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
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
fn test_0020() {
    parse(Test {
        data: "<!doctype html><frameset></frameset></html><plaintext></plaintext>",
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
fn test_0021() {
    parse(Test {
        data: "<!doctype html><svg><plaintext>a</plaintext>b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg plaintext>"),
            (4, "\"a\""),
            (3, "\"b\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!doctype html><svg><title><plaintext>a</plaintext>b",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg title>"),
            (4, "<plaintext>"),
            (5, "\"a</plaintext>b\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!doctype html><table><tr><style></script></style>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"abc\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<style>"),
            (6, "\"</script>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!doctype html><table><tr><script></style></script>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"abc\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<script>"),
            (6, "\"</style>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!doctype html><table><caption><style></script></style>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "<style>"),
            (5, "\"</script>\""),
            (4, "\"abc\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!doctype html><table><td><style></script></style>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<style>"),
            (7, "\"</script>\""),
            (6, "\"abc\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!doctype html><select><script></style></script>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<script>"),
            (4, "\"</style>\""),
            (3, "\"abc\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!doctype html><table><select><script></style></script>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<script>"),
            (4, "\"</style>\""),
            (3, "\"abc\""),
            (2, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!doctype html><table><tr><select><script></style></script>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<script>"),
            (4, "\"</style>\""),
            (3, "\"abc\""),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<!doctype html><frameset></frameset><noframes>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (1, "<noframes>"),
            (2, "\"abc\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<!doctype html><frameset></frameset><noframes>abc</noframes><!--abc-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (1, "<noframes>"),
            (2, "\"abc\""),
            (1, "<!-- abc -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!doctype html><frameset></frameset></html><noframes>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (1, "<noframes>"),
            (2, "\"abc\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!doctype html><frameset></frameset></html><noframes>abc</noframes><!--abc-->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (1, "<noframes>"),
            (2, "\"abc\""),
            (0, "<!-- abc -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!doctype html><table><tr></tbody><tfoot>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (3, "<tfoot>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<!doctype html><table><td><svg></svg>abc<td>",
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
            (6, "\"abc\""),
            (5, "<td>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
