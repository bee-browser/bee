//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!doctype html><table><tbody><select><tr>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!doctype html><table><tr><select><td>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!doctype html><table><tr><td><select><td>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<select>"),
            (5, "<td>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!doctype html><table><tr><th><select><td>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<th>"),
            (6, "<select>"),
            (5, "<td>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!doctype html><table><caption><select><tr>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<caption>"),
            (4, "<select>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!doctype html><select><tr>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!doctype html><select><td>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!doctype html><select><th>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!doctype html><select><tbody>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!doctype html><select><thead>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!doctype html><select><tfoot>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!doctype html><select><caption>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
        ],
        context_element: None,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!doctype html><table><tr></table>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (2, "\"a\""),
        ],
        context_element: None,
    });
}
//</coverage:exclude>
