//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"Invalid unterminated numeric entity character overflow before EOF","initialState":"Data","input":"&#11111111111","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":14}},{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"Invalid unterminated numeric entity character overflow before EOF","initialState":"Data","input":"&#1111111111","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":13}},{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"Invalid unterminated numeric entity character overflow before EOF","initialState":"Data","input":"&#111111111111","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,49],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":15}},{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"Invalid unterminated numeric entity character overflow","initialState":"Data","input":"&#11111111111x","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,120],"output":[{"Character":{"data":"�x"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":14}},{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"Invalid unterminated numeric entity character overflow","initialState":"Data","input":"&#1111111111x","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,120],"output":[{"Character":{"data":"�x"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":13}},{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_5() {
    tokenize(
        r##"{"description":"Invalid unterminated numeric entity character overflow","initialState":"Data","input":"&#111111111111x","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,49,120],"output":[{"Character":{"data":"�x"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":15}},{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_6() {
    tokenize(
        r##"{"description":"Invalid numeric entity character overflow","initialState":"Data","input":"&#11111111111;","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_7() {
    tokenize(
        r##"{"description":"Invalid numeric entity character overflow","initialState":"Data","input":"&#1111111111;","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_8() {
    tokenize(
        r##"{"description":"Invalid numeric entity character overflow","initialState":"Data","input":"&#111111111111;","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_9() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0000","initialState":"Data","input":"&#x0000;","inputUtf16":[38,35,120,48,48,48,48,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"NullCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_10() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0001","initialState":"Data","input":"&#x0001;","inputUtf16":[38,35,120,48,48,48,49,59],"output":[{"Character":{"data":"\u0001"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_11() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0002","initialState":"Data","input":"&#x0002;","inputUtf16":[38,35,120,48,48,48,50,59],"output":[{"Character":{"data":"\u0002"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_12() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0003","initialState":"Data","input":"&#x0003;","inputUtf16":[38,35,120,48,48,48,51,59],"output":[{"Character":{"data":"\u0003"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_13() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0004","initialState":"Data","input":"&#x0004;","inputUtf16":[38,35,120,48,48,48,52,59],"output":[{"Character":{"data":"\u0004"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_14() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0005","initialState":"Data","input":"&#x0005;","inputUtf16":[38,35,120,48,48,48,53,59],"output":[{"Character":{"data":"\u0005"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_15() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0006","initialState":"Data","input":"&#x0006;","inputUtf16":[38,35,120,48,48,48,54,59],"output":[{"Character":{"data":"\u0006"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_16() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0007","initialState":"Data","input":"&#x0007;","inputUtf16":[38,35,120,48,48,48,55,59],"output":[{"Character":{"data":"\u0007"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_17() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0008","initialState":"Data","input":"&#x0008;","inputUtf16":[38,35,120,48,48,48,56,59],"output":[{"Character":{"data":"\b"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_18() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+000B","initialState":"Data","input":"&#x000b;","inputUtf16":[38,35,120,48,48,48,98,59],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_19() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+000E","initialState":"Data","input":"&#x000e;","inputUtf16":[38,35,120,48,48,48,101,59],"output":[{"Character":{"data":"\u000e"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_20() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+000F","initialState":"Data","input":"&#x000f;","inputUtf16":[38,35,120,48,48,48,102,59],"output":[{"Character":{"data":"\u000f"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_21() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0010","initialState":"Data","input":"&#x0010;","inputUtf16":[38,35,120,48,48,49,48,59],"output":[{"Character":{"data":"\u0010"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_22() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0011","initialState":"Data","input":"&#x0011;","inputUtf16":[38,35,120,48,48,49,49,59],"output":[{"Character":{"data":"\u0011"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_23() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0012","initialState":"Data","input":"&#x0012;","inputUtf16":[38,35,120,48,48,49,50,59],"output":[{"Character":{"data":"\u0012"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_24() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0013","initialState":"Data","input":"&#x0013;","inputUtf16":[38,35,120,48,48,49,51,59],"output":[{"Character":{"data":"\u0013"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_25() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0014","initialState":"Data","input":"&#x0014;","inputUtf16":[38,35,120,48,48,49,52,59],"output":[{"Character":{"data":"\u0014"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_26() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0015","initialState":"Data","input":"&#x0015;","inputUtf16":[38,35,120,48,48,49,53,59],"output":[{"Character":{"data":"\u0015"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_27() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0016","initialState":"Data","input":"&#x0016;","inputUtf16":[38,35,120,48,48,49,54,59],"output":[{"Character":{"data":"\u0016"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_28() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0017","initialState":"Data","input":"&#x0017;","inputUtf16":[38,35,120,48,48,49,55,59],"output":[{"Character":{"data":"\u0017"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_29() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0018","initialState":"Data","input":"&#x0018;","inputUtf16":[38,35,120,48,48,49,56,59],"output":[{"Character":{"data":"\u0018"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_30() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+0019","initialState":"Data","input":"&#x0019;","inputUtf16":[38,35,120,48,48,49,57,59],"output":[{"Character":{"data":"\u0019"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_31() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+001A","initialState":"Data","input":"&#x001a;","inputUtf16":[38,35,120,48,48,49,97,59],"output":[{"Character":{"data":"\u001a"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_32() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+001B","initialState":"Data","input":"&#x001b;","inputUtf16":[38,35,120,48,48,49,98,59],"output":[{"Character":{"data":"\u001b"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_33() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+001C","initialState":"Data","input":"&#x001c;","inputUtf16":[38,35,120,48,48,49,99,59],"output":[{"Character":{"data":"\u001c"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_34() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+001D","initialState":"Data","input":"&#x001d;","inputUtf16":[38,35,120,48,48,49,100,59],"output":[{"Character":{"data":"\u001d"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_35() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+001E","initialState":"Data","input":"&#x001e;","inputUtf16":[38,35,120,48,48,49,101,59],"output":[{"Character":{"data":"\u001e"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_36() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+001F","initialState":"Data","input":"&#x001f;","inputUtf16":[38,35,120,48,48,49,102,59],"output":[{"Character":{"data":"\u001f"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_37() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+007F","initialState":"Data","input":"&#x007f;","inputUtf16":[38,35,120,48,48,55,102,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_38() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+D800","initialState":"Data","input":"&#xd800;","inputUtf16":[38,35,120,100,56,48,48,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"SurrogateCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_39() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+DFFF","initialState":"Data","input":"&#xdfff;","inputUtf16":[38,35,120,100,102,102,102,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"SurrogateCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_40() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD0","initialState":"Data","input":"&#xfdd0;","inputUtf16":[38,35,120,102,100,100,48,59],"output":[{"Character":{"data":"﷐"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_41() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD1","initialState":"Data","input":"&#xfdd1;","inputUtf16":[38,35,120,102,100,100,49,59],"output":[{"Character":{"data":"﷑"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_42() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD2","initialState":"Data","input":"&#xfdd2;","inputUtf16":[38,35,120,102,100,100,50,59],"output":[{"Character":{"data":"﷒"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_43() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD3","initialState":"Data","input":"&#xfdd3;","inputUtf16":[38,35,120,102,100,100,51,59],"output":[{"Character":{"data":"﷓"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_44() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD4","initialState":"Data","input":"&#xfdd4;","inputUtf16":[38,35,120,102,100,100,52,59],"output":[{"Character":{"data":"﷔"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_45() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD5","initialState":"Data","input":"&#xfdd5;","inputUtf16":[38,35,120,102,100,100,53,59],"output":[{"Character":{"data":"﷕"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_46() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD6","initialState":"Data","input":"&#xfdd6;","inputUtf16":[38,35,120,102,100,100,54,59],"output":[{"Character":{"data":"﷖"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_47() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD7","initialState":"Data","input":"&#xfdd7;","inputUtf16":[38,35,120,102,100,100,55,59],"output":[{"Character":{"data":"﷗"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_48() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD8","initialState":"Data","input":"&#xfdd8;","inputUtf16":[38,35,120,102,100,100,56,59],"output":[{"Character":{"data":"﷘"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_49() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDD9","initialState":"Data","input":"&#xfdd9;","inputUtf16":[38,35,120,102,100,100,57,59],"output":[{"Character":{"data":"﷙"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_50() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDDA","initialState":"Data","input":"&#xfdda;","inputUtf16":[38,35,120,102,100,100,97,59],"output":[{"Character":{"data":"﷚"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_51() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDDB","initialState":"Data","input":"&#xfddb;","inputUtf16":[38,35,120,102,100,100,98,59],"output":[{"Character":{"data":"﷛"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_52() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDDC","initialState":"Data","input":"&#xfddc;","inputUtf16":[38,35,120,102,100,100,99,59],"output":[{"Character":{"data":"﷜"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_53() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDDD","initialState":"Data","input":"&#xfddd;","inputUtf16":[38,35,120,102,100,100,100,59],"output":[{"Character":{"data":"﷝"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_54() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDDE","initialState":"Data","input":"&#xfdde;","inputUtf16":[38,35,120,102,100,100,101,59],"output":[{"Character":{"data":"﷞"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_55() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDDF","initialState":"Data","input":"&#xfddf;","inputUtf16":[38,35,120,102,100,100,102,59],"output":[{"Character":{"data":"﷟"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_56() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE0","initialState":"Data","input":"&#xfde0;","inputUtf16":[38,35,120,102,100,101,48,59],"output":[{"Character":{"data":"﷠"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_57() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE1","initialState":"Data","input":"&#xfde1;","inputUtf16":[38,35,120,102,100,101,49,59],"output":[{"Character":{"data":"﷡"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_58() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE2","initialState":"Data","input":"&#xfde2;","inputUtf16":[38,35,120,102,100,101,50,59],"output":[{"Character":{"data":"﷢"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_59() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE3","initialState":"Data","input":"&#xfde3;","inputUtf16":[38,35,120,102,100,101,51,59],"output":[{"Character":{"data":"﷣"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_60() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE4","initialState":"Data","input":"&#xfde4;","inputUtf16":[38,35,120,102,100,101,52,59],"output":[{"Character":{"data":"﷤"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_61() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE5","initialState":"Data","input":"&#xfde5;","inputUtf16":[38,35,120,102,100,101,53,59],"output":[{"Character":{"data":"﷥"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_62() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE6","initialState":"Data","input":"&#xfde6;","inputUtf16":[38,35,120,102,100,101,54,59],"output":[{"Character":{"data":"﷦"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_63() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE7","initialState":"Data","input":"&#xfde7;","inputUtf16":[38,35,120,102,100,101,55,59],"output":[{"Character":{"data":"﷧"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_64() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE8","initialState":"Data","input":"&#xfde8;","inputUtf16":[38,35,120,102,100,101,56,59],"output":[{"Character":{"data":"﷨"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_65() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDE9","initialState":"Data","input":"&#xfde9;","inputUtf16":[38,35,120,102,100,101,57,59],"output":[{"Character":{"data":"﷩"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_66() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDEA","initialState":"Data","input":"&#xfdea;","inputUtf16":[38,35,120,102,100,101,97,59],"output":[{"Character":{"data":"﷪"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_67() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDEB","initialState":"Data","input":"&#xfdeb;","inputUtf16":[38,35,120,102,100,101,98,59],"output":[{"Character":{"data":"﷫"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_68() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDEC","initialState":"Data","input":"&#xfdec;","inputUtf16":[38,35,120,102,100,101,99,59],"output":[{"Character":{"data":"﷬"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_69() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDED","initialState":"Data","input":"&#xfded;","inputUtf16":[38,35,120,102,100,101,100,59],"output":[{"Character":{"data":"﷭"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_70() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDEE","initialState":"Data","input":"&#xfdee;","inputUtf16":[38,35,120,102,100,101,101,59],"output":[{"Character":{"data":"﷮"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_71() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FDEF","initialState":"Data","input":"&#xfdef;","inputUtf16":[38,35,120,102,100,101,102,59],"output":[{"Character":{"data":"﷯"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_72() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FFFE","initialState":"Data","input":"&#xfffe;","inputUtf16":[38,35,120,102,102,102,101,59],"output":[{"Character":{"data":"￾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_73() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FFFF","initialState":"Data","input":"&#xffff;","inputUtf16":[38,35,120,102,102,102,102,59],"output":[{"Character":{"data":"￿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_74() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+1FFFE","initialState":"Data","input":"&#x1fffe;","inputUtf16":[38,35,120,49,102,102,102,101,59],"output":[{"Character":{"data":"🿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_75() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+1FFFF","initialState":"Data","input":"&#x1ffff;","inputUtf16":[38,35,120,49,102,102,102,102,59],"output":[{"Character":{"data":"🿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_76() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+2FFFE","initialState":"Data","input":"&#x2fffe;","inputUtf16":[38,35,120,50,102,102,102,101,59],"output":[{"Character":{"data":"𯿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_77() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+2FFFF","initialState":"Data","input":"&#x2ffff;","inputUtf16":[38,35,120,50,102,102,102,102,59],"output":[{"Character":{"data":"𯿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_78() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+3FFFE","initialState":"Data","input":"&#x3fffe;","inputUtf16":[38,35,120,51,102,102,102,101,59],"output":[{"Character":{"data":"𿿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_79() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+3FFFF","initialState":"Data","input":"&#x3ffff;","inputUtf16":[38,35,120,51,102,102,102,102,59],"output":[{"Character":{"data":"𿿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_80() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+4FFFE","initialState":"Data","input":"&#x4fffe;","inputUtf16":[38,35,120,52,102,102,102,101,59],"output":[{"Character":{"data":"񏿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_81() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+4FFFF","initialState":"Data","input":"&#x4ffff;","inputUtf16":[38,35,120,52,102,102,102,102,59],"output":[{"Character":{"data":"񏿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_82() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+5FFFE","initialState":"Data","input":"&#x5fffe;","inputUtf16":[38,35,120,53,102,102,102,101,59],"output":[{"Character":{"data":"񟿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_83() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+5FFFF","initialState":"Data","input":"&#x5ffff;","inputUtf16":[38,35,120,53,102,102,102,102,59],"output":[{"Character":{"data":"񟿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_84() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+6FFFE","initialState":"Data","input":"&#x6fffe;","inputUtf16":[38,35,120,54,102,102,102,101,59],"output":[{"Character":{"data":"񯿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_85() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+6FFFF","initialState":"Data","input":"&#x6ffff;","inputUtf16":[38,35,120,54,102,102,102,102,59],"output":[{"Character":{"data":"񯿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_86() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+7FFFE","initialState":"Data","input":"&#x7fffe;","inputUtf16":[38,35,120,55,102,102,102,101,59],"output":[{"Character":{"data":"񿿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_87() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+7FFFF","initialState":"Data","input":"&#x7ffff;","inputUtf16":[38,35,120,55,102,102,102,102,59],"output":[{"Character":{"data":"񿿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_88() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+8FFFE","initialState":"Data","input":"&#x8fffe;","inputUtf16":[38,35,120,56,102,102,102,101,59],"output":[{"Character":{"data":"򏿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_89() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+8FFFF","initialState":"Data","input":"&#x8ffff;","inputUtf16":[38,35,120,56,102,102,102,102,59],"output":[{"Character":{"data":"򏿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_90() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+9FFFE","initialState":"Data","input":"&#x9fffe;","inputUtf16":[38,35,120,57,102,102,102,101,59],"output":[{"Character":{"data":"򟿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_91() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+9FFFF","initialState":"Data","input":"&#x9ffff;","inputUtf16":[38,35,120,57,102,102,102,102,59],"output":[{"Character":{"data":"򟿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_92() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+AFFFE","initialState":"Data","input":"&#xafffe;","inputUtf16":[38,35,120,97,102,102,102,101,59],"output":[{"Character":{"data":"򯿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_93() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+AFFFF","initialState":"Data","input":"&#xaffff;","inputUtf16":[38,35,120,97,102,102,102,102,59],"output":[{"Character":{"data":"򯿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_94() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+BFFFE","initialState":"Data","input":"&#xbfffe;","inputUtf16":[38,35,120,98,102,102,102,101,59],"output":[{"Character":{"data":"򿿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_95() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+BFFFF","initialState":"Data","input":"&#xbffff;","inputUtf16":[38,35,120,98,102,102,102,102,59],"output":[{"Character":{"data":"򿿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_96() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+CFFFE","initialState":"Data","input":"&#xcfffe;","inputUtf16":[38,35,120,99,102,102,102,101,59],"output":[{"Character":{"data":"󏿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_97() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+CFFFF","initialState":"Data","input":"&#xcffff;","inputUtf16":[38,35,120,99,102,102,102,102,59],"output":[{"Character":{"data":"󏿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_98() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+DFFFE","initialState":"Data","input":"&#xdfffe;","inputUtf16":[38,35,120,100,102,102,102,101,59],"output":[{"Character":{"data":"󟿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_99() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+DFFFF","initialState":"Data","input":"&#xdffff;","inputUtf16":[38,35,120,100,102,102,102,102,59],"output":[{"Character":{"data":"󟿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_100() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+EFFFE","initialState":"Data","input":"&#xefffe;","inputUtf16":[38,35,120,101,102,102,102,101,59],"output":[{"Character":{"data":"󯿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_101() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+EFFFF","initialState":"Data","input":"&#xeffff;","inputUtf16":[38,35,120,101,102,102,102,102,59],"output":[{"Character":{"data":"󯿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_102() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FFFFE","initialState":"Data","input":"&#xffffe;","inputUtf16":[38,35,120,102,102,102,102,101,59],"output":[{"Character":{"data":"󿿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_103() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+FFFFF","initialState":"Data","input":"&#xfffff;","inputUtf16":[38,35,120,102,102,102,102,102,59],"output":[{"Character":{"data":"󿿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_104() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+10FFFE","initialState":"Data","input":"&#x10fffe;","inputUtf16":[38,35,120,49,48,102,102,102,101,59],"output":[{"Character":{"data":"􏿾"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_105() {
    tokenize(
        r##"{"description":"Invalid numeric entity character U+10FFFF","initialState":"Data","input":"&#x10ffff;","inputUtf16":[38,35,120,49,48,102,102,102,102,59],"output":[{"Character":{"data":"􏿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_106() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0009","initialState":"Data","input":"&#x0009;","inputUtf16":[38,35,120,48,48,48,57,59],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_107() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+000A","initialState":"Data","input":"&#x000a;","inputUtf16":[38,35,120,48,48,48,97,59],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_108() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0020","initialState":"Data","input":"&#x0020;","inputUtf16":[38,35,120,48,48,50,48,59],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_109() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0021","initialState":"Data","input":"&#x0021;","inputUtf16":[38,35,120,48,48,50,49,59],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_110() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0022","initialState":"Data","input":"&#x0022;","inputUtf16":[38,35,120,48,48,50,50,59],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_111() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0023","initialState":"Data","input":"&#x0023;","inputUtf16":[38,35,120,48,48,50,51,59],"output":[{"Character":{"data":"#"}}],"errors":[]}"##,
    );
}

#[test]
fn test_112() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0024","initialState":"Data","input":"&#x0024;","inputUtf16":[38,35,120,48,48,50,52,59],"output":[{"Character":{"data":"$"}}],"errors":[]}"##,
    );
}

#[test]
fn test_113() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0025","initialState":"Data","input":"&#x0025;","inputUtf16":[38,35,120,48,48,50,53,59],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_114() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0026","initialState":"Data","input":"&#x0026;","inputUtf16":[38,35,120,48,48,50,54,59],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_115() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0027","initialState":"Data","input":"&#x0027;","inputUtf16":[38,35,120,48,48,50,55,59],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_116() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0028","initialState":"Data","input":"&#x0028;","inputUtf16":[38,35,120,48,48,50,56,59],"output":[{"Character":{"data":"("}}],"errors":[]}"##,
    );
}

#[test]
fn test_117() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0029","initialState":"Data","input":"&#x0029;","inputUtf16":[38,35,120,48,48,50,57,59],"output":[{"Character":{"data":")"}}],"errors":[]}"##,
    );
}

#[test]
fn test_118() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+002A","initialState":"Data","input":"&#x002a;","inputUtf16":[38,35,120,48,48,50,97,59],"output":[{"Character":{"data":"*"}}],"errors":[]}"##,
    );
}

#[test]
fn test_119() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+002B","initialState":"Data","input":"&#x002b;","inputUtf16":[38,35,120,48,48,50,98,59],"output":[{"Character":{"data":"+"}}],"errors":[]}"##,
    );
}

#[test]
fn test_120() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+002C","initialState":"Data","input":"&#x002c;","inputUtf16":[38,35,120,48,48,50,99,59],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_121() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+002D","initialState":"Data","input":"&#x002d;","inputUtf16":[38,35,120,48,48,50,100,59],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_122() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+002E","initialState":"Data","input":"&#x002e;","inputUtf16":[38,35,120,48,48,50,101,59],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_123() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+002F","initialState":"Data","input":"&#x002f;","inputUtf16":[38,35,120,48,48,50,102,59],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_124() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0030","initialState":"Data","input":"&#x0030;","inputUtf16":[38,35,120,48,48,51,48,59],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_125() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0031","initialState":"Data","input":"&#x0031;","inputUtf16":[38,35,120,48,48,51,49,59],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_126() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0032","initialState":"Data","input":"&#x0032;","inputUtf16":[38,35,120,48,48,51,50,59],"output":[{"Character":{"data":"2"}}],"errors":[]}"##,
    );
}

#[test]
fn test_127() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0033","initialState":"Data","input":"&#x0033;","inputUtf16":[38,35,120,48,48,51,51,59],"output":[{"Character":{"data":"3"}}],"errors":[]}"##,
    );
}

#[test]
fn test_128() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0034","initialState":"Data","input":"&#x0034;","inputUtf16":[38,35,120,48,48,51,52,59],"output":[{"Character":{"data":"4"}}],"errors":[]}"##,
    );
}

#[test]
fn test_129() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0035","initialState":"Data","input":"&#x0035;","inputUtf16":[38,35,120,48,48,51,53,59],"output":[{"Character":{"data":"5"}}],"errors":[]}"##,
    );
}

#[test]
fn test_130() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0036","initialState":"Data","input":"&#x0036;","inputUtf16":[38,35,120,48,48,51,54,59],"output":[{"Character":{"data":"6"}}],"errors":[]}"##,
    );
}

#[test]
fn test_131() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0037","initialState":"Data","input":"&#x0037;","inputUtf16":[38,35,120,48,48,51,55,59],"output":[{"Character":{"data":"7"}}],"errors":[]}"##,
    );
}

#[test]
fn test_132() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0038","initialState":"Data","input":"&#x0038;","inputUtf16":[38,35,120,48,48,51,56,59],"output":[{"Character":{"data":"8"}}],"errors":[]}"##,
    );
}

#[test]
fn test_133() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0039","initialState":"Data","input":"&#x0039;","inputUtf16":[38,35,120,48,48,51,57,59],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_134() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+003A","initialState":"Data","input":"&#x003a;","inputUtf16":[38,35,120,48,48,51,97,59],"output":[{"Character":{"data":":"}}],"errors":[]}"##,
    );
}

#[test]
fn test_135() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+003B","initialState":"Data","input":"&#x003b;","inputUtf16":[38,35,120,48,48,51,98,59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_136() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+003C","initialState":"Data","input":"&#x003c;","inputUtf16":[38,35,120,48,48,51,99,59],"output":[{"Character":{"data":"<"}}],"errors":[]}"##,
    );
}

#[test]
fn test_137() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+003D","initialState":"Data","input":"&#x003d;","inputUtf16":[38,35,120,48,48,51,100,59],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_138() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+003E","initialState":"Data","input":"&#x003e;","inputUtf16":[38,35,120,48,48,51,101,59],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_139() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+003F","initialState":"Data","input":"&#x003f;","inputUtf16":[38,35,120,48,48,51,102,59],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_140() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0040","initialState":"Data","input":"&#x0040;","inputUtf16":[38,35,120,48,48,52,48,59],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_141() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0041","initialState":"Data","input":"&#x0041;","inputUtf16":[38,35,120,48,48,52,49,59],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_142() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0042","initialState":"Data","input":"&#x0042;","inputUtf16":[38,35,120,48,48,52,50,59],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_143() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0043","initialState":"Data","input":"&#x0043;","inputUtf16":[38,35,120,48,48,52,51,59],"output":[{"Character":{"data":"C"}}],"errors":[]}"##,
    );
}

#[test]
fn test_144() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0044","initialState":"Data","input":"&#x0044;","inputUtf16":[38,35,120,48,48,52,52,59],"output":[{"Character":{"data":"D"}}],"errors":[]}"##,
    );
}

#[test]
fn test_145() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0045","initialState":"Data","input":"&#x0045;","inputUtf16":[38,35,120,48,48,52,53,59],"output":[{"Character":{"data":"E"}}],"errors":[]}"##,
    );
}

#[test]
fn test_146() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0046","initialState":"Data","input":"&#x0046;","inputUtf16":[38,35,120,48,48,52,54,59],"output":[{"Character":{"data":"F"}}],"errors":[]}"##,
    );
}

#[test]
fn test_147() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0047","initialState":"Data","input":"&#x0047;","inputUtf16":[38,35,120,48,48,52,55,59],"output":[{"Character":{"data":"G"}}],"errors":[]}"##,
    );
}

#[test]
fn test_148() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0048","initialState":"Data","input":"&#x0048;","inputUtf16":[38,35,120,48,48,52,56,59],"output":[{"Character":{"data":"H"}}],"errors":[]}"##,
    );
}

#[test]
fn test_149() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0049","initialState":"Data","input":"&#x0049;","inputUtf16":[38,35,120,48,48,52,57,59],"output":[{"Character":{"data":"I"}}],"errors":[]}"##,
    );
}

#[test]
fn test_150() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+004A","initialState":"Data","input":"&#x004a;","inputUtf16":[38,35,120,48,48,52,97,59],"output":[{"Character":{"data":"J"}}],"errors":[]}"##,
    );
}

#[test]
fn test_151() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+004B","initialState":"Data","input":"&#x004b;","inputUtf16":[38,35,120,48,48,52,98,59],"output":[{"Character":{"data":"K"}}],"errors":[]}"##,
    );
}

#[test]
fn test_152() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+004C","initialState":"Data","input":"&#x004c;","inputUtf16":[38,35,120,48,48,52,99,59],"output":[{"Character":{"data":"L"}}],"errors":[]}"##,
    );
}

#[test]
fn test_153() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+004D","initialState":"Data","input":"&#x004d;","inputUtf16":[38,35,120,48,48,52,100,59],"output":[{"Character":{"data":"M"}}],"errors":[]}"##,
    );
}

#[test]
fn test_154() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+004E","initialState":"Data","input":"&#x004e;","inputUtf16":[38,35,120,48,48,52,101,59],"output":[{"Character":{"data":"N"}}],"errors":[]}"##,
    );
}

#[test]
fn test_155() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+004F","initialState":"Data","input":"&#x004f;","inputUtf16":[38,35,120,48,48,52,102,59],"output":[{"Character":{"data":"O"}}],"errors":[]}"##,
    );
}

#[test]
fn test_156() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0050","initialState":"Data","input":"&#x0050;","inputUtf16":[38,35,120,48,48,53,48,59],"output":[{"Character":{"data":"P"}}],"errors":[]}"##,
    );
}

#[test]
fn test_157() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0051","initialState":"Data","input":"&#x0051;","inputUtf16":[38,35,120,48,48,53,49,59],"output":[{"Character":{"data":"Q"}}],"errors":[]}"##,
    );
}

#[test]
fn test_158() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0052","initialState":"Data","input":"&#x0052;","inputUtf16":[38,35,120,48,48,53,50,59],"output":[{"Character":{"data":"R"}}],"errors":[]}"##,
    );
}

#[test]
fn test_159() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0053","initialState":"Data","input":"&#x0053;","inputUtf16":[38,35,120,48,48,53,51,59],"output":[{"Character":{"data":"S"}}],"errors":[]}"##,
    );
}

#[test]
fn test_160() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0054","initialState":"Data","input":"&#x0054;","inputUtf16":[38,35,120,48,48,53,52,59],"output":[{"Character":{"data":"T"}}],"errors":[]}"##,
    );
}

#[test]
fn test_161() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0055","initialState":"Data","input":"&#x0055;","inputUtf16":[38,35,120,48,48,53,53,59],"output":[{"Character":{"data":"U"}}],"errors":[]}"##,
    );
}

#[test]
fn test_162() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0056","initialState":"Data","input":"&#x0056;","inputUtf16":[38,35,120,48,48,53,54,59],"output":[{"Character":{"data":"V"}}],"errors":[]}"##,
    );
}

#[test]
fn test_163() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0057","initialState":"Data","input":"&#x0057;","inputUtf16":[38,35,120,48,48,53,55,59],"output":[{"Character":{"data":"W"}}],"errors":[]}"##,
    );
}

#[test]
fn test_164() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0058","initialState":"Data","input":"&#x0058;","inputUtf16":[38,35,120,48,48,53,56,59],"output":[{"Character":{"data":"X"}}],"errors":[]}"##,
    );
}

#[test]
fn test_165() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0059","initialState":"Data","input":"&#x0059;","inputUtf16":[38,35,120,48,48,53,57,59],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_166() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+005A","initialState":"Data","input":"&#x005a;","inputUtf16":[38,35,120,48,48,53,97,59],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_167() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+005B","initialState":"Data","input":"&#x005b;","inputUtf16":[38,35,120,48,48,53,98,59],"output":[{"Character":{"data":"["}}],"errors":[]}"##,
    );
}

#[test]
fn test_168() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+005C","initialState":"Data","input":"&#x005c;","inputUtf16":[38,35,120,48,48,53,99,59],"output":[{"Character":{"data":"\\"}}],"errors":[]}"##,
    );
}

#[test]
fn test_169() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+005D","initialState":"Data","input":"&#x005d;","inputUtf16":[38,35,120,48,48,53,100,59],"output":[{"Character":{"data":"]"}}],"errors":[]}"##,
    );
}

#[test]
fn test_170() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+005E","initialState":"Data","input":"&#x005e;","inputUtf16":[38,35,120,48,48,53,101,59],"output":[{"Character":{"data":"^"}}],"errors":[]}"##,
    );
}

#[test]
fn test_171() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+005F","initialState":"Data","input":"&#x005f;","inputUtf16":[38,35,120,48,48,53,102,59],"output":[{"Character":{"data":"_"}}],"errors":[]}"##,
    );
}

#[test]
fn test_172() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0060","initialState":"Data","input":"&#x0060;","inputUtf16":[38,35,120,48,48,54,48,59],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_173() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0061","initialState":"Data","input":"&#x0061;","inputUtf16":[38,35,120,48,48,54,49,59],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_174() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0062","initialState":"Data","input":"&#x0062;","inputUtf16":[38,35,120,48,48,54,50,59],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_175() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0063","initialState":"Data","input":"&#x0063;","inputUtf16":[38,35,120,48,48,54,51,59],"output":[{"Character":{"data":"c"}}],"errors":[]}"##,
    );
}

#[test]
fn test_176() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0064","initialState":"Data","input":"&#x0064;","inputUtf16":[38,35,120,48,48,54,52,59],"output":[{"Character":{"data":"d"}}],"errors":[]}"##,
    );
}

#[test]
fn test_177() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0065","initialState":"Data","input":"&#x0065;","inputUtf16":[38,35,120,48,48,54,53,59],"output":[{"Character":{"data":"e"}}],"errors":[]}"##,
    );
}

#[test]
fn test_178() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0066","initialState":"Data","input":"&#x0066;","inputUtf16":[38,35,120,48,48,54,54,59],"output":[{"Character":{"data":"f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_179() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0067","initialState":"Data","input":"&#x0067;","inputUtf16":[38,35,120,48,48,54,55,59],"output":[{"Character":{"data":"g"}}],"errors":[]}"##,
    );
}

#[test]
fn test_180() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0068","initialState":"Data","input":"&#x0068;","inputUtf16":[38,35,120,48,48,54,56,59],"output":[{"Character":{"data":"h"}}],"errors":[]}"##,
    );
}

#[test]
fn test_181() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0069","initialState":"Data","input":"&#x0069;","inputUtf16":[38,35,120,48,48,54,57,59],"output":[{"Character":{"data":"i"}}],"errors":[]}"##,
    );
}

#[test]
fn test_182() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+006A","initialState":"Data","input":"&#x006a;","inputUtf16":[38,35,120,48,48,54,97,59],"output":[{"Character":{"data":"j"}}],"errors":[]}"##,
    );
}

#[test]
fn test_183() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+006B","initialState":"Data","input":"&#x006b;","inputUtf16":[38,35,120,48,48,54,98,59],"output":[{"Character":{"data":"k"}}],"errors":[]}"##,
    );
}

#[test]
fn test_184() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+006C","initialState":"Data","input":"&#x006c;","inputUtf16":[38,35,120,48,48,54,99,59],"output":[{"Character":{"data":"l"}}],"errors":[]}"##,
    );
}

#[test]
fn test_185() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+006D","initialState":"Data","input":"&#x006d;","inputUtf16":[38,35,120,48,48,54,100,59],"output":[{"Character":{"data":"m"}}],"errors":[]}"##,
    );
}

#[test]
fn test_186() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+006E","initialState":"Data","input":"&#x006e;","inputUtf16":[38,35,120,48,48,54,101,59],"output":[{"Character":{"data":"n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_187() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+006F","initialState":"Data","input":"&#x006f;","inputUtf16":[38,35,120,48,48,54,102,59],"output":[{"Character":{"data":"o"}}],"errors":[]}"##,
    );
}

#[test]
fn test_188() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0070","initialState":"Data","input":"&#x0070;","inputUtf16":[38,35,120,48,48,55,48,59],"output":[{"Character":{"data":"p"}}],"errors":[]}"##,
    );
}

#[test]
fn test_189() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0071","initialState":"Data","input":"&#x0071;","inputUtf16":[38,35,120,48,48,55,49,59],"output":[{"Character":{"data":"q"}}],"errors":[]}"##,
    );
}

#[test]
fn test_190() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0072","initialState":"Data","input":"&#x0072;","inputUtf16":[38,35,120,48,48,55,50,59],"output":[{"Character":{"data":"r"}}],"errors":[]}"##,
    );
}

#[test]
fn test_191() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0073","initialState":"Data","input":"&#x0073;","inputUtf16":[38,35,120,48,48,55,51,59],"output":[{"Character":{"data":"s"}}],"errors":[]}"##,
    );
}

#[test]
fn test_192() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0074","initialState":"Data","input":"&#x0074;","inputUtf16":[38,35,120,48,48,55,52,59],"output":[{"Character":{"data":"t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_193() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0075","initialState":"Data","input":"&#x0075;","inputUtf16":[38,35,120,48,48,55,53,59],"output":[{"Character":{"data":"u"}}],"errors":[]}"##,
    );
}

#[test]
fn test_194() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0076","initialState":"Data","input":"&#x0076;","inputUtf16":[38,35,120,48,48,55,54,59],"output":[{"Character":{"data":"v"}}],"errors":[]}"##,
    );
}

#[test]
fn test_195() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0077","initialState":"Data","input":"&#x0077;","inputUtf16":[38,35,120,48,48,55,55,59],"output":[{"Character":{"data":"w"}}],"errors":[]}"##,
    );
}

#[test]
fn test_196() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0078","initialState":"Data","input":"&#x0078;","inputUtf16":[38,35,120,48,48,55,56,59],"output":[{"Character":{"data":"x"}}],"errors":[]}"##,
    );
}

#[test]
fn test_197() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+0079","initialState":"Data","input":"&#x0079;","inputUtf16":[38,35,120,48,48,55,57,59],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_198() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+007A","initialState":"Data","input":"&#x007a;","inputUtf16":[38,35,120,48,48,55,97,59],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_199() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+007B","initialState":"Data","input":"&#x007b;","inputUtf16":[38,35,120,48,48,55,98,59],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_200() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+007C","initialState":"Data","input":"&#x007c;","inputUtf16":[38,35,120,48,48,55,99,59],"output":[{"Character":{"data":"|"}}],"errors":[]}"##,
    );
}

#[test]
fn test_201() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+007D","initialState":"Data","input":"&#x007d;","inputUtf16":[38,35,120,48,48,55,100,59],"output":[{"Character":{"data":"}"}}],"errors":[]}"##,
    );
}

#[test]
fn test_202() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+007E","initialState":"Data","input":"&#x007e;","inputUtf16":[38,35,120,48,48,55,101,59],"output":[{"Character":{"data":"~"}}],"errors":[]}"##,
    );
}

#[test]
fn test_203() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A0","initialState":"Data","input":"&#x00a0;","inputUtf16":[38,35,120,48,48,97,48,59],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_204() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A1","initialState":"Data","input":"&#x00a1;","inputUtf16":[38,35,120,48,48,97,49,59],"output":[{"Character":{"data":"¡"}}],"errors":[]}"##,
    );
}

#[test]
fn test_205() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A2","initialState":"Data","input":"&#x00a2;","inputUtf16":[38,35,120,48,48,97,50,59],"output":[{"Character":{"data":"¢"}}],"errors":[]}"##,
    );
}

#[test]
fn test_206() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A3","initialState":"Data","input":"&#x00a3;","inputUtf16":[38,35,120,48,48,97,51,59],"output":[{"Character":{"data":"£"}}],"errors":[]}"##,
    );
}

#[test]
fn test_207() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A4","initialState":"Data","input":"&#x00a4;","inputUtf16":[38,35,120,48,48,97,52,59],"output":[{"Character":{"data":"¤"}}],"errors":[]}"##,
    );
}

#[test]
fn test_208() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A5","initialState":"Data","input":"&#x00a5;","inputUtf16":[38,35,120,48,48,97,53,59],"output":[{"Character":{"data":"¥"}}],"errors":[]}"##,
    );
}

#[test]
fn test_209() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A6","initialState":"Data","input":"&#x00a6;","inputUtf16":[38,35,120,48,48,97,54,59],"output":[{"Character":{"data":"¦"}}],"errors":[]}"##,
    );
}

#[test]
fn test_210() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A7","initialState":"Data","input":"&#x00a7;","inputUtf16":[38,35,120,48,48,97,55,59],"output":[{"Character":{"data":"§"}}],"errors":[]}"##,
    );
}

#[test]
fn test_211() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A8","initialState":"Data","input":"&#x00a8;","inputUtf16":[38,35,120,48,48,97,56,59],"output":[{"Character":{"data":"¨"}}],"errors":[]}"##,
    );
}

#[test]
fn test_212() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00A9","initialState":"Data","input":"&#x00a9;","inputUtf16":[38,35,120,48,48,97,57,59],"output":[{"Character":{"data":"©"}}],"errors":[]}"##,
    );
}

#[test]
fn test_213() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00AA","initialState":"Data","input":"&#x00aa;","inputUtf16":[38,35,120,48,48,97,97,59],"output":[{"Character":{"data":"ª"}}],"errors":[]}"##,
    );
}

#[test]
fn test_214() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00AB","initialState":"Data","input":"&#x00ab;","inputUtf16":[38,35,120,48,48,97,98,59],"output":[{"Character":{"data":"«"}}],"errors":[]}"##,
    );
}

#[test]
fn test_215() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00AC","initialState":"Data","input":"&#x00ac;","inputUtf16":[38,35,120,48,48,97,99,59],"output":[{"Character":{"data":"¬"}}],"errors":[]}"##,
    );
}

#[test]
fn test_216() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00AD","initialState":"Data","input":"&#x00ad;","inputUtf16":[38,35,120,48,48,97,100,59],"output":[{"Character":{"data":"­"}}],"errors":[]}"##,
    );
}

#[test]
fn test_217() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00AE","initialState":"Data","input":"&#x00ae;","inputUtf16":[38,35,120,48,48,97,101,59],"output":[{"Character":{"data":"®"}}],"errors":[]}"##,
    );
}

#[test]
fn test_218() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00AF","initialState":"Data","input":"&#x00af;","inputUtf16":[38,35,120,48,48,97,102,59],"output":[{"Character":{"data":"¯"}}],"errors":[]}"##,
    );
}

#[test]
fn test_219() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B0","initialState":"Data","input":"&#x00b0;","inputUtf16":[38,35,120,48,48,98,48,59],"output":[{"Character":{"data":"°"}}],"errors":[]}"##,
    );
}

#[test]
fn test_220() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B1","initialState":"Data","input":"&#x00b1;","inputUtf16":[38,35,120,48,48,98,49,59],"output":[{"Character":{"data":"±"}}],"errors":[]}"##,
    );
}

#[test]
fn test_221() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B2","initialState":"Data","input":"&#x00b2;","inputUtf16":[38,35,120,48,48,98,50,59],"output":[{"Character":{"data":"²"}}],"errors":[]}"##,
    );
}

#[test]
fn test_222() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B3","initialState":"Data","input":"&#x00b3;","inputUtf16":[38,35,120,48,48,98,51,59],"output":[{"Character":{"data":"³"}}],"errors":[]}"##,
    );
}

#[test]
fn test_223() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B4","initialState":"Data","input":"&#x00b4;","inputUtf16":[38,35,120,48,48,98,52,59],"output":[{"Character":{"data":"´"}}],"errors":[]}"##,
    );
}

#[test]
fn test_224() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B5","initialState":"Data","input":"&#x00b5;","inputUtf16":[38,35,120,48,48,98,53,59],"output":[{"Character":{"data":"µ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_225() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B6","initialState":"Data","input":"&#x00b6;","inputUtf16":[38,35,120,48,48,98,54,59],"output":[{"Character":{"data":"¶"}}],"errors":[]}"##,
    );
}

#[test]
fn test_226() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B7","initialState":"Data","input":"&#x00b7;","inputUtf16":[38,35,120,48,48,98,55,59],"output":[{"Character":{"data":"·"}}],"errors":[]}"##,
    );
}

#[test]
fn test_227() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B8","initialState":"Data","input":"&#x00b8;","inputUtf16":[38,35,120,48,48,98,56,59],"output":[{"Character":{"data":"¸"}}],"errors":[]}"##,
    );
}

#[test]
fn test_228() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00B9","initialState":"Data","input":"&#x00b9;","inputUtf16":[38,35,120,48,48,98,57,59],"output":[{"Character":{"data":"¹"}}],"errors":[]}"##,
    );
}

#[test]
fn test_229() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00BA","initialState":"Data","input":"&#x00ba;","inputUtf16":[38,35,120,48,48,98,97,59],"output":[{"Character":{"data":"º"}}],"errors":[]}"##,
    );
}

#[test]
fn test_230() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00BB","initialState":"Data","input":"&#x00bb;","inputUtf16":[38,35,120,48,48,98,98,59],"output":[{"Character":{"data":"»"}}],"errors":[]}"##,
    );
}

#[test]
fn test_231() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00BC","initialState":"Data","input":"&#x00bc;","inputUtf16":[38,35,120,48,48,98,99,59],"output":[{"Character":{"data":"¼"}}],"errors":[]}"##,
    );
}

#[test]
fn test_232() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00BD","initialState":"Data","input":"&#x00bd;","inputUtf16":[38,35,120,48,48,98,100,59],"output":[{"Character":{"data":"½"}}],"errors":[]}"##,
    );
}

#[test]
fn test_233() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00BE","initialState":"Data","input":"&#x00be;","inputUtf16":[38,35,120,48,48,98,101,59],"output":[{"Character":{"data":"¾"}}],"errors":[]}"##,
    );
}

#[test]
fn test_234() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00BF","initialState":"Data","input":"&#x00bf;","inputUtf16":[38,35,120,48,48,98,102,59],"output":[{"Character":{"data":"¿"}}],"errors":[]}"##,
    );
}

#[test]
fn test_235() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C0","initialState":"Data","input":"&#x00c0;","inputUtf16":[38,35,120,48,48,99,48,59],"output":[{"Character":{"data":"À"}}],"errors":[]}"##,
    );
}

#[test]
fn test_236() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C1","initialState":"Data","input":"&#x00c1;","inputUtf16":[38,35,120,48,48,99,49,59],"output":[{"Character":{"data":"Á"}}],"errors":[]}"##,
    );
}

#[test]
fn test_237() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C2","initialState":"Data","input":"&#x00c2;","inputUtf16":[38,35,120,48,48,99,50,59],"output":[{"Character":{"data":"Â"}}],"errors":[]}"##,
    );
}

#[test]
fn test_238() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C3","initialState":"Data","input":"&#x00c3;","inputUtf16":[38,35,120,48,48,99,51,59],"output":[{"Character":{"data":"Ã"}}],"errors":[]}"##,
    );
}

#[test]
fn test_239() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C4","initialState":"Data","input":"&#x00c4;","inputUtf16":[38,35,120,48,48,99,52,59],"output":[{"Character":{"data":"Ä"}}],"errors":[]}"##,
    );
}

#[test]
fn test_240() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C5","initialState":"Data","input":"&#x00c5;","inputUtf16":[38,35,120,48,48,99,53,59],"output":[{"Character":{"data":"Å"}}],"errors":[]}"##,
    );
}

#[test]
fn test_241() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C6","initialState":"Data","input":"&#x00c6;","inputUtf16":[38,35,120,48,48,99,54,59],"output":[{"Character":{"data":"Æ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_242() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C7","initialState":"Data","input":"&#x00c7;","inputUtf16":[38,35,120,48,48,99,55,59],"output":[{"Character":{"data":"Ç"}}],"errors":[]}"##,
    );
}

#[test]
fn test_243() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C8","initialState":"Data","input":"&#x00c8;","inputUtf16":[38,35,120,48,48,99,56,59],"output":[{"Character":{"data":"È"}}],"errors":[]}"##,
    );
}

#[test]
fn test_244() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00C9","initialState":"Data","input":"&#x00c9;","inputUtf16":[38,35,120,48,48,99,57,59],"output":[{"Character":{"data":"É"}}],"errors":[]}"##,
    );
}

#[test]
fn test_245() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00CA","initialState":"Data","input":"&#x00ca;","inputUtf16":[38,35,120,48,48,99,97,59],"output":[{"Character":{"data":"Ê"}}],"errors":[]}"##,
    );
}

#[test]
fn test_246() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00CB","initialState":"Data","input":"&#x00cb;","inputUtf16":[38,35,120,48,48,99,98,59],"output":[{"Character":{"data":"Ë"}}],"errors":[]}"##,
    );
}

#[test]
fn test_247() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00CC","initialState":"Data","input":"&#x00cc;","inputUtf16":[38,35,120,48,48,99,99,59],"output":[{"Character":{"data":"Ì"}}],"errors":[]}"##,
    );
}

#[test]
fn test_248() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00CD","initialState":"Data","input":"&#x00cd;","inputUtf16":[38,35,120,48,48,99,100,59],"output":[{"Character":{"data":"Í"}}],"errors":[]}"##,
    );
}

#[test]
fn test_249() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00CE","initialState":"Data","input":"&#x00ce;","inputUtf16":[38,35,120,48,48,99,101,59],"output":[{"Character":{"data":"Î"}}],"errors":[]}"##,
    );
}

#[test]
fn test_250() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00CF","initialState":"Data","input":"&#x00cf;","inputUtf16":[38,35,120,48,48,99,102,59],"output":[{"Character":{"data":"Ï"}}],"errors":[]}"##,
    );
}

#[test]
fn test_251() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D0","initialState":"Data","input":"&#x00d0;","inputUtf16":[38,35,120,48,48,100,48,59],"output":[{"Character":{"data":"Ð"}}],"errors":[]}"##,
    );
}

#[test]
fn test_252() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D1","initialState":"Data","input":"&#x00d1;","inputUtf16":[38,35,120,48,48,100,49,59],"output":[{"Character":{"data":"Ñ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_253() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D2","initialState":"Data","input":"&#x00d2;","inputUtf16":[38,35,120,48,48,100,50,59],"output":[{"Character":{"data":"Ò"}}],"errors":[]}"##,
    );
}

#[test]
fn test_254() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D3","initialState":"Data","input":"&#x00d3;","inputUtf16":[38,35,120,48,48,100,51,59],"output":[{"Character":{"data":"Ó"}}],"errors":[]}"##,
    );
}

#[test]
fn test_255() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D4","initialState":"Data","input":"&#x00d4;","inputUtf16":[38,35,120,48,48,100,52,59],"output":[{"Character":{"data":"Ô"}}],"errors":[]}"##,
    );
}

#[test]
fn test_256() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D5","initialState":"Data","input":"&#x00d5;","inputUtf16":[38,35,120,48,48,100,53,59],"output":[{"Character":{"data":"Õ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_257() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D6","initialState":"Data","input":"&#x00d6;","inputUtf16":[38,35,120,48,48,100,54,59],"output":[{"Character":{"data":"Ö"}}],"errors":[]}"##,
    );
}

#[test]
fn test_258() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D7","initialState":"Data","input":"&#x00d7;","inputUtf16":[38,35,120,48,48,100,55,59],"output":[{"Character":{"data":"×"}}],"errors":[]}"##,
    );
}

#[test]
fn test_259() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D8","initialState":"Data","input":"&#x00d8;","inputUtf16":[38,35,120,48,48,100,56,59],"output":[{"Character":{"data":"Ø"}}],"errors":[]}"##,
    );
}

#[test]
fn test_260() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00D9","initialState":"Data","input":"&#x00d9;","inputUtf16":[38,35,120,48,48,100,57,59],"output":[{"Character":{"data":"Ù"}}],"errors":[]}"##,
    );
}

#[test]
fn test_261() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00DA","initialState":"Data","input":"&#x00da;","inputUtf16":[38,35,120,48,48,100,97,59],"output":[{"Character":{"data":"Ú"}}],"errors":[]}"##,
    );
}

#[test]
fn test_262() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00DB","initialState":"Data","input":"&#x00db;","inputUtf16":[38,35,120,48,48,100,98,59],"output":[{"Character":{"data":"Û"}}],"errors":[]}"##,
    );
}

#[test]
fn test_263() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00DC","initialState":"Data","input":"&#x00dc;","inputUtf16":[38,35,120,48,48,100,99,59],"output":[{"Character":{"data":"Ü"}}],"errors":[]}"##,
    );
}

#[test]
fn test_264() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00DD","initialState":"Data","input":"&#x00dd;","inputUtf16":[38,35,120,48,48,100,100,59],"output":[{"Character":{"data":"Ý"}}],"errors":[]}"##,
    );
}

#[test]
fn test_265() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00DE","initialState":"Data","input":"&#x00de;","inputUtf16":[38,35,120,48,48,100,101,59],"output":[{"Character":{"data":"Þ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_266() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00DF","initialState":"Data","input":"&#x00df;","inputUtf16":[38,35,120,48,48,100,102,59],"output":[{"Character":{"data":"ß"}}],"errors":[]}"##,
    );
}

#[test]
fn test_267() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E0","initialState":"Data","input":"&#x00e0;","inputUtf16":[38,35,120,48,48,101,48,59],"output":[{"Character":{"data":"à"}}],"errors":[]}"##,
    );
}

#[test]
fn test_268() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E1","initialState":"Data","input":"&#x00e1;","inputUtf16":[38,35,120,48,48,101,49,59],"output":[{"Character":{"data":"á"}}],"errors":[]}"##,
    );
}

#[test]
fn test_269() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E2","initialState":"Data","input":"&#x00e2;","inputUtf16":[38,35,120,48,48,101,50,59],"output":[{"Character":{"data":"â"}}],"errors":[]}"##,
    );
}

#[test]
fn test_270() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E3","initialState":"Data","input":"&#x00e3;","inputUtf16":[38,35,120,48,48,101,51,59],"output":[{"Character":{"data":"ã"}}],"errors":[]}"##,
    );
}

#[test]
fn test_271() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E4","initialState":"Data","input":"&#x00e4;","inputUtf16":[38,35,120,48,48,101,52,59],"output":[{"Character":{"data":"ä"}}],"errors":[]}"##,
    );
}

#[test]
fn test_272() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E5","initialState":"Data","input":"&#x00e5;","inputUtf16":[38,35,120,48,48,101,53,59],"output":[{"Character":{"data":"å"}}],"errors":[]}"##,
    );
}

#[test]
fn test_273() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E6","initialState":"Data","input":"&#x00e6;","inputUtf16":[38,35,120,48,48,101,54,59],"output":[{"Character":{"data":"æ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_274() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E7","initialState":"Data","input":"&#x00e7;","inputUtf16":[38,35,120,48,48,101,55,59],"output":[{"Character":{"data":"ç"}}],"errors":[]}"##,
    );
}

#[test]
fn test_275() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E8","initialState":"Data","input":"&#x00e8;","inputUtf16":[38,35,120,48,48,101,56,59],"output":[{"Character":{"data":"è"}}],"errors":[]}"##,
    );
}

#[test]
fn test_276() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00E9","initialState":"Data","input":"&#x00e9;","inputUtf16":[38,35,120,48,48,101,57,59],"output":[{"Character":{"data":"é"}}],"errors":[]}"##,
    );
}

#[test]
fn test_277() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00EA","initialState":"Data","input":"&#x00ea;","inputUtf16":[38,35,120,48,48,101,97,59],"output":[{"Character":{"data":"ê"}}],"errors":[]}"##,
    );
}

#[test]
fn test_278() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00EB","initialState":"Data","input":"&#x00eb;","inputUtf16":[38,35,120,48,48,101,98,59],"output":[{"Character":{"data":"ë"}}],"errors":[]}"##,
    );
}

#[test]
fn test_279() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00EC","initialState":"Data","input":"&#x00ec;","inputUtf16":[38,35,120,48,48,101,99,59],"output":[{"Character":{"data":"ì"}}],"errors":[]}"##,
    );
}

#[test]
fn test_280() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00ED","initialState":"Data","input":"&#x00ed;","inputUtf16":[38,35,120,48,48,101,100,59],"output":[{"Character":{"data":"í"}}],"errors":[]}"##,
    );
}

#[test]
fn test_281() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00EE","initialState":"Data","input":"&#x00ee;","inputUtf16":[38,35,120,48,48,101,101,59],"output":[{"Character":{"data":"î"}}],"errors":[]}"##,
    );
}

#[test]
fn test_282() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00EF","initialState":"Data","input":"&#x00ef;","inputUtf16":[38,35,120,48,48,101,102,59],"output":[{"Character":{"data":"ï"}}],"errors":[]}"##,
    );
}

#[test]
fn test_283() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F0","initialState":"Data","input":"&#x00f0;","inputUtf16":[38,35,120,48,48,102,48,59],"output":[{"Character":{"data":"ð"}}],"errors":[]}"##,
    );
}

#[test]
fn test_284() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F1","initialState":"Data","input":"&#x00f1;","inputUtf16":[38,35,120,48,48,102,49,59],"output":[{"Character":{"data":"ñ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_285() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F2","initialState":"Data","input":"&#x00f2;","inputUtf16":[38,35,120,48,48,102,50,59],"output":[{"Character":{"data":"ò"}}],"errors":[]}"##,
    );
}

#[test]
fn test_286() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F3","initialState":"Data","input":"&#x00f3;","inputUtf16":[38,35,120,48,48,102,51,59],"output":[{"Character":{"data":"ó"}}],"errors":[]}"##,
    );
}

#[test]
fn test_287() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F4","initialState":"Data","input":"&#x00f4;","inputUtf16":[38,35,120,48,48,102,52,59],"output":[{"Character":{"data":"ô"}}],"errors":[]}"##,
    );
}

#[test]
fn test_288() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F5","initialState":"Data","input":"&#x00f5;","inputUtf16":[38,35,120,48,48,102,53,59],"output":[{"Character":{"data":"õ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_289() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F6","initialState":"Data","input":"&#x00f6;","inputUtf16":[38,35,120,48,48,102,54,59],"output":[{"Character":{"data":"ö"}}],"errors":[]}"##,
    );
}

#[test]
fn test_290() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F7","initialState":"Data","input":"&#x00f7;","inputUtf16":[38,35,120,48,48,102,55,59],"output":[{"Character":{"data":"÷"}}],"errors":[]}"##,
    );
}

#[test]
fn test_291() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F8","initialState":"Data","input":"&#x00f8;","inputUtf16":[38,35,120,48,48,102,56,59],"output":[{"Character":{"data":"ø"}}],"errors":[]}"##,
    );
}

#[test]
fn test_292() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00F9","initialState":"Data","input":"&#x00f9;","inputUtf16":[38,35,120,48,48,102,57,59],"output":[{"Character":{"data":"ù"}}],"errors":[]}"##,
    );
}

#[test]
fn test_293() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00FA","initialState":"Data","input":"&#x00fa;","inputUtf16":[38,35,120,48,48,102,97,59],"output":[{"Character":{"data":"ú"}}],"errors":[]}"##,
    );
}

#[test]
fn test_294() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00FB","initialState":"Data","input":"&#x00fb;","inputUtf16":[38,35,120,48,48,102,98,59],"output":[{"Character":{"data":"û"}}],"errors":[]}"##,
    );
}

#[test]
fn test_295() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00FC","initialState":"Data","input":"&#x00fc;","inputUtf16":[38,35,120,48,48,102,99,59],"output":[{"Character":{"data":"ü"}}],"errors":[]}"##,
    );
}

#[test]
fn test_296() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00FD","initialState":"Data","input":"&#x00fd;","inputUtf16":[38,35,120,48,48,102,100,59],"output":[{"Character":{"data":"ý"}}],"errors":[]}"##,
    );
}

#[test]
fn test_297() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00FE","initialState":"Data","input":"&#x00fe;","inputUtf16":[38,35,120,48,48,102,101,59],"output":[{"Character":{"data":"þ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_298() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+00FF","initialState":"Data","input":"&#x00ff;","inputUtf16":[38,35,120,48,48,102,102,59],"output":[{"Character":{"data":"ÿ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_299() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+D7FF","initialState":"Data","input":"&#xd7ff;","inputUtf16":[38,35,120,100,55,102,102,59],"output":[{"Character":{"data":"퟿"}}],"errors":[]}"##,
    );
}

#[test]
fn test_300() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+E000","initialState":"Data","input":"&#xe000;","inputUtf16":[38,35,120,101,48,48,48,59],"output":[{"Character":{"data":""}}],"errors":[]}"##,
    );
}

#[test]
fn test_301() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+FDCF","initialState":"Data","input":"&#xfdcf;","inputUtf16":[38,35,120,102,100,99,102,59],"output":[{"Character":{"data":"﷏"}}],"errors":[]}"##,
    );
}

#[test]
fn test_302() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+FDF0","initialState":"Data","input":"&#xfdf0;","inputUtf16":[38,35,120,102,100,102,48,59],"output":[{"Character":{"data":"ﷰ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_303() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+FFFD","initialState":"Data","input":"&#xfffd;","inputUtf16":[38,35,120,102,102,102,100,59],"output":[{"Character":{"data":"�"}}],"errors":[]}"##,
    );
}

#[test]
fn test_304() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+10000","initialState":"Data","input":"&#x10000;","inputUtf16":[38,35,120,49,48,48,48,48,59],"output":[{"Character":{"data":"𐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_305() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+1FFFD","initialState":"Data","input":"&#x1fffd;","inputUtf16":[38,35,120,49,102,102,102,100,59],"output":[{"Character":{"data":"🿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_306() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+20000","initialState":"Data","input":"&#x20000;","inputUtf16":[38,35,120,50,48,48,48,48,59],"output":[{"Character":{"data":"𠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_307() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+2FFFD","initialState":"Data","input":"&#x2fffd;","inputUtf16":[38,35,120,50,102,102,102,100,59],"output":[{"Character":{"data":"𯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_308() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+30000","initialState":"Data","input":"&#x30000;","inputUtf16":[38,35,120,51,48,48,48,48,59],"output":[{"Character":{"data":"𰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_309() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+3FFFD","initialState":"Data","input":"&#x3fffd;","inputUtf16":[38,35,120,51,102,102,102,100,59],"output":[{"Character":{"data":"𿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_310() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+40000","initialState":"Data","input":"&#x40000;","inputUtf16":[38,35,120,52,48,48,48,48,59],"output":[{"Character":{"data":"񀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_311() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+4FFFD","initialState":"Data","input":"&#x4fffd;","inputUtf16":[38,35,120,52,102,102,102,100,59],"output":[{"Character":{"data":"񏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_312() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+50000","initialState":"Data","input":"&#x50000;","inputUtf16":[38,35,120,53,48,48,48,48,59],"output":[{"Character":{"data":"񐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_313() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+5FFFD","initialState":"Data","input":"&#x5fffd;","inputUtf16":[38,35,120,53,102,102,102,100,59],"output":[{"Character":{"data":"񟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_314() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+60000","initialState":"Data","input":"&#x60000;","inputUtf16":[38,35,120,54,48,48,48,48,59],"output":[{"Character":{"data":"񠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_315() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+6FFFD","initialState":"Data","input":"&#x6fffd;","inputUtf16":[38,35,120,54,102,102,102,100,59],"output":[{"Character":{"data":"񯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_316() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+70000","initialState":"Data","input":"&#x70000;","inputUtf16":[38,35,120,55,48,48,48,48,59],"output":[{"Character":{"data":"񰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_317() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+7FFFD","initialState":"Data","input":"&#x7fffd;","inputUtf16":[38,35,120,55,102,102,102,100,59],"output":[{"Character":{"data":"񿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_318() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+80000","initialState":"Data","input":"&#x80000;","inputUtf16":[38,35,120,56,48,48,48,48,59],"output":[{"Character":{"data":"򀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_319() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+8FFFD","initialState":"Data","input":"&#x8fffd;","inputUtf16":[38,35,120,56,102,102,102,100,59],"output":[{"Character":{"data":"򏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_320() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+90000","initialState":"Data","input":"&#x90000;","inputUtf16":[38,35,120,57,48,48,48,48,59],"output":[{"Character":{"data":"򐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_321() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+9FFFD","initialState":"Data","input":"&#x9fffd;","inputUtf16":[38,35,120,57,102,102,102,100,59],"output":[{"Character":{"data":"򟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_322() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+A0000","initialState":"Data","input":"&#xa0000;","inputUtf16":[38,35,120,97,48,48,48,48,59],"output":[{"Character":{"data":"򠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_323() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+AFFFD","initialState":"Data","input":"&#xafffd;","inputUtf16":[38,35,120,97,102,102,102,100,59],"output":[{"Character":{"data":"򯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_324() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+B0000","initialState":"Data","input":"&#xb0000;","inputUtf16":[38,35,120,98,48,48,48,48,59],"output":[{"Character":{"data":"򰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_325() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+BFFFD","initialState":"Data","input":"&#xbfffd;","inputUtf16":[38,35,120,98,102,102,102,100,59],"output":[{"Character":{"data":"򿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_326() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+C0000","initialState":"Data","input":"&#xc0000;","inputUtf16":[38,35,120,99,48,48,48,48,59],"output":[{"Character":{"data":"󀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_327() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+CFFFD","initialState":"Data","input":"&#xcfffd;","inputUtf16":[38,35,120,99,102,102,102,100,59],"output":[{"Character":{"data":"󏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_328() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+D0000","initialState":"Data","input":"&#xd0000;","inputUtf16":[38,35,120,100,48,48,48,48,59],"output":[{"Character":{"data":"󐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_329() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+DFFFD","initialState":"Data","input":"&#xdfffd;","inputUtf16":[38,35,120,100,102,102,102,100,59],"output":[{"Character":{"data":"󟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_330() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+E0000","initialState":"Data","input":"&#xe0000;","inputUtf16":[38,35,120,101,48,48,48,48,59],"output":[{"Character":{"data":"󠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_331() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+EFFFD","initialState":"Data","input":"&#xefffd;","inputUtf16":[38,35,120,101,102,102,102,100,59],"output":[{"Character":{"data":"󯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_332() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+F0000","initialState":"Data","input":"&#xf0000;","inputUtf16":[38,35,120,102,48,48,48,48,59],"output":[{"Character":{"data":"󰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_333() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+FFFFD","initialState":"Data","input":"&#xffffd;","inputUtf16":[38,35,120,102,102,102,102,100,59],"output":[{"Character":{"data":"󿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_334() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+100000","initialState":"Data","input":"&#x100000;","inputUtf16":[38,35,120,49,48,48,48,48,48,59],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_335() {
    tokenize(
        r##"{"description":"Valid numeric entity character U+10FFFD","initialState":"Data","input":"&#x10fffd;","inputUtf16":[38,35,120,49,48,102,102,102,100,59],"output":[{"Character":{"data":"􏿽"}}],"errors":[]}"##,
    );
}
//</coverage:exclude>
