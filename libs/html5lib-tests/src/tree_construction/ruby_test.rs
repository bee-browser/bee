//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<html><ruby>a<rb>b<rb></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rb>"),
            (4, "\"b\""),
            (3, "<rb>"),
        ],
    });
}

#[test]
fn test_0001() {
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
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<html><ruby>a<rb>b<rtc></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rb>"),
            (4, "\"b\""),
            (3, "<rtc>"),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<html><ruby>a<rb>b<rp></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rb>"),
            (4, "\"b\""),
            (3, "<rp>"),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<html><ruby>a<rb>b<span></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rb>"),
            (4, "\"b\""),
            (4, "<span>"),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<html><ruby>a<rt>b<rb></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rt>"),
            (4, "\"b\""),
            (3, "<rb>"),
        ],
    });
}

#[test]
fn test_0006() {
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
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<html><ruby>a<rt>b<rtc></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rt>"),
            (4, "\"b\""),
            (3, "<rtc>"),
        ],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<html><ruby>a<rt>b<rp></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rt>"),
            (4, "\"b\""),
            (3, "<rp>"),
        ],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<html><ruby>a<rt>b<span></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rt>"),
            (4, "\"b\""),
            (4, "<span>"),
        ],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<html><ruby>a<rtc>b<rb></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rtc>"),
            (4, "\"b\""),
            (3, "<rb>"),
        ],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<html><ruby>a<rtc>b<rt>c<rt>d</ruby></html>",
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
            (4, "<rt>"),
            (5, "\"d\""),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<html><ruby>a<rtc>b<rtc></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rtc>"),
            (4, "\"b\""),
            (3, "<rtc>"),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<html><ruby>a<rtc>b<rp></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rtc>"),
            (4, "\"b\""),
            (4, "<rp>"),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<html><ruby>a<rtc>b<span></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rtc>"),
            (4, "\"b\""),
            (4, "<span>"),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<html><ruby>a<rp>b<rb></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rp>"),
            (4, "\"b\""),
            (3, "<rb>"),
        ],
    });
}

#[test]
fn test_0016() {
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
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<html><ruby>a<rp>b<rtc></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rp>"),
            (4, "\"b\""),
            (3, "<rtc>"),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<html><ruby>a<rp>b<rp></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rp>"),
            (4, "\"b\""),
            (3, "<rp>"),
        ],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<html><ruby>a<rp>b<span></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "\"a\""),
            (3, "<rp>"),
            (4, "\"b\""),
            (4, "<span>"),
        ],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<html><ruby><rtc><ruby>a<rb>b<rt></ruby></ruby></html>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<ruby>"),
            (3, "<rtc>"),
            (4, "<ruby>"),
            (5, "\"a\""),
            (5, "<rb>"),
            (6, "\"b\""),
            (5, "<rt>"),
        ],
    });
}
//</coverage:exclude>
