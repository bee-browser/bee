//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<body><template>Hello</template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<template>Hello</template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "\"Hello\""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<template></template><div></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (1, "<body>"),
            (2, "<div>"),
        ],
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<html><template>Hello</template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "\"Hello\""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<head><template><div></div></template></head>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<div><template><div><span></template><b>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<div>"),
            (6, "<span>"),
            (3, "<b>"),
        ],
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<div><template></div>Hello",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<template>"),
            (4, "content"),
            (5, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<div></template></div>",
        document: vec![(0, "<html>"), (1, "<head>"), (1, "<body>"), (2, "<div>")],
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<table><template></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
        ],
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<table><template></template></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
        ],
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<table><div><template></template></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<template>"),
            (4, "content"),
            (2, "<table>"),
        ],
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<table><template></template><div></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
        ],
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<table>   <template></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "\"   \""),
            (3, "<template>"),
            (4, "content"),
        ],
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<table><tbody><template></template></tbody>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<template>"),
            (5, "content"),
        ],
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<table><tbody><template></tbody></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<template>"),
            (5, "content"),
        ],
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<table><tbody><template></template></tbody></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<template>"),
            (5, "content"),
        ],
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<table><thead><template></template></thead>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<thead>"),
            (4, "<template>"),
            (5, "content"),
        ],
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<table><tfoot><template></template></tfoot>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tfoot>"),
            (4, "<template>"),
            (5, "content"),
        ],
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<select><template></template></select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<template>"),
            (4, "content"),
        ],
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<select><template><option></option></template></select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<option>"),
        ],
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<template><option></option></select><option></option></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<option>"),
            (4, "<option>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<select><template></template><option></select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<template>"),
            (4, "content"),
            (3, "<option>"),
        ],
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<select><option><template></template></select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (4, "<template>"),
            (5, "content"),
        ],
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<select><template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<template>"),
            (4, "content"),
        ],
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<select><option></option><template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (3, "<template>"),
            (4, "content"),
        ],
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<select><option></option><template><option>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<select>"),
            (3, "<option>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<option>"),
        ],
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<table><thead><template><td></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<thead>"),
            (4, "<template>"),
            (5, "content"),
            (6, "<td>"),
        ],
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<table><template><thead></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<thead>"),
        ],
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<body><table><template><td></tr><div></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<td>"),
            (6, "<div>"),
        ],
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<table><template><thead></template></thead></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<thead>"),
        ],
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<table><thead><template><tr></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<thead>"),
            (4, "<template>"),
            (5, "content"),
            (6, "<tr>"),
        ],
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<table><template><tr></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<tr>"),
        ],
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<table><tr><template><td>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<template>"),
            (6, "content"),
            (7, "<td>"),
        ],
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<table><template><tr><template><td></template></tr></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<tr>"),
            (6, "<template>"),
            (7, "content"),
            (8, "<td>"),
        ],
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<table><template><tr><template><td></td></template></tr></template></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<tr>"),
            (6, "<template>"),
            (7, "content"),
            (8, "<td>"),
        ],
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<table><template><td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<td>"),
        ],
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<body><template><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<body><template><template><tr></tr></template><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<tr>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<table><colgroup><template><col>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
            (4, "<template>"),
            (5, "content"),
            (6, "<col>"),
        ],
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<frameset><template><frame></frame></template></frameset>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<frameset>"),
            (2, "<frame>"),
        ],
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<template><frame></frame></frameset><frame></frame></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<template><div><frameset><span></span></div><span></span></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
            (5, "<span>"),
            (4, "<span>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<body><template><div><frameset><span></span></div><span></span></template></body>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
            (5, "<span>"),
            (4, "<span>"),
        ],
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<body><template><script>var i = 1;</script><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<script>"),
            (5, "\"var i = 1;\""),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "<body><template><tr><div></div></tr></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (4, "<div>"),
        ],
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<body><template><tr></tr><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (4, "<tr>"),
            (5, "<td>"),
        ],
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<body><template><td></td></tr><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<td>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<body><template><td></td><tbody><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<td>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<body><template><td></td><caption></caption><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<td>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<body><template><td></td><colgroup></caption><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<td>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "<body><template><td></td></table><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<td>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<body><template><tr></tr><tbody><tr></tr></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (4, "<tr>"),
        ],
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<body><template><tr></tr><caption><tr></tr></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (4, "<tr>"),
        ],
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<body><template><tr></tr></table><tr></tr></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (4, "<tr>"),
        ],
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<body><template><thead></thead><caption></caption><tbody></tbody></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<thead>"),
            (4, "<caption>"),
            (4, "<tbody>"),
        ],
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<body><template><thead></thead></table><tbody></tbody></template></body>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<thead>"),
            (4, "<tbody>"),
        ],
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<body><template><div><tr></tr></div></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
        ],
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<body><template><em>Hello</em></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<em>"),
            (5, "\"Hello\""),
        ],
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<body><template><!--comment--></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<!-- comment -->"),
        ],
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "<body><template><style></style><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<style>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<body><template><meta><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<meta>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "<body><template><link><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<link>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0062() {
    parse(Test {
        data: "<body><template><template><tr></tr></template><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<tr>"),
            (4, "<td>"),
        ],
    });
}

#[test]
fn test_0063() {
    parse(Test {
        data: "<body><table><colgroup><template><col></col></template></colgroup></table></body>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<colgroup>"),
            (4, "<template>"),
            (5, "content"),
            (6, "<col>"),
        ],
    });
}

#[test]
fn test_0064() {
    parse(Test {
        data: "<body a=b><template><div></div><body c=d><div></div></body></template></body>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "a=\"b\""),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
            (4, "<div>"),
        ],
    });
}

#[test]
fn test_0065() {
    parse(Test {
        data: "<html a=b><template><div><html b=c><span></template>",
        document: vec![
            (0, "<html>"),
            (1, "a=\"b\""),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
            (5, "<span>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0066() {
    parse(Test {
        data: "<html a=b><template><col></col><html b=c><col></col></template>",
        document: vec![
            (0, "<html>"),
            (1, "a=\"b\""),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<col>"),
            (4, "<col>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0067() {
    parse(Test {
        data: "<html a=b><template><frame></frame><html b=c><frame></frame></template>",
        document: vec![
            (0, "<html>"),
            (1, "a=\"b\""),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0068() {
    parse(Test {
        data: "<body><template><tr></tr><template></template><td></td></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (4, "<template>"),
            (5, "content"),
            (4, "<tr>"),
            (5, "<td>"),
        ],
    });
}

#[test]
fn test_0069() {
    parse(Test {
        data: "<body><template><thead></thead><template><tr></tr></template><tr></tr><tfoot></tfoot></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<thead>"),
            (4, "<template>"),
            (5, "content"),
            (6, "<tr>"),
            (4, "<tbody>"),
            (5, "<tr>"),
            (4, "<tfoot>"),
        ],
    });
}

#[test]
fn test_0070() {
    parse(Test {
        data: "<body><template><template><b><template></template></template>text</template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<b>"),
            (7, "<template>"),
            (8, "content"),
            (4, "\"text\""),
        ],
    });
}

#[test]
fn test_0071() {
    parse(Test {
        data: "<body><template><col><colgroup>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<col>"),
        ],
    });
}

#[test]
fn test_0072() {
    parse(Test {
        data: "<body><template><col></colgroup>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<col>"),
        ],
    });
}

#[test]
fn test_0073() {
    parse(Test {
        data: "<body><template><col><colgroup></template></body>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<col>"),
        ],
    });
}

#[test]
fn test_0074() {
    parse(Test {
        data: "<body><template><col><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<col>"),
        ],
    });
}

#[test]
fn test_0075() {
    parse(Test {
        data: "<body><template><col></div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<col>"),
        ],
    });
}

#[test]
fn test_0076() {
    parse(Test {
        data: "<body><template><col>Hello",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<col>"),
        ],
    });
}

#[test]
fn test_0077() {
    parse(Test {
        data: "<body><template><i><menu>Foo</i>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<i>"),
            (4, "<menu>"),
            (5, "<i>"),
            (6, "\"Foo\""),
        ],
    });
}

#[test]
fn test_0078() {
    parse(Test {
        data: "<body><template></div><div>Foo</div><template></template><tr></tr>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
            (5, "\"Foo\""),
            (4, "<template>"),
            (5, "content"),
        ],
    });
}

#[test]
fn test_0079() {
    parse(Test {
        data: "<body><div><template></div><tr><td>Foo</td></tr></template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<div>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<tr>"),
            (6, "<td>"),
            (7, "\"Foo\""),
        ],
    });
}

#[test]
fn test_0080() {
    parse(Test {
        data: "<template></figcaption><sub><table></table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<sub>"),
            (5, "<table>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0081() {
    parse(Test {
        data: "<template><template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0082() {
    parse(Test {
        data: "<template><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<div>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0083() {
    parse(Test {
        data: "<template><template><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<div>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0084() {
    parse(Test {
        data: "<template><template><table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<table>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0085() {
    parse(Test {
        data: "<template><template><tbody>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<tbody>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0086() {
    parse(Test {
        data: "<template><template><tr>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<tr>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0087() {
    parse(Test {
        data: "<template><template><td>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<td>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0088() {
    parse(Test {
        data: "<template><template><caption>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<caption>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0089() {
    parse(Test {
        data: "<template><template><colgroup>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<colgroup>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0090() {
    parse(Test {
        data: "<template><template><col>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<col>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0091() {
    parse(Test {
        data: "<template><template><tbody><select>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<tbody>"),
            (6, "<select>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0092() {
    parse(Test {
        data: "<template><template><table>Foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "\"Foo\""),
            (6, "<table>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0093() {
    parse(Test {
        data: "<template><template><frame>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0094() {
    parse(Test {
        data: "<template><template><script>var i",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<script>"),
            (7, "\"var i\""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0095() {
    parse(Test {
        data: "<template><template><style>var i",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<template>"),
            (5, "content"),
            (6, "<style>"),
            (7, "\"var i\""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0096() {
    parse(Test {
        data: "<template><table></template><body><span>Foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<table>"),
            (1, "<body>"),
            (2, "<span>"),
            (3, "\"Foo\""),
        ],
    });
}

#[test]
fn test_0097() {
    parse(Test {
        data: "<template><td></template><body><span>Foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<td>"),
            (1, "<body>"),
            (2, "<span>"),
            (3, "\"Foo\""),
        ],
    });
}

#[test]
fn test_0098() {
    parse(Test {
        data: "<template><object></template><body><span>Foo",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<object>"),
            (1, "<body>"),
            (2, "<span>"),
            (3, "\"Foo\""),
        ],
    });
}

#[test]
fn test_0099() {
    parse(Test {
        data: "<template><svg><template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<svg svg>"),
            (5, "<svg template>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0100() {
    parse(Test {
        data: "<template><svg><foo><template><foreignObject><div></template><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<svg svg>"),
            (5, "<svg foo>"),
            (6, "<svg template>"),
            (7, "<svg foreignObject>"),
            (8, "<div>"),
            (1, "<body>"),
            (2, "<div>"),
        ],
    });
}

#[test]
fn test_0101() {
    parse(Test {
        data: "<dummy><template><span></dummy>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dummy>"),
            (3, "<template>"),
            (4, "content"),
            (5, "<span>"),
        ],
    });
}

#[test]
fn test_0102() {
    parse(Test {
        data: "<body><table><tr><td><select><template>Foo</template><caption>A</table>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<tbody>"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "<select>"),
            (7, "<template>"),
            (8, "content"),
            (9, "\"Foo\""),
            (3, "<caption>"),
            (4, "\"A\""),
        ],
    });
}

#[test]
fn test_0103() {
    parse(Test {
        data: "<body></body><template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<template>"),
            (3, "content"),
        ],
    });
}

#[test]
fn test_0104() {
    parse(Test {
        data: "<head></head><template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0105() {
    parse(Test {
        data: "<head></head><template>Foo</template>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "\"Foo\""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0106() {
    parse(Test {
        data: "<!DOCTYPE HTML><dummy><table><template><table><template><table><script>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<dummy>"),
            (3, "<table>"),
            (4, "<template>"),
            (5, "content"),
            (6, "<table>"),
            (7, "<template>"),
            (8, "content"),
            (9, "<table>"),
            (10, "<script>"),
        ],
    });
}

#[test]
fn test_0107() {
    parse(Test {
        data: "<template><a><table><a>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<a>"),
            (5, "<a>"),
            (5, "<table>"),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0108() {
    parse(Test {
        data: "<template><form><input name=\"q\"></form><div>second</div></template>",
        document: vec![
            (0, "<template>"),
            (1, "content"),
            (2, "<form>"),
            (3, "<input>"),
            (4, "name=\"q\""),
            (2, "<div>"),
            (3, "\"second\""),
        ],
    });
}

#[test]
fn test_0109() {
    parse(Test {
        data: "<!DOCTYPE HTML><template><tr><td>cell</td></tr></template>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"cell\""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0110() {
    parse(Test {
        data: "<!DOCTYPE HTML><template> <tr> <td>cell</td> </tr> </template>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "\" \""),
            (4, "<tr>"),
            (5, "\" \""),
            (5, "<td>"),
            (6, "\"cell\""),
            (5, "\" \""),
            (4, "\" \""),
            (1, "<body>"),
        ],
    });
}

#[test]
fn test_0111() {
    parse(Test {
        data: "<!DOCTYPE HTML><template><tr><td>cell</td></tr>a</template>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (2, "<template>"),
            (3, "content"),
            (4, "<tr>"),
            (5, "<td>"),
            (6, "\"cell\""),
            (4, "\"a\""),
            (1, "<body>"),
        ],
    });
}
//</coverage:exclude>
