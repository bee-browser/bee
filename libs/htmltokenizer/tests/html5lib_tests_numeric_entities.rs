mod helper;

use helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid unterminated numeric entity character overflow before EOF","initialState":"Data","input":"&#11111111111","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"missing-semicolon-after-character-reference","location":{"line":1,"column":14}},{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0001() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid unterminated numeric entity character overflow before EOF","initialState":"Data","input":"&#1111111111","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"missing-semicolon-after-character-reference","location":{"line":1,"column":13}},{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0002() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid unterminated numeric entity character overflow before EOF","initialState":"Data","input":"&#111111111111","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,49],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"missing-semicolon-after-character-reference","location":{"line":1,"column":15}},{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_0003() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid unterminated numeric entity character overflow","initialState":"Data","input":"&#11111111111x","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,120],"output":[{"Character":{"data":"�x"}}],"errors":[{"code":"missing-semicolon-after-character-reference","location":{"line":1,"column":14}},{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0004() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid unterminated numeric entity character overflow","initialState":"Data","input":"&#1111111111x","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,120],"output":[{"Character":{"data":"�x"}}],"errors":[{"code":"missing-semicolon-after-character-reference","location":{"line":1,"column":13}},{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0005() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid unterminated numeric entity character overflow","initialState":"Data","input":"&#111111111111x","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,49,120],"output":[{"Character":{"data":"�x"}}],"errors":[{"code":"missing-semicolon-after-character-reference","location":{"line":1,"column":15}},{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_0006() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character overflow","initialState":"Data","input":"&#11111111111;","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_0007() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character overflow","initialState":"Data","input":"&#1111111111;","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0008() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character overflow","initialState":"Data","input":"&#111111111111;","inputUtf16":[38,35,49,49,49,49,49,49,49,49,49,49,49,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_0009() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0000","initialState":"Data","input":"&#x0000;","inputUtf16":[38,35,120,48,48,48,48,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"null-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0010() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0001","initialState":"Data","input":"&#x0001;","inputUtf16":[38,35,120,48,48,48,49,59],"output":[{"Character":{"data":"\u0001"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0011() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0002","initialState":"Data","input":"&#x0002;","inputUtf16":[38,35,120,48,48,48,50,59],"output":[{"Character":{"data":"\u0002"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0012() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0003","initialState":"Data","input":"&#x0003;","inputUtf16":[38,35,120,48,48,48,51,59],"output":[{"Character":{"data":"\u0003"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0013() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0004","initialState":"Data","input":"&#x0004;","inputUtf16":[38,35,120,48,48,48,52,59],"output":[{"Character":{"data":"\u0004"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0014() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0005","initialState":"Data","input":"&#x0005;","inputUtf16":[38,35,120,48,48,48,53,59],"output":[{"Character":{"data":"\u0005"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0015() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0006","initialState":"Data","input":"&#x0006;","inputUtf16":[38,35,120,48,48,48,54,59],"output":[{"Character":{"data":"\u0006"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0016() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0007","initialState":"Data","input":"&#x0007;","inputUtf16":[38,35,120,48,48,48,55,59],"output":[{"Character":{"data":"\u0007"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0017() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0008","initialState":"Data","input":"&#x0008;","inputUtf16":[38,35,120,48,48,48,56,59],"output":[{"Character":{"data":"\b"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0018() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+000B","initialState":"Data","input":"&#x000b;","inputUtf16":[38,35,120,48,48,48,98,59],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0019() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+000E","initialState":"Data","input":"&#x000e;","inputUtf16":[38,35,120,48,48,48,101,59],"output":[{"Character":{"data":"\u000e"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0020() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+000F","initialState":"Data","input":"&#x000f;","inputUtf16":[38,35,120,48,48,48,102,59],"output":[{"Character":{"data":"\u000f"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0021() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0010","initialState":"Data","input":"&#x0010;","inputUtf16":[38,35,120,48,48,49,48,59],"output":[{"Character":{"data":"\u0010"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0022() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0011","initialState":"Data","input":"&#x0011;","inputUtf16":[38,35,120,48,48,49,49,59],"output":[{"Character":{"data":"\u0011"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0023() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0012","initialState":"Data","input":"&#x0012;","inputUtf16":[38,35,120,48,48,49,50,59],"output":[{"Character":{"data":"\u0012"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0024() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0013","initialState":"Data","input":"&#x0013;","inputUtf16":[38,35,120,48,48,49,51,59],"output":[{"Character":{"data":"\u0013"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0025() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0014","initialState":"Data","input":"&#x0014;","inputUtf16":[38,35,120,48,48,49,52,59],"output":[{"Character":{"data":"\u0014"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0026() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0015","initialState":"Data","input":"&#x0015;","inputUtf16":[38,35,120,48,48,49,53,59],"output":[{"Character":{"data":"\u0015"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0027() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0016","initialState":"Data","input":"&#x0016;","inputUtf16":[38,35,120,48,48,49,54,59],"output":[{"Character":{"data":"\u0016"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0028() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0017","initialState":"Data","input":"&#x0017;","inputUtf16":[38,35,120,48,48,49,55,59],"output":[{"Character":{"data":"\u0017"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0029() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0018","initialState":"Data","input":"&#x0018;","inputUtf16":[38,35,120,48,48,49,56,59],"output":[{"Character":{"data":"\u0018"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0030() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+0019","initialState":"Data","input":"&#x0019;","inputUtf16":[38,35,120,48,48,49,57,59],"output":[{"Character":{"data":"\u0019"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0031() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+001A","initialState":"Data","input":"&#x001a;","inputUtf16":[38,35,120,48,48,49,97,59],"output":[{"Character":{"data":"\u001a"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0032() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+001B","initialState":"Data","input":"&#x001b;","inputUtf16":[38,35,120,48,48,49,98,59],"output":[{"Character":{"data":"\u001b"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0033() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+001C","initialState":"Data","input":"&#x001c;","inputUtf16":[38,35,120,48,48,49,99,59],"output":[{"Character":{"data":"\u001c"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0034() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+001D","initialState":"Data","input":"&#x001d;","inputUtf16":[38,35,120,48,48,49,100,59],"output":[{"Character":{"data":"\u001d"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0035() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+001E","initialState":"Data","input":"&#x001e;","inputUtf16":[38,35,120,48,48,49,101,59],"output":[{"Character":{"data":"\u001e"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0036() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+001F","initialState":"Data","input":"&#x001f;","inputUtf16":[38,35,120,48,48,49,102,59],"output":[{"Character":{"data":"\u001f"}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0037() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+007F","initialState":"Data","input":"&#x007f;","inputUtf16":[38,35,120,48,48,55,102,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"control-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0038() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+D800","initialState":"Data","input":"&#xd800;","inputUtf16":[38,35,120,100,56,48,48,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"surrogate-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0039() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+DFFF","initialState":"Data","input":"&#xdfff;","inputUtf16":[38,35,120,100,102,102,102,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"surrogate-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0040() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD0","initialState":"Data","input":"&#xfdd0;","inputUtf16":[38,35,120,102,100,100,48,59],"output":[{"Character":{"data":"﷐"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0041() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD1","initialState":"Data","input":"&#xfdd1;","inputUtf16":[38,35,120,102,100,100,49,59],"output":[{"Character":{"data":"﷑"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0042() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD2","initialState":"Data","input":"&#xfdd2;","inputUtf16":[38,35,120,102,100,100,50,59],"output":[{"Character":{"data":"﷒"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0043() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD3","initialState":"Data","input":"&#xfdd3;","inputUtf16":[38,35,120,102,100,100,51,59],"output":[{"Character":{"data":"﷓"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0044() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD4","initialState":"Data","input":"&#xfdd4;","inputUtf16":[38,35,120,102,100,100,52,59],"output":[{"Character":{"data":"﷔"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0045() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD5","initialState":"Data","input":"&#xfdd5;","inputUtf16":[38,35,120,102,100,100,53,59],"output":[{"Character":{"data":"﷕"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0046() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD6","initialState":"Data","input":"&#xfdd6;","inputUtf16":[38,35,120,102,100,100,54,59],"output":[{"Character":{"data":"﷖"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0047() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD7","initialState":"Data","input":"&#xfdd7;","inputUtf16":[38,35,120,102,100,100,55,59],"output":[{"Character":{"data":"﷗"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0048() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD8","initialState":"Data","input":"&#xfdd8;","inputUtf16":[38,35,120,102,100,100,56,59],"output":[{"Character":{"data":"﷘"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0049() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDD9","initialState":"Data","input":"&#xfdd9;","inputUtf16":[38,35,120,102,100,100,57,59],"output":[{"Character":{"data":"﷙"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0050() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDDA","initialState":"Data","input":"&#xfdda;","inputUtf16":[38,35,120,102,100,100,97,59],"output":[{"Character":{"data":"﷚"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0051() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDDB","initialState":"Data","input":"&#xfddb;","inputUtf16":[38,35,120,102,100,100,98,59],"output":[{"Character":{"data":"﷛"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0052() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDDC","initialState":"Data","input":"&#xfddc;","inputUtf16":[38,35,120,102,100,100,99,59],"output":[{"Character":{"data":"﷜"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0053() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDDD","initialState":"Data","input":"&#xfddd;","inputUtf16":[38,35,120,102,100,100,100,59],"output":[{"Character":{"data":"﷝"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0054() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDDE","initialState":"Data","input":"&#xfdde;","inputUtf16":[38,35,120,102,100,100,101,59],"output":[{"Character":{"data":"﷞"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0055() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDDF","initialState":"Data","input":"&#xfddf;","inputUtf16":[38,35,120,102,100,100,102,59],"output":[{"Character":{"data":"﷟"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0056() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE0","initialState":"Data","input":"&#xfde0;","inputUtf16":[38,35,120,102,100,101,48,59],"output":[{"Character":{"data":"﷠"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0057() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE1","initialState":"Data","input":"&#xfde1;","inputUtf16":[38,35,120,102,100,101,49,59],"output":[{"Character":{"data":"﷡"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0058() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE2","initialState":"Data","input":"&#xfde2;","inputUtf16":[38,35,120,102,100,101,50,59],"output":[{"Character":{"data":"﷢"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0059() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE3","initialState":"Data","input":"&#xfde3;","inputUtf16":[38,35,120,102,100,101,51,59],"output":[{"Character":{"data":"﷣"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0060() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE4","initialState":"Data","input":"&#xfde4;","inputUtf16":[38,35,120,102,100,101,52,59],"output":[{"Character":{"data":"﷤"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0061() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE5","initialState":"Data","input":"&#xfde5;","inputUtf16":[38,35,120,102,100,101,53,59],"output":[{"Character":{"data":"﷥"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0062() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE6","initialState":"Data","input":"&#xfde6;","inputUtf16":[38,35,120,102,100,101,54,59],"output":[{"Character":{"data":"﷦"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0063() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE7","initialState":"Data","input":"&#xfde7;","inputUtf16":[38,35,120,102,100,101,55,59],"output":[{"Character":{"data":"﷧"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0064() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE8","initialState":"Data","input":"&#xfde8;","inputUtf16":[38,35,120,102,100,101,56,59],"output":[{"Character":{"data":"﷨"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0065() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDE9","initialState":"Data","input":"&#xfde9;","inputUtf16":[38,35,120,102,100,101,57,59],"output":[{"Character":{"data":"﷩"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0066() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDEA","initialState":"Data","input":"&#xfdea;","inputUtf16":[38,35,120,102,100,101,97,59],"output":[{"Character":{"data":"﷪"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0067() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDEB","initialState":"Data","input":"&#xfdeb;","inputUtf16":[38,35,120,102,100,101,98,59],"output":[{"Character":{"data":"﷫"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0068() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDEC","initialState":"Data","input":"&#xfdec;","inputUtf16":[38,35,120,102,100,101,99,59],"output":[{"Character":{"data":"﷬"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0069() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDED","initialState":"Data","input":"&#xfded;","inputUtf16":[38,35,120,102,100,101,100,59],"output":[{"Character":{"data":"﷭"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0070() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDEE","initialState":"Data","input":"&#xfdee;","inputUtf16":[38,35,120,102,100,101,101,59],"output":[{"Character":{"data":"﷮"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0071() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FDEF","initialState":"Data","input":"&#xfdef;","inputUtf16":[38,35,120,102,100,101,102,59],"output":[{"Character":{"data":"﷯"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0072() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FFFE","initialState":"Data","input":"&#xfffe;","inputUtf16":[38,35,120,102,102,102,101,59],"output":[{"Character":{"data":"￾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0073() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FFFF","initialState":"Data","input":"&#xffff;","inputUtf16":[38,35,120,102,102,102,102,59],"output":[{"Character":{"data":"￿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0074() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+1FFFE","initialState":"Data","input":"&#x1fffe;","inputUtf16":[38,35,120,49,102,102,102,101,59],"output":[{"Character":{"data":"🿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0075() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+1FFFF","initialState":"Data","input":"&#x1ffff;","inputUtf16":[38,35,120,49,102,102,102,102,59],"output":[{"Character":{"data":"🿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0076() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+2FFFE","initialState":"Data","input":"&#x2fffe;","inputUtf16":[38,35,120,50,102,102,102,101,59],"output":[{"Character":{"data":"𯿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0077() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+2FFFF","initialState":"Data","input":"&#x2ffff;","inputUtf16":[38,35,120,50,102,102,102,102,59],"output":[{"Character":{"data":"𯿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0078() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+3FFFE","initialState":"Data","input":"&#x3fffe;","inputUtf16":[38,35,120,51,102,102,102,101,59],"output":[{"Character":{"data":"𿿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0079() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+3FFFF","initialState":"Data","input":"&#x3ffff;","inputUtf16":[38,35,120,51,102,102,102,102,59],"output":[{"Character":{"data":"𿿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0080() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+4FFFE","initialState":"Data","input":"&#x4fffe;","inputUtf16":[38,35,120,52,102,102,102,101,59],"output":[{"Character":{"data":"񏿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0081() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+4FFFF","initialState":"Data","input":"&#x4ffff;","inputUtf16":[38,35,120,52,102,102,102,102,59],"output":[{"Character":{"data":"񏿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0082() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+5FFFE","initialState":"Data","input":"&#x5fffe;","inputUtf16":[38,35,120,53,102,102,102,101,59],"output":[{"Character":{"data":"񟿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0083() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+5FFFF","initialState":"Data","input":"&#x5ffff;","inputUtf16":[38,35,120,53,102,102,102,102,59],"output":[{"Character":{"data":"񟿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0084() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+6FFFE","initialState":"Data","input":"&#x6fffe;","inputUtf16":[38,35,120,54,102,102,102,101,59],"output":[{"Character":{"data":"񯿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0085() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+6FFFF","initialState":"Data","input":"&#x6ffff;","inputUtf16":[38,35,120,54,102,102,102,102,59],"output":[{"Character":{"data":"񯿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0086() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+7FFFE","initialState":"Data","input":"&#x7fffe;","inputUtf16":[38,35,120,55,102,102,102,101,59],"output":[{"Character":{"data":"񿿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0087() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+7FFFF","initialState":"Data","input":"&#x7ffff;","inputUtf16":[38,35,120,55,102,102,102,102,59],"output":[{"Character":{"data":"񿿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0088() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+8FFFE","initialState":"Data","input":"&#x8fffe;","inputUtf16":[38,35,120,56,102,102,102,101,59],"output":[{"Character":{"data":"򏿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0089() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+8FFFF","initialState":"Data","input":"&#x8ffff;","inputUtf16":[38,35,120,56,102,102,102,102,59],"output":[{"Character":{"data":"򏿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0090() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+9FFFE","initialState":"Data","input":"&#x9fffe;","inputUtf16":[38,35,120,57,102,102,102,101,59],"output":[{"Character":{"data":"򟿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0091() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+9FFFF","initialState":"Data","input":"&#x9ffff;","inputUtf16":[38,35,120,57,102,102,102,102,59],"output":[{"Character":{"data":"򟿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0092() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+AFFFE","initialState":"Data","input":"&#xafffe;","inputUtf16":[38,35,120,97,102,102,102,101,59],"output":[{"Character":{"data":"򯿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0093() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+AFFFF","initialState":"Data","input":"&#xaffff;","inputUtf16":[38,35,120,97,102,102,102,102,59],"output":[{"Character":{"data":"򯿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0094() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+BFFFE","initialState":"Data","input":"&#xbfffe;","inputUtf16":[38,35,120,98,102,102,102,101,59],"output":[{"Character":{"data":"򿿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0095() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+BFFFF","initialState":"Data","input":"&#xbffff;","inputUtf16":[38,35,120,98,102,102,102,102,59],"output":[{"Character":{"data":"򿿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0096() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+CFFFE","initialState":"Data","input":"&#xcfffe;","inputUtf16":[38,35,120,99,102,102,102,101,59],"output":[{"Character":{"data":"󏿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0097() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+CFFFF","initialState":"Data","input":"&#xcffff;","inputUtf16":[38,35,120,99,102,102,102,102,59],"output":[{"Character":{"data":"󏿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0098() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+DFFFE","initialState":"Data","input":"&#xdfffe;","inputUtf16":[38,35,120,100,102,102,102,101,59],"output":[{"Character":{"data":"󟿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0099() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+DFFFF","initialState":"Data","input":"&#xdffff;","inputUtf16":[38,35,120,100,102,102,102,102,59],"output":[{"Character":{"data":"󟿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0100() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+EFFFE","initialState":"Data","input":"&#xefffe;","inputUtf16":[38,35,120,101,102,102,102,101,59],"output":[{"Character":{"data":"󯿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0101() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+EFFFF","initialState":"Data","input":"&#xeffff;","inputUtf16":[38,35,120,101,102,102,102,102,59],"output":[{"Character":{"data":"󯿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0102() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FFFFE","initialState":"Data","input":"&#xffffe;","inputUtf16":[38,35,120,102,102,102,102,101,59],"output":[{"Character":{"data":"󿿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0103() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+FFFFF","initialState":"Data","input":"&#xfffff;","inputUtf16":[38,35,120,102,102,102,102,102,59],"output":[{"Character":{"data":"󿿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0104() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+10FFFE","initialState":"Data","input":"&#x10fffe;","inputUtf16":[38,35,120,49,48,102,102,102,101,59],"output":[{"Character":{"data":"􏿾"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0105() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid numeric entity character U+10FFFF","initialState":"Data","input":"&#x10ffff;","inputUtf16":[38,35,120,49,48,102,102,102,102,59],"output":[{"Character":{"data":"􏿿"}}],"errors":[{"code":"noncharacter-character-reference","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0106() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0009","initialState":"Data","input":"&#x0009;","inputUtf16":[38,35,120,48,48,48,57,59],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0107() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+000A","initialState":"Data","input":"&#x000a;","inputUtf16":[38,35,120,48,48,48,97,59],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0108() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0020","initialState":"Data","input":"&#x0020;","inputUtf16":[38,35,120,48,48,50,48,59],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0109() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0021","initialState":"Data","input":"&#x0021;","inputUtf16":[38,35,120,48,48,50,49,59],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0110() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0022","initialState":"Data","input":"&#x0022;","inputUtf16":[38,35,120,48,48,50,50,59],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0111() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0023","initialState":"Data","input":"&#x0023;","inputUtf16":[38,35,120,48,48,50,51,59],"output":[{"Character":{"data":"#"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0112() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0024","initialState":"Data","input":"&#x0024;","inputUtf16":[38,35,120,48,48,50,52,59],"output":[{"Character":{"data":"$"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0113() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0025","initialState":"Data","input":"&#x0025;","inputUtf16":[38,35,120,48,48,50,53,59],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0114() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0026","initialState":"Data","input":"&#x0026;","inputUtf16":[38,35,120,48,48,50,54,59],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0115() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0027","initialState":"Data","input":"&#x0027;","inputUtf16":[38,35,120,48,48,50,55,59],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0116() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0028","initialState":"Data","input":"&#x0028;","inputUtf16":[38,35,120,48,48,50,56,59],"output":[{"Character":{"data":"("}}],"errors":[]}"##,
    );
}

#[test]
fn test_0117() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0029","initialState":"Data","input":"&#x0029;","inputUtf16":[38,35,120,48,48,50,57,59],"output":[{"Character":{"data":")"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0118() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+002A","initialState":"Data","input":"&#x002a;","inputUtf16":[38,35,120,48,48,50,97,59],"output":[{"Character":{"data":"*"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0119() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+002B","initialState":"Data","input":"&#x002b;","inputUtf16":[38,35,120,48,48,50,98,59],"output":[{"Character":{"data":"+"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0120() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+002C","initialState":"Data","input":"&#x002c;","inputUtf16":[38,35,120,48,48,50,99,59],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_0121() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+002D","initialState":"Data","input":"&#x002d;","inputUtf16":[38,35,120,48,48,50,100,59],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0122() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+002E","initialState":"Data","input":"&#x002e;","inputUtf16":[38,35,120,48,48,50,101,59],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_0123() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+002F","initialState":"Data","input":"&#x002f;","inputUtf16":[38,35,120,48,48,50,102,59],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0124() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0030","initialState":"Data","input":"&#x0030;","inputUtf16":[38,35,120,48,48,51,48,59],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0125() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0031","initialState":"Data","input":"&#x0031;","inputUtf16":[38,35,120,48,48,51,49,59],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0126() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0032","initialState":"Data","input":"&#x0032;","inputUtf16":[38,35,120,48,48,51,50,59],"output":[{"Character":{"data":"2"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0127() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0033","initialState":"Data","input":"&#x0033;","inputUtf16":[38,35,120,48,48,51,51,59],"output":[{"Character":{"data":"3"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0128() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0034","initialState":"Data","input":"&#x0034;","inputUtf16":[38,35,120,48,48,51,52,59],"output":[{"Character":{"data":"4"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0129() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0035","initialState":"Data","input":"&#x0035;","inputUtf16":[38,35,120,48,48,51,53,59],"output":[{"Character":{"data":"5"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0130() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0036","initialState":"Data","input":"&#x0036;","inputUtf16":[38,35,120,48,48,51,54,59],"output":[{"Character":{"data":"6"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0131() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0037","initialState":"Data","input":"&#x0037;","inputUtf16":[38,35,120,48,48,51,55,59],"output":[{"Character":{"data":"7"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0132() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0038","initialState":"Data","input":"&#x0038;","inputUtf16":[38,35,120,48,48,51,56,59],"output":[{"Character":{"data":"8"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0133() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0039","initialState":"Data","input":"&#x0039;","inputUtf16":[38,35,120,48,48,51,57,59],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0134() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+003A","initialState":"Data","input":"&#x003a;","inputUtf16":[38,35,120,48,48,51,97,59],"output":[{"Character":{"data":":"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0135() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+003B","initialState":"Data","input":"&#x003b;","inputUtf16":[38,35,120,48,48,51,98,59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0136() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+003C","initialState":"Data","input":"&#x003c;","inputUtf16":[38,35,120,48,48,51,99,59],"output":[{"Character":{"data":"<"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0137() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+003D","initialState":"Data","input":"&#x003d;","inputUtf16":[38,35,120,48,48,51,100,59],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_0138() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+003E","initialState":"Data","input":"&#x003e;","inputUtf16":[38,35,120,48,48,51,101,59],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0139() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+003F","initialState":"Data","input":"&#x003f;","inputUtf16":[38,35,120,48,48,51,102,59],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0140() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0040","initialState":"Data","input":"&#x0040;","inputUtf16":[38,35,120,48,48,52,48,59],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0141() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0041","initialState":"Data","input":"&#x0041;","inputUtf16":[38,35,120,48,48,52,49,59],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0142() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0042","initialState":"Data","input":"&#x0042;","inputUtf16":[38,35,120,48,48,52,50,59],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0143() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0043","initialState":"Data","input":"&#x0043;","inputUtf16":[38,35,120,48,48,52,51,59],"output":[{"Character":{"data":"C"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0144() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0044","initialState":"Data","input":"&#x0044;","inputUtf16":[38,35,120,48,48,52,52,59],"output":[{"Character":{"data":"D"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0145() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0045","initialState":"Data","input":"&#x0045;","inputUtf16":[38,35,120,48,48,52,53,59],"output":[{"Character":{"data":"E"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0146() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0046","initialState":"Data","input":"&#x0046;","inputUtf16":[38,35,120,48,48,52,54,59],"output":[{"Character":{"data":"F"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0147() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0047","initialState":"Data","input":"&#x0047;","inputUtf16":[38,35,120,48,48,52,55,59],"output":[{"Character":{"data":"G"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0148() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0048","initialState":"Data","input":"&#x0048;","inputUtf16":[38,35,120,48,48,52,56,59],"output":[{"Character":{"data":"H"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0149() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0049","initialState":"Data","input":"&#x0049;","inputUtf16":[38,35,120,48,48,52,57,59],"output":[{"Character":{"data":"I"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0150() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+004A","initialState":"Data","input":"&#x004a;","inputUtf16":[38,35,120,48,48,52,97,59],"output":[{"Character":{"data":"J"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0151() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+004B","initialState":"Data","input":"&#x004b;","inputUtf16":[38,35,120,48,48,52,98,59],"output":[{"Character":{"data":"K"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0152() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+004C","initialState":"Data","input":"&#x004c;","inputUtf16":[38,35,120,48,48,52,99,59],"output":[{"Character":{"data":"L"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0153() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+004D","initialState":"Data","input":"&#x004d;","inputUtf16":[38,35,120,48,48,52,100,59],"output":[{"Character":{"data":"M"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0154() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+004E","initialState":"Data","input":"&#x004e;","inputUtf16":[38,35,120,48,48,52,101,59],"output":[{"Character":{"data":"N"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0155() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+004F","initialState":"Data","input":"&#x004f;","inputUtf16":[38,35,120,48,48,52,102,59],"output":[{"Character":{"data":"O"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0156() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0050","initialState":"Data","input":"&#x0050;","inputUtf16":[38,35,120,48,48,53,48,59],"output":[{"Character":{"data":"P"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0157() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0051","initialState":"Data","input":"&#x0051;","inputUtf16":[38,35,120,48,48,53,49,59],"output":[{"Character":{"data":"Q"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0158() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0052","initialState":"Data","input":"&#x0052;","inputUtf16":[38,35,120,48,48,53,50,59],"output":[{"Character":{"data":"R"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0159() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0053","initialState":"Data","input":"&#x0053;","inputUtf16":[38,35,120,48,48,53,51,59],"output":[{"Character":{"data":"S"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0160() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0054","initialState":"Data","input":"&#x0054;","inputUtf16":[38,35,120,48,48,53,52,59],"output":[{"Character":{"data":"T"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0161() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0055","initialState":"Data","input":"&#x0055;","inputUtf16":[38,35,120,48,48,53,53,59],"output":[{"Character":{"data":"U"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0162() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0056","initialState":"Data","input":"&#x0056;","inputUtf16":[38,35,120,48,48,53,54,59],"output":[{"Character":{"data":"V"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0163() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0057","initialState":"Data","input":"&#x0057;","inputUtf16":[38,35,120,48,48,53,55,59],"output":[{"Character":{"data":"W"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0164() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0058","initialState":"Data","input":"&#x0058;","inputUtf16":[38,35,120,48,48,53,56,59],"output":[{"Character":{"data":"X"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0165() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0059","initialState":"Data","input":"&#x0059;","inputUtf16":[38,35,120,48,48,53,57,59],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0166() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+005A","initialState":"Data","input":"&#x005a;","inputUtf16":[38,35,120,48,48,53,97,59],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0167() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+005B","initialState":"Data","input":"&#x005b;","inputUtf16":[38,35,120,48,48,53,98,59],"output":[{"Character":{"data":"["}}],"errors":[]}"##,
    );
}

#[test]
fn test_0168() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+005C","initialState":"Data","input":"&#x005c;","inputUtf16":[38,35,120,48,48,53,99,59],"output":[{"Character":{"data":"\\"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0169() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+005D","initialState":"Data","input":"&#x005d;","inputUtf16":[38,35,120,48,48,53,100,59],"output":[{"Character":{"data":"]"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0170() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+005E","initialState":"Data","input":"&#x005e;","inputUtf16":[38,35,120,48,48,53,101,59],"output":[{"Character":{"data":"^"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0171() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+005F","initialState":"Data","input":"&#x005f;","inputUtf16":[38,35,120,48,48,53,102,59],"output":[{"Character":{"data":"_"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0172() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0060","initialState":"Data","input":"&#x0060;","inputUtf16":[38,35,120,48,48,54,48,59],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0173() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0061","initialState":"Data","input":"&#x0061;","inputUtf16":[38,35,120,48,48,54,49,59],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0174() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0062","initialState":"Data","input":"&#x0062;","inputUtf16":[38,35,120,48,48,54,50,59],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0175() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0063","initialState":"Data","input":"&#x0063;","inputUtf16":[38,35,120,48,48,54,51,59],"output":[{"Character":{"data":"c"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0176() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0064","initialState":"Data","input":"&#x0064;","inputUtf16":[38,35,120,48,48,54,52,59],"output":[{"Character":{"data":"d"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0177() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0065","initialState":"Data","input":"&#x0065;","inputUtf16":[38,35,120,48,48,54,53,59],"output":[{"Character":{"data":"e"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0178() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0066","initialState":"Data","input":"&#x0066;","inputUtf16":[38,35,120,48,48,54,54,59],"output":[{"Character":{"data":"f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0179() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0067","initialState":"Data","input":"&#x0067;","inputUtf16":[38,35,120,48,48,54,55,59],"output":[{"Character":{"data":"g"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0180() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0068","initialState":"Data","input":"&#x0068;","inputUtf16":[38,35,120,48,48,54,56,59],"output":[{"Character":{"data":"h"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0181() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0069","initialState":"Data","input":"&#x0069;","inputUtf16":[38,35,120,48,48,54,57,59],"output":[{"Character":{"data":"i"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0182() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+006A","initialState":"Data","input":"&#x006a;","inputUtf16":[38,35,120,48,48,54,97,59],"output":[{"Character":{"data":"j"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0183() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+006B","initialState":"Data","input":"&#x006b;","inputUtf16":[38,35,120,48,48,54,98,59],"output":[{"Character":{"data":"k"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0184() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+006C","initialState":"Data","input":"&#x006c;","inputUtf16":[38,35,120,48,48,54,99,59],"output":[{"Character":{"data":"l"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0185() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+006D","initialState":"Data","input":"&#x006d;","inputUtf16":[38,35,120,48,48,54,100,59],"output":[{"Character":{"data":"m"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0186() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+006E","initialState":"Data","input":"&#x006e;","inputUtf16":[38,35,120,48,48,54,101,59],"output":[{"Character":{"data":"n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0187() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+006F","initialState":"Data","input":"&#x006f;","inputUtf16":[38,35,120,48,48,54,102,59],"output":[{"Character":{"data":"o"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0188() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0070","initialState":"Data","input":"&#x0070;","inputUtf16":[38,35,120,48,48,55,48,59],"output":[{"Character":{"data":"p"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0189() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0071","initialState":"Data","input":"&#x0071;","inputUtf16":[38,35,120,48,48,55,49,59],"output":[{"Character":{"data":"q"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0190() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0072","initialState":"Data","input":"&#x0072;","inputUtf16":[38,35,120,48,48,55,50,59],"output":[{"Character":{"data":"r"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0191() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0073","initialState":"Data","input":"&#x0073;","inputUtf16":[38,35,120,48,48,55,51,59],"output":[{"Character":{"data":"s"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0192() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0074","initialState":"Data","input":"&#x0074;","inputUtf16":[38,35,120,48,48,55,52,59],"output":[{"Character":{"data":"t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0193() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0075","initialState":"Data","input":"&#x0075;","inputUtf16":[38,35,120,48,48,55,53,59],"output":[{"Character":{"data":"u"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0194() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0076","initialState":"Data","input":"&#x0076;","inputUtf16":[38,35,120,48,48,55,54,59],"output":[{"Character":{"data":"v"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0195() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0077","initialState":"Data","input":"&#x0077;","inputUtf16":[38,35,120,48,48,55,55,59],"output":[{"Character":{"data":"w"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0196() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0078","initialState":"Data","input":"&#x0078;","inputUtf16":[38,35,120,48,48,55,56,59],"output":[{"Character":{"data":"x"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0197() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+0079","initialState":"Data","input":"&#x0079;","inputUtf16":[38,35,120,48,48,55,57,59],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0198() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+007A","initialState":"Data","input":"&#x007a;","inputUtf16":[38,35,120,48,48,55,97,59],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0199() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+007B","initialState":"Data","input":"&#x007b;","inputUtf16":[38,35,120,48,48,55,98,59],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0200() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+007C","initialState":"Data","input":"&#x007c;","inputUtf16":[38,35,120,48,48,55,99,59],"output":[{"Character":{"data":"|"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0201() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+007D","initialState":"Data","input":"&#x007d;","inputUtf16":[38,35,120,48,48,55,100,59],"output":[{"Character":{"data":"}"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0202() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+007E","initialState":"Data","input":"&#x007e;","inputUtf16":[38,35,120,48,48,55,101,59],"output":[{"Character":{"data":"~"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0203() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A0","initialState":"Data","input":"&#x00a0;","inputUtf16":[38,35,120,48,48,97,48,59],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0204() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A1","initialState":"Data","input":"&#x00a1;","inputUtf16":[38,35,120,48,48,97,49,59],"output":[{"Character":{"data":"¡"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0205() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A2","initialState":"Data","input":"&#x00a2;","inputUtf16":[38,35,120,48,48,97,50,59],"output":[{"Character":{"data":"¢"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0206() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A3","initialState":"Data","input":"&#x00a3;","inputUtf16":[38,35,120,48,48,97,51,59],"output":[{"Character":{"data":"£"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0207() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A4","initialState":"Data","input":"&#x00a4;","inputUtf16":[38,35,120,48,48,97,52,59],"output":[{"Character":{"data":"¤"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0208() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A5","initialState":"Data","input":"&#x00a5;","inputUtf16":[38,35,120,48,48,97,53,59],"output":[{"Character":{"data":"¥"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0209() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A6","initialState":"Data","input":"&#x00a6;","inputUtf16":[38,35,120,48,48,97,54,59],"output":[{"Character":{"data":"¦"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0210() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A7","initialState":"Data","input":"&#x00a7;","inputUtf16":[38,35,120,48,48,97,55,59],"output":[{"Character":{"data":"§"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0211() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A8","initialState":"Data","input":"&#x00a8;","inputUtf16":[38,35,120,48,48,97,56,59],"output":[{"Character":{"data":"¨"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0212() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00A9","initialState":"Data","input":"&#x00a9;","inputUtf16":[38,35,120,48,48,97,57,59],"output":[{"Character":{"data":"©"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0213() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00AA","initialState":"Data","input":"&#x00aa;","inputUtf16":[38,35,120,48,48,97,97,59],"output":[{"Character":{"data":"ª"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0214() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00AB","initialState":"Data","input":"&#x00ab;","inputUtf16":[38,35,120,48,48,97,98,59],"output":[{"Character":{"data":"«"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0215() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00AC","initialState":"Data","input":"&#x00ac;","inputUtf16":[38,35,120,48,48,97,99,59],"output":[{"Character":{"data":"¬"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0216() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00AD","initialState":"Data","input":"&#x00ad;","inputUtf16":[38,35,120,48,48,97,100,59],"output":[{"Character":{"data":"­"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0217() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00AE","initialState":"Data","input":"&#x00ae;","inputUtf16":[38,35,120,48,48,97,101,59],"output":[{"Character":{"data":"®"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0218() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00AF","initialState":"Data","input":"&#x00af;","inputUtf16":[38,35,120,48,48,97,102,59],"output":[{"Character":{"data":"¯"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0219() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B0","initialState":"Data","input":"&#x00b0;","inputUtf16":[38,35,120,48,48,98,48,59],"output":[{"Character":{"data":"°"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0220() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B1","initialState":"Data","input":"&#x00b1;","inputUtf16":[38,35,120,48,48,98,49,59],"output":[{"Character":{"data":"±"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0221() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B2","initialState":"Data","input":"&#x00b2;","inputUtf16":[38,35,120,48,48,98,50,59],"output":[{"Character":{"data":"²"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0222() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B3","initialState":"Data","input":"&#x00b3;","inputUtf16":[38,35,120,48,48,98,51,59],"output":[{"Character":{"data":"³"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0223() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B4","initialState":"Data","input":"&#x00b4;","inputUtf16":[38,35,120,48,48,98,52,59],"output":[{"Character":{"data":"´"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0224() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B5","initialState":"Data","input":"&#x00b5;","inputUtf16":[38,35,120,48,48,98,53,59],"output":[{"Character":{"data":"µ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0225() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B6","initialState":"Data","input":"&#x00b6;","inputUtf16":[38,35,120,48,48,98,54,59],"output":[{"Character":{"data":"¶"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0226() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B7","initialState":"Data","input":"&#x00b7;","inputUtf16":[38,35,120,48,48,98,55,59],"output":[{"Character":{"data":"·"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0227() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B8","initialState":"Data","input":"&#x00b8;","inputUtf16":[38,35,120,48,48,98,56,59],"output":[{"Character":{"data":"¸"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0228() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00B9","initialState":"Data","input":"&#x00b9;","inputUtf16":[38,35,120,48,48,98,57,59],"output":[{"Character":{"data":"¹"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0229() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00BA","initialState":"Data","input":"&#x00ba;","inputUtf16":[38,35,120,48,48,98,97,59],"output":[{"Character":{"data":"º"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0230() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00BB","initialState":"Data","input":"&#x00bb;","inputUtf16":[38,35,120,48,48,98,98,59],"output":[{"Character":{"data":"»"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0231() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00BC","initialState":"Data","input":"&#x00bc;","inputUtf16":[38,35,120,48,48,98,99,59],"output":[{"Character":{"data":"¼"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0232() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00BD","initialState":"Data","input":"&#x00bd;","inputUtf16":[38,35,120,48,48,98,100,59],"output":[{"Character":{"data":"½"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0233() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00BE","initialState":"Data","input":"&#x00be;","inputUtf16":[38,35,120,48,48,98,101,59],"output":[{"Character":{"data":"¾"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0234() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00BF","initialState":"Data","input":"&#x00bf;","inputUtf16":[38,35,120,48,48,98,102,59],"output":[{"Character":{"data":"¿"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0235() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C0","initialState":"Data","input":"&#x00c0;","inputUtf16":[38,35,120,48,48,99,48,59],"output":[{"Character":{"data":"À"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0236() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C1","initialState":"Data","input":"&#x00c1;","inputUtf16":[38,35,120,48,48,99,49,59],"output":[{"Character":{"data":"Á"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0237() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C2","initialState":"Data","input":"&#x00c2;","inputUtf16":[38,35,120,48,48,99,50,59],"output":[{"Character":{"data":"Â"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0238() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C3","initialState":"Data","input":"&#x00c3;","inputUtf16":[38,35,120,48,48,99,51,59],"output":[{"Character":{"data":"Ã"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0239() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C4","initialState":"Data","input":"&#x00c4;","inputUtf16":[38,35,120,48,48,99,52,59],"output":[{"Character":{"data":"Ä"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0240() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C5","initialState":"Data","input":"&#x00c5;","inputUtf16":[38,35,120,48,48,99,53,59],"output":[{"Character":{"data":"Å"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0241() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C6","initialState":"Data","input":"&#x00c6;","inputUtf16":[38,35,120,48,48,99,54,59],"output":[{"Character":{"data":"Æ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0242() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C7","initialState":"Data","input":"&#x00c7;","inputUtf16":[38,35,120,48,48,99,55,59],"output":[{"Character":{"data":"Ç"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0243() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C8","initialState":"Data","input":"&#x00c8;","inputUtf16":[38,35,120,48,48,99,56,59],"output":[{"Character":{"data":"È"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0244() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00C9","initialState":"Data","input":"&#x00c9;","inputUtf16":[38,35,120,48,48,99,57,59],"output":[{"Character":{"data":"É"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0245() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00CA","initialState":"Data","input":"&#x00ca;","inputUtf16":[38,35,120,48,48,99,97,59],"output":[{"Character":{"data":"Ê"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0246() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00CB","initialState":"Data","input":"&#x00cb;","inputUtf16":[38,35,120,48,48,99,98,59],"output":[{"Character":{"data":"Ë"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0247() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00CC","initialState":"Data","input":"&#x00cc;","inputUtf16":[38,35,120,48,48,99,99,59],"output":[{"Character":{"data":"Ì"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0248() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00CD","initialState":"Data","input":"&#x00cd;","inputUtf16":[38,35,120,48,48,99,100,59],"output":[{"Character":{"data":"Í"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0249() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00CE","initialState":"Data","input":"&#x00ce;","inputUtf16":[38,35,120,48,48,99,101,59],"output":[{"Character":{"data":"Î"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0250() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00CF","initialState":"Data","input":"&#x00cf;","inputUtf16":[38,35,120,48,48,99,102,59],"output":[{"Character":{"data":"Ï"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0251() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D0","initialState":"Data","input":"&#x00d0;","inputUtf16":[38,35,120,48,48,100,48,59],"output":[{"Character":{"data":"Ð"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0252() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D1","initialState":"Data","input":"&#x00d1;","inputUtf16":[38,35,120,48,48,100,49,59],"output":[{"Character":{"data":"Ñ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0253() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D2","initialState":"Data","input":"&#x00d2;","inputUtf16":[38,35,120,48,48,100,50,59],"output":[{"Character":{"data":"Ò"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0254() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D3","initialState":"Data","input":"&#x00d3;","inputUtf16":[38,35,120,48,48,100,51,59],"output":[{"Character":{"data":"Ó"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0255() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D4","initialState":"Data","input":"&#x00d4;","inputUtf16":[38,35,120,48,48,100,52,59],"output":[{"Character":{"data":"Ô"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0256() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D5","initialState":"Data","input":"&#x00d5;","inputUtf16":[38,35,120,48,48,100,53,59],"output":[{"Character":{"data":"Õ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0257() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D6","initialState":"Data","input":"&#x00d6;","inputUtf16":[38,35,120,48,48,100,54,59],"output":[{"Character":{"data":"Ö"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0258() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D7","initialState":"Data","input":"&#x00d7;","inputUtf16":[38,35,120,48,48,100,55,59],"output":[{"Character":{"data":"×"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0259() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D8","initialState":"Data","input":"&#x00d8;","inputUtf16":[38,35,120,48,48,100,56,59],"output":[{"Character":{"data":"Ø"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0260() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00D9","initialState":"Data","input":"&#x00d9;","inputUtf16":[38,35,120,48,48,100,57,59],"output":[{"Character":{"data":"Ù"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0261() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00DA","initialState":"Data","input":"&#x00da;","inputUtf16":[38,35,120,48,48,100,97,59],"output":[{"Character":{"data":"Ú"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0262() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00DB","initialState":"Data","input":"&#x00db;","inputUtf16":[38,35,120,48,48,100,98,59],"output":[{"Character":{"data":"Û"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0263() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00DC","initialState":"Data","input":"&#x00dc;","inputUtf16":[38,35,120,48,48,100,99,59],"output":[{"Character":{"data":"Ü"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0264() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00DD","initialState":"Data","input":"&#x00dd;","inputUtf16":[38,35,120,48,48,100,100,59],"output":[{"Character":{"data":"Ý"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0265() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00DE","initialState":"Data","input":"&#x00de;","inputUtf16":[38,35,120,48,48,100,101,59],"output":[{"Character":{"data":"Þ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0266() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00DF","initialState":"Data","input":"&#x00df;","inputUtf16":[38,35,120,48,48,100,102,59],"output":[{"Character":{"data":"ß"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0267() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E0","initialState":"Data","input":"&#x00e0;","inputUtf16":[38,35,120,48,48,101,48,59],"output":[{"Character":{"data":"à"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0268() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E1","initialState":"Data","input":"&#x00e1;","inputUtf16":[38,35,120,48,48,101,49,59],"output":[{"Character":{"data":"á"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0269() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E2","initialState":"Data","input":"&#x00e2;","inputUtf16":[38,35,120,48,48,101,50,59],"output":[{"Character":{"data":"â"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0270() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E3","initialState":"Data","input":"&#x00e3;","inputUtf16":[38,35,120,48,48,101,51,59],"output":[{"Character":{"data":"ã"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0271() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E4","initialState":"Data","input":"&#x00e4;","inputUtf16":[38,35,120,48,48,101,52,59],"output":[{"Character":{"data":"ä"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0272() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E5","initialState":"Data","input":"&#x00e5;","inputUtf16":[38,35,120,48,48,101,53,59],"output":[{"Character":{"data":"å"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0273() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E6","initialState":"Data","input":"&#x00e6;","inputUtf16":[38,35,120,48,48,101,54,59],"output":[{"Character":{"data":"æ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0274() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E7","initialState":"Data","input":"&#x00e7;","inputUtf16":[38,35,120,48,48,101,55,59],"output":[{"Character":{"data":"ç"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0275() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E8","initialState":"Data","input":"&#x00e8;","inputUtf16":[38,35,120,48,48,101,56,59],"output":[{"Character":{"data":"è"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0276() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00E9","initialState":"Data","input":"&#x00e9;","inputUtf16":[38,35,120,48,48,101,57,59],"output":[{"Character":{"data":"é"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0277() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00EA","initialState":"Data","input":"&#x00ea;","inputUtf16":[38,35,120,48,48,101,97,59],"output":[{"Character":{"data":"ê"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0278() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00EB","initialState":"Data","input":"&#x00eb;","inputUtf16":[38,35,120,48,48,101,98,59],"output":[{"Character":{"data":"ë"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0279() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00EC","initialState":"Data","input":"&#x00ec;","inputUtf16":[38,35,120,48,48,101,99,59],"output":[{"Character":{"data":"ì"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0280() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00ED","initialState":"Data","input":"&#x00ed;","inputUtf16":[38,35,120,48,48,101,100,59],"output":[{"Character":{"data":"í"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0281() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00EE","initialState":"Data","input":"&#x00ee;","inputUtf16":[38,35,120,48,48,101,101,59],"output":[{"Character":{"data":"î"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0282() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00EF","initialState":"Data","input":"&#x00ef;","inputUtf16":[38,35,120,48,48,101,102,59],"output":[{"Character":{"data":"ï"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0283() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F0","initialState":"Data","input":"&#x00f0;","inputUtf16":[38,35,120,48,48,102,48,59],"output":[{"Character":{"data":"ð"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0284() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F1","initialState":"Data","input":"&#x00f1;","inputUtf16":[38,35,120,48,48,102,49,59],"output":[{"Character":{"data":"ñ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0285() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F2","initialState":"Data","input":"&#x00f2;","inputUtf16":[38,35,120,48,48,102,50,59],"output":[{"Character":{"data":"ò"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0286() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F3","initialState":"Data","input":"&#x00f3;","inputUtf16":[38,35,120,48,48,102,51,59],"output":[{"Character":{"data":"ó"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0287() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F4","initialState":"Data","input":"&#x00f4;","inputUtf16":[38,35,120,48,48,102,52,59],"output":[{"Character":{"data":"ô"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0288() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F5","initialState":"Data","input":"&#x00f5;","inputUtf16":[38,35,120,48,48,102,53,59],"output":[{"Character":{"data":"õ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0289() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F6","initialState":"Data","input":"&#x00f6;","inputUtf16":[38,35,120,48,48,102,54,59],"output":[{"Character":{"data":"ö"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0290() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F7","initialState":"Data","input":"&#x00f7;","inputUtf16":[38,35,120,48,48,102,55,59],"output":[{"Character":{"data":"÷"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0291() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F8","initialState":"Data","input":"&#x00f8;","inputUtf16":[38,35,120,48,48,102,56,59],"output":[{"Character":{"data":"ø"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0292() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00F9","initialState":"Data","input":"&#x00f9;","inputUtf16":[38,35,120,48,48,102,57,59],"output":[{"Character":{"data":"ù"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0293() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00FA","initialState":"Data","input":"&#x00fa;","inputUtf16":[38,35,120,48,48,102,97,59],"output":[{"Character":{"data":"ú"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0294() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00FB","initialState":"Data","input":"&#x00fb;","inputUtf16":[38,35,120,48,48,102,98,59],"output":[{"Character":{"data":"û"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0295() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00FC","initialState":"Data","input":"&#x00fc;","inputUtf16":[38,35,120,48,48,102,99,59],"output":[{"Character":{"data":"ü"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0296() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00FD","initialState":"Data","input":"&#x00fd;","inputUtf16":[38,35,120,48,48,102,100,59],"output":[{"Character":{"data":"ý"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0297() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00FE","initialState":"Data","input":"&#x00fe;","inputUtf16":[38,35,120,48,48,102,101,59],"output":[{"Character":{"data":"þ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0298() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+00FF","initialState":"Data","input":"&#x00ff;","inputUtf16":[38,35,120,48,48,102,102,59],"output":[{"Character":{"data":"ÿ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0299() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+D7FF","initialState":"Data","input":"&#xd7ff;","inputUtf16":[38,35,120,100,55,102,102,59],"output":[{"Character":{"data":"퟿"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0300() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+E000","initialState":"Data","input":"&#xe000;","inputUtf16":[38,35,120,101,48,48,48,59],"output":[{"Character":{"data":""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0301() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+FDCF","initialState":"Data","input":"&#xfdcf;","inputUtf16":[38,35,120,102,100,99,102,59],"output":[{"Character":{"data":"﷏"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0302() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+FDF0","initialState":"Data","input":"&#xfdf0;","inputUtf16":[38,35,120,102,100,102,48,59],"output":[{"Character":{"data":"ﷰ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0303() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+FFFD","initialState":"Data","input":"&#xfffd;","inputUtf16":[38,35,120,102,102,102,100,59],"output":[{"Character":{"data":"�"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0304() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+10000","initialState":"Data","input":"&#x10000;","inputUtf16":[38,35,120,49,48,48,48,48,59],"output":[{"Character":{"data":"𐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0305() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+1FFFD","initialState":"Data","input":"&#x1fffd;","inputUtf16":[38,35,120,49,102,102,102,100,59],"output":[{"Character":{"data":"🿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0306() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+20000","initialState":"Data","input":"&#x20000;","inputUtf16":[38,35,120,50,48,48,48,48,59],"output":[{"Character":{"data":"𠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0307() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+2FFFD","initialState":"Data","input":"&#x2fffd;","inputUtf16":[38,35,120,50,102,102,102,100,59],"output":[{"Character":{"data":"𯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0308() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+30000","initialState":"Data","input":"&#x30000;","inputUtf16":[38,35,120,51,48,48,48,48,59],"output":[{"Character":{"data":"𰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0309() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+3FFFD","initialState":"Data","input":"&#x3fffd;","inputUtf16":[38,35,120,51,102,102,102,100,59],"output":[{"Character":{"data":"𿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0310() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+40000","initialState":"Data","input":"&#x40000;","inputUtf16":[38,35,120,52,48,48,48,48,59],"output":[{"Character":{"data":"񀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0311() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+4FFFD","initialState":"Data","input":"&#x4fffd;","inputUtf16":[38,35,120,52,102,102,102,100,59],"output":[{"Character":{"data":"񏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0312() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+50000","initialState":"Data","input":"&#x50000;","inputUtf16":[38,35,120,53,48,48,48,48,59],"output":[{"Character":{"data":"񐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0313() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+5FFFD","initialState":"Data","input":"&#x5fffd;","inputUtf16":[38,35,120,53,102,102,102,100,59],"output":[{"Character":{"data":"񟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0314() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+60000","initialState":"Data","input":"&#x60000;","inputUtf16":[38,35,120,54,48,48,48,48,59],"output":[{"Character":{"data":"񠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0315() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+6FFFD","initialState":"Data","input":"&#x6fffd;","inputUtf16":[38,35,120,54,102,102,102,100,59],"output":[{"Character":{"data":"񯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0316() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+70000","initialState":"Data","input":"&#x70000;","inputUtf16":[38,35,120,55,48,48,48,48,59],"output":[{"Character":{"data":"񰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0317() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+7FFFD","initialState":"Data","input":"&#x7fffd;","inputUtf16":[38,35,120,55,102,102,102,100,59],"output":[{"Character":{"data":"񿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0318() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+80000","initialState":"Data","input":"&#x80000;","inputUtf16":[38,35,120,56,48,48,48,48,59],"output":[{"Character":{"data":"򀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0319() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+8FFFD","initialState":"Data","input":"&#x8fffd;","inputUtf16":[38,35,120,56,102,102,102,100,59],"output":[{"Character":{"data":"򏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0320() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+90000","initialState":"Data","input":"&#x90000;","inputUtf16":[38,35,120,57,48,48,48,48,59],"output":[{"Character":{"data":"򐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0321() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+9FFFD","initialState":"Data","input":"&#x9fffd;","inputUtf16":[38,35,120,57,102,102,102,100,59],"output":[{"Character":{"data":"򟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0322() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+A0000","initialState":"Data","input":"&#xa0000;","inputUtf16":[38,35,120,97,48,48,48,48,59],"output":[{"Character":{"data":"򠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0323() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+AFFFD","initialState":"Data","input":"&#xafffd;","inputUtf16":[38,35,120,97,102,102,102,100,59],"output":[{"Character":{"data":"򯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0324() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+B0000","initialState":"Data","input":"&#xb0000;","inputUtf16":[38,35,120,98,48,48,48,48,59],"output":[{"Character":{"data":"򰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0325() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+BFFFD","initialState":"Data","input":"&#xbfffd;","inputUtf16":[38,35,120,98,102,102,102,100,59],"output":[{"Character":{"data":"򿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0326() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+C0000","initialState":"Data","input":"&#xc0000;","inputUtf16":[38,35,120,99,48,48,48,48,59],"output":[{"Character":{"data":"󀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0327() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+CFFFD","initialState":"Data","input":"&#xcfffd;","inputUtf16":[38,35,120,99,102,102,102,100,59],"output":[{"Character":{"data":"󏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0328() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+D0000","initialState":"Data","input":"&#xd0000;","inputUtf16":[38,35,120,100,48,48,48,48,59],"output":[{"Character":{"data":"󐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0329() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+DFFFD","initialState":"Data","input":"&#xdfffd;","inputUtf16":[38,35,120,100,102,102,102,100,59],"output":[{"Character":{"data":"󟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0330() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+E0000","initialState":"Data","input":"&#xe0000;","inputUtf16":[38,35,120,101,48,48,48,48,59],"output":[{"Character":{"data":"󠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0331() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+EFFFD","initialState":"Data","input":"&#xefffd;","inputUtf16":[38,35,120,101,102,102,102,100,59],"output":[{"Character":{"data":"󯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0332() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+F0000","initialState":"Data","input":"&#xf0000;","inputUtf16":[38,35,120,102,48,48,48,48,59],"output":[{"Character":{"data":"󰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0333() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+FFFFD","initialState":"Data","input":"&#xffffd;","inputUtf16":[38,35,120,102,102,102,102,100,59],"output":[{"Character":{"data":"󿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0334() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+100000","initialState":"Data","input":"&#x100000;","inputUtf16":[38,35,120,49,48,48,48,48,48,59],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0335() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid numeric entity character U+10FFFD","initialState":"Data","input":"&#x10fffd;","inputUtf16":[38,35,120,49,48,102,102,102,100,59],"output":[{"Character":{"data":"􏿽"}}],"errors":[]}"##,
    );
}
