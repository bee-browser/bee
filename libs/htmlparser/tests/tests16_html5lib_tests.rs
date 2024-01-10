//<coverage:exclude>
mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

logging::init!();

#[test]
fn test_0000() {
    parse(Test {
        data: "<!doctype html><script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!doctype html><script>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!doctype html><script><",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!doctype html><script></",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!doctype html><script></S",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</S\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!doctype html><script></SC",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SC\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!doctype html><script></SCR",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCR\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!doctype html><script></SCRI",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCRI\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!doctype html><script></SCRIP",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCRIP\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!doctype html><script></SCRIPT",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCRIPT\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!doctype html><script></SCRIPT ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!doctype html><script></s",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</s\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!doctype html><script></sc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</sc\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!doctype html><script></scr",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</scr\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!doctype html><script></scri",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</scri\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<!doctype html><script></scrip",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</scrip\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!doctype html><script></script",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!doctype html><script></script ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!doctype html><script><!",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!doctype html><script><!a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!doctype html><script><!-",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!-\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!doctype html><script><!-a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!-a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!doctype html><script><!--",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!doctype html><script><!--a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!doctype html><script><!--<",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!doctype html><script><!--<a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!doctype html><script><!--</",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--</\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!doctype html><script><!--</script",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--</script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!doctype html><script><!--</script ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!doctype html><script><!--<s",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<s\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<!doctype html><script><!--<script",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<!doctype html><script><!--<script ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!doctype html><script><!--<script <",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script <\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!doctype html><script><!--<script <a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script <a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!doctype html><script><!--<script </",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<!doctype html><script><!--<script </s",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </s\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<!doctype html><script><!--<script </scripta",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </scripta\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script/",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script/\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script <",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script <\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script <a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script <a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script </",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script </\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script </script",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script </script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script </script ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script </script/",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<!doctype html><script><!--<script </script </script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<!doctype html><script><!--<script -",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<!doctype html><script><!--<script -a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "<!doctype html><script><!--<script -<",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -<\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<!doctype html><script><!--<script --",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<!doctype html><script><!--<script --a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<!doctype html><script><!--<script --<",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --<\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<!doctype html><script><!--<script -->",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<!doctype html><script><!--<script --><",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --><\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<!doctype html><script><!--<script --></",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --></\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<!doctype html><script><!--<script --></script",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --></script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<!doctype html><script><!--<script --></script ",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "<!doctype html><script><!--<script --></script/",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<!doctype html><script><!--<script --></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "<!doctype html><script><!--<script><\\/script>--></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script><\\/script>-->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0062() {
    parse(Test {
        data: "<!doctype html><script><!--<script></scr\'+\'ipt>--></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></scr\'+\'ipt>-->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0063() {
    parse(Test {
        data: "<!doctype html><script><!--<script></script><script></script></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0064() {
    parse(Test {
        data: "<!doctype html><script><!--<script></script><script></script>--><!--</script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>--><!--\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0065() {
    parse(Test {
        data: "<!doctype html><script><!--<script></script><script></script>-- ></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>-- >\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0066() {
    parse(Test {
        data: "<!doctype html><script><!--<script></script><script></script>- -></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>- ->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0067() {
    parse(Test {
        data: "<!doctype html><script><!--<script></script><script></script>- - ></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>- - >\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0068() {
    parse(Test {
        data: "<!doctype html><script><!--<script></script><script></script>-></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0069() {
    parse(Test {
        data: "<!doctype html><script><!--<script>--!></script>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script>--!></script>X\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0070() {
    parse(Test {
        data: "<!doctype html><script><!--<scr\'+\'ipt></script>--></script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<scr\'+\'ipt>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0071() {
    parse(Test {
        data: "<!doctype html><script><!--<script></scr\'+\'ipt></script>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></scr\'+\'ipt></script>X\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0072() {
    parse(Test {
        data: "<!doctype html><style><!--<style></style>--></style>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--<style>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0073() {
    parse(Test {
        data: "<!doctype html><style><!--</style>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0074() {
    parse(Test {
        data: "<!doctype html><style><!--...</style>...--></style>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--...\""),
            (1, "<body>"),
            (2, "\"...-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0075() {
    parse(Test {
        data: "<!doctype html><style><!--<br><html xmlns:v=\"urn:schemas-microsoft-com:vml\"><!--[if !mso]><style></style>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--<br><html xmlns:v=\"urn:schemas-microsoft-com:vml\"><!--[if !mso]><style>\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0076() {
    parse(Test {
        data: "<!doctype html><style><!--...<style><!--...--!></style>--></style>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--...<style><!--...--!>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0077() {
    parse(Test {
        data: "<!doctype html><style><!--...</style><!-- --><style>@import ...</style>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--...\""),
            (2, "<!--   -->"),
            (2, "<style>"),
            (3, "\"@import ...\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0078() {
    parse(Test {
        data: "<!doctype html><style>...<style><!--...</style><!-- --></style>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"...<style><!--...\""),
            (2, "<!--   -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0079() {
    parse(Test {
        data: "<!doctype html><style>...<!--[if IE]><style>...</style>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"...<!--[if IE]><style>...\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0080() {
    parse(Test {
        data: "<!doctype html><title><!--<title></title>--></title>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"<!--<title>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0081() {
    parse(Test {
        data: "<!doctype html><title>&lt;/title></title>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"</title>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0082() {
    parse(Test {
        data: "<!doctype html><title>foo/title><link></head><body>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"foo/title><link></head><body>X\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0083() {
    parse(Test {
        data: "<!doctype html><noscript><!--<noscript></noscript>--></noscript>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"<!--<noscript>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0084() {
    parse(Test {
        data: "<!doctype html><noscript><!--<noscript></noscript>--></noscript>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- <noscript></noscript> -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0085() {
    parse(Test {
        data: "<!doctype html><noscript><!--</noscript>X<noscript>--></noscript>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"<!--\""),
            (1, "<body>"),
            (2, "\"X\""),
            (2, "<noscript>"),
            (3, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0086() {
    parse(Test {
        data: "<!doctype html><noscript><!--</noscript>X<noscript>--></noscript>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- </noscript>X<noscript> -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0087() {
    parse(Test {
        data: "<!doctype html><noscript><iframe></noscript>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"<iframe>\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0088() {
    parse(Test {
        data: "<!doctype html><noscript><iframe></noscript>X",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\"</noscript>X\""),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0089() {
    parse(Test {
        data: "<!doctype html><noframes><!--<noframes></noframes>--></noframes>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noframes>"),
            (3, "\"<!--<noframes>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0090() {
    parse(Test {
        data: "<!doctype html><noframes><body><script><!--...</script></body></noframes></html>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noframes>"),
            (3, "\"<body><script><!--...</script></body>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0091() {
    parse(Test {
        data: "<!doctype html><textarea><!--<textarea></textarea>--></textarea>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"<!--<textarea>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0092() {
    parse(Test {
        data: "<!doctype html><textarea>&lt;/textarea></textarea>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"</textarea>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0093() {
    parse(Test {
        data: "<!doctype html><textarea>&lt;</textarea>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"<\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0094() {
    parse(Test {
        data: "<!doctype html><textarea>a&lt;b</textarea>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"a<b\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0095() {
    parse(Test {
        data: "<!doctype html><iframe><!--<iframe></iframe>--></iframe>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\"<!--<iframe>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0096() {
    parse(Test {
        data: "<!doctype html><iframe>...<!--X->...<!--/X->...</iframe>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\"...<!--X->...<!--/X->...\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0097() {
    parse(Test {
        data: "<!doctype html><xmp><!--<xmp></xmp>--></xmp>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<xmp>"),
            (3, "\"<!--<xmp>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0098() {
    parse(Test {
        data: "<!doctype html><noembed><!--<noembed></noembed>--></noembed>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<noembed>"),
            (3, "\"<!--<noembed>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0099() {
    parse(Test {
        data: "<script>",
        document: vec![(0, "<html>"), (1, "<head>"), (2, "<script>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0100() {
    parse(Test {
        data: "<script>a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0101() {
    parse(Test {
        data: "<script><",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0102() {
    parse(Test {
        data: "<script></",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0103() {
    parse(Test {
        data: "<script></S",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</S\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0104() {
    parse(Test {
        data: "<script></SC",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SC\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0105() {
    parse(Test {
        data: "<script></SCR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCR\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0106() {
    parse(Test {
        data: "<script></SCRI",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCRI\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0107() {
    parse(Test {
        data: "<script></SCRIP",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCRIP\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0108() {
    parse(Test {
        data: "<script></SCRIPT",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</SCRIPT\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0109() {
    parse(Test {
        data: "<script></SCRIPT ",
        document: vec![(0, "<html>"), (1, "<head>"), (2, "<script>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0110() {
    parse(Test {
        data: "<script></s",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</s\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0111() {
    parse(Test {
        data: "<script></sc",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</sc\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0112() {
    parse(Test {
        data: "<script></scr",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</scr\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0113() {
    parse(Test {
        data: "<script></scri",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</scri\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0114() {
    parse(Test {
        data: "<script></scrip",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</scrip\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0115() {
    parse(Test {
        data: "<script></script",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"</script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0116() {
    parse(Test {
        data: "<script></script ",
        document: vec![(0, "<html>"), (1, "<head>"), (2, "<script>"), (1, "<body>")],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0117() {
    parse(Test {
        data: "<script><!",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0118() {
    parse(Test {
        data: "<script><!a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0119() {
    parse(Test {
        data: "<script><!-",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!-\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0120() {
    parse(Test {
        data: "<script><!-a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!-a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0121() {
    parse(Test {
        data: "<script><!--",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0122() {
    parse(Test {
        data: "<script><!--a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0123() {
    parse(Test {
        data: "<script><!--<",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0124() {
    parse(Test {
        data: "<script><!--<a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0125() {
    parse(Test {
        data: "<script><!--</",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--</\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0126() {
    parse(Test {
        data: "<script><!--</script",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--</script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0127() {
    parse(Test {
        data: "<script><!--</script ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0128() {
    parse(Test {
        data: "<script><!--<s",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<s\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0129() {
    parse(Test {
        data: "<script><!--<script",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0130() {
    parse(Test {
        data: "<script><!--<script ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0131() {
    parse(Test {
        data: "<script><!--<script <",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script <\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0132() {
    parse(Test {
        data: "<script><!--<script <a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script <a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0133() {
    parse(Test {
        data: "<script><!--<script </",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0134() {
    parse(Test {
        data: "<script><!--<script </s",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </s\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0135() {
    parse(Test {
        data: "<script><!--<script </script",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0136() {
    parse(Test {
        data: "<script><!--<script </scripta",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </scripta\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0137() {
    parse(Test {
        data: "<script><!--<script </script ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0138() {
    parse(Test {
        data: "<script><!--<script </script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0139() {
    parse(Test {
        data: "<script><!--<script </script/",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script/\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0140() {
    parse(Test {
        data: "<script><!--<script </script <",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script <\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0141() {
    parse(Test {
        data: "<script><!--<script </script <a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script <a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0142() {
    parse(Test {
        data: "<script><!--<script </script </",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script </\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0143() {
    parse(Test {
        data: "<script><!--<script </script </script",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script </script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0144() {
    parse(Test {
        data: "<script><!--<script </script </script ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0145() {
    parse(Test {
        data: "<script><!--<script </script </script/",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0146() {
    parse(Test {
        data: "<script><!--<script </script </script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script </script \""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0147() {
    parse(Test {
        data: "<script><!--<script -",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0148() {
    parse(Test {
        data: "<script><!--<script -a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0149() {
    parse(Test {
        data: "<script><!--<script --",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0150() {
    parse(Test {
        data: "<script><!--<script --a",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --a\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0151() {
    parse(Test {
        data: "<script><!--<script -->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0152() {
    parse(Test {
        data: "<script><!--<script --><",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --><\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0153() {
    parse(Test {
        data: "<script><!--<script --></",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --></\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0154() {
    parse(Test {
        data: "<script><!--<script --></script",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script --></script\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0155() {
    parse(Test {
        data: "<script><!--<script --></script ",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0156() {
    parse(Test {
        data: "<script><!--<script --></script/",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0157() {
    parse(Test {
        data: "<script><!--<script --></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script -->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0158() {
    parse(Test {
        data: "<script><!--<script><\\/script>--></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script><\\/script>-->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0159() {
    parse(Test {
        data: "<script><!--<script></scr\'+\'ipt>--></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></scr\'+\'ipt>-->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0160() {
    parse(Test {
        data: "<script><!--<script></script><script></script></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0161() {
    parse(Test {
        data: "<script><!--<script></script><script></script>--><!--</script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>--><!--\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0162() {
    parse(Test {
        data: "<script><!--<script></script><script></script>-- ></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>-- >\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0163() {
    parse(Test {
        data: "<script><!--<script></script><script></script>- -></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>- ->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0164() {
    parse(Test {
        data: "<script><!--<script></script><script></script>- - ></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>- - >\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0165() {
    parse(Test {
        data: "<script><!--<script></script><script></script>-></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></script><script></script>->\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0166() {
    parse(Test {
        data: "<script><!--<script>--!></script>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script>--!></script>X\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0167() {
    parse(Test {
        data: "<script><!--<scr\'+\'ipt></script>--></script>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<scr\'+\'ipt>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0168() {
    parse(Test {
        data: "<script><!--<script></scr\'+\'ipt></script>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<script>"),
            (3, "\"<!--<script></scr\'+\'ipt></script>X\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0169() {
    parse(Test {
        data: "<style><!--<style></style>--></style>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--<style>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0170() {
    parse(Test {
        data: "<style><!--</style>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0171() {
    parse(Test {
        data: "<style><!--...</style>...--></style>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--...\""),
            (1, "<body>"),
            (2, "\"...-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0172() {
    parse(Test {
        data: "<style><!--<br><html xmlns:v=\"urn:schemas-microsoft-com:vml\"><!--[if !mso]><style></style>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--<br><html xmlns:v=\"urn:schemas-microsoft-com:vml\"><!--[if !mso]><style>\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0173() {
    parse(Test {
        data: "<style><!--...<style><!--...--!></style>--></style>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--...<style><!--...--!>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0174() {
    parse(Test {
        data: "<style><!--...</style><!-- --><style>@import ...</style>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"<!--...\""),
            (2, "<!--   -->"),
            (2, "<style>"),
            (3, "\"@import ...\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0175() {
    parse(Test {
        data: "<style>...<style><!--...</style><!-- --></style>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"...<style><!--...\""),
            (2, "<!--   -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0176() {
    parse(Test {
        data: "<style>...<!--[if IE]><style>...</style>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<style>"),
            (3, "\"...<!--[if IE]><style>...\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0177() {
    parse(Test {
        data: "<title><!--<title></title>--></title>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"<!--<title>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0178() {
    parse(Test {
        data: "<title>&lt;/title></title>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"</title>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0179() {
    parse(Test {
        data: "<title>foo/title><link></head><body>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<title>"),
            (3, "\"foo/title><link></head><body>X\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0180() {
    parse(Test {
        data: "<noscript><!--<noscript></noscript>--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"<!--<noscript>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0181() {
    parse(Test {
        data: "<noscript><!--<noscript></noscript>--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- <noscript></noscript> -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0182() {
    parse(Test {
        data: "<noscript><!--</noscript>X<noscript>--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"<!--\""),
            (1, "<body>"),
            (2, "\"X\""),
            (2, "<noscript>"),
            (3, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0183() {
    parse(Test {
        data: "<noscript><!--</noscript>X<noscript>--></noscript>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "<!-- </noscript>X<noscript> -->"),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0184() {
    parse(Test {
        data: "<noscript><iframe></noscript>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (3, "\"<iframe>\""),
            (1, "<body>"),
            (2, "\"X\""),
        ],
        context_element: None,
        scripting: Scripting::On,
    });
}

#[test]
fn test_0185() {
    parse(Test {
        data: "<noscript><iframe></noscript>X",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noscript>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\"</noscript>X\""),
        ],
        context_element: None,
        scripting: Scripting::Off,
    });
}

#[test]
fn test_0186() {
    parse(Test {
        data: "<noframes><!--<noframes></noframes>--></noframes>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noframes>"),
            (3, "\"<!--<noframes>\""),
            (1, "<body>"),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0187() {
    parse(Test {
        data: "<noframes><body><script><!--...</script></body></noframes></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<noframes>"),
            (3, "\"<body><script><!--...</script></body>\""),
            (1, "<body>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0188() {
    parse(Test {
        data: "<textarea><!--<textarea></textarea>--></textarea>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"<!--<textarea>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0189() {
    parse(Test {
        data: "<textarea>&lt;/textarea></textarea>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<textarea>"),
            (3, "\"</textarea>\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0190() {
    parse(Test {
        data: "<iframe><!--<iframe></iframe>--></iframe>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\"<!--<iframe>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0191() {
    parse(Test {
        data: "<iframe>...<!--X->...<!--/X->...</iframe>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<iframe>"),
            (3, "\"...<!--X->...<!--/X->...\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0192() {
    parse(Test {
        data: "<xmp><!--<xmp></xmp>--></xmp>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<xmp>"),
            (3, "\"<!--<xmp>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0193() {
    parse(Test {
        data: "<noembed><!--<noembed></noembed>--></noembed>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<noembed>"),
            (3, "\"<!--<noembed>\""),
            (2, "\"-->\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0194() {
    parse(Test {
        data: "<!doctype html><table>\n",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "\"\n\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0195() {
    parse(Test {
        data: "<!doctype html><table><td><span><font></span><span>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<span>"),
            (7, "<font>"),
            (6, "<font>"),
            (7, "<span>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0196() {
    parse(Test {
        data: "<!doctype html><form><table></form><form></table></form>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<form>"),
            (3, "<table>"),
            (4, "<form>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
