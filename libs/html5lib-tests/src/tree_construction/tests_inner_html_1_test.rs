//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<body><span>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<span><body>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<span><body>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<body><span>",
        document: vec![(0, "<head>"), (0, "<body>"), (1, "<span>")],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<frameset><span>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<span><frameset>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<span><frameset>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<frameset><span>",
        document: vec![(0, "<head>"), (0, "<frameset>")],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<table><tr>",
        document: vec![(0, "<tbody>"), (1, "<tr>")],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "</table><tr>",
        document: vec![(0, "<tbody>"), (1, "<tr>")],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<a><caption>a",
        document: vec![(0, "<a>"), (0, "<caption>"), (1, "\"a\"")],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<a><colgroup><col>",
        document: vec![(0, "<a>"), (0, "<colgroup>"), (1, "<col>")],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<a><tbody><tr>",
        document: vec![(0, "<a>"), (0, "<tbody>"), (1, "<tr>")],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<a><tfoot><tr>",
        document: vec![(0, "<a>"), (0, "<tfoot>"), (1, "<tr>")],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<a><thead><tr>",
        document: vec![(0, "<a>"), (0, "<thead>"), (1, "<tr>")],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<a><tr>",
        document: vec![(0, "<a>"), (0, "<tbody>"), (1, "<tr>")],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<a><th>",
        document: vec![(0, "<a>"), (0, "<tbody>"), (1, "<tr>"), (2, "<th>")],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<a><td>",
        document: vec![(0, "<a>"), (0, "<tbody>"), (1, "<tr>"), (2, "<td>")],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<table></table><tbody>",
        document: vec![(0, "<table>")],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "</table><span>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<span></table>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "</caption><span>",
        document: vec![(0, "<span>")],
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<span></caption><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<span><caption><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<span><col><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<span><colgroup><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<span><html><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<span><tbody><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<span><td><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<span><tfoot><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<span><thead><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<span><th><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<span><tr><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<span></table><span>",
        document: vec![(0, "<span>"), (1, "<span>")],
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "</colgroup><col>",
        document: vec![(0, "<col>")],
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<a><col>",
        document: vec![(0, "<col>")],
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<caption><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<col><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<colgroup><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<tbody><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<tfoot><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<thead><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "</table><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<a><tr>",
        document: vec![(0, "<a>"), (0, "<tr>")],
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<a><td>",
        document: vec![(0, "<a>"), (0, "<tr>"), (1, "<td>")],
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<a><td>",
        document: vec![(0, "<a>"), (0, "<tr>"), (1, "<td>")],
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<a><td>",
        document: vec![(0, "<a>"), (0, "<tr>"), (1, "<td>")],
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<td><table><tbody><a><tr>",
        document: vec![
            (0, "<tr>"),
            (1, "<td>"),
            (2, "<a>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
        ],
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "</tr><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<td><table><a><tr></tr><tr>",
        document: vec![
            (0, "<td>"),
            (1, "<a>"),
            (1, "<table>"),
            (2, "<tbody>"),
            (3, "<tr>"),
            (3, "<tr>"),
        ],
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<caption><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<col><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<colgroup><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<tbody><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<tfoot><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<thead><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<tr><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "</table><td>",
        document: vec![(0, "<td>")],
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<td><table></table><td>",
        document: vec![(0, "<td>"), (1, "<table>"), (0, "<td>")],
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "<td><table></table><td>",
        document: vec![(0, "<td>"), (1, "<table>"), (0, "<td>")],
    });
}

#[test]
fn test_0062() {
    parse(Test {
        data: "<caption><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0063() {
    parse(Test {
        data: "<col><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0064() {
    parse(Test {
        data: "<colgroup><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0065() {
    parse(Test {
        data: "<tbody><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0066() {
    parse(Test {
        data: "<tfoot><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0067() {
    parse(Test {
        data: "<th><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0068() {
    parse(Test {
        data: "<thead><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0069() {
    parse(Test {
        data: "<tr><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0070() {
    parse(Test {
        data: "</table><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0071() {
    parse(Test {
        data: "</tbody><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0072() {
    parse(Test {
        data: "</td><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0073() {
    parse(Test {
        data: "</tfoot><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0074() {
    parse(Test {
        data: "</thead><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0075() {
    parse(Test {
        data: "</th><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0076() {
    parse(Test {
        data: "</tr><a>",
        document: vec![(0, "<a>")],
    });
}

#[test]
fn test_0077() {
    parse(Test {
        data: "<table><td><td>",
        document: vec![
            (0, "<table>"),
            (1, "<tbody>"),
            (2, "<tr>"),
            (3, "<td>"),
            (3, "<td>"),
        ],
    });
}

#[test]
fn test_0078() {
    parse(Test {
        data: "</select><option>",
        document: vec![(0, "<option>")],
    });
}

#[test]
fn test_0079() {
    parse(Test {
        data: "<input><option>",
        document: vec![(0, "<option>")],
    });
}

#[test]
fn test_0080() {
    parse(Test {
        data: "<keygen><option>",
        document: vec![(0, "<option>")],
    });
}

#[test]
fn test_0081() {
    parse(Test {
        data: "<textarea><option>",
        document: vec![(0, "<option>")],
    });
}

#[test]
fn test_0082() {
    parse(Test {
        data: "</html><!--abc-->",
        document: vec![(0, "<head>"), (0, "<body>"), (0, "<!-- abc -->")],
    });
}

#[test]
fn test_0083() {
    parse(Test {
        data: "</frameset><frame>",
        document: vec![(0, "<frame>")],
    });
}
//</coverage:exclude>