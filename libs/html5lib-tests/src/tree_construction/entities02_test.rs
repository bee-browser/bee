//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<div bar=\"ZZ&gt;YY\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ>YY\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<div bar=\"ZZ&\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&\""),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<div bar=\'ZZ&\'></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&\""),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<div bar=ZZ&></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&\""),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<div bar=\"ZZ&gt=YY\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&gt=YY\""),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<div bar=\"ZZ&gt0YY\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&gt0YY\""),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<div bar=\"ZZ&gt9YY\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&gt9YY\""),
        ],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<div bar=\"ZZ&gtaYY\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&gtaYY\""),
        ],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<div bar=\"ZZ&gtZYY\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&gtZYY\""),
        ],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<div bar=\"ZZ&gt YY\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ> YY\""),
        ],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<div bar=\"ZZ&gt\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ>\""),
        ],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<div bar=\'ZZ&gt\'></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ>\""),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<div bar=ZZ&gt></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ>\""),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<div bar=\"ZZ&pound_id=23\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ£_id=23\""),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<div bar=\"ZZ&prod_id=23\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&prod_id=23\""),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<div bar=\"ZZ&pound;_id=23\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ£_id=23\""),
        ],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<div bar=\"ZZ&prod;_id=23\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ∏_id=23\""),
        ],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<div bar=\"ZZ&pound=23\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&pound=23\""),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<div bar=\"ZZ&prod=23\"></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "bar=\"ZZ&prod=23\""),
        ],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<div>ZZ&pound_id=23</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"ZZ£_id=23\""),
        ],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<div>ZZ&prod_id=23</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"ZZ&prod_id=23\""),
        ],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<div>ZZ&pound;_id=23</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"ZZ£_id=23\""),
        ],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<div>ZZ&prod;_id=23</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"ZZ∏_id=23\""),
        ],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<div>ZZ&pound=23</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"ZZ£=23\""),
        ],
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<div>ZZ&prod=23</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"ZZ&prod=23\""),
        ],
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<div>ZZ&AElig=</div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "\"ZZÆ=\""),
        ],
    });
}
//</coverage:exclude>