//<coverage:exclude>
use super::helper::parse;
use super::helper::Test;

#[test]
fn test_0() {
    parse(Test {
        data: r#"FOO&gt;BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO>BAR""#),
        ],
    });
}

#[test]
fn test_1() {
    parse(Test {
        data: r#"FOO&gtBAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO>BAR""#),
        ],
    });
}

#[test]
fn test_2() {
    parse(Test {
        data: r#"FOO&gt BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO> BAR""#),
        ],
    });
}

#[test]
fn test_3() {
    parse(Test {
        data: r#"FOO&gt;;;BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO>;;BAR""#),
        ],
    });
}

#[test]
fn test_4() {
    parse(Test {
        data: r#"I'm &notit; I tell you"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""I'm ¬it; I tell you""#),
        ],
    });
}

#[test]
fn test_5() {
    parse(Test {
        data: r#"I'm &notin; I tell you"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""I'm ∉ I tell you""#),
        ],
    });
}

#[test]
fn test_6() {
    parse(Test {
        data: r#"&ammmp;"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""&ammmp;""#),
        ],
    });
}

#[test]
fn test_7() {
    parse(Test {
        data: r#"&ammmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmp;"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (
                2,
                r#""&ammmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmp;""#,
            ),
        ],
    });
}

#[test]
fn test_8() {
    parse(Test {
        data: r#"FOO& BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO& BAR""#),
        ],
    });
}

#[test]
fn test_9() {
    parse(Test {
        data: r#"FOO&<BAR>"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO&""#),
            (2, r#"<bar>"#),
        ],
    });
}

#[test]
fn test_10() {
    parse(Test {
        data: r#"FOO&&&&gt;BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO&&&>BAR""#),
        ],
    });
}

#[test]
fn test_11() {
    parse(Test {
        data: r#"FOO&#41;BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO)BAR""#),
        ],
    });
}

#[test]
fn test_12() {
    parse(Test {
        data: r#"FOO&#x41;BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOABAR""#),
        ],
    });
}

#[test]
fn test_13() {
    parse(Test {
        data: r#"FOO&#X41;BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOABAR""#),
        ],
    });
}

#[test]
fn test_14() {
    parse(Test {
        data: r#"FOO&#BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO&#BAR""#),
        ],
    });
}

#[test]
fn test_15() {
    parse(Test {
        data: r#"FOO&#ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO&#ZOO""#),
        ],
    });
}

#[test]
fn test_16() {
    parse(Test {
        data: r#"FOO&#xBAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOºR""#),
        ],
    });
}

#[test]
fn test_17() {
    parse(Test {
        data: r#"FOO&#xZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO&#xZOO""#),
        ],
    });
}

#[test]
fn test_18() {
    parse(Test {
        data: r#"FOO&#XZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO&#XZOO""#),
        ],
    });
}

#[test]
fn test_19() {
    parse(Test {
        data: r#"FOO&#41BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO)BAR""#),
        ],
    });
}

#[test]
fn test_20() {
    parse(Test {
        data: r#"FOO&#x41BAR"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO䆺R""#),
        ],
    });
}

#[test]
fn test_21() {
    parse(Test {
        data: r#"FOO&#x41ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOAZOO""#),
        ],
    });
}

#[test]
fn test_22() {
    parse(Test {
        data: r#"FOO&#x0000;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_23() {
    parse(Test {
        data: r#"FOO&#x0078;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOxZOO""#),
        ],
    });
}

#[test]
fn test_24() {
    parse(Test {
        data: r#"FOO&#x0079;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOyZOO""#),
        ],
    });
}

#[test]
fn test_25() {
    parse(Test {
        data: r#"FOO&#x0080;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO€ZOO""#),
        ],
    });
}

#[test]
fn test_26() {
    parse(Test {
        data: r#"FOO&#x0081;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOZOO""#),
        ],
    });
}

#[test]
fn test_27() {
    parse(Test {
        data: r#"FOO&#x0082;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO‚ZOO""#),
        ],
    });
}

#[test]
fn test_28() {
    parse(Test {
        data: r#"FOO&#x0083;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOƒZOO""#),
        ],
    });
}

#[test]
fn test_29() {
    parse(Test {
        data: r#"FOO&#x0084;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO„ZOO""#),
        ],
    });
}

#[test]
fn test_30() {
    parse(Test {
        data: r#"FOO&#x0085;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO…ZOO""#),
        ],
    });
}

#[test]
fn test_31() {
    parse(Test {
        data: r#"FOO&#x0086;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO†ZOO""#),
        ],
    });
}

#[test]
fn test_32() {
    parse(Test {
        data: r#"FOO&#x0087;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO‡ZOO""#),
        ],
    });
}

#[test]
fn test_33() {
    parse(Test {
        data: r#"FOO&#x0088;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOˆZOO""#),
        ],
    });
}

#[test]
fn test_34() {
    parse(Test {
        data: r#"FOO&#x0089;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO‰ZOO""#),
        ],
    });
}

#[test]
fn test_35() {
    parse(Test {
        data: r#"FOO&#x008A;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOŠZOO""#),
        ],
    });
}

#[test]
fn test_36() {
    parse(Test {
        data: r#"FOO&#x008B;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO‹ZOO""#),
        ],
    });
}

#[test]
fn test_37() {
    parse(Test {
        data: r#"FOO&#x008C;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOŒZOO""#),
        ],
    });
}

#[test]
fn test_38() {
    parse(Test {
        data: r#"FOO&#x008D;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOZOO""#),
        ],
    });
}

#[test]
fn test_39() {
    parse(Test {
        data: r#"FOO&#x008E;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOŽZOO""#),
        ],
    });
}

#[test]
fn test_40() {
    parse(Test {
        data: r#"FOO&#x008F;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOZOO""#),
        ],
    });
}

#[test]
fn test_41() {
    parse(Test {
        data: r#"FOO&#x0090;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOZOO""#),
        ],
    });
}

#[test]
fn test_42() {
    parse(Test {
        data: r#"FOO&#x0091;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO‘ZOO""#),
        ],
    });
}

#[test]
fn test_43() {
    parse(Test {
        data: r#"FOO&#x0092;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO’ZOO""#),
        ],
    });
}

#[test]
fn test_44() {
    parse(Test {
        data: r#"FOO&#x0093;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO“ZOO""#),
        ],
    });
}

#[test]
fn test_45() {
    parse(Test {
        data: r#"FOO&#x0094;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO”ZOO""#),
        ],
    });
}

#[test]
fn test_46() {
    parse(Test {
        data: r#"FOO&#x0095;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO•ZOO""#),
        ],
    });
}

#[test]
fn test_47() {
    parse(Test {
        data: r#"FOO&#x0096;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO–ZOO""#),
        ],
    });
}

#[test]
fn test_48() {
    parse(Test {
        data: r#"FOO&#x0097;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO—ZOO""#),
        ],
    });
}

#[test]
fn test_49() {
    parse(Test {
        data: r#"FOO&#x0098;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO˜ZOO""#),
        ],
    });
}

#[test]
fn test_50() {
    parse(Test {
        data: r#"FOO&#x0099;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO™ZOO""#),
        ],
    });
}

#[test]
fn test_51() {
    parse(Test {
        data: r#"FOO&#x009A;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOšZOO""#),
        ],
    });
}

#[test]
fn test_52() {
    parse(Test {
        data: r#"FOO&#x009B;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO›ZOO""#),
        ],
    });
}

#[test]
fn test_53() {
    parse(Test {
        data: r#"FOO&#x009C;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOœZOO""#),
        ],
    });
}

#[test]
fn test_54() {
    parse(Test {
        data: r#"FOO&#x009D;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOZOO""#),
        ],
    });
}

#[test]
fn test_55() {
    parse(Test {
        data: r#"FOO&#x009E;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOžZOO""#),
        ],
    });
}

#[test]
fn test_56() {
    parse(Test {
        data: r#"FOO&#x009F;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOŸZOO""#),
        ],
    });
}

#[test]
fn test_57() {
    parse(Test {
        data: r#"FOO&#x00A0;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO ZOO""#),
        ],
    });
}

#[test]
fn test_58() {
    parse(Test {
        data: r#"FOO&#xD7FF;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO퟿ZOO""#),
        ],
    });
}

#[test]
fn test_59() {
    parse(Test {
        data: r#"FOO&#xD800;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_60() {
    parse(Test {
        data: r#"FOO&#xD801;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_61() {
    parse(Test {
        data: r#"FOO&#xDFFE;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_62() {
    parse(Test {
        data: r#"FOO&#xDFFF;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_63() {
    parse(Test {
        data: r#"FOO&#xE000;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOOZOO""#),
        ],
    });
}

#[test]
fn test_64() {
    parse(Test {
        data: r#"FOO&#x10FFFE;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO􏿾ZOO""#),
        ],
    });
}

#[test]
fn test_65() {
    parse(Test {
        data: r#"FOO&#x1087D4;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO􈟔ZOO""#),
        ],
    });
}

#[test]
fn test_66() {
    parse(Test {
        data: r#"FOO&#x10FFFF;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO􏿿ZOO""#),
        ],
    });
}

#[test]
fn test_67() {
    parse(Test {
        data: r#"FOO&#x110000;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_68() {
    parse(Test {
        data: r#"FOO&#xFFFFFF;ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_69() {
    parse(Test {
        data: r#"FOO&#11111111111"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�""#),
        ],
    });
}

#[test]
fn test_70() {
    parse(Test {
        data: r#"FOO&#1111111111"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�""#),
        ],
    });
}

#[test]
fn test_71() {
    parse(Test {
        data: r#"FOO&#111111111111"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�""#),
        ],
    });
}

#[test]
fn test_72() {
    parse(Test {
        data: r#"FOO&#11111111111ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_73() {
    parse(Test {
        data: r#"FOO&#1111111111ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}

#[test]
fn test_74() {
    parse(Test {
        data: r#"FOO&#111111111111ZOO"#,
        document: vec![
            (0, r#"<html>"#),
            (1, r#"<head>"#),
            (1, r#"<body>"#),
            (2, r#""FOO�ZOO""#),
        ],
    });
}
//</coverage:exclude>
