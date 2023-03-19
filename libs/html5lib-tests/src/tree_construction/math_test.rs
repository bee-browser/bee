//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<math><tr><td><mo><tr>",
        document: vec![
            (0, "<math math>"),
            (1, "<math tr>"),
            (2, "<math td>"),
            (3, "<math mo>"),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<math><tr><td><mo><tr>",
        document: vec![
            (0, "<math math>"),
            (1, "<math tr>"),
            (2, "<math td>"),
            (3, "<math mo>"),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<math><thead><mo><tbody>",
        document: vec![(0, "<math math>"), (1, "<math thead>"), (2, "<math mo>")],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<math><tfoot><mo><tbody>",
        document: vec![(0, "<math math>"), (1, "<math tfoot>"), (2, "<math mo>")],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<math><tbody><mo><tfoot>",
        document: vec![(0, "<math math>"), (1, "<math tbody>"), (2, "<math mo>")],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<math><tbody><mo></table>",
        document: vec![(0, "<math math>"), (1, "<math tbody>"), (2, "<math mo>")],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<math><thead><mo></table>",
        document: vec![(0, "<math math>"), (1, "<math thead>"), (2, "<math mo>")],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<math><tfoot><mo></table>",
        document: vec![(0, "<math math>"), (1, "<math tfoot>"), (2, "<math mo>")],
    });
}
//</coverage:exclude>