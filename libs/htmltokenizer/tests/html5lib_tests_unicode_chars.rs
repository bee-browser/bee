mod helper;

use helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0001","initialState":"Data","input":"\u0001","inputUtf16":[1],"output":[{"Character":{"data":"\u0001"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0001() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0002","initialState":"Data","input":"\u0002","inputUtf16":[2],"output":[{"Character":{"data":"\u0002"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0002() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0003","initialState":"Data","input":"\u0003","inputUtf16":[3],"output":[{"Character":{"data":"\u0003"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0003() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0004","initialState":"Data","input":"\u0004","inputUtf16":[4],"output":[{"Character":{"data":"\u0004"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0004() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0005","initialState":"Data","input":"\u0005","inputUtf16":[5],"output":[{"Character":{"data":"\u0005"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0005() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0006","initialState":"Data","input":"\u0006","inputUtf16":[6],"output":[{"Character":{"data":"\u0006"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0006() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0007","initialState":"Data","input":"\u0007","inputUtf16":[7],"output":[{"Character":{"data":"\u0007"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0007() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0008","initialState":"Data","input":"\b","inputUtf16":[8],"output":[{"Character":{"data":"\b"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0008() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+000B","initialState":"Data","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0009() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+000E","initialState":"Data","input":"\u000e","inputUtf16":[14],"output":[{"Character":{"data":"\u000e"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0010() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+000F","initialState":"Data","input":"\u000f","inputUtf16":[15],"output":[{"Character":{"data":"\u000f"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0011() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0010","initialState":"Data","input":"\u0010","inputUtf16":[16],"output":[{"Character":{"data":"\u0010"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0012() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0011","initialState":"Data","input":"\u0011","inputUtf16":[17],"output":[{"Character":{"data":"\u0011"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0013() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0012","initialState":"Data","input":"\u0012","inputUtf16":[18],"output":[{"Character":{"data":"\u0012"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0014() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0013","initialState":"Data","input":"\u0013","inputUtf16":[19],"output":[{"Character":{"data":"\u0013"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0015() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0014","initialState":"Data","input":"\u0014","inputUtf16":[20],"output":[{"Character":{"data":"\u0014"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0016() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0015","initialState":"Data","input":"\u0015","inputUtf16":[21],"output":[{"Character":{"data":"\u0015"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0017() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0016","initialState":"Data","input":"\u0016","inputUtf16":[22],"output":[{"Character":{"data":"\u0016"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0018() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0017","initialState":"Data","input":"\u0017","inputUtf16":[23],"output":[{"Character":{"data":"\u0017"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0019() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0018","initialState":"Data","input":"\u0018","inputUtf16":[24],"output":[{"Character":{"data":"\u0018"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0020() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+0019","initialState":"Data","input":"\u0019","inputUtf16":[25],"output":[{"Character":{"data":"\u0019"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0021() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+001A","initialState":"Data","input":"\u001a","inputUtf16":[26],"output":[{"Character":{"data":"\u001a"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0022() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+001B","initialState":"Data","input":"\u001b","inputUtf16":[27],"output":[{"Character":{"data":"\u001b"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0023() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+001C","initialState":"Data","input":"\u001c","inputUtf16":[28],"output":[{"Character":{"data":"\u001c"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0024() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+001D","initialState":"Data","input":"\u001d","inputUtf16":[29],"output":[{"Character":{"data":"\u001d"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0025() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+001E","initialState":"Data","input":"\u001e","inputUtf16":[30],"output":[{"Character":{"data":"\u001e"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0026() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+001F","initialState":"Data","input":"\u001f","inputUtf16":[31],"output":[{"Character":{"data":"\u001f"}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0027() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+007F","initialState":"Data","input":"","inputUtf16":[127],"output":[{"Character":{"data":""}}],"errors":[{"code":"control-character-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0028() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD0","initialState":"Data","input":"﷐","inputUtf16":[64976],"output":[{"Character":{"data":"﷐"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0029() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD1","initialState":"Data","input":"﷑","inputUtf16":[64977],"output":[{"Character":{"data":"﷑"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0030() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD2","initialState":"Data","input":"﷒","inputUtf16":[64978],"output":[{"Character":{"data":"﷒"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0031() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD3","initialState":"Data","input":"﷓","inputUtf16":[64979],"output":[{"Character":{"data":"﷓"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0032() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD4","initialState":"Data","input":"﷔","inputUtf16":[64980],"output":[{"Character":{"data":"﷔"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0033() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD5","initialState":"Data","input":"﷕","inputUtf16":[64981],"output":[{"Character":{"data":"﷕"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0034() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD6","initialState":"Data","input":"﷖","inputUtf16":[64982],"output":[{"Character":{"data":"﷖"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0035() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD7","initialState":"Data","input":"﷗","inputUtf16":[64983],"output":[{"Character":{"data":"﷗"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0036() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD8","initialState":"Data","input":"﷘","inputUtf16":[64984],"output":[{"Character":{"data":"﷘"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0037() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDD9","initialState":"Data","input":"﷙","inputUtf16":[64985],"output":[{"Character":{"data":"﷙"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0038() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDDA","initialState":"Data","input":"﷚","inputUtf16":[64986],"output":[{"Character":{"data":"﷚"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0039() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDDB","initialState":"Data","input":"﷛","inputUtf16":[64987],"output":[{"Character":{"data":"﷛"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0040() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDDC","initialState":"Data","input":"﷜","inputUtf16":[64988],"output":[{"Character":{"data":"﷜"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0041() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDDD","initialState":"Data","input":"﷝","inputUtf16":[64989],"output":[{"Character":{"data":"﷝"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0042() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDDE","initialState":"Data","input":"﷞","inputUtf16":[64990],"output":[{"Character":{"data":"﷞"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0043() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDDF","initialState":"Data","input":"﷟","inputUtf16":[64991],"output":[{"Character":{"data":"﷟"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0044() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE0","initialState":"Data","input":"﷠","inputUtf16":[64992],"output":[{"Character":{"data":"﷠"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0045() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE1","initialState":"Data","input":"﷡","inputUtf16":[64993],"output":[{"Character":{"data":"﷡"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0046() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE2","initialState":"Data","input":"﷢","inputUtf16":[64994],"output":[{"Character":{"data":"﷢"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0047() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE3","initialState":"Data","input":"﷣","inputUtf16":[64995],"output":[{"Character":{"data":"﷣"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0048() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE4","initialState":"Data","input":"﷤","inputUtf16":[64996],"output":[{"Character":{"data":"﷤"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0049() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE5","initialState":"Data","input":"﷥","inputUtf16":[64997],"output":[{"Character":{"data":"﷥"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0050() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE6","initialState":"Data","input":"﷦","inputUtf16":[64998],"output":[{"Character":{"data":"﷦"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0051() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE7","initialState":"Data","input":"﷧","inputUtf16":[64999],"output":[{"Character":{"data":"﷧"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0052() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE8","initialState":"Data","input":"﷨","inputUtf16":[65000],"output":[{"Character":{"data":"﷨"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0053() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDE9","initialState":"Data","input":"﷩","inputUtf16":[65001],"output":[{"Character":{"data":"﷩"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0054() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDEA","initialState":"Data","input":"﷪","inputUtf16":[65002],"output":[{"Character":{"data":"﷪"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0055() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDEB","initialState":"Data","input":"﷫","inputUtf16":[65003],"output":[{"Character":{"data":"﷫"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0056() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDEC","initialState":"Data","input":"﷬","inputUtf16":[65004],"output":[{"Character":{"data":"﷬"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0057() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDED","initialState":"Data","input":"﷭","inputUtf16":[65005],"output":[{"Character":{"data":"﷭"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0058() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDEE","initialState":"Data","input":"﷮","inputUtf16":[65006],"output":[{"Character":{"data":"﷮"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0059() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FDEF","initialState":"Data","input":"﷯","inputUtf16":[65007],"output":[{"Character":{"data":"﷯"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0060() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FFFE","initialState":"Data","input":"￾","inputUtf16":[65534],"output":[{"Character":{"data":"￾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0061() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FFFF","initialState":"Data","input":"￿","inputUtf16":[65535],"output":[{"Character":{"data":"￿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0062() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+1FFFE","initialState":"Data","input":"🿾","inputUtf16":[55359,57342],"output":[{"Character":{"data":"🿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0063() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+1FFFF","initialState":"Data","input":"🿿","inputUtf16":[55359,57343],"output":[{"Character":{"data":"🿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0064() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+2FFFE","initialState":"Data","input":"𯿾","inputUtf16":[55423,57342],"output":[{"Character":{"data":"𯿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0065() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+2FFFF","initialState":"Data","input":"𯿿","inputUtf16":[55423,57343],"output":[{"Character":{"data":"𯿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0066() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+3FFFE","initialState":"Data","input":"𿿾","inputUtf16":[55487,57342],"output":[{"Character":{"data":"𿿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0067() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+3FFFF","initialState":"Data","input":"𿿿","inputUtf16":[55487,57343],"output":[{"Character":{"data":"𿿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0068() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+4FFFE","initialState":"Data","input":"񏿾","inputUtf16":[55551,57342],"output":[{"Character":{"data":"񏿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0069() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+4FFFF","initialState":"Data","input":"񏿿","inputUtf16":[55551,57343],"output":[{"Character":{"data":"񏿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0070() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+5FFFE","initialState":"Data","input":"񟿾","inputUtf16":[55615,57342],"output":[{"Character":{"data":"񟿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0071() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+5FFFF","initialState":"Data","input":"񟿿","inputUtf16":[55615,57343],"output":[{"Character":{"data":"񟿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0072() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+6FFFE","initialState":"Data","input":"񯿾","inputUtf16":[55679,57342],"output":[{"Character":{"data":"񯿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0073() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+6FFFF","initialState":"Data","input":"񯿿","inputUtf16":[55679,57343],"output":[{"Character":{"data":"񯿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0074() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+7FFFE","initialState":"Data","input":"񿿾","inputUtf16":[55743,57342],"output":[{"Character":{"data":"񿿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0075() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+7FFFF","initialState":"Data","input":"񿿿","inputUtf16":[55743,57343],"output":[{"Character":{"data":"񿿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0076() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+8FFFE","initialState":"Data","input":"򏿾","inputUtf16":[55807,57342],"output":[{"Character":{"data":"򏿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0077() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+8FFFF","initialState":"Data","input":"򏿿","inputUtf16":[55807,57343],"output":[{"Character":{"data":"򏿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0078() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+9FFFE","initialState":"Data","input":"򟿾","inputUtf16":[55871,57342],"output":[{"Character":{"data":"򟿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0079() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+9FFFF","initialState":"Data","input":"򟿿","inputUtf16":[55871,57343],"output":[{"Character":{"data":"򟿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0080() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+AFFFE","initialState":"Data","input":"򯿾","inputUtf16":[55935,57342],"output":[{"Character":{"data":"򯿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0081() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+AFFFF","initialState":"Data","input":"򯿿","inputUtf16":[55935,57343],"output":[{"Character":{"data":"򯿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0082() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+BFFFE","initialState":"Data","input":"򿿾","inputUtf16":[55999,57342],"output":[{"Character":{"data":"򿿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0083() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+BFFFF","initialState":"Data","input":"򿿿","inputUtf16":[55999,57343],"output":[{"Character":{"data":"򿿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0084() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+CFFFE","initialState":"Data","input":"󏿾","inputUtf16":[56063,57342],"output":[{"Character":{"data":"󏿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0085() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+CFFFF","initialState":"Data","input":"󏿿","inputUtf16":[56063,57343],"output":[{"Character":{"data":"󏿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0086() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+DFFFE","initialState":"Data","input":"󟿾","inputUtf16":[56127,57342],"output":[{"Character":{"data":"󟿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0087() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+DFFFF","initialState":"Data","input":"󟿿","inputUtf16":[56127,57343],"output":[{"Character":{"data":"󟿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0088() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+EFFFE","initialState":"Data","input":"󯿾","inputUtf16":[56191,57342],"output":[{"Character":{"data":"󯿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0089() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+EFFFF","initialState":"Data","input":"󯿿","inputUtf16":[56191,57343],"output":[{"Character":{"data":"󯿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0090() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FFFFE","initialState":"Data","input":"󿿾","inputUtf16":[56255,57342],"output":[{"Character":{"data":"󿿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0091() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+FFFFF","initialState":"Data","input":"󿿿","inputUtf16":[56255,57343],"output":[{"Character":{"data":"󿿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0092() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+10FFFE","initialState":"Data","input":"􏿾","inputUtf16":[56319,57342],"output":[{"Character":{"data":"􏿾"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0093() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Invalid Unicode character U+10FFFF","initialState":"Data","input":"􏿿","inputUtf16":[56319,57343],"output":[{"Character":{"data":"􏿿"}}],"errors":[{"code":"noncharacter-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0094() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0009","initialState":"Data","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0095() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+000A","initialState":"Data","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0096() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0020","initialState":"Data","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0097() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0021","initialState":"Data","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0098() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0022","initialState":"Data","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0099() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0023","initialState":"Data","input":"#","inputUtf16":[35],"output":[{"Character":{"data":"#"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0100() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0024","initialState":"Data","input":"$","inputUtf16":[36],"output":[{"Character":{"data":"$"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0101() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0025","initialState":"Data","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0102() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0026","initialState":"Data","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0103() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0027","initialState":"Data","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0104() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0028","initialState":"Data","input":"(","inputUtf16":[40],"output":[{"Character":{"data":"("}}],"errors":[]}"##,
    );
}

#[test]
fn test_0105() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0029","initialState":"Data","input":")","inputUtf16":[41],"output":[{"Character":{"data":")"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0106() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+002A","initialState":"Data","input":"*","inputUtf16":[42],"output":[{"Character":{"data":"*"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0107() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+002B","initialState":"Data","input":"+","inputUtf16":[43],"output":[{"Character":{"data":"+"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0108() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+002C","initialState":"Data","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_0109() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+002D","initialState":"Data","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0110() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+002E","initialState":"Data","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_0111() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+002F","initialState":"Data","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0112() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0030","initialState":"Data","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0113() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0031","initialState":"Data","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0114() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0032","initialState":"Data","input":"2","inputUtf16":[50],"output":[{"Character":{"data":"2"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0115() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0033","initialState":"Data","input":"3","inputUtf16":[51],"output":[{"Character":{"data":"3"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0116() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0034","initialState":"Data","input":"4","inputUtf16":[52],"output":[{"Character":{"data":"4"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0117() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0035","initialState":"Data","input":"5","inputUtf16":[53],"output":[{"Character":{"data":"5"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0118() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0036","initialState":"Data","input":"6","inputUtf16":[54],"output":[{"Character":{"data":"6"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0119() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0037","initialState":"Data","input":"7","inputUtf16":[55],"output":[{"Character":{"data":"7"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0120() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0038","initialState":"Data","input":"8","inputUtf16":[56],"output":[{"Character":{"data":"8"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0121() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0039","initialState":"Data","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0122() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+003A","initialState":"Data","input":":","inputUtf16":[58],"output":[{"Character":{"data":":"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0123() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+003B","initialState":"Data","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0124() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+003D","initialState":"Data","input":"=","inputUtf16":[61],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_0125() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+003E","initialState":"Data","input":">","inputUtf16":[62],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0126() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+003F","initialState":"Data","input":"?","inputUtf16":[63],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0127() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0040","initialState":"Data","input":"@","inputUtf16":[64],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0128() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0041","initialState":"Data","input":"A","inputUtf16":[65],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0129() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0042","initialState":"Data","input":"B","inputUtf16":[66],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0130() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0043","initialState":"Data","input":"C","inputUtf16":[67],"output":[{"Character":{"data":"C"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0131() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0044","initialState":"Data","input":"D","inputUtf16":[68],"output":[{"Character":{"data":"D"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0132() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0045","initialState":"Data","input":"E","inputUtf16":[69],"output":[{"Character":{"data":"E"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0133() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0046","initialState":"Data","input":"F","inputUtf16":[70],"output":[{"Character":{"data":"F"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0134() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0047","initialState":"Data","input":"G","inputUtf16":[71],"output":[{"Character":{"data":"G"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0135() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0048","initialState":"Data","input":"H","inputUtf16":[72],"output":[{"Character":{"data":"H"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0136() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0049","initialState":"Data","input":"I","inputUtf16":[73],"output":[{"Character":{"data":"I"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0137() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+004A","initialState":"Data","input":"J","inputUtf16":[74],"output":[{"Character":{"data":"J"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0138() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+004B","initialState":"Data","input":"K","inputUtf16":[75],"output":[{"Character":{"data":"K"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0139() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+004C","initialState":"Data","input":"L","inputUtf16":[76],"output":[{"Character":{"data":"L"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0140() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+004D","initialState":"Data","input":"M","inputUtf16":[77],"output":[{"Character":{"data":"M"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0141() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+004E","initialState":"Data","input":"N","inputUtf16":[78],"output":[{"Character":{"data":"N"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0142() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+004F","initialState":"Data","input":"O","inputUtf16":[79],"output":[{"Character":{"data":"O"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0143() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0050","initialState":"Data","input":"P","inputUtf16":[80],"output":[{"Character":{"data":"P"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0144() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0051","initialState":"Data","input":"Q","inputUtf16":[81],"output":[{"Character":{"data":"Q"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0145() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0052","initialState":"Data","input":"R","inputUtf16":[82],"output":[{"Character":{"data":"R"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0146() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0053","initialState":"Data","input":"S","inputUtf16":[83],"output":[{"Character":{"data":"S"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0147() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0054","initialState":"Data","input":"T","inputUtf16":[84],"output":[{"Character":{"data":"T"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0148() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0055","initialState":"Data","input":"U","inputUtf16":[85],"output":[{"Character":{"data":"U"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0149() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0056","initialState":"Data","input":"V","inputUtf16":[86],"output":[{"Character":{"data":"V"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0150() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0057","initialState":"Data","input":"W","inputUtf16":[87],"output":[{"Character":{"data":"W"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0151() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0058","initialState":"Data","input":"X","inputUtf16":[88],"output":[{"Character":{"data":"X"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0152() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0059","initialState":"Data","input":"Y","inputUtf16":[89],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0153() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+005A","initialState":"Data","input":"Z","inputUtf16":[90],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0154() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+005B","initialState":"Data","input":"[","inputUtf16":[91],"output":[{"Character":{"data":"["}}],"errors":[]}"##,
    );
}

#[test]
fn test_0155() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+005C","initialState":"Data","input":"\\","inputUtf16":[92],"output":[{"Character":{"data":"\\"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0156() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+005D","initialState":"Data","input":"]","inputUtf16":[93],"output":[{"Character":{"data":"]"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0157() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+005E","initialState":"Data","input":"^","inputUtf16":[94],"output":[{"Character":{"data":"^"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0158() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+005F","initialState":"Data","input":"_","inputUtf16":[95],"output":[{"Character":{"data":"_"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0159() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0060","initialState":"Data","input":"`","inputUtf16":[96],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0160() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0061","initialState":"Data","input":"a","inputUtf16":[97],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0161() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0062","initialState":"Data","input":"b","inputUtf16":[98],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0162() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0063","initialState":"Data","input":"c","inputUtf16":[99],"output":[{"Character":{"data":"c"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0163() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0064","initialState":"Data","input":"d","inputUtf16":[100],"output":[{"Character":{"data":"d"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0164() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0065","initialState":"Data","input":"e","inputUtf16":[101],"output":[{"Character":{"data":"e"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0165() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0066","initialState":"Data","input":"f","inputUtf16":[102],"output":[{"Character":{"data":"f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0166() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0067","initialState":"Data","input":"g","inputUtf16":[103],"output":[{"Character":{"data":"g"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0167() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0068","initialState":"Data","input":"h","inputUtf16":[104],"output":[{"Character":{"data":"h"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0168() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0069","initialState":"Data","input":"i","inputUtf16":[105],"output":[{"Character":{"data":"i"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0169() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+006A","initialState":"Data","input":"j","inputUtf16":[106],"output":[{"Character":{"data":"j"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0170() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+006B","initialState":"Data","input":"k","inputUtf16":[107],"output":[{"Character":{"data":"k"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0171() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+006C","initialState":"Data","input":"l","inputUtf16":[108],"output":[{"Character":{"data":"l"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0172() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+006D","initialState":"Data","input":"m","inputUtf16":[109],"output":[{"Character":{"data":"m"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0173() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+006E","initialState":"Data","input":"n","inputUtf16":[110],"output":[{"Character":{"data":"n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0174() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+006F","initialState":"Data","input":"o","inputUtf16":[111],"output":[{"Character":{"data":"o"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0175() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0070","initialState":"Data","input":"p","inputUtf16":[112],"output":[{"Character":{"data":"p"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0176() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0071","initialState":"Data","input":"q","inputUtf16":[113],"output":[{"Character":{"data":"q"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0177() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0072","initialState":"Data","input":"r","inputUtf16":[114],"output":[{"Character":{"data":"r"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0178() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0073","initialState":"Data","input":"s","inputUtf16":[115],"output":[{"Character":{"data":"s"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0179() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0074","initialState":"Data","input":"t","inputUtf16":[116],"output":[{"Character":{"data":"t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0180() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0075","initialState":"Data","input":"u","inputUtf16":[117],"output":[{"Character":{"data":"u"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0181() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0076","initialState":"Data","input":"v","inputUtf16":[118],"output":[{"Character":{"data":"v"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0182() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0077","initialState":"Data","input":"w","inputUtf16":[119],"output":[{"Character":{"data":"w"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0183() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0078","initialState":"Data","input":"x","inputUtf16":[120],"output":[{"Character":{"data":"x"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0184() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+0079","initialState":"Data","input":"y","inputUtf16":[121],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0185() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+007A","initialState":"Data","input":"z","inputUtf16":[122],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0186() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+007B","initialState":"Data","input":"{","inputUtf16":[123],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0187() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+007C","initialState":"Data","input":"|","inputUtf16":[124],"output":[{"Character":{"data":"|"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0188() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+007D","initialState":"Data","input":"}","inputUtf16":[125],"output":[{"Character":{"data":"}"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0189() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+007E","initialState":"Data","input":"~","inputUtf16":[126],"output":[{"Character":{"data":"~"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0190() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A0","initialState":"Data","input":" ","inputUtf16":[160],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0191() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A1","initialState":"Data","input":"¡","inputUtf16":[161],"output":[{"Character":{"data":"¡"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0192() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A2","initialState":"Data","input":"¢","inputUtf16":[162],"output":[{"Character":{"data":"¢"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0193() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A3","initialState":"Data","input":"£","inputUtf16":[163],"output":[{"Character":{"data":"£"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0194() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A4","initialState":"Data","input":"¤","inputUtf16":[164],"output":[{"Character":{"data":"¤"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0195() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A5","initialState":"Data","input":"¥","inputUtf16":[165],"output":[{"Character":{"data":"¥"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0196() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A6","initialState":"Data","input":"¦","inputUtf16":[166],"output":[{"Character":{"data":"¦"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0197() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A7","initialState":"Data","input":"§","inputUtf16":[167],"output":[{"Character":{"data":"§"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0198() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A8","initialState":"Data","input":"¨","inputUtf16":[168],"output":[{"Character":{"data":"¨"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0199() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00A9","initialState":"Data","input":"©","inputUtf16":[169],"output":[{"Character":{"data":"©"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0200() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00AA","initialState":"Data","input":"ª","inputUtf16":[170],"output":[{"Character":{"data":"ª"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0201() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00AB","initialState":"Data","input":"«","inputUtf16":[171],"output":[{"Character":{"data":"«"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0202() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00AC","initialState":"Data","input":"¬","inputUtf16":[172],"output":[{"Character":{"data":"¬"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0203() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00AD","initialState":"Data","input":"­","inputUtf16":[173],"output":[{"Character":{"data":"­"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0204() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00AE","initialState":"Data","input":"®","inputUtf16":[174],"output":[{"Character":{"data":"®"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0205() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00AF","initialState":"Data","input":"¯","inputUtf16":[175],"output":[{"Character":{"data":"¯"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0206() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B0","initialState":"Data","input":"°","inputUtf16":[176],"output":[{"Character":{"data":"°"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0207() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B1","initialState":"Data","input":"±","inputUtf16":[177],"output":[{"Character":{"data":"±"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0208() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B2","initialState":"Data","input":"²","inputUtf16":[178],"output":[{"Character":{"data":"²"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0209() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B3","initialState":"Data","input":"³","inputUtf16":[179],"output":[{"Character":{"data":"³"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0210() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B4","initialState":"Data","input":"´","inputUtf16":[180],"output":[{"Character":{"data":"´"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0211() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B5","initialState":"Data","input":"µ","inputUtf16":[181],"output":[{"Character":{"data":"µ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0212() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B6","initialState":"Data","input":"¶","inputUtf16":[182],"output":[{"Character":{"data":"¶"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0213() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B7","initialState":"Data","input":"·","inputUtf16":[183],"output":[{"Character":{"data":"·"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0214() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B8","initialState":"Data","input":"¸","inputUtf16":[184],"output":[{"Character":{"data":"¸"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0215() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00B9","initialState":"Data","input":"¹","inputUtf16":[185],"output":[{"Character":{"data":"¹"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0216() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00BA","initialState":"Data","input":"º","inputUtf16":[186],"output":[{"Character":{"data":"º"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0217() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00BB","initialState":"Data","input":"»","inputUtf16":[187],"output":[{"Character":{"data":"»"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0218() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00BC","initialState":"Data","input":"¼","inputUtf16":[188],"output":[{"Character":{"data":"¼"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0219() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00BD","initialState":"Data","input":"½","inputUtf16":[189],"output":[{"Character":{"data":"½"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0220() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00BE","initialState":"Data","input":"¾","inputUtf16":[190],"output":[{"Character":{"data":"¾"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0221() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00BF","initialState":"Data","input":"¿","inputUtf16":[191],"output":[{"Character":{"data":"¿"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0222() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C0","initialState":"Data","input":"À","inputUtf16":[192],"output":[{"Character":{"data":"À"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0223() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C1","initialState":"Data","input":"Á","inputUtf16":[193],"output":[{"Character":{"data":"Á"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0224() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C2","initialState":"Data","input":"Â","inputUtf16":[194],"output":[{"Character":{"data":"Â"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0225() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C3","initialState":"Data","input":"Ã","inputUtf16":[195],"output":[{"Character":{"data":"Ã"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0226() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C4","initialState":"Data","input":"Ä","inputUtf16":[196],"output":[{"Character":{"data":"Ä"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0227() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C5","initialState":"Data","input":"Å","inputUtf16":[197],"output":[{"Character":{"data":"Å"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0228() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C6","initialState":"Data","input":"Æ","inputUtf16":[198],"output":[{"Character":{"data":"Æ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0229() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C7","initialState":"Data","input":"Ç","inputUtf16":[199],"output":[{"Character":{"data":"Ç"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0230() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C8","initialState":"Data","input":"È","inputUtf16":[200],"output":[{"Character":{"data":"È"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0231() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00C9","initialState":"Data","input":"É","inputUtf16":[201],"output":[{"Character":{"data":"É"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0232() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00CA","initialState":"Data","input":"Ê","inputUtf16":[202],"output":[{"Character":{"data":"Ê"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0233() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00CB","initialState":"Data","input":"Ë","inputUtf16":[203],"output":[{"Character":{"data":"Ë"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0234() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00CC","initialState":"Data","input":"Ì","inputUtf16":[204],"output":[{"Character":{"data":"Ì"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0235() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00CD","initialState":"Data","input":"Í","inputUtf16":[205],"output":[{"Character":{"data":"Í"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0236() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00CE","initialState":"Data","input":"Î","inputUtf16":[206],"output":[{"Character":{"data":"Î"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0237() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00CF","initialState":"Data","input":"Ï","inputUtf16":[207],"output":[{"Character":{"data":"Ï"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0238() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D0","initialState":"Data","input":"Ð","inputUtf16":[208],"output":[{"Character":{"data":"Ð"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0239() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D1","initialState":"Data","input":"Ñ","inputUtf16":[209],"output":[{"Character":{"data":"Ñ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0240() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D2","initialState":"Data","input":"Ò","inputUtf16":[210],"output":[{"Character":{"data":"Ò"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0241() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D3","initialState":"Data","input":"Ó","inputUtf16":[211],"output":[{"Character":{"data":"Ó"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0242() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D4","initialState":"Data","input":"Ô","inputUtf16":[212],"output":[{"Character":{"data":"Ô"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0243() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D5","initialState":"Data","input":"Õ","inputUtf16":[213],"output":[{"Character":{"data":"Õ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0244() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D6","initialState":"Data","input":"Ö","inputUtf16":[214],"output":[{"Character":{"data":"Ö"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0245() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D7","initialState":"Data","input":"×","inputUtf16":[215],"output":[{"Character":{"data":"×"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0246() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D8","initialState":"Data","input":"Ø","inputUtf16":[216],"output":[{"Character":{"data":"Ø"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0247() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00D9","initialState":"Data","input":"Ù","inputUtf16":[217],"output":[{"Character":{"data":"Ù"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0248() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00DA","initialState":"Data","input":"Ú","inputUtf16":[218],"output":[{"Character":{"data":"Ú"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0249() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00DB","initialState":"Data","input":"Û","inputUtf16":[219],"output":[{"Character":{"data":"Û"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0250() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00DC","initialState":"Data","input":"Ü","inputUtf16":[220],"output":[{"Character":{"data":"Ü"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0251() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00DD","initialState":"Data","input":"Ý","inputUtf16":[221],"output":[{"Character":{"data":"Ý"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0252() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00DE","initialState":"Data","input":"Þ","inputUtf16":[222],"output":[{"Character":{"data":"Þ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0253() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00DF","initialState":"Data","input":"ß","inputUtf16":[223],"output":[{"Character":{"data":"ß"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0254() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E0","initialState":"Data","input":"à","inputUtf16":[224],"output":[{"Character":{"data":"à"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0255() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E1","initialState":"Data","input":"á","inputUtf16":[225],"output":[{"Character":{"data":"á"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0256() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E2","initialState":"Data","input":"â","inputUtf16":[226],"output":[{"Character":{"data":"â"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0257() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E3","initialState":"Data","input":"ã","inputUtf16":[227],"output":[{"Character":{"data":"ã"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0258() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E4","initialState":"Data","input":"ä","inputUtf16":[228],"output":[{"Character":{"data":"ä"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0259() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E5","initialState":"Data","input":"å","inputUtf16":[229],"output":[{"Character":{"data":"å"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0260() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E6","initialState":"Data","input":"æ","inputUtf16":[230],"output":[{"Character":{"data":"æ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0261() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E7","initialState":"Data","input":"ç","inputUtf16":[231],"output":[{"Character":{"data":"ç"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0262() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E8","initialState":"Data","input":"è","inputUtf16":[232],"output":[{"Character":{"data":"è"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0263() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00E9","initialState":"Data","input":"é","inputUtf16":[233],"output":[{"Character":{"data":"é"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0264() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00EA","initialState":"Data","input":"ê","inputUtf16":[234],"output":[{"Character":{"data":"ê"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0265() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00EB","initialState":"Data","input":"ë","inputUtf16":[235],"output":[{"Character":{"data":"ë"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0266() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00EC","initialState":"Data","input":"ì","inputUtf16":[236],"output":[{"Character":{"data":"ì"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0267() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00ED","initialState":"Data","input":"í","inputUtf16":[237],"output":[{"Character":{"data":"í"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0268() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00EE","initialState":"Data","input":"î","inputUtf16":[238],"output":[{"Character":{"data":"î"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0269() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00EF","initialState":"Data","input":"ï","inputUtf16":[239],"output":[{"Character":{"data":"ï"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0270() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F0","initialState":"Data","input":"ð","inputUtf16":[240],"output":[{"Character":{"data":"ð"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0271() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F1","initialState":"Data","input":"ñ","inputUtf16":[241],"output":[{"Character":{"data":"ñ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0272() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F2","initialState":"Data","input":"ò","inputUtf16":[242],"output":[{"Character":{"data":"ò"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0273() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F3","initialState":"Data","input":"ó","inputUtf16":[243],"output":[{"Character":{"data":"ó"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0274() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F4","initialState":"Data","input":"ô","inputUtf16":[244],"output":[{"Character":{"data":"ô"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0275() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F5","initialState":"Data","input":"õ","inputUtf16":[245],"output":[{"Character":{"data":"õ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0276() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F6","initialState":"Data","input":"ö","inputUtf16":[246],"output":[{"Character":{"data":"ö"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0277() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F7","initialState":"Data","input":"÷","inputUtf16":[247],"output":[{"Character":{"data":"÷"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0278() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F8","initialState":"Data","input":"ø","inputUtf16":[248],"output":[{"Character":{"data":"ø"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0279() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00F9","initialState":"Data","input":"ù","inputUtf16":[249],"output":[{"Character":{"data":"ù"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0280() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00FA","initialState":"Data","input":"ú","inputUtf16":[250],"output":[{"Character":{"data":"ú"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0281() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00FB","initialState":"Data","input":"û","inputUtf16":[251],"output":[{"Character":{"data":"û"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0282() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00FC","initialState":"Data","input":"ü","inputUtf16":[252],"output":[{"Character":{"data":"ü"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0283() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00FD","initialState":"Data","input":"ý","inputUtf16":[253],"output":[{"Character":{"data":"ý"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0284() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00FE","initialState":"Data","input":"þ","inputUtf16":[254],"output":[{"Character":{"data":"þ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0285() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+00FF","initialState":"Data","input":"ÿ","inputUtf16":[255],"output":[{"Character":{"data":"ÿ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0286() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+D7FF","initialState":"Data","input":"퟿","inputUtf16":[55295],"output":[{"Character":{"data":"퟿"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0287() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+E000","initialState":"Data","input":"","inputUtf16":[57344],"output":[{"Character":{"data":""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0288() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+FDCF","initialState":"Data","input":"﷏","inputUtf16":[64975],"output":[{"Character":{"data":"﷏"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0289() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+FDF0","initialState":"Data","input":"ﷰ","inputUtf16":[65008],"output":[{"Character":{"data":"ﷰ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0290() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+FFFD","initialState":"Data","input":"�","inputUtf16":[65533],"output":[{"Character":{"data":"�"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0291() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+10000","initialState":"Data","input":"𐀀","inputUtf16":[55296,56320],"output":[{"Character":{"data":"𐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0292() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+1FFFD","initialState":"Data","input":"🿽","inputUtf16":[55359,57341],"output":[{"Character":{"data":"🿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0293() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+20000","initialState":"Data","input":"𠀀","inputUtf16":[55360,56320],"output":[{"Character":{"data":"𠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0294() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+2FFFD","initialState":"Data","input":"𯿽","inputUtf16":[55423,57341],"output":[{"Character":{"data":"𯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0295() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+30000","initialState":"Data","input":"𰀀","inputUtf16":[55424,56320],"output":[{"Character":{"data":"𰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0296() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+3FFFD","initialState":"Data","input":"𿿽","inputUtf16":[55487,57341],"output":[{"Character":{"data":"𿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0297() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+40000","initialState":"Data","input":"񀀀","inputUtf16":[55488,56320],"output":[{"Character":{"data":"񀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0298() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+4FFFD","initialState":"Data","input":"񏿽","inputUtf16":[55551,57341],"output":[{"Character":{"data":"񏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0299() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+50000","initialState":"Data","input":"񐀀","inputUtf16":[55552,56320],"output":[{"Character":{"data":"񐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0300() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+5FFFD","initialState":"Data","input":"񟿽","inputUtf16":[55615,57341],"output":[{"Character":{"data":"񟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0301() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+60000","initialState":"Data","input":"񠀀","inputUtf16":[55616,56320],"output":[{"Character":{"data":"񠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0302() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+6FFFD","initialState":"Data","input":"񯿽","inputUtf16":[55679,57341],"output":[{"Character":{"data":"񯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0303() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+70000","initialState":"Data","input":"񰀀","inputUtf16":[55680,56320],"output":[{"Character":{"data":"񰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0304() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+7FFFD","initialState":"Data","input":"񿿽","inputUtf16":[55743,57341],"output":[{"Character":{"data":"񿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0305() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+80000","initialState":"Data","input":"򀀀","inputUtf16":[55744,56320],"output":[{"Character":{"data":"򀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0306() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+8FFFD","initialState":"Data","input":"򏿽","inputUtf16":[55807,57341],"output":[{"Character":{"data":"򏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0307() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+90000","initialState":"Data","input":"򐀀","inputUtf16":[55808,56320],"output":[{"Character":{"data":"򐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0308() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+9FFFD","initialState":"Data","input":"򟿽","inputUtf16":[55871,57341],"output":[{"Character":{"data":"򟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0309() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+A0000","initialState":"Data","input":"򠀀","inputUtf16":[55872,56320],"output":[{"Character":{"data":"򠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0310() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+AFFFD","initialState":"Data","input":"򯿽","inputUtf16":[55935,57341],"output":[{"Character":{"data":"򯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0311() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+B0000","initialState":"Data","input":"򰀀","inputUtf16":[55936,56320],"output":[{"Character":{"data":"򰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0312() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+BFFFD","initialState":"Data","input":"򿿽","inputUtf16":[55999,57341],"output":[{"Character":{"data":"򿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0313() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+C0000","initialState":"Data","input":"󀀀","inputUtf16":[56000,56320],"output":[{"Character":{"data":"󀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0314() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+CFFFD","initialState":"Data","input":"󏿽","inputUtf16":[56063,57341],"output":[{"Character":{"data":"󏿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0315() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+D0000","initialState":"Data","input":"󐀀","inputUtf16":[56064,56320],"output":[{"Character":{"data":"󐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0316() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+DFFFD","initialState":"Data","input":"󟿽","inputUtf16":[56127,57341],"output":[{"Character":{"data":"󟿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0317() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+E0000","initialState":"Data","input":"󠀀","inputUtf16":[56128,56320],"output":[{"Character":{"data":"󠀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0318() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+EFFFD","initialState":"Data","input":"󯿽","inputUtf16":[56191,57341],"output":[{"Character":{"data":"󯿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0319() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+F0000","initialState":"Data","input":"󰀀","inputUtf16":[56192,56320],"output":[{"Character":{"data":"󰀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0320() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+FFFFD","initialState":"Data","input":"󿿽","inputUtf16":[56255,57341],"output":[{"Character":{"data":"󿿽"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0321() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+100000","initialState":"Data","input":"􀀀","inputUtf16":[56256,56320],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0322() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Valid Unicode character U+10FFFD","initialState":"Data","input":"􏿽","inputUtf16":[56319,57341],"output":[{"Character":{"data":"􏿽"}}],"errors":[]}"##,
    );
}
