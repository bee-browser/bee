//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "FOO<script>\'Hello\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'Hello\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "FOO<script></script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "FOO<script></script >BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "FOO<script></script/>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "FOO<script></script/ >BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "FOO<script type=\"text/plain\"></scriptx>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"</scriptx>BAR\""),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "FOO<script></script foo=\">\" dd>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "FOO<script>\'<\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "FOO<script>\'<!\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "FOO<script>\'<!-\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!-\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "FOO<script>\'<!--\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!--\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "FOO<script>\'<!---\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!---\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "FOO<script>\'<!-->\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!-->\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "FOO<script>\'<!-->\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!-->\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "FOO<script>\'<!-- potato\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!-- potato\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "FOO<script>\'<!-- <sCrIpt\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!-- <sCrIpt\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt>\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt>\'</script>BAR\""),
        ],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt> -\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt> -\'</script>BAR\""),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt> --\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt> --\'</script>BAR\""),
        ],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "FOO<script>\'<!-- <sCrIpt> -->\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"\'<!-- <sCrIpt> -->\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt> --!>\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt> --!>\'</script>BAR\""),
        ],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt> -- >\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt> -- >\'</script>BAR\""),
        ],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt \'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt \'</script>BAR\""),
        ],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt/\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt/\'</script>BAR\""),
        ],
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt\\\'</script>BAR",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt\\\'\""),
            (2, "\"BAR\""),
        ],
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "FOO<script type=\"text/plain\">\'<!-- <sCrIpt/\'</script>BAR</script>QUX",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "type=\"text/plain\""),
            (3, "\"\'<!-- <sCrIpt/\'</script>BAR\""),
            (2, "\"QUX\""),
        ],
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "FOO<script><!--<script>-></script>--></script>QUX",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"FOO\""),
            (2, "<script>"),
            (3, "\"<!--<script>-></script>-->\""),
            (2, "\"QUX\""),
        ],
    });
}
//</coverage:exclude>
