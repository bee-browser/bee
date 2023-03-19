//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!DOCTYPE html>&NotEqualTilde;",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"≂̸\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!DOCTYPE html>&NotEqualTilde;A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"≂̸A\""),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!DOCTYPE html>&ThickSpace;",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"  \""),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!DOCTYPE html>&ThickSpace;A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"  A\""),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!DOCTYPE html>&NotSubset;",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"⊂⃒\""),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!DOCTYPE html>&NotSubset;A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"⊂⃒A\""),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!DOCTYPE html>&Gopf;",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"𝔾\""),
        ],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!DOCTYPE html>&Gopf;A",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "\"𝔾A\""),
        ],
    });
}
//</coverage:exclude>
