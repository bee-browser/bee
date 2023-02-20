//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Data","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_0001() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Plaintext","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_0002() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Rcdata","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_0003() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Rawtext","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_0004() {
    tokenize(
        r##"{"description":"[empty]","initialState":"ScriptData","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_0005() {
    tokenize(
        r##"{"description":"[empty]","initialState":"CdataSection","input":"","inputUtf16":[],"output":[],"errors":[{"code":"EofInCdata","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0006() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Data","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0007() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Plaintext","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0008() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Rcdata","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0009() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Rawtext","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0010() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"ScriptData","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0011() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"CdataSection","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0012() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Data","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0013() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Plaintext","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0014() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Rcdata","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0015() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Rawtext","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0016() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"ScriptData","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0017() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"CdataSection","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[{"code":"EofInCdata","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0018() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Data","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0019() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Plaintext","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0020() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Rcdata","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0021() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Rawtext","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0022() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"ScriptData","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0023() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"CdataSection","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}},{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0024() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Data","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0025() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Plaintext","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0026() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Rcdata","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0027() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Rawtext","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0028() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"ScriptData","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0029() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"CdataSection","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0030() {
    tokenize(
        r##"{"description":" ","initialState":"Data","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0031() {
    tokenize(
        r##"{"description":" ","initialState":"Plaintext","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0032() {
    tokenize(
        r##"{"description":" ","initialState":"Rcdata","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0033() {
    tokenize(
        r##"{"description":" ","initialState":"Rawtext","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0034() {
    tokenize(
        r##"{"description":" ","initialState":"ScriptData","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0035() {
    tokenize(
        r##"{"description":" ","initialState":"CdataSection","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0036() {
    tokenize(
        r##"{"description":"!","initialState":"Data","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0037() {
    tokenize(
        r##"{"description":"!","initialState":"Plaintext","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0038() {
    tokenize(
        r##"{"description":"!","initialState":"Rcdata","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0039() {
    tokenize(
        r##"{"description":"!","initialState":"Rawtext","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0040() {
    tokenize(
        r##"{"description":"!","initialState":"ScriptData","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0041() {
    tokenize(
        r##"{"description":"!","initialState":"CdataSection","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0042() {
    tokenize(
        r##"{"description":"\"","initialState":"Data","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0043() {
    tokenize(
        r##"{"description":"\"","initialState":"Plaintext","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0044() {
    tokenize(
        r##"{"description":"\"","initialState":"Rcdata","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0045() {
    tokenize(
        r##"{"description":"\"","initialState":"Rawtext","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0046() {
    tokenize(
        r##"{"description":"\"","initialState":"ScriptData","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0047() {
    tokenize(
        r##"{"description":"\"","initialState":"CdataSection","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0048() {
    tokenize(
        r##"{"description":"%","initialState":"Data","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0049() {
    tokenize(
        r##"{"description":"%","initialState":"Plaintext","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0050() {
    tokenize(
        r##"{"description":"%","initialState":"Rcdata","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0051() {
    tokenize(
        r##"{"description":"%","initialState":"Rawtext","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0052() {
    tokenize(
        r##"{"description":"%","initialState":"ScriptData","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0053() {
    tokenize(
        r##"{"description":"%","initialState":"CdataSection","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0054() {
    tokenize(
        r##"{"description":"&","initialState":"Data","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0055() {
    tokenize(
        r##"{"description":"&","initialState":"Plaintext","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0056() {
    tokenize(
        r##"{"description":"&","initialState":"Rcdata","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0057() {
    tokenize(
        r##"{"description":"&","initialState":"Rawtext","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0058() {
    tokenize(
        r##"{"description":"&","initialState":"ScriptData","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0059() {
    tokenize(
        r##"{"description":"&","initialState":"CdataSection","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0060() {
    tokenize(
        r##"{"description":"'","initialState":"Data","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0061() {
    tokenize(
        r##"{"description":"'","initialState":"Plaintext","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0062() {
    tokenize(
        r##"{"description":"'","initialState":"Rcdata","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0063() {
    tokenize(
        r##"{"description":"'","initialState":"Rawtext","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0064() {
    tokenize(
        r##"{"description":"'","initialState":"ScriptData","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0065() {
    tokenize(
        r##"{"description":"'","initialState":"CdataSection","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0066() {
    tokenize(
        r##"{"description":",","initialState":"Data","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_0067() {
    tokenize(
        r##"{"description":",","initialState":"Plaintext","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_0068() {
    tokenize(
        r##"{"description":",","initialState":"Rcdata","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_0069() {
    tokenize(
        r##"{"description":",","initialState":"Rawtext","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_0070() {
    tokenize(
        r##"{"description":",","initialState":"ScriptData","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_0071() {
    tokenize(
        r##"{"description":",","initialState":"CdataSection","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0072() {
    tokenize(
        r##"{"description":"-","initialState":"Data","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0073() {
    tokenize(
        r##"{"description":"-","initialState":"Plaintext","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0074() {
    tokenize(
        r##"{"description":"-","initialState":"Rcdata","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0075() {
    tokenize(
        r##"{"description":"-","initialState":"Rawtext","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0076() {
    tokenize(
        r##"{"description":"-","initialState":"ScriptData","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0077() {
    tokenize(
        r##"{"description":"-","initialState":"CdataSection","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0078() {
    tokenize(
        r##"{"description":".","initialState":"Data","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_0079() {
    tokenize(
        r##"{"description":".","initialState":"Plaintext","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_0080() {
    tokenize(
        r##"{"description":".","initialState":"Rcdata","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_0081() {
    tokenize(
        r##"{"description":".","initialState":"Rawtext","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_0082() {
    tokenize(
        r##"{"description":".","initialState":"ScriptData","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_0083() {
    tokenize(
        r##"{"description":".","initialState":"CdataSection","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0084() {
    tokenize(
        r##"{"description":"/","initialState":"Data","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0085() {
    tokenize(
        r##"{"description":"/","initialState":"Plaintext","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0086() {
    tokenize(
        r##"{"description":"/","initialState":"Rcdata","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0087() {
    tokenize(
        r##"{"description":"/","initialState":"Rawtext","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0088() {
    tokenize(
        r##"{"description":"/","initialState":"ScriptData","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0089() {
    tokenize(
        r##"{"description":"/","initialState":"CdataSection","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0090() {
    tokenize(
        r##"{"description":"0","initialState":"Data","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0091() {
    tokenize(
        r##"{"description":"0","initialState":"Plaintext","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0092() {
    tokenize(
        r##"{"description":"0","initialState":"Rcdata","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0093() {
    tokenize(
        r##"{"description":"0","initialState":"Rawtext","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0094() {
    tokenize(
        r##"{"description":"0","initialState":"ScriptData","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0095() {
    tokenize(
        r##"{"description":"0","initialState":"CdataSection","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0096() {
    tokenize(
        r##"{"description":"1","initialState":"Data","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0097() {
    tokenize(
        r##"{"description":"1","initialState":"Plaintext","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0098() {
    tokenize(
        r##"{"description":"1","initialState":"Rcdata","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0099() {
    tokenize(
        r##"{"description":"1","initialState":"Rawtext","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0100() {
    tokenize(
        r##"{"description":"1","initialState":"ScriptData","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0101() {
    tokenize(
        r##"{"description":"1","initialState":"CdataSection","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0102() {
    tokenize(
        r##"{"description":"9","initialState":"Data","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0103() {
    tokenize(
        r##"{"description":"9","initialState":"Plaintext","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0104() {
    tokenize(
        r##"{"description":"9","initialState":"Rcdata","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0105() {
    tokenize(
        r##"{"description":"9","initialState":"Rawtext","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0106() {
    tokenize(
        r##"{"description":"9","initialState":"ScriptData","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0107() {
    tokenize(
        r##"{"description":"9","initialState":"CdataSection","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0108() {
    tokenize(
        r##"{"description":";","initialState":"Data","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0109() {
    tokenize(
        r##"{"description":";","initialState":"Plaintext","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0110() {
    tokenize(
        r##"{"description":";","initialState":"Rcdata","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0111() {
    tokenize(
        r##"{"description":";","initialState":"Rawtext","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0112() {
    tokenize(
        r##"{"description":";","initialState":"ScriptData","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0113() {
    tokenize(
        r##"{"description":";","initialState":"CdataSection","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0114() {
    tokenize(
        r##"{"description":";=","initialState":"Data","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_0115() {
    tokenize(
        r##"{"description":";=","initialState":"Plaintext","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_0116() {
    tokenize(
        r##"{"description":";=","initialState":"Rcdata","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_0117() {
    tokenize(
        r##"{"description":";=","initialState":"Rawtext","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_0118() {
    tokenize(
        r##"{"description":";=","initialState":"ScriptData","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_0119() {
    tokenize(
        r##"{"description":";=","initialState":"CdataSection","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0120() {
    tokenize(
        r##"{"description":";>","initialState":"Data","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0121() {
    tokenize(
        r##"{"description":";>","initialState":"Plaintext","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0122() {
    tokenize(
        r##"{"description":";>","initialState":"Rcdata","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0123() {
    tokenize(
        r##"{"description":";>","initialState":"Rawtext","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0124() {
    tokenize(
        r##"{"description":";>","initialState":"ScriptData","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0125() {
    tokenize(
        r##"{"description":";>","initialState":"CdataSection","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0126() {
    tokenize(
        r##"{"description":";?","initialState":"Data","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0127() {
    tokenize(
        r##"{"description":";?","initialState":"Plaintext","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0128() {
    tokenize(
        r##"{"description":";?","initialState":"Rcdata","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0129() {
    tokenize(
        r##"{"description":";?","initialState":"Rawtext","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0130() {
    tokenize(
        r##"{"description":";?","initialState":"ScriptData","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0131() {
    tokenize(
        r##"{"description":";?","initialState":"CdataSection","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0132() {
    tokenize(
        r##"{"description":";@","initialState":"Data","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0133() {
    tokenize(
        r##"{"description":";@","initialState":"Plaintext","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0134() {
    tokenize(
        r##"{"description":";@","initialState":"Rcdata","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0135() {
    tokenize(
        r##"{"description":";@","initialState":"Rawtext","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0136() {
    tokenize(
        r##"{"description":";@","initialState":"ScriptData","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0137() {
    tokenize(
        r##"{"description":";@","initialState":"CdataSection","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0138() {
    tokenize(
        r##"{"description":";A","initialState":"Data","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0139() {
    tokenize(
        r##"{"description":";A","initialState":"Plaintext","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0140() {
    tokenize(
        r##"{"description":";A","initialState":"Rcdata","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0141() {
    tokenize(
        r##"{"description":";A","initialState":"Rawtext","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0142() {
    tokenize(
        r##"{"description":";A","initialState":"ScriptData","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0143() {
    tokenize(
        r##"{"description":";A","initialState":"CdataSection","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0144() {
    tokenize(
        r##"{"description":";B","initialState":"Data","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0145() {
    tokenize(
        r##"{"description":";B","initialState":"Plaintext","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0146() {
    tokenize(
        r##"{"description":";B","initialState":"Rcdata","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0147() {
    tokenize(
        r##"{"description":";B","initialState":"Rawtext","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0148() {
    tokenize(
        r##"{"description":";B","initialState":"ScriptData","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0149() {
    tokenize(
        r##"{"description":";B","initialState":"CdataSection","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0150() {
    tokenize(
        r##"{"description":";Y","initialState":"Data","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0151() {
    tokenize(
        r##"{"description":";Y","initialState":"Plaintext","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0152() {
    tokenize(
        r##"{"description":";Y","initialState":"Rcdata","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0153() {
    tokenize(
        r##"{"description":";Y","initialState":"Rawtext","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0154() {
    tokenize(
        r##"{"description":";Y","initialState":"ScriptData","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0155() {
    tokenize(
        r##"{"description":";Y","initialState":"CdataSection","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0156() {
    tokenize(
        r##"{"description":";Z","initialState":"Data","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0157() {
    tokenize(
        r##"{"description":";Z","initialState":"Plaintext","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0158() {
    tokenize(
        r##"{"description":";Z","initialState":"Rcdata","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0159() {
    tokenize(
        r##"{"description":";Z","initialState":"Rawtext","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0160() {
    tokenize(
        r##"{"description":";Z","initialState":"ScriptData","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0161() {
    tokenize(
        r##"{"description":";Z","initialState":"CdataSection","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0162() {
    tokenize(
        r##"{"description":";`","initialState":"Data","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0163() {
    tokenize(
        r##"{"description":";`","initialState":"Plaintext","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0164() {
    tokenize(
        r##"{"description":";`","initialState":"Rcdata","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0165() {
    tokenize(
        r##"{"description":";`","initialState":"Rawtext","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0166() {
    tokenize(
        r##"{"description":";`","initialState":"ScriptData","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0167() {
    tokenize(
        r##"{"description":";`","initialState":"CdataSection","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0168() {
    tokenize(
        r##"{"description":";a","initialState":"Data","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0169() {
    tokenize(
        r##"{"description":";a","initialState":"Plaintext","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0170() {
    tokenize(
        r##"{"description":";a","initialState":"Rcdata","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0171() {
    tokenize(
        r##"{"description":";a","initialState":"Rawtext","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0172() {
    tokenize(
        r##"{"description":";a","initialState":"ScriptData","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0173() {
    tokenize(
        r##"{"description":";a","initialState":"CdataSection","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0174() {
    tokenize(
        r##"{"description":";b","initialState":"Data","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0175() {
    tokenize(
        r##"{"description":";b","initialState":"Plaintext","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0176() {
    tokenize(
        r##"{"description":";b","initialState":"Rcdata","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0177() {
    tokenize(
        r##"{"description":";b","initialState":"Rawtext","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0178() {
    tokenize(
        r##"{"description":";b","initialState":"ScriptData","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0179() {
    tokenize(
        r##"{"description":";b","initialState":"CdataSection","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0180() {
    tokenize(
        r##"{"description":";y","initialState":"Data","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0181() {
    tokenize(
        r##"{"description":";y","initialState":"Plaintext","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0182() {
    tokenize(
        r##"{"description":";y","initialState":"Rcdata","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0183() {
    tokenize(
        r##"{"description":";y","initialState":"Rawtext","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0184() {
    tokenize(
        r##"{"description":";y","initialState":"ScriptData","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0185() {
    tokenize(
        r##"{"description":";y","initialState":"CdataSection","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0186() {
    tokenize(
        r##"{"description":";z","initialState":"Data","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0187() {
    tokenize(
        r##"{"description":";z","initialState":"Plaintext","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0188() {
    tokenize(
        r##"{"description":";z","initialState":"Rcdata","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0189() {
    tokenize(
        r##"{"description":";z","initialState":"Rawtext","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0190() {
    tokenize(
        r##"{"description":";z","initialState":"ScriptData","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0191() {
    tokenize(
        r##"{"description":";z","initialState":"CdataSection","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0192() {
    tokenize(
        r##"{"description":";{","initialState":"Data","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0193() {
    tokenize(
        r##"{"description":";{","initialState":"Plaintext","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0194() {
    tokenize(
        r##"{"description":";{","initialState":"Rcdata","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0195() {
    tokenize(
        r##"{"description":";{","initialState":"Rawtext","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0196() {
    tokenize(
        r##"{"description":";{","initialState":"ScriptData","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0197() {
    tokenize(
        r##"{"description":";{","initialState":"CdataSection","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0198() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Data","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0199() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Plaintext","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0200() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Rcdata","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0201() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Rawtext","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0202() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"ScriptData","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0203() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"CdataSection","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0204() {
    tokenize(
        r##"{"description":"<","initialState":"Data","input":"<","inputUtf16":[60],"output":[{"Character":{"data":"<"}}],"errors":[{"code":"EofBeforeTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0205() {
    tokenize(
        r##"{"description":"<\\u0000","initialState":"Data","input":"<\u0000","inputUtf16":[60,0],"output":[{"Character":{"data":"<\u0000"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0206() {
    tokenize(
        r##"{"description":"<\\u0009","initialState":"Data","input":"<\t","inputUtf16":[60,9],"output":[{"Character":{"data":"<\t"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0207() {
    tokenize(
        r##"{"description":"<\\u000A","initialState":"Data","input":"<\n","inputUtf16":[60,10],"output":[{"Character":{"data":"<\n"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0208() {
    tokenize(
        r##"{"description":"<\\u000B","initialState":"Data","input":"<\u000b","inputUtf16":[60,11],"output":[{"Character":{"data":"<\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":2}},{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0209() {
    tokenize(
        r##"{"description":"<\\u000C","initialState":"Data","input":"<\f","inputUtf16":[60,12],"output":[{"Character":{"data":"<\f"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0210() {
    tokenize(
        r##"{"description":"< ","initialState":"Data","input":"< ","inputUtf16":[60,32],"output":[{"Character":{"data":"< "}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0211() {
    tokenize(
        r##"{"description":"<!","initialState":"Data","input":"<!","inputUtf16":[60,33],"output":[{"Comment":{"data":""}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0212() {
    tokenize(
        r##"{"description":"<!\\u0000","initialState":"Data","input":"<!\u0000","inputUtf16":[60,33,0],"output":[{"Comment":{"data":"�"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0213() {
    tokenize(
        r##"{"description":"<!\\u0009","initialState":"Data","input":"<!\t","inputUtf16":[60,33,9],"output":[{"Comment":{"data":"\t"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0214() {
    tokenize(
        r##"{"description":"<!\\u000A","initialState":"Data","input":"<!\n","inputUtf16":[60,33,10],"output":[{"Comment":{"data":"\n"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0215() {
    tokenize(
        r##"{"description":"<!\\u000B","initialState":"Data","input":"<!\u000b","inputUtf16":[60,33,11],"output":[{"Comment":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":3}},{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0216() {
    tokenize(
        r##"{"description":"<!\\u000C","initialState":"Data","input":"<!\f","inputUtf16":[60,33,12],"output":[{"Comment":{"data":"\f"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0217() {
    tokenize(
        r##"{"description":"<! ","initialState":"Data","input":"<! ","inputUtf16":[60,33,32],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0218() {
    tokenize(
        r##"{"description":"<! \\u0000","initialState":"Data","input":"<! \u0000","inputUtf16":[60,33,32,0],"output":[{"Comment":{"data":" �"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_0219() {
    tokenize(
        r##"{"description":"<!!","initialState":"Data","input":"<!!","inputUtf16":[60,33,33],"output":[{"Comment":{"data":"!"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0220() {
    tokenize(
        r##"{"description":"<!\"","initialState":"Data","input":"<!\"","inputUtf16":[60,33,34],"output":[{"Comment":{"data":"\""}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0221() {
    tokenize(
        r##"{"description":"<!&","initialState":"Data","input":"<!&","inputUtf16":[60,33,38],"output":[{"Comment":{"data":"&"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0222() {
    tokenize(
        r##"{"description":"<!'","initialState":"Data","input":"<!'","inputUtf16":[60,33,39],"output":[{"Comment":{"data":"'"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0223() {
    tokenize(
        r##"{"description":"<!-","initialState":"Data","input":"<!-","inputUtf16":[60,33,45],"output":[{"Comment":{"data":"-"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0224() {
    tokenize(
        r##"{"description":"<!--","initialState":"Data","input":"<!--","inputUtf16":[60,33,45,45],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_0225() {
    tokenize(
        r##"{"description":"<!--\\u0000","initialState":"Data","input":"<!--\u0000","inputUtf16":[60,33,45,45,0],"output":[{"Comment":{"data":"�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":5}},{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0226() {
    tokenize(
        r##"{"description":"<!--\\u0009","initialState":"Data","input":"<!--\t","inputUtf16":[60,33,45,45,9],"output":[{"Comment":{"data":"\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0227() {
    tokenize(
        r##"{"description":"<!--\\u000A","initialState":"Data","input":"<!--\n","inputUtf16":[60,33,45,45,10],"output":[{"Comment":{"data":"\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0228() {
    tokenize(
        r##"{"description":"<!--\\u000B","initialState":"Data","input":"<!--\u000b","inputUtf16":[60,33,45,45,11],"output":[{"Comment":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":5}},{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0229() {
    tokenize(
        r##"{"description":"<!--\\u000C","initialState":"Data","input":"<!--\f","inputUtf16":[60,33,45,45,12],"output":[{"Comment":{"data":"\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0230() {
    tokenize(
        r##"{"description":"<!-- ","initialState":"Data","input":"<!-- ","inputUtf16":[60,33,45,45,32],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0231() {
    tokenize(
        r##"{"description":"<!-- \\u0000","initialState":"Data","input":"<!-- \u0000","inputUtf16":[60,33,45,45,32,0],"output":[{"Comment":{"data":" �"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0232() {
    tokenize(
        r##"{"description":"<!-- \\u0009","initialState":"Data","input":"<!-- \t","inputUtf16":[60,33,45,45,32,9],"output":[{"Comment":{"data":" \t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0233() {
    tokenize(
        r##"{"description":"<!-- \\u000A","initialState":"Data","input":"<!-- \n","inputUtf16":[60,33,45,45,32,10],"output":[{"Comment":{"data":" \n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0234() {
    tokenize(
        r##"{"description":"<!-- \\u000B","initialState":"Data","input":"<!-- \u000b","inputUtf16":[60,33,45,45,32,11],"output":[{"Comment":{"data":" \u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0235() {
    tokenize(
        r##"{"description":"<!-- \\u000C","initialState":"Data","input":"<!-- \f","inputUtf16":[60,33,45,45,32,12],"output":[{"Comment":{"data":" \f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0236() {
    tokenize(
        r##"{"description":"<!--  ","initialState":"Data","input":"<!--  ","inputUtf16":[60,33,45,45,32,32],"output":[{"Comment":{"data":"  "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0237() {
    tokenize(
        r##"{"description":"<!-- !","initialState":"Data","input":"<!-- !","inputUtf16":[60,33,45,45,32,33],"output":[{"Comment":{"data":" !"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0238() {
    tokenize(
        r##"{"description":"<!-- \"","initialState":"Data","input":"<!-- \"","inputUtf16":[60,33,45,45,32,34],"output":[{"Comment":{"data":" \""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0239() {
    tokenize(
        r##"{"description":"<!-- &","initialState":"Data","input":"<!-- &","inputUtf16":[60,33,45,45,32,38],"output":[{"Comment":{"data":" &"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0240() {
    tokenize(
        r##"{"description":"<!-- '","initialState":"Data","input":"<!-- '","inputUtf16":[60,33,45,45,32,39],"output":[{"Comment":{"data":" '"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0241() {
    tokenize(
        r##"{"description":"<!-- ,","initialState":"Data","input":"<!-- ,","inputUtf16":[60,33,45,45,32,44],"output":[{"Comment":{"data":" ,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0242() {
    tokenize(
        r##"{"description":"<!-- -","initialState":"Data","input":"<!-- -","inputUtf16":[60,33,45,45,32,45],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0243() {
    tokenize(
        r##"{"description":"<!-- -\\u0000","initialState":"Data","input":"<!-- -\u0000","inputUtf16":[60,33,45,45,32,45,0],"output":[{"Comment":{"data":" -�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0244() {
    tokenize(
        r##"{"description":"<!-- -\\u0009","initialState":"Data","input":"<!-- -\t","inputUtf16":[60,33,45,45,32,45,9],"output":[{"Comment":{"data":" -\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0245() {
    tokenize(
        r##"{"description":"<!-- -\\u000A","initialState":"Data","input":"<!-- -\n","inputUtf16":[60,33,45,45,32,45,10],"output":[{"Comment":{"data":" -\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0246() {
    tokenize(
        r##"{"description":"<!-- -\\u000B","initialState":"Data","input":"<!-- -\u000b","inputUtf16":[60,33,45,45,32,45,11],"output":[{"Comment":{"data":" -\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0247() {
    tokenize(
        r##"{"description":"<!-- -\\u000C","initialState":"Data","input":"<!-- -\f","inputUtf16":[60,33,45,45,32,45,12],"output":[{"Comment":{"data":" -\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0248() {
    tokenize(
        r##"{"description":"<!-- - ","initialState":"Data","input":"<!-- - ","inputUtf16":[60,33,45,45,32,45,32],"output":[{"Comment":{"data":" - "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0249() {
    tokenize(
        r##"{"description":"<!-- -!","initialState":"Data","input":"<!-- -!","inputUtf16":[60,33,45,45,32,45,33],"output":[{"Comment":{"data":" -!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0250() {
    tokenize(
        r##"{"description":"<!-- -\"","initialState":"Data","input":"<!-- -\"","inputUtf16":[60,33,45,45,32,45,34],"output":[{"Comment":{"data":" -\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0251() {
    tokenize(
        r##"{"description":"<!-- -&","initialState":"Data","input":"<!-- -&","inputUtf16":[60,33,45,45,32,45,38],"output":[{"Comment":{"data":" -&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0252() {
    tokenize(
        r##"{"description":"<!-- -'","initialState":"Data","input":"<!-- -'","inputUtf16":[60,33,45,45,32,45,39],"output":[{"Comment":{"data":" -'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0253() {
    tokenize(
        r##"{"description":"<!-- -,","initialState":"Data","input":"<!-- -,","inputUtf16":[60,33,45,45,32,45,44],"output":[{"Comment":{"data":" -,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0254() {
    tokenize(
        r##"{"description":"<!-- --","initialState":"Data","input":"<!-- --","inputUtf16":[60,33,45,45,32,45,45],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0255() {
    tokenize(
        r##"{"description":"<!-- -.","initialState":"Data","input":"<!-- -.","inputUtf16":[60,33,45,45,32,45,46],"output":[{"Comment":{"data":" -."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0256() {
    tokenize(
        r##"{"description":"<!-- -/","initialState":"Data","input":"<!-- -/","inputUtf16":[60,33,45,45,32,45,47],"output":[{"Comment":{"data":" -/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0257() {
    tokenize(
        r##"{"description":"<!-- -0","initialState":"Data","input":"<!-- -0","inputUtf16":[60,33,45,45,32,45,48],"output":[{"Comment":{"data":" -0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0258() {
    tokenize(
        r##"{"description":"<!-- -1","initialState":"Data","input":"<!-- -1","inputUtf16":[60,33,45,45,32,45,49],"output":[{"Comment":{"data":" -1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0259() {
    tokenize(
        r##"{"description":"<!-- -9","initialState":"Data","input":"<!-- -9","inputUtf16":[60,33,45,45,32,45,57],"output":[{"Comment":{"data":" -9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0260() {
    tokenize(
        r##"{"description":"<!-- -<","initialState":"Data","input":"<!-- -<","inputUtf16":[60,33,45,45,32,45,60],"output":[{"Comment":{"data":" -<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0261() {
    tokenize(
        r##"{"description":"<!-- -=","initialState":"Data","input":"<!-- -=","inputUtf16":[60,33,45,45,32,45,61],"output":[{"Comment":{"data":" -="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0262() {
    tokenize(
        r##"{"description":"<!-- ->","initialState":"Data","input":"<!-- ->","inputUtf16":[60,33,45,45,32,45,62],"output":[{"Comment":{"data":" ->"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0263() {
    tokenize(
        r##"{"description":"<!-- -?","initialState":"Data","input":"<!-- -?","inputUtf16":[60,33,45,45,32,45,63],"output":[{"Comment":{"data":" -?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0264() {
    tokenize(
        r##"{"description":"<!-- -@","initialState":"Data","input":"<!-- -@","inputUtf16":[60,33,45,45,32,45,64],"output":[{"Comment":{"data":" -@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0265() {
    tokenize(
        r##"{"description":"<!-- -A","initialState":"Data","input":"<!-- -A","inputUtf16":[60,33,45,45,32,45,65],"output":[{"Comment":{"data":" -A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0266() {
    tokenize(
        r##"{"description":"<!-- -B","initialState":"Data","input":"<!-- -B","inputUtf16":[60,33,45,45,32,45,66],"output":[{"Comment":{"data":" -B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0267() {
    tokenize(
        r##"{"description":"<!-- -Y","initialState":"Data","input":"<!-- -Y","inputUtf16":[60,33,45,45,32,45,89],"output":[{"Comment":{"data":" -Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0268() {
    tokenize(
        r##"{"description":"<!-- -Z","initialState":"Data","input":"<!-- -Z","inputUtf16":[60,33,45,45,32,45,90],"output":[{"Comment":{"data":" -Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0269() {
    tokenize(
        r##"{"description":"<!-- -`","initialState":"Data","input":"<!-- -`","inputUtf16":[60,33,45,45,32,45,96],"output":[{"Comment":{"data":" -`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0270() {
    tokenize(
        r##"{"description":"<!-- -a","initialState":"Data","input":"<!-- -a","inputUtf16":[60,33,45,45,32,45,97],"output":[{"Comment":{"data":" -a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0271() {
    tokenize(
        r##"{"description":"<!-- -b","initialState":"Data","input":"<!-- -b","inputUtf16":[60,33,45,45,32,45,98],"output":[{"Comment":{"data":" -b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0272() {
    tokenize(
        r##"{"description":"<!-- -y","initialState":"Data","input":"<!-- -y","inputUtf16":[60,33,45,45,32,45,121],"output":[{"Comment":{"data":" -y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0273() {
    tokenize(
        r##"{"description":"<!-- -z","initialState":"Data","input":"<!-- -z","inputUtf16":[60,33,45,45,32,45,122],"output":[{"Comment":{"data":" -z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0274() {
    tokenize(
        r##"{"description":"<!-- -{","initialState":"Data","input":"<!-- -{","inputUtf16":[60,33,45,45,32,45,123],"output":[{"Comment":{"data":" -{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0275() {
    tokenize(
        r##"{"description":"<!-- -\\uDBC0\\uDC00","initialState":"Data","input":"<!-- -􀀀","inputUtf16":[60,33,45,45,32,45,56256,56320],"output":[{"Comment":{"data":" -􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0276() {
    tokenize(
        r##"{"description":"<!-- .","initialState":"Data","input":"<!-- .","inputUtf16":[60,33,45,45,32,46],"output":[{"Comment":{"data":" ."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0277() {
    tokenize(
        r##"{"description":"<!-- /","initialState":"Data","input":"<!-- /","inputUtf16":[60,33,45,45,32,47],"output":[{"Comment":{"data":" /"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0278() {
    tokenize(
        r##"{"description":"<!-- 0","initialState":"Data","input":"<!-- 0","inputUtf16":[60,33,45,45,32,48],"output":[{"Comment":{"data":" 0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0279() {
    tokenize(
        r##"{"description":"<!-- 1","initialState":"Data","input":"<!-- 1","inputUtf16":[60,33,45,45,32,49],"output":[{"Comment":{"data":" 1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0280() {
    tokenize(
        r##"{"description":"<!-- 9","initialState":"Data","input":"<!-- 9","inputUtf16":[60,33,45,45,32,57],"output":[{"Comment":{"data":" 9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0281() {
    tokenize(
        r##"{"description":"<!-- <","initialState":"Data","input":"<!-- <","inputUtf16":[60,33,45,45,32,60],"output":[{"Comment":{"data":" <"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0282() {
    tokenize(
        r##"{"description":"<!-- =","initialState":"Data","input":"<!-- =","inputUtf16":[60,33,45,45,32,61],"output":[{"Comment":{"data":" ="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0283() {
    tokenize(
        r##"{"description":"<!-- >","initialState":"Data","input":"<!-- >","inputUtf16":[60,33,45,45,32,62],"output":[{"Comment":{"data":" >"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0284() {
    tokenize(
        r##"{"description":"<!-- ?","initialState":"Data","input":"<!-- ?","inputUtf16":[60,33,45,45,32,63],"output":[{"Comment":{"data":" ?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0285() {
    tokenize(
        r##"{"description":"<!-- @","initialState":"Data","input":"<!-- @","inputUtf16":[60,33,45,45,32,64],"output":[{"Comment":{"data":" @"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0286() {
    tokenize(
        r##"{"description":"<!-- A","initialState":"Data","input":"<!-- A","inputUtf16":[60,33,45,45,32,65],"output":[{"Comment":{"data":" A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0287() {
    tokenize(
        r##"{"description":"<!-- B","initialState":"Data","input":"<!-- B","inputUtf16":[60,33,45,45,32,66],"output":[{"Comment":{"data":" B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0288() {
    tokenize(
        r##"{"description":"<!-- Y","initialState":"Data","input":"<!-- Y","inputUtf16":[60,33,45,45,32,89],"output":[{"Comment":{"data":" Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0289() {
    tokenize(
        r##"{"description":"<!-- Z","initialState":"Data","input":"<!-- Z","inputUtf16":[60,33,45,45,32,90],"output":[{"Comment":{"data":" Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0290() {
    tokenize(
        r##"{"description":"<!-- `","initialState":"Data","input":"<!-- `","inputUtf16":[60,33,45,45,32,96],"output":[{"Comment":{"data":" `"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0291() {
    tokenize(
        r##"{"description":"<!-- a","initialState":"Data","input":"<!-- a","inputUtf16":[60,33,45,45,32,97],"output":[{"Comment":{"data":" a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0292() {
    tokenize(
        r##"{"description":"<!-- b","initialState":"Data","input":"<!-- b","inputUtf16":[60,33,45,45,32,98],"output":[{"Comment":{"data":" b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0293() {
    tokenize(
        r##"{"description":"<!-- y","initialState":"Data","input":"<!-- y","inputUtf16":[60,33,45,45,32,121],"output":[{"Comment":{"data":" y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0294() {
    tokenize(
        r##"{"description":"<!-- z","initialState":"Data","input":"<!-- z","inputUtf16":[60,33,45,45,32,122],"output":[{"Comment":{"data":" z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0295() {
    tokenize(
        r##"{"description":"<!-- {","initialState":"Data","input":"<!-- {","inputUtf16":[60,33,45,45,32,123],"output":[{"Comment":{"data":" {"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0296() {
    tokenize(
        r##"{"description":"<!-- \\uDBC0\\uDC00","initialState":"Data","input":"<!-- 􀀀","inputUtf16":[60,33,45,45,32,56256,56320],"output":[{"Comment":{"data":" 􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0297() {
    tokenize(
        r##"{"description":"<!--!","initialState":"Data","input":"<!--!","inputUtf16":[60,33,45,45,33],"output":[{"Comment":{"data":"!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0298() {
    tokenize(
        r##"{"description":"<!--\"","initialState":"Data","input":"<!--\"","inputUtf16":[60,33,45,45,34],"output":[{"Comment":{"data":"\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0299() {
    tokenize(
        r##"{"description":"<!--&","initialState":"Data","input":"<!--&","inputUtf16":[60,33,45,45,38],"output":[{"Comment":{"data":"&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0300() {
    tokenize(
        r##"{"description":"<!--'","initialState":"Data","input":"<!--'","inputUtf16":[60,33,45,45,39],"output":[{"Comment":{"data":"'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0301() {
    tokenize(
        r##"{"description":"<!--,","initialState":"Data","input":"<!--,","inputUtf16":[60,33,45,45,44],"output":[{"Comment":{"data":","}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0302() {
    tokenize(
        r##"{"description":"<!---","initialState":"Data","input":"<!---","inputUtf16":[60,33,45,45,45],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0303() {
    tokenize(
        r##"{"description":"<!---\\u0000","initialState":"Data","input":"<!---\u0000","inputUtf16":[60,33,45,45,45,0],"output":[{"Comment":{"data":"-�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0304() {
    tokenize(
        r##"{"description":"<!---\\u0009","initialState":"Data","input":"<!---\t","inputUtf16":[60,33,45,45,45,9],"output":[{"Comment":{"data":"-\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0305() {
    tokenize(
        r##"{"description":"<!---\\u000A","initialState":"Data","input":"<!---\n","inputUtf16":[60,33,45,45,45,10],"output":[{"Comment":{"data":"-\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0306() {
    tokenize(
        r##"{"description":"<!---\\u000B","initialState":"Data","input":"<!---\u000b","inputUtf16":[60,33,45,45,45,11],"output":[{"Comment":{"data":"-\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0307() {
    tokenize(
        r##"{"description":"<!---\\u000C","initialState":"Data","input":"<!---\f","inputUtf16":[60,33,45,45,45,12],"output":[{"Comment":{"data":"-\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0308() {
    tokenize(
        r##"{"description":"<!--- ","initialState":"Data","input":"<!--- ","inputUtf16":[60,33,45,45,45,32],"output":[{"Comment":{"data":"- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0309() {
    tokenize(
        r##"{"description":"<!---!","initialState":"Data","input":"<!---!","inputUtf16":[60,33,45,45,45,33],"output":[{"Comment":{"data":"-!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0310() {
    tokenize(
        r##"{"description":"<!---\"","initialState":"Data","input":"<!---\"","inputUtf16":[60,33,45,45,45,34],"output":[{"Comment":{"data":"-\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0311() {
    tokenize(
        r##"{"description":"<!---&","initialState":"Data","input":"<!---&","inputUtf16":[60,33,45,45,45,38],"output":[{"Comment":{"data":"-&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0312() {
    tokenize(
        r##"{"description":"<!---'","initialState":"Data","input":"<!---'","inputUtf16":[60,33,45,45,45,39],"output":[{"Comment":{"data":"-'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0313() {
    tokenize(
        r##"{"description":"<!---,","initialState":"Data","input":"<!---,","inputUtf16":[60,33,45,45,45,44],"output":[{"Comment":{"data":"-,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0314() {
    tokenize(
        r##"{"description":"<!----","initialState":"Data","input":"<!----","inputUtf16":[60,33,45,45,45,45],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0315() {
    tokenize(
        r##"{"description":"<!----\\u0000","initialState":"Data","input":"<!----\u0000","inputUtf16":[60,33,45,45,45,45,0],"output":[{"Comment":{"data":"--�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0316() {
    tokenize(
        r##"{"description":"<!----\\u0009","initialState":"Data","input":"<!----\t","inputUtf16":[60,33,45,45,45,45,9],"output":[{"Comment":{"data":"--\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0317() {
    tokenize(
        r##"{"description":"<!----\\u000A","initialState":"Data","input":"<!----\n","inputUtf16":[60,33,45,45,45,45,10],"output":[{"Comment":{"data":"--\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0318() {
    tokenize(
        r##"{"description":"<!----\\u000B","initialState":"Data","input":"<!----\u000b","inputUtf16":[60,33,45,45,45,45,11],"output":[{"Comment":{"data":"--\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0319() {
    tokenize(
        r##"{"description":"<!----\\u000C","initialState":"Data","input":"<!----\f","inputUtf16":[60,33,45,45,45,45,12],"output":[{"Comment":{"data":"--\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0320() {
    tokenize(
        r##"{"description":"<!---- ","initialState":"Data","input":"<!---- ","inputUtf16":[60,33,45,45,45,45,32],"output":[{"Comment":{"data":"-- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0321() {
    tokenize(
        r##"{"description":"<!---- -","initialState":"Data","input":"<!---- -","inputUtf16":[60,33,45,45,45,45,32,45],"output":[{"Comment":{"data":"-- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0322() {
    tokenize(
        r##"{"description":"<!---- --","initialState":"Data","input":"<!---- --","inputUtf16":[60,33,45,45,45,45,32,45,45],"output":[{"Comment":{"data":"-- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0323() {
    tokenize(
        r##"{"description":"<!---- -->","initialState":"Data","input":"<!---- -->","inputUtf16":[60,33,45,45,45,45,32,45,45,62],"output":[{"Comment":{"data":"-- "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0324() {
    tokenize(
        r##"{"description":"<!----  -->","initialState":"Data","input":"<!----  -->","inputUtf16":[60,33,45,45,45,45,32,32,45,45,62],"output":[{"Comment":{"data":"--  "}}],"errors":[]}"##,
    );
}

#[test]
fn test_0325() {
    tokenize(
        r##"{"description":"<!---- a-->","initialState":"Data","input":"<!---- a-->","inputUtf16":[60,33,45,45,45,45,32,97,45,45,62],"output":[{"Comment":{"data":"-- a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0326() {
    tokenize(
        r##"{"description":"<!----!","initialState":"Data","input":"<!----!","inputUtf16":[60,33,45,45,45,45,33],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0327() {
    tokenize(
        r##"{"description":"<!----!>","initialState":"Data","input":"<!----!>","inputUtf16":[60,33,45,45,45,45,33,62],"output":[{"Comment":{"data":""}}],"errors":[{"code":"IncorrectlyClosedComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0328() {
    tokenize(
        r##"{"description":"<!----! >","initialState":"Data","input":"<!----! >","inputUtf16":[60,33,45,45,45,45,33,32,62],"output":[{"Comment":{"data":"--! >"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0329() {
    tokenize(
        r##"{"description":"<!----!LF>","initialState":"Data","input":"<!----!\n>","inputUtf16":[60,33,45,45,45,45,33,10,62],"output":[{"Comment":{"data":"--!\n>"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":2}}]}"##,
    );
}

#[test]
fn test_0330() {
    tokenize(
        r##"{"description":"<!----!CR>","initialState":"Data","input":"<!----!\r>","inputUtf16":[60,33,45,45,45,45,33,13,62],"output":[{"Comment":{"data":"--!\n>"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":2}}]}"##,
    );
}

#[test]
fn test_0331() {
    tokenize(
        r##"{"description":"<!----!CRLF>","initialState":"Data","input":"<!----!\r\n>","inputUtf16":[60,33,45,45,45,45,33,13,10,62],"output":[{"Comment":{"data":"--!\n>"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":2}}]}"##,
    );
}

#[test]
fn test_0332() {
    tokenize(
        r##"{"description":"<!----!a","initialState":"Data","input":"<!----!a","inputUtf16":[60,33,45,45,45,45,33,97],"output":[{"Comment":{"data":"--!a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0333() {
    tokenize(
        r##"{"description":"<!----!a-","initialState":"Data","input":"<!----!a-","inputUtf16":[60,33,45,45,45,45,33,97,45],"output":[{"Comment":{"data":"--!a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0334() {
    tokenize(
        r##"{"description":"<!----!a--","initialState":"Data","input":"<!----!a--","inputUtf16":[60,33,45,45,45,45,33,97,45,45],"output":[{"Comment":{"data":"--!a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0335() {
    tokenize(
        r##"{"description":"<!----!a-->","initialState":"Data","input":"<!----!a-->","inputUtf16":[60,33,45,45,45,45,33,97,45,45,62],"output":[{"Comment":{"data":"--!a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0336() {
    tokenize(
        r##"{"description":"<!----!-","initialState":"Data","input":"<!----!-","inputUtf16":[60,33,45,45,45,45,33,45],"output":[{"Comment":{"data":"--!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0337() {
    tokenize(
        r##"{"description":"<!----!--","initialState":"Data","input":"<!----!--","inputUtf16":[60,33,45,45,45,45,33,45,45],"output":[{"Comment":{"data":"--!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0338() {
    tokenize(
        r##"{"description":"<!----!-->","initialState":"Data","input":"<!----!-->","inputUtf16":[60,33,45,45,45,45,33,45,45,62],"output":[{"Comment":{"data":"--!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0339() {
    tokenize(
        r##"{"description":"<!----\"","initialState":"Data","input":"<!----\"","inputUtf16":[60,33,45,45,45,45,34],"output":[{"Comment":{"data":"--\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0340() {
    tokenize(
        r##"{"description":"<!----&","initialState":"Data","input":"<!----&","inputUtf16":[60,33,45,45,45,45,38],"output":[{"Comment":{"data":"--&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0341() {
    tokenize(
        r##"{"description":"<!----'","initialState":"Data","input":"<!----'","inputUtf16":[60,33,45,45,45,45,39],"output":[{"Comment":{"data":"--'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0342() {
    tokenize(
        r##"{"description":"<!----,","initialState":"Data","input":"<!----,","inputUtf16":[60,33,45,45,45,45,44],"output":[{"Comment":{"data":"--,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0343() {
    tokenize(
        r##"{"description":"<!-----","initialState":"Data","input":"<!-----","inputUtf16":[60,33,45,45,45,45,45],"output":[{"Comment":{"data":"-"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0344() {
    tokenize(
        r##"{"description":"<!----.","initialState":"Data","input":"<!----.","inputUtf16":[60,33,45,45,45,45,46],"output":[{"Comment":{"data":"--."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0345() {
    tokenize(
        r##"{"description":"<!----/","initialState":"Data","input":"<!----/","inputUtf16":[60,33,45,45,45,45,47],"output":[{"Comment":{"data":"--/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0346() {
    tokenize(
        r##"{"description":"<!----0","initialState":"Data","input":"<!----0","inputUtf16":[60,33,45,45,45,45,48],"output":[{"Comment":{"data":"--0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0347() {
    tokenize(
        r##"{"description":"<!----1","initialState":"Data","input":"<!----1","inputUtf16":[60,33,45,45,45,45,49],"output":[{"Comment":{"data":"--1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0348() {
    tokenize(
        r##"{"description":"<!----9","initialState":"Data","input":"<!----9","inputUtf16":[60,33,45,45,45,45,57],"output":[{"Comment":{"data":"--9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0349() {
    tokenize(
        r##"{"description":"<!----<","initialState":"Data","input":"<!----<","inputUtf16":[60,33,45,45,45,45,60],"output":[{"Comment":{"data":"--<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0350() {
    tokenize(
        r##"{"description":"<!----=","initialState":"Data","input":"<!----=","inputUtf16":[60,33,45,45,45,45,61],"output":[{"Comment":{"data":"--="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0351() {
    tokenize(
        r##"{"description":"<!---->","initialState":"Data","input":"<!---->","inputUtf16":[60,33,45,45,45,45,62],"output":[{"Comment":{"data":""}}],"errors":[]}"##,
    );
}

#[test]
fn test_0352() {
    tokenize(
        r##"{"description":"<!----?","initialState":"Data","input":"<!----?","inputUtf16":[60,33,45,45,45,45,63],"output":[{"Comment":{"data":"--?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0353() {
    tokenize(
        r##"{"description":"<!----@","initialState":"Data","input":"<!----@","inputUtf16":[60,33,45,45,45,45,64],"output":[{"Comment":{"data":"--@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0354() {
    tokenize(
        r##"{"description":"<!----A","initialState":"Data","input":"<!----A","inputUtf16":[60,33,45,45,45,45,65],"output":[{"Comment":{"data":"--A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0355() {
    tokenize(
        r##"{"description":"<!----B","initialState":"Data","input":"<!----B","inputUtf16":[60,33,45,45,45,45,66],"output":[{"Comment":{"data":"--B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0356() {
    tokenize(
        r##"{"description":"<!----Y","initialState":"Data","input":"<!----Y","inputUtf16":[60,33,45,45,45,45,89],"output":[{"Comment":{"data":"--Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0357() {
    tokenize(
        r##"{"description":"<!----Z","initialState":"Data","input":"<!----Z","inputUtf16":[60,33,45,45,45,45,90],"output":[{"Comment":{"data":"--Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0358() {
    tokenize(
        r##"{"description":"<!----`","initialState":"Data","input":"<!----`","inputUtf16":[60,33,45,45,45,45,96],"output":[{"Comment":{"data":"--`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0359() {
    tokenize(
        r##"{"description":"<!----a","initialState":"Data","input":"<!----a","inputUtf16":[60,33,45,45,45,45,97],"output":[{"Comment":{"data":"--a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0360() {
    tokenize(
        r##"{"description":"<!----b","initialState":"Data","input":"<!----b","inputUtf16":[60,33,45,45,45,45,98],"output":[{"Comment":{"data":"--b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0361() {
    tokenize(
        r##"{"description":"<!----y","initialState":"Data","input":"<!----y","inputUtf16":[60,33,45,45,45,45,121],"output":[{"Comment":{"data":"--y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0362() {
    tokenize(
        r##"{"description":"<!----z","initialState":"Data","input":"<!----z","inputUtf16":[60,33,45,45,45,45,122],"output":[{"Comment":{"data":"--z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0363() {
    tokenize(
        r##"{"description":"<!----{","initialState":"Data","input":"<!----{","inputUtf16":[60,33,45,45,45,45,123],"output":[{"Comment":{"data":"--{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0364() {
    tokenize(
        r##"{"description":"<!----\\uDBC0\\uDC00","initialState":"Data","input":"<!----􀀀","inputUtf16":[60,33,45,45,45,45,56256,56320],"output":[{"Comment":{"data":"--􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0365() {
    tokenize(
        r##"{"description":"<!---.","initialState":"Data","input":"<!---.","inputUtf16":[60,33,45,45,45,46],"output":[{"Comment":{"data":"-."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0366() {
    tokenize(
        r##"{"description":"<!---/","initialState":"Data","input":"<!---/","inputUtf16":[60,33,45,45,45,47],"output":[{"Comment":{"data":"-/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0367() {
    tokenize(
        r##"{"description":"<!---0","initialState":"Data","input":"<!---0","inputUtf16":[60,33,45,45,45,48],"output":[{"Comment":{"data":"-0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0368() {
    tokenize(
        r##"{"description":"<!---1","initialState":"Data","input":"<!---1","inputUtf16":[60,33,45,45,45,49],"output":[{"Comment":{"data":"-1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0369() {
    tokenize(
        r##"{"description":"<!---9","initialState":"Data","input":"<!---9","inputUtf16":[60,33,45,45,45,57],"output":[{"Comment":{"data":"-9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0370() {
    tokenize(
        r##"{"description":"<!---<","initialState":"Data","input":"<!---<","inputUtf16":[60,33,45,45,45,60],"output":[{"Comment":{"data":"-<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0371() {
    tokenize(
        r##"{"description":"<!---=","initialState":"Data","input":"<!---=","inputUtf16":[60,33,45,45,45,61],"output":[{"Comment":{"data":"-="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0372() {
    tokenize(
        r##"{"description":"<!---?","initialState":"Data","input":"<!---?","inputUtf16":[60,33,45,45,45,63],"output":[{"Comment":{"data":"-?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0373() {
    tokenize(
        r##"{"description":"<!---@","initialState":"Data","input":"<!---@","inputUtf16":[60,33,45,45,45,64],"output":[{"Comment":{"data":"-@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0374() {
    tokenize(
        r##"{"description":"<!---A","initialState":"Data","input":"<!---A","inputUtf16":[60,33,45,45,45,65],"output":[{"Comment":{"data":"-A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0375() {
    tokenize(
        r##"{"description":"<!---B","initialState":"Data","input":"<!---B","inputUtf16":[60,33,45,45,45,66],"output":[{"Comment":{"data":"-B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0376() {
    tokenize(
        r##"{"description":"<!---Y","initialState":"Data","input":"<!---Y","inputUtf16":[60,33,45,45,45,89],"output":[{"Comment":{"data":"-Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0377() {
    tokenize(
        r##"{"description":"<!---Z","initialState":"Data","input":"<!---Z","inputUtf16":[60,33,45,45,45,90],"output":[{"Comment":{"data":"-Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0378() {
    tokenize(
        r##"{"description":"<!---`","initialState":"Data","input":"<!---`","inputUtf16":[60,33,45,45,45,96],"output":[{"Comment":{"data":"-`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0379() {
    tokenize(
        r##"{"description":"<!---a","initialState":"Data","input":"<!---a","inputUtf16":[60,33,45,45,45,97],"output":[{"Comment":{"data":"-a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0380() {
    tokenize(
        r##"{"description":"<!---b","initialState":"Data","input":"<!---b","inputUtf16":[60,33,45,45,45,98],"output":[{"Comment":{"data":"-b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0381() {
    tokenize(
        r##"{"description":"<!---y","initialState":"Data","input":"<!---y","inputUtf16":[60,33,45,45,45,121],"output":[{"Comment":{"data":"-y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0382() {
    tokenize(
        r##"{"description":"<!---z","initialState":"Data","input":"<!---z","inputUtf16":[60,33,45,45,45,122],"output":[{"Comment":{"data":"-z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0383() {
    tokenize(
        r##"{"description":"<!---{","initialState":"Data","input":"<!---{","inputUtf16":[60,33,45,45,45,123],"output":[{"Comment":{"data":"-{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0384() {
    tokenize(
        r##"{"description":"<!---\\uDBC0\\uDC00","initialState":"Data","input":"<!---􀀀","inputUtf16":[60,33,45,45,45,56256,56320],"output":[{"Comment":{"data":"-􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0385() {
    tokenize(
        r##"{"description":"<!--.","initialState":"Data","input":"<!--.","inputUtf16":[60,33,45,45,46],"output":[{"Comment":{"data":"."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0386() {
    tokenize(
        r##"{"description":"<!--/","initialState":"Data","input":"<!--/","inputUtf16":[60,33,45,45,47],"output":[{"Comment":{"data":"/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0387() {
    tokenize(
        r##"{"description":"<!--0","initialState":"Data","input":"<!--0","inputUtf16":[60,33,45,45,48],"output":[{"Comment":{"data":"0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0388() {
    tokenize(
        r##"{"description":"<!--1","initialState":"Data","input":"<!--1","inputUtf16":[60,33,45,45,49],"output":[{"Comment":{"data":"1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0389() {
    tokenize(
        r##"{"description":"<!--9","initialState":"Data","input":"<!--9","inputUtf16":[60,33,45,45,57],"output":[{"Comment":{"data":"9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0390() {
    tokenize(
        r##"{"description":"<!--<","initialState":"Data","input":"<!--<","inputUtf16":[60,33,45,45,60],"output":[{"Comment":{"data":"<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0391() {
    tokenize(
        r##"{"description":"<!--=","initialState":"Data","input":"<!--=","inputUtf16":[60,33,45,45,61],"output":[{"Comment":{"data":"="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0392() {
    tokenize(
        r##"{"description":"<!--?","initialState":"Data","input":"<!--?","inputUtf16":[60,33,45,45,63],"output":[{"Comment":{"data":"?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0393() {
    tokenize(
        r##"{"description":"<!--@","initialState":"Data","input":"<!--@","inputUtf16":[60,33,45,45,64],"output":[{"Comment":{"data":"@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0394() {
    tokenize(
        r##"{"description":"<!--A","initialState":"Data","input":"<!--A","inputUtf16":[60,33,45,45,65],"output":[{"Comment":{"data":"A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0395() {
    tokenize(
        r##"{"description":"<!--B","initialState":"Data","input":"<!--B","inputUtf16":[60,33,45,45,66],"output":[{"Comment":{"data":"B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0396() {
    tokenize(
        r##"{"description":"<!--Y","initialState":"Data","input":"<!--Y","inputUtf16":[60,33,45,45,89],"output":[{"Comment":{"data":"Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0397() {
    tokenize(
        r##"{"description":"<!--Z","initialState":"Data","input":"<!--Z","inputUtf16":[60,33,45,45,90],"output":[{"Comment":{"data":"Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0398() {
    tokenize(
        r##"{"description":"<!--`","initialState":"Data","input":"<!--`","inputUtf16":[60,33,45,45,96],"output":[{"Comment":{"data":"`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0399() {
    tokenize(
        r##"{"description":"<!--a","initialState":"Data","input":"<!--a","inputUtf16":[60,33,45,45,97],"output":[{"Comment":{"data":"a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0400() {
    tokenize(
        r##"{"description":"<!--b","initialState":"Data","input":"<!--b","inputUtf16":[60,33,45,45,98],"output":[{"Comment":{"data":"b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0401() {
    tokenize(
        r##"{"description":"<!--y","initialState":"Data","input":"<!--y","inputUtf16":[60,33,45,45,121],"output":[{"Comment":{"data":"y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0402() {
    tokenize(
        r##"{"description":"<!--z","initialState":"Data","input":"<!--z","inputUtf16":[60,33,45,45,122],"output":[{"Comment":{"data":"z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0403() {
    tokenize(
        r##"{"description":"<!--{","initialState":"Data","input":"<!--{","inputUtf16":[60,33,45,45,123],"output":[{"Comment":{"data":"{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0404() {
    tokenize(
        r##"{"description":"<!--\\uDBC0\\uDC00","initialState":"Data","input":"<!--􀀀","inputUtf16":[60,33,45,45,56256,56320],"output":[{"Comment":{"data":"􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0405() {
    tokenize(
        r##"{"description":"<!/","initialState":"Data","input":"<!/","inputUtf16":[60,33,47],"output":[{"Comment":{"data":"/"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0406() {
    tokenize(
        r##"{"description":"<!0","initialState":"Data","input":"<!0","inputUtf16":[60,33,48],"output":[{"Comment":{"data":"0"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0407() {
    tokenize(
        r##"{"description":"<!1","initialState":"Data","input":"<!1","inputUtf16":[60,33,49],"output":[{"Comment":{"data":"1"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0408() {
    tokenize(
        r##"{"description":"<!9","initialState":"Data","input":"<!9","inputUtf16":[60,33,57],"output":[{"Comment":{"data":"9"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0409() {
    tokenize(
        r##"{"description":"<!<","initialState":"Data","input":"<!<","inputUtf16":[60,33,60],"output":[{"Comment":{"data":"<"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0410() {
    tokenize(
        r##"{"description":"<!=","initialState":"Data","input":"<!=","inputUtf16":[60,33,61],"output":[{"Comment":{"data":"="}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0411() {
    tokenize(
        r##"{"description":"<!>","initialState":"Data","input":"<!>","inputUtf16":[60,33,62],"output":[{"Comment":{"data":""}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0412() {
    tokenize(
        r##"{"description":"<!?","initialState":"Data","input":"<!?","inputUtf16":[60,33,63],"output":[{"Comment":{"data":"?"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0413() {
    tokenize(
        r##"{"description":"<!@","initialState":"Data","input":"<!@","inputUtf16":[60,33,64],"output":[{"Comment":{"data":"@"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0414() {
    tokenize(
        r##"{"description":"<!A","initialState":"Data","input":"<!A","inputUtf16":[60,33,65],"output":[{"Comment":{"data":"A"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0415() {
    tokenize(
        r##"{"description":"<!B","initialState":"Data","input":"<!B","inputUtf16":[60,33,66],"output":[{"Comment":{"data":"B"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0416() {
    tokenize(
        r##"{"description":"<!DOCTYPE","initialState":"Data","input":"<!DOCTYPE","inputUtf16":[60,33,68,79,67,84,89,80,69],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0417() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u0000","initialState":"Data","input":"<!DOCTYPE\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,0],"output":[{"Doctype":{"name":"�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0418() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u0008","initialState":"Data","input":"<!DOCTYPE\b","inputUtf16":[60,33,68,79,67,84,89,80,69,8],"output":[{"Doctype":{"name":"\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":10}},{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0419() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u0009","initialState":"Data","input":"<!DOCTYPE\t","inputUtf16":[60,33,68,79,67,84,89,80,69,9],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0420() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000A","initialState":"Data","input":"<!DOCTYPE\n","inputUtf16":[60,33,68,79,67,84,89,80,69,10],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0421() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000B","initialState":"Data","input":"<!DOCTYPE\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,11],"output":[{"Doctype":{"name":"\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":10}},{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0422() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000C","initialState":"Data","input":"<!DOCTYPE\f","inputUtf16":[60,33,68,79,67,84,89,80,69,12],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0423() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000D","initialState":"Data","input":"<!DOCTYPE\r","inputUtf16":[60,33,68,79,67,84,89,80,69,13],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0424() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u001F","initialState":"Data","input":"<!DOCTYPE\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,31],"output":[{"Doctype":{"name":"\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":10}},{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0425() {
    tokenize(
        r##"{"description":"<!DOCTYPE ","initialState":"Data","input":"<!DOCTYPE ","inputUtf16":[60,33,68,79,67,84,89,80,69,32],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0426() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u0000","initialState":"Data","input":"<!DOCTYPE \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,0],"output":[{"Doctype":{"name":"�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0427() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u0008","initialState":"Data","input":"<!DOCTYPE \b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,8],"output":[{"Doctype":{"name":"\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0428() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u0009","initialState":"Data","input":"<!DOCTYPE \t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,9],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0429() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000A","initialState":"Data","input":"<!DOCTYPE \n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,10],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0430() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000B","initialState":"Data","input":"<!DOCTYPE \u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,11],"output":[{"Doctype":{"name":"\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0431() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000C","initialState":"Data","input":"<!DOCTYPE \f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,12],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0432() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000D","initialState":"Data","input":"<!DOCTYPE \r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,13],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0433() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u001F","initialState":"Data","input":"<!DOCTYPE \u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,31],"output":[{"Doctype":{"name":"\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0434() {
    tokenize(
        r##"{"description":"<!DOCTYPE  ","initialState":"Data","input":"<!DOCTYPE  ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,32],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0435() {
    tokenize(
        r##"{"description":"<!DOCTYPE !","initialState":"Data","input":"<!DOCTYPE !","inputUtf16":[60,33,68,79,67,84,89,80,69,32,33],"output":[{"Doctype":{"name":"!","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0436() {
    tokenize(
        r##"{"description":"<!DOCTYPE \"","initialState":"Data","input":"<!DOCTYPE \"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,34],"output":[{"Doctype":{"name":"\"","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0437() {
    tokenize(
        r##"{"description":"<!DOCTYPE &","initialState":"Data","input":"<!DOCTYPE &","inputUtf16":[60,33,68,79,67,84,89,80,69,32,38],"output":[{"Doctype":{"name":"&","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0438() {
    tokenize(
        r##"{"description":"<!DOCTYPE '","initialState":"Data","input":"<!DOCTYPE '","inputUtf16":[60,33,68,79,67,84,89,80,69,32,39],"output":[{"Doctype":{"name":"'","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0439() {
    tokenize(
        r##"{"description":"<!DOCTYPE -","initialState":"Data","input":"<!DOCTYPE -","inputUtf16":[60,33,68,79,67,84,89,80,69,32,45],"output":[{"Doctype":{"name":"-","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0440() {
    tokenize(
        r##"{"description":"<!DOCTYPE /","initialState":"Data","input":"<!DOCTYPE /","inputUtf16":[60,33,68,79,67,84,89,80,69,32,47],"output":[{"Doctype":{"name":"/","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0441() {
    tokenize(
        r##"{"description":"<!DOCTYPE 0","initialState":"Data","input":"<!DOCTYPE 0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,48],"output":[{"Doctype":{"name":"0","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0442() {
    tokenize(
        r##"{"description":"<!DOCTYPE 1","initialState":"Data","input":"<!DOCTYPE 1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,49],"output":[{"Doctype":{"name":"1","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0443() {
    tokenize(
        r##"{"description":"<!DOCTYPE 9","initialState":"Data","input":"<!DOCTYPE 9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,57],"output":[{"Doctype":{"name":"9","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0444() {
    tokenize(
        r##"{"description":"<!DOCTYPE <","initialState":"Data","input":"<!DOCTYPE <","inputUtf16":[60,33,68,79,67,84,89,80,69,32,60],"output":[{"Doctype":{"name":"<","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0445() {
    tokenize(
        r##"{"description":"<!DOCTYPE =","initialState":"Data","input":"<!DOCTYPE =","inputUtf16":[60,33,68,79,67,84,89,80,69,32,61],"output":[{"Doctype":{"name":"=","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0446() {
    tokenize(
        r##"{"description":"<!DOCTYPE >","initialState":"Data","input":"<!DOCTYPE >","inputUtf16":[60,33,68,79,67,84,89,80,69,32,62],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypeName","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0447() {
    tokenize(
        r##"{"description":"<!DOCTYPE ?","initialState":"Data","input":"<!DOCTYPE ?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,63],"output":[{"Doctype":{"name":"?","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0448() {
    tokenize(
        r##"{"description":"<!DOCTYPE @","initialState":"Data","input":"<!DOCTYPE @","inputUtf16":[60,33,68,79,67,84,89,80,69,32,64],"output":[{"Doctype":{"name":"@","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0449() {
    tokenize(
        r##"{"description":"<!DOCTYPE A","initialState":"Data","input":"<!DOCTYPE A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0450() {
    tokenize(
        r##"{"description":"<!DOCTYPE B","initialState":"Data","input":"<!DOCTYPE B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,66],"output":[{"Doctype":{"name":"b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0451() {
    tokenize(
        r##"{"description":"<!DOCTYPE Y","initialState":"Data","input":"<!DOCTYPE Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,89],"output":[{"Doctype":{"name":"y","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0452() {
    tokenize(
        r##"{"description":"<!DOCTYPE Z","initialState":"Data","input":"<!DOCTYPE Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,90],"output":[{"Doctype":{"name":"z","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0453() {
    tokenize(
        r##"{"description":"<!DOCTYPE [","initialState":"Data","input":"<!DOCTYPE [","inputUtf16":[60,33,68,79,67,84,89,80,69,32,91],"output":[{"Doctype":{"name":"[","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0454() {
    tokenize(
        r##"{"description":"<!DOCTYPE `","initialState":"Data","input":"<!DOCTYPE `","inputUtf16":[60,33,68,79,67,84,89,80,69,32,96],"output":[{"Doctype":{"name":"`","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0455() {
    tokenize(
        r##"{"description":"<!DOCTYPE a","initialState":"Data","input":"<!DOCTYPE a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0456() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u0000","initialState":"Data","input":"<!DOCTYPE a\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,0],"output":[{"Doctype":{"name":"a�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0457() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u0008","initialState":"Data","input":"<!DOCTYPE a\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,8],"output":[{"Doctype":{"name":"a\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0458() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u0009","initialState":"Data","input":"<!DOCTYPE a\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0459() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000A","initialState":"Data","input":"<!DOCTYPE a\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0460() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000B","initialState":"Data","input":"<!DOCTYPE a\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,11],"output":[{"Doctype":{"name":"a\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0461() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000C","initialState":"Data","input":"<!DOCTYPE a\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0462() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000D","initialState":"Data","input":"<!DOCTYPE a\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0463() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u001F","initialState":"Data","input":"<!DOCTYPE a\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,31],"output":[{"Doctype":{"name":"a\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0464() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ","initialState":"Data","input":"<!DOCTYPE a ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0465() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u0000","initialState":"Data","input":"<!DOCTYPE a \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0466() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u0008","initialState":"Data","input":"<!DOCTYPE a \b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":13}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0467() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u0009","initialState":"Data","input":"<!DOCTYPE a \t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0468() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000A","initialState":"Data","input":"<!DOCTYPE a \n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0469() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000B","initialState":"Data","input":"<!DOCTYPE a \u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":13}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0470() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000C","initialState":"Data","input":"<!DOCTYPE a \f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0471() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000D","initialState":"Data","input":"<!DOCTYPE a \r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0472() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u001F","initialState":"Data","input":"<!DOCTYPE a \u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":13}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0473() {
    tokenize(
        r##"{"description":"<!DOCTYPE a  ","initialState":"Data","input":"<!DOCTYPE a  ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0474() {
    tokenize(
        r##"{"description":"<!DOCTYPE a !","initialState":"Data","input":"<!DOCTYPE a !","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0475() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \"","initialState":"Data","input":"<!DOCTYPE a \"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0476() {
    tokenize(
        r##"{"description":"<!DOCTYPE a &","initialState":"Data","input":"<!DOCTYPE a &","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0477() {
    tokenize(
        r##"{"description":"<!DOCTYPE a '","initialState":"Data","input":"<!DOCTYPE a '","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0478() {
    tokenize(
        r##"{"description":"<!DOCTYPE a -","initialState":"Data","input":"<!DOCTYPE a -","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0479() {
    tokenize(
        r##"{"description":"<!DOCTYPE a /","initialState":"Data","input":"<!DOCTYPE a /","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0480() {
    tokenize(
        r##"{"description":"<!DOCTYPE a 0","initialState":"Data","input":"<!DOCTYPE a 0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0481() {
    tokenize(
        r##"{"description":"<!DOCTYPE a 1","initialState":"Data","input":"<!DOCTYPE a 1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0482() {
    tokenize(
        r##"{"description":"<!DOCTYPE a 9","initialState":"Data","input":"<!DOCTYPE a 9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0483() {
    tokenize(
        r##"{"description":"<!DOCTYPE a <","initialState":"Data","input":"<!DOCTYPE a <","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0484() {
    tokenize(
        r##"{"description":"<!DOCTYPE a =","initialState":"Data","input":"<!DOCTYPE a =","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0485() {
    tokenize(
        r##"{"description":"<!DOCTYPE a >","initialState":"Data","input":"<!DOCTYPE a >","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0486() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ?","initialState":"Data","input":"<!DOCTYPE a ?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0487() {
    tokenize(
        r##"{"description":"<!DOCTYPE a @","initialState":"Data","input":"<!DOCTYPE a @","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0488() {
    tokenize(
        r##"{"description":"<!DOCTYPE a A","initialState":"Data","input":"<!DOCTYPE a A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0489() {
    tokenize(
        r##"{"description":"<!DOCTYPE a B","initialState":"Data","input":"<!DOCTYPE a B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0490() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC","initialState":"Data","input":"<!DOCTYPE a PUBLIC","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0491() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0492() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u0008","initialState":"Data","input":"<!DOCTYPE a PUBLIC\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0493() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0494() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0495() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0496() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0497() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000D","initialState":"Data","input":"<!DOCTYPE a PUBLIC\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0498() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u001F","initialState":"Data","input":"<!DOCTYPE a PUBLIC\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0499() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC ","initialState":"Data","input":"<!DOCTYPE a PUBLIC ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0500() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC!","initialState":"Data","input":"<!DOCTYPE a PUBLIC!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0501() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0502() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0503() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0504() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0505() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0506() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0507() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\" ","initialState":"Data","input":"<!DOCTYPE a PUBLIC\" ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0508() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"!","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0509() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0510() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\"\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,34,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0511() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\" \\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\" \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,34,32,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":22}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0512() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"#","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,35],"output":[{"Doctype":{"name":"a","public_id":"#","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0513() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"&","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0514() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"'","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,39],"output":[{"Doctype":{"name":"a","public_id":"'","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0515() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"-","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0516() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"/","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0517() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"0","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0518() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"1","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0519() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"9","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0520() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"<","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0521() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"=","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0522() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\">","initialState":"Data","input":"<!DOCTYPE a PUBLIC\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0523() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"?","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,63],"output":[{"Doctype":{"name":"a","public_id":"?","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0524() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"@","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,64],"output":[{"Doctype":{"name":"a","public_id":"@","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0525() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"A","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,65],"output":[{"Doctype":{"name":"a","public_id":"A","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0526() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"B","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,66],"output":[{"Doctype":{"name":"a","public_id":"B","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0527() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"Y","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,89],"output":[{"Doctype":{"name":"a","public_id":"Y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0528() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"Z","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,90],"output":[{"Doctype":{"name":"a","public_id":"Z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0529() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"`","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,96],"output":[{"Doctype":{"name":"a","public_id":"`","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0530() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"a","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,97],"output":[{"Doctype":{"name":"a","public_id":"a","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0531() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"b","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,98],"output":[{"Doctype":{"name":"a","public_id":"b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0532() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"y","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,121],"output":[{"Doctype":{"name":"a","public_id":"y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0533() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"z","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,122],"output":[{"Doctype":{"name":"a","public_id":"z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0534() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"{","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,123],"output":[{"Doctype":{"name":"a","public_id":"{","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0535() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"􀀀","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0536() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC#","initialState":"Data","input":"<!DOCTYPE a PUBLIC#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0537() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC&","initialState":"Data","input":"<!DOCTYPE a PUBLIC&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0538() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'","initialState":"Data","input":"<!DOCTYPE a PUBLIC'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0539() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0540() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0541() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0542() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0543() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0544() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC' ","initialState":"Data","input":"<!DOCTYPE a PUBLIC' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0545() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'!","initialState":"Data","input":"<!DOCTYPE a PUBLIC'!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0546() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,34],"output":[{"Doctype":{"name":"a","public_id":"\"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0547() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'&","initialState":"Data","input":"<!DOCTYPE a PUBLIC'&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0548() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''","initialState":"Data","input":"<!DOCTYPE a PUBLIC''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0549() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0550() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u0008","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,8],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0551() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,9],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0552() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,10],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0553() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,11],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0554() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,12],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0555() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000D","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,13],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0556() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u001F","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,31],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0557() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'' ","initialState":"Data","input":"<!DOCTYPE a PUBLIC'' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,32],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0558() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''!","initialState":"Data","input":"<!DOCTYPE a PUBLIC''!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,33],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0559() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0560() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''#","initialState":"Data","input":"<!DOCTYPE a PUBLIC''#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,35],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0561() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''&","initialState":"Data","input":"<!DOCTYPE a PUBLIC''&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,38],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0562() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'''","initialState":"Data","input":"<!DOCTYPE a PUBLIC'''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0563() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''''\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC''''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":23}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":23}}]}"##,
    );
}

#[test]
fn test_0564() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''''x\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC''''x\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,120,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":23}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_0565() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'''' \\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC'''' \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,32,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":24}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_0566() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'''' x\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC'''' x\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,32,120,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":24}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":25}}]}"##,
    );
}

#[test]
fn test_0567() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''(","initialState":"Data","input":"<!DOCTYPE a PUBLIC''(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,40],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0568() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''-","initialState":"Data","input":"<!DOCTYPE a PUBLIC''-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,45],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0569() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''/","initialState":"Data","input":"<!DOCTYPE a PUBLIC''/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,47],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0570() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''0","initialState":"Data","input":"<!DOCTYPE a PUBLIC''0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,48],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0571() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''1","initialState":"Data","input":"<!DOCTYPE a PUBLIC''1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,49],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0572() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''9","initialState":"Data","input":"<!DOCTYPE a PUBLIC''9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,57],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0573() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''<","initialState":"Data","input":"<!DOCTYPE a PUBLIC''<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,60],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0574() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''=","initialState":"Data","input":"<!DOCTYPE a PUBLIC''=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,61],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0575() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''>","initialState":"Data","input":"<!DOCTYPE a PUBLIC''>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0576() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''?","initialState":"Data","input":"<!DOCTYPE a PUBLIC''?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,63],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0577() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''@","initialState":"Data","input":"<!DOCTYPE a PUBLIC''@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,64],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0578() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''A","initialState":"Data","input":"<!DOCTYPE a PUBLIC''A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,65],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0579() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''B","initialState":"Data","input":"<!DOCTYPE a PUBLIC''B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,66],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0580() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''Y","initialState":"Data","input":"<!DOCTYPE a PUBLIC''Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,89],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0581() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''Z","initialState":"Data","input":"<!DOCTYPE a PUBLIC''Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,90],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0582() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''`","initialState":"Data","input":"<!DOCTYPE a PUBLIC''`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,96],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0583() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''a","initialState":"Data","input":"<!DOCTYPE a PUBLIC''a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,97],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0584() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''b","initialState":"Data","input":"<!DOCTYPE a PUBLIC''b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,98],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0585() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''y","initialState":"Data","input":"<!DOCTYPE a PUBLIC''y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,121],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0586() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''z","initialState":"Data","input":"<!DOCTYPE a PUBLIC''z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,122],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0587() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''{","initialState":"Data","input":"<!DOCTYPE a PUBLIC''{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,123],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0588() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC''􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0589() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'(","initialState":"Data","input":"<!DOCTYPE a PUBLIC'(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,40],"output":[{"Doctype":{"name":"a","public_id":"(","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0590() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'-","initialState":"Data","input":"<!DOCTYPE a PUBLIC'-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0591() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'/","initialState":"Data","input":"<!DOCTYPE a PUBLIC'/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0592() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'0","initialState":"Data","input":"<!DOCTYPE a PUBLIC'0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0593() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'1","initialState":"Data","input":"<!DOCTYPE a PUBLIC'1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0594() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'9","initialState":"Data","input":"<!DOCTYPE a PUBLIC'9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0595() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'<","initialState":"Data","input":"<!DOCTYPE a PUBLIC'<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0596() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'=","initialState":"Data","input":"<!DOCTYPE a PUBLIC'=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0597() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'>","initialState":"Data","input":"<!DOCTYPE a PUBLIC'>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0598() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'?","initialState":"Data","input":"<!DOCTYPE a PUBLIC'?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,63],"output":[{"Doctype":{"name":"a","public_id":"?","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0599() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'@","initialState":"Data","input":"<!DOCTYPE a PUBLIC'@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,64],"output":[{"Doctype":{"name":"a","public_id":"@","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0600() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'A","initialState":"Data","input":"<!DOCTYPE a PUBLIC'A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,65],"output":[{"Doctype":{"name":"a","public_id":"A","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0601() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'B","initialState":"Data","input":"<!DOCTYPE a PUBLIC'B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,66],"output":[{"Doctype":{"name":"a","public_id":"B","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0602() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'Y","initialState":"Data","input":"<!DOCTYPE a PUBLIC'Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,89],"output":[{"Doctype":{"name":"a","public_id":"Y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0603() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'Z","initialState":"Data","input":"<!DOCTYPE a PUBLIC'Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,90],"output":[{"Doctype":{"name":"a","public_id":"Z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0604() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'`","initialState":"Data","input":"<!DOCTYPE a PUBLIC'`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,96],"output":[{"Doctype":{"name":"a","public_id":"`","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0605() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'a","initialState":"Data","input":"<!DOCTYPE a PUBLIC'a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,97],"output":[{"Doctype":{"name":"a","public_id":"a","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0606() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'b","initialState":"Data","input":"<!DOCTYPE a PUBLIC'b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,98],"output":[{"Doctype":{"name":"a","public_id":"b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0607() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'y","initialState":"Data","input":"<!DOCTYPE a PUBLIC'y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,121],"output":[{"Doctype":{"name":"a","public_id":"y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0608() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'z","initialState":"Data","input":"<!DOCTYPE a PUBLIC'z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,122],"output":[{"Doctype":{"name":"a","public_id":"z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0609() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'{","initialState":"Data","input":"<!DOCTYPE a PUBLIC'{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,123],"output":[{"Doctype":{"name":"a","public_id":"{","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0610() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC'􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"􀀀","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0611() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC(","initialState":"Data","input":"<!DOCTYPE a PUBLIC(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0612() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC-","initialState":"Data","input":"<!DOCTYPE a PUBLIC-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0613() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC/","initialState":"Data","input":"<!DOCTYPE a PUBLIC/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0614() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC0","initialState":"Data","input":"<!DOCTYPE a PUBLIC0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0615() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC1","initialState":"Data","input":"<!DOCTYPE a PUBLIC1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0616() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC9","initialState":"Data","input":"<!DOCTYPE a PUBLIC9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0617() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC<","initialState":"Data","input":"<!DOCTYPE a PUBLIC<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0618() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC=","initialState":"Data","input":"<!DOCTYPE a PUBLIC=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0619() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC>","initialState":"Data","input":"<!DOCTYPE a PUBLIC>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0620() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC?","initialState":"Data","input":"<!DOCTYPE a PUBLIC?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0621() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC@","initialState":"Data","input":"<!DOCTYPE a PUBLIC@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0622() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICA","initialState":"Data","input":"<!DOCTYPE a PUBLICA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0623() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICB","initialState":"Data","input":"<!DOCTYPE a PUBLICB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0624() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICY","initialState":"Data","input":"<!DOCTYPE a PUBLICY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0625() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICZ","initialState":"Data","input":"<!DOCTYPE a PUBLICZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0626() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC`","initialState":"Data","input":"<!DOCTYPE a PUBLIC`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0627() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICa","initialState":"Data","input":"<!DOCTYPE a PUBLICa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0628() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICb","initialState":"Data","input":"<!DOCTYPE a PUBLICb","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0629() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICy","initialState":"Data","input":"<!DOCTYPE a PUBLICy","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0630() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICz","initialState":"Data","input":"<!DOCTYPE a PUBLICz","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0631() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC{","initialState":"Data","input":"<!DOCTYPE a PUBLIC{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0632() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0633() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM","initialState":"Data","input":"<!DOCTYPE a SYSTEM","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0634() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0635() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM \\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0636() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM x\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0637() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u0008","initialState":"Data","input":"<!DOCTYPE a SYSTEM\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0638() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0639() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0640() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0641() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0642() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000D","initialState":"Data","input":"<!DOCTYPE a SYSTEM\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0643() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u001F","initialState":"Data","input":"<!DOCTYPE a SYSTEM\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0644() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM ","initialState":"Data","input":"<!DOCTYPE a SYSTEM ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0645() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM!","initialState":"Data","input":"<!DOCTYPE a SYSTEM!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0646() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0647() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"�","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0648() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\t","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0649() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\n","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0650() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\u000b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0651() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\f","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0652() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\" ","initialState":"Data","input":"<!DOCTYPE a SYSTEM\" ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":" ","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0653() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"!","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"!","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0654() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0655() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"#","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"#","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0656() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"&","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"&","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0657() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"'","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"'","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0658() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"-","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"-","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0659() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"/","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"/","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0660() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"0","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"0","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0661() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"1","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"1","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0662() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"9","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"9","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0663() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"<","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"<","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0664() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"=","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"=","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0665() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\">","initialState":"Data","input":"<!DOCTYPE a SYSTEM\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0666() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"?","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"?","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0667() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"@","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"@","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0668() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"A","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"A","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0669() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"B","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"B","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0670() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"Y","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0671() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"Z","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0672() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"`","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"`","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0673() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"a","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"a","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0674() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"b","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0675() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"y","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0676() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"z","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0677() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"{","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"{","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0678() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"􀀀","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0679() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM#","initialState":"Data","input":"<!DOCTYPE a SYSTEM#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0680() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM&","initialState":"Data","input":"<!DOCTYPE a SYSTEM&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0681() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'","initialState":"Data","input":"<!DOCTYPE a SYSTEM'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0682() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"�","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0683() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\t","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0684() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\n","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0685() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\u000b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0686() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\f","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0687() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM' ","initialState":"Data","input":"<!DOCTYPE a SYSTEM' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":" ","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0688() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'!","initialState":"Data","input":"<!DOCTYPE a SYSTEM'!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"!","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0689() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0690() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'&","initialState":"Data","input":"<!DOCTYPE a SYSTEM'&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"&","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0691() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''","initialState":"Data","input":"<!DOCTYPE a SYSTEM''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0692() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0693() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u0008","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0694() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0695() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0696() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0697() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0698() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000D","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0699() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u001F","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0700() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'' ","initialState":"Data","input":"<!DOCTYPE a SYSTEM'' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0701() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'' \\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM'' \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":22}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0702() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'' x\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM'' x\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,32,120,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":22}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":23}}]}"##,
    );
}

#[test]
fn test_0703() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''!","initialState":"Data","input":"<!DOCTYPE a SYSTEM''!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0704() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0705() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''&","initialState":"Data","input":"<!DOCTYPE a SYSTEM''&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0706() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'''","initialState":"Data","input":"<!DOCTYPE a SYSTEM'''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0707() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''-","initialState":"Data","input":"<!DOCTYPE a SYSTEM''-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0708() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''/","initialState":"Data","input":"<!DOCTYPE a SYSTEM''/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0709() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''0","initialState":"Data","input":"<!DOCTYPE a SYSTEM''0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0710() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''1","initialState":"Data","input":"<!DOCTYPE a SYSTEM''1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0711() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''9","initialState":"Data","input":"<!DOCTYPE a SYSTEM''9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0712() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''<","initialState":"Data","input":"<!DOCTYPE a SYSTEM''<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0713() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''=","initialState":"Data","input":"<!DOCTYPE a SYSTEM''=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0714() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''>","initialState":"Data","input":"<!DOCTYPE a SYSTEM''>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0715() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''?","initialState":"Data","input":"<!DOCTYPE a SYSTEM''?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0716() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''@","initialState":"Data","input":"<!DOCTYPE a SYSTEM''@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0717() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''A","initialState":"Data","input":"<!DOCTYPE a SYSTEM''A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0718() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''B","initialState":"Data","input":"<!DOCTYPE a SYSTEM''B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0719() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''Y","initialState":"Data","input":"<!DOCTYPE a SYSTEM''Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0720() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''Z","initialState":"Data","input":"<!DOCTYPE a SYSTEM''Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0721() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''`","initialState":"Data","input":"<!DOCTYPE a SYSTEM''`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0722() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''a","initialState":"Data","input":"<!DOCTYPE a SYSTEM''a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0723() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''b","initialState":"Data","input":"<!DOCTYPE a SYSTEM''b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0724() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''y","initialState":"Data","input":"<!DOCTYPE a SYSTEM''y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0725() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''z","initialState":"Data","input":"<!DOCTYPE a SYSTEM''z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0726() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''{","initialState":"Data","input":"<!DOCTYPE a SYSTEM''{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0727() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM''􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0728() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'(","initialState":"Data","input":"<!DOCTYPE a SYSTEM'(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"(","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0729() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'-","initialState":"Data","input":"<!DOCTYPE a SYSTEM'-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"-","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0730() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'/","initialState":"Data","input":"<!DOCTYPE a SYSTEM'/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"/","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0731() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'0","initialState":"Data","input":"<!DOCTYPE a SYSTEM'0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"0","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0732() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'1","initialState":"Data","input":"<!DOCTYPE a SYSTEM'1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"1","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0733() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'9","initialState":"Data","input":"<!DOCTYPE a SYSTEM'9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"9","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0734() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'<","initialState":"Data","input":"<!DOCTYPE a SYSTEM'<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"<","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0735() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'=","initialState":"Data","input":"<!DOCTYPE a SYSTEM'=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"=","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0736() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'>","initialState":"Data","input":"<!DOCTYPE a SYSTEM'>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0737() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'?","initialState":"Data","input":"<!DOCTYPE a SYSTEM'?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"?","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0738() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'@","initialState":"Data","input":"<!DOCTYPE a SYSTEM'@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"@","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0739() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'A","initialState":"Data","input":"<!DOCTYPE a SYSTEM'A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"A","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0740() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'B","initialState":"Data","input":"<!DOCTYPE a SYSTEM'B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"B","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0741() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'Y","initialState":"Data","input":"<!DOCTYPE a SYSTEM'Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0742() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'Z","initialState":"Data","input":"<!DOCTYPE a SYSTEM'Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0743() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'`","initialState":"Data","input":"<!DOCTYPE a SYSTEM'`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"`","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0744() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'a","initialState":"Data","input":"<!DOCTYPE a SYSTEM'a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"a","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0745() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'b","initialState":"Data","input":"<!DOCTYPE a SYSTEM'b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0746() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'y","initialState":"Data","input":"<!DOCTYPE a SYSTEM'y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0747() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'z","initialState":"Data","input":"<!DOCTYPE a SYSTEM'z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0748() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'{","initialState":"Data","input":"<!DOCTYPE a SYSTEM'{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"{","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0749() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM'􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"􀀀","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0750() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM(","initialState":"Data","input":"<!DOCTYPE a SYSTEM(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0751() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM-","initialState":"Data","input":"<!DOCTYPE a SYSTEM-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0752() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM/","initialState":"Data","input":"<!DOCTYPE a SYSTEM/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0753() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM0","initialState":"Data","input":"<!DOCTYPE a SYSTEM0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0754() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM1","initialState":"Data","input":"<!DOCTYPE a SYSTEM1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0755() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM9","initialState":"Data","input":"<!DOCTYPE a SYSTEM9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0756() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM<","initialState":"Data","input":"<!DOCTYPE a SYSTEM<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0757() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM=","initialState":"Data","input":"<!DOCTYPE a SYSTEM=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0758() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM>","initialState":"Data","input":"<!DOCTYPE a SYSTEM>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0759() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM?","initialState":"Data","input":"<!DOCTYPE a SYSTEM?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0760() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM@","initialState":"Data","input":"<!DOCTYPE a SYSTEM@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0761() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMA","initialState":"Data","input":"<!DOCTYPE a SYSTEMA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0762() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMB","initialState":"Data","input":"<!DOCTYPE a SYSTEMB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0763() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMY","initialState":"Data","input":"<!DOCTYPE a SYSTEMY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0764() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMZ","initialState":"Data","input":"<!DOCTYPE a SYSTEMZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0765() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM`","initialState":"Data","input":"<!DOCTYPE a SYSTEM`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0766() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMa","initialState":"Data","input":"<!DOCTYPE a SYSTEMa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0767() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMb","initialState":"Data","input":"<!DOCTYPE a SYSTEMb","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0768() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMy","initialState":"Data","input":"<!DOCTYPE a SYSTEMy","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0769() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMz","initialState":"Data","input":"<!DOCTYPE a SYSTEMz","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0770() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM{","initialState":"Data","input":"<!DOCTYPE a SYSTEM{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0771() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0772() {
    tokenize(
        r##"{"description":"<!DOCTYPE a Y","initialState":"Data","input":"<!DOCTYPE a Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0773() {
    tokenize(
        r##"{"description":"<!DOCTYPE a Z","initialState":"Data","input":"<!DOCTYPE a Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0774() {
    tokenize(
        r##"{"description":"<!DOCTYPE a `","initialState":"Data","input":"<!DOCTYPE a `","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0775() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a","initialState":"Data","input":"<!DOCTYPE a a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0776() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u0000","initialState":"Data","input":"<!DOCTYPE a a\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0777() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u0009","initialState":"Data","input":"<!DOCTYPE a a\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0778() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u000A","initialState":"Data","input":"<!DOCTYPE a a\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0779() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u000B","initialState":"Data","input":"<!DOCTYPE a a\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0780() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u000C","initialState":"Data","input":"<!DOCTYPE a a\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0781() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a ","initialState":"Data","input":"<!DOCTYPE a a ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0782() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a!","initialState":"Data","input":"<!DOCTYPE a a!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0783() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\"","initialState":"Data","input":"<!DOCTYPE a a\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0784() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a&","initialState":"Data","input":"<!DOCTYPE a a&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0785() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a'","initialState":"Data","input":"<!DOCTYPE a a'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0786() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a-","initialState":"Data","input":"<!DOCTYPE a a-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0787() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a/","initialState":"Data","input":"<!DOCTYPE a a/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0788() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a0","initialState":"Data","input":"<!DOCTYPE a a0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0789() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a1","initialState":"Data","input":"<!DOCTYPE a a1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0790() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a9","initialState":"Data","input":"<!DOCTYPE a a9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0791() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a<","initialState":"Data","input":"<!DOCTYPE a a<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0792() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a=","initialState":"Data","input":"<!DOCTYPE a a=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0793() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a>","initialState":"Data","input":"<!DOCTYPE a a>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0794() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a?","initialState":"Data","input":"<!DOCTYPE a a?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0795() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a@","initialState":"Data","input":"<!DOCTYPE a a@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0796() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aA","initialState":"Data","input":"<!DOCTYPE a aA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0797() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aB","initialState":"Data","input":"<!DOCTYPE a aB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0798() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aY","initialState":"Data","input":"<!DOCTYPE a aY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0799() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aZ","initialState":"Data","input":"<!DOCTYPE a aZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0800() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a`","initialState":"Data","input":"<!DOCTYPE a a`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0801() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aa","initialState":"Data","input":"<!DOCTYPE a aa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0802() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ab","initialState":"Data","input":"<!DOCTYPE a ab","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0803() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ay","initialState":"Data","input":"<!DOCTYPE a ay","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0804() {
    tokenize(
        r##"{"description":"<!DOCTYPE a az","initialState":"Data","input":"<!DOCTYPE a az","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0805() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a{","initialState":"Data","input":"<!DOCTYPE a a{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0806() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a a􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0807() {
    tokenize(
        r##"{"description":"<!DOCTYPE a b","initialState":"Data","input":"<!DOCTYPE a b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0808() {
    tokenize(
        r##"{"description":"<!DOCTYPE a y","initialState":"Data","input":"<!DOCTYPE a y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0809() {
    tokenize(
        r##"{"description":"<!DOCTYPE a z","initialState":"Data","input":"<!DOCTYPE a z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0810() {
    tokenize(
        r##"{"description":"<!DOCTYPE a {","initialState":"Data","input":"<!DOCTYPE a {","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0811() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a 􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0812() {
    tokenize(
        r##"{"description":"<!DOCTYPE a!","initialState":"Data","input":"<!DOCTYPE a!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,33],"output":[{"Doctype":{"name":"a!","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0813() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\"","initialState":"Data","input":"<!DOCTYPE a\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,34],"output":[{"Doctype":{"name":"a\"","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0814() {
    tokenize(
        r##"{"description":"<!DOCTYPE a&","initialState":"Data","input":"<!DOCTYPE a&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,38],"output":[{"Doctype":{"name":"a&","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0815() {
    tokenize(
        r##"{"description":"<!DOCTYPE a'","initialState":"Data","input":"<!DOCTYPE a'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,39],"output":[{"Doctype":{"name":"a'","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0816() {
    tokenize(
        r##"{"description":"<!DOCTYPE a-","initialState":"Data","input":"<!DOCTYPE a-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,45],"output":[{"Doctype":{"name":"a-","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0817() {
    tokenize(
        r##"{"description":"<!DOCTYPE a/","initialState":"Data","input":"<!DOCTYPE a/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,47],"output":[{"Doctype":{"name":"a/","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0818() {
    tokenize(
        r##"{"description":"<!DOCTYPE a0","initialState":"Data","input":"<!DOCTYPE a0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,48],"output":[{"Doctype":{"name":"a0","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0819() {
    tokenize(
        r##"{"description":"<!DOCTYPE a1","initialState":"Data","input":"<!DOCTYPE a1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,49],"output":[{"Doctype":{"name":"a1","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0820() {
    tokenize(
        r##"{"description":"<!DOCTYPE a9","initialState":"Data","input":"<!DOCTYPE a9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,57],"output":[{"Doctype":{"name":"a9","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0821() {
    tokenize(
        r##"{"description":"<!DOCTYPE a<","initialState":"Data","input":"<!DOCTYPE a<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,60],"output":[{"Doctype":{"name":"a<","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0822() {
    tokenize(
        r##"{"description":"<!DOCTYPE a=","initialState":"Data","input":"<!DOCTYPE a=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,61],"output":[{"Doctype":{"name":"a=","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0823() {
    tokenize(
        r##"{"description":"<!DOCTYPE a>","initialState":"Data","input":"<!DOCTYPE a>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0824() {
    tokenize(
        r##"{"description":"<!DOCTYPE a?","initialState":"Data","input":"<!DOCTYPE a?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,63],"output":[{"Doctype":{"name":"a?","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0825() {
    tokenize(
        r##"{"description":"<!DOCTYPE a@","initialState":"Data","input":"<!DOCTYPE a@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,64],"output":[{"Doctype":{"name":"a@","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0826() {
    tokenize(
        r##"{"description":"<!DOCTYPE aA","initialState":"Data","input":"<!DOCTYPE aA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,65],"output":[{"Doctype":{"name":"aa","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0827() {
    tokenize(
        r##"{"description":"<!DOCTYPE aB","initialState":"Data","input":"<!DOCTYPE aB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,66],"output":[{"Doctype":{"name":"ab","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0828() {
    tokenize(
        r##"{"description":"<!DOCTYPE aY","initialState":"Data","input":"<!DOCTYPE aY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,89],"output":[{"Doctype":{"name":"ay","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0829() {
    tokenize(
        r##"{"description":"<!DOCTYPE aZ","initialState":"Data","input":"<!DOCTYPE aZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,90],"output":[{"Doctype":{"name":"az","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0830() {
    tokenize(
        r##"{"description":"<!DOCTYPE a[","initialState":"Data","input":"<!DOCTYPE a[","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,91],"output":[{"Doctype":{"name":"a[","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0831() {
    tokenize(
        r##"{"description":"<!DOCTYPE a`","initialState":"Data","input":"<!DOCTYPE a`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,96],"output":[{"Doctype":{"name":"a`","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0832() {
    tokenize(
        r##"{"description":"<!DOCTYPE aa","initialState":"Data","input":"<!DOCTYPE aa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,97],"output":[{"Doctype":{"name":"aa","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0833() {
    tokenize(
        r##"{"description":"<!DOCTYPE ab","initialState":"Data","input":"<!DOCTYPE ab","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,98],"output":[{"Doctype":{"name":"ab","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0834() {
    tokenize(
        r##"{"description":"<!DOCTYPE ay","initialState":"Data","input":"<!DOCTYPE ay","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,121],"output":[{"Doctype":{"name":"ay","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0835() {
    tokenize(
        r##"{"description":"<!DOCTYPE az","initialState":"Data","input":"<!DOCTYPE az","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,122],"output":[{"Doctype":{"name":"az","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0836() {
    tokenize(
        r##"{"description":"<!DOCTYPE a{","initialState":"Data","input":"<!DOCTYPE a{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,123],"output":[{"Doctype":{"name":"a{","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0837() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,56256,56320],"output":[{"Doctype":{"name":"a􀀀","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0838() {
    tokenize(
        r##"{"description":"<!DOCTYPE b","initialState":"Data","input":"<!DOCTYPE b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,98],"output":[{"Doctype":{"name":"b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0839() {
    tokenize(
        r##"{"description":"<!DOCTYPE y","initialState":"Data","input":"<!DOCTYPE y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,121],"output":[{"Doctype":{"name":"y","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0840() {
    tokenize(
        r##"{"description":"<!DOCTYPE z","initialState":"Data","input":"<!DOCTYPE z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,122],"output":[{"Doctype":{"name":"z","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0841() {
    tokenize(
        r##"{"description":"<!DOCTYPE {","initialState":"Data","input":"<!DOCTYPE {","inputUtf16":[60,33,68,79,67,84,89,80,69,32,123],"output":[{"Doctype":{"name":"{","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0842() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE 􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,56256,56320],"output":[{"Doctype":{"name":"􀀀","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0843() {
    tokenize(
        r##"{"description":"<!DOCTYPE!","initialState":"Data","input":"<!DOCTYPE!","inputUtf16":[60,33,68,79,67,84,89,80,69,33],"output":[{"Doctype":{"name":"!","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0844() {
    tokenize(
        r##"{"description":"<!DOCTYPE\"","initialState":"Data","input":"<!DOCTYPE\"","inputUtf16":[60,33,68,79,67,84,89,80,69,34],"output":[{"Doctype":{"name":"\"","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0845() {
    tokenize(
        r##"{"description":"<!DOCTYPE&","initialState":"Data","input":"<!DOCTYPE&","inputUtf16":[60,33,68,79,67,84,89,80,69,38],"output":[{"Doctype":{"name":"&","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0846() {
    tokenize(
        r##"{"description":"<!DOCTYPE'","initialState":"Data","input":"<!DOCTYPE'","inputUtf16":[60,33,68,79,67,84,89,80,69,39],"output":[{"Doctype":{"name":"'","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0847() {
    tokenize(
        r##"{"description":"<!DOCTYPE-","initialState":"Data","input":"<!DOCTYPE-","inputUtf16":[60,33,68,79,67,84,89,80,69,45],"output":[{"Doctype":{"name":"-","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0848() {
    tokenize(
        r##"{"description":"<!DOCTYPE/","initialState":"Data","input":"<!DOCTYPE/","inputUtf16":[60,33,68,79,67,84,89,80,69,47],"output":[{"Doctype":{"name":"/","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0849() {
    tokenize(
        r##"{"description":"<!DOCTYPE0","initialState":"Data","input":"<!DOCTYPE0","inputUtf16":[60,33,68,79,67,84,89,80,69,48],"output":[{"Doctype":{"name":"0","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0850() {
    tokenize(
        r##"{"description":"<!DOCTYPE1","initialState":"Data","input":"<!DOCTYPE1","inputUtf16":[60,33,68,79,67,84,89,80,69,49],"output":[{"Doctype":{"name":"1","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0851() {
    tokenize(
        r##"{"description":"<!DOCTYPE9","initialState":"Data","input":"<!DOCTYPE9","inputUtf16":[60,33,68,79,67,84,89,80,69,57],"output":[{"Doctype":{"name":"9","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0852() {
    tokenize(
        r##"{"description":"<!DOCTYPE<","initialState":"Data","input":"<!DOCTYPE<","inputUtf16":[60,33,68,79,67,84,89,80,69,60],"output":[{"Doctype":{"name":"<","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0853() {
    tokenize(
        r##"{"description":"<!DOCTYPE=","initialState":"Data","input":"<!DOCTYPE=","inputUtf16":[60,33,68,79,67,84,89,80,69,61],"output":[{"Doctype":{"name":"=","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0854() {
    tokenize(
        r##"{"description":"<!DOCTYPE>","initialState":"Data","input":"<!DOCTYPE>","inputUtf16":[60,33,68,79,67,84,89,80,69,62],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypeName","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0855() {
    tokenize(
        r##"{"description":"<!DOCTYPE?","initialState":"Data","input":"<!DOCTYPE?","inputUtf16":[60,33,68,79,67,84,89,80,69,63],"output":[{"Doctype":{"name":"?","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0856() {
    tokenize(
        r##"{"description":"<!DOCTYPE@","initialState":"Data","input":"<!DOCTYPE@","inputUtf16":[60,33,68,79,67,84,89,80,69,64],"output":[{"Doctype":{"name":"@","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0857() {
    tokenize(
        r##"{"description":"<!DOCTYPEA","initialState":"Data","input":"<!DOCTYPEA","inputUtf16":[60,33,68,79,67,84,89,80,69,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0858() {
    tokenize(
        r##"{"description":"<!DOCTYPEB","initialState":"Data","input":"<!DOCTYPEB","inputUtf16":[60,33,68,79,67,84,89,80,69,66],"output":[{"Doctype":{"name":"b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0859() {
    tokenize(
        r##"{"description":"<!DOCTYPEY","initialState":"Data","input":"<!DOCTYPEY","inputUtf16":[60,33,68,79,67,84,89,80,69,89],"output":[{"Doctype":{"name":"y","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0860() {
    tokenize(
        r##"{"description":"<!DOCTYPEZ","initialState":"Data","input":"<!DOCTYPEZ","inputUtf16":[60,33,68,79,67,84,89,80,69,90],"output":[{"Doctype":{"name":"z","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0861() {
    tokenize(
        r##"{"description":"<!DOCTYPE`","initialState":"Data","input":"<!DOCTYPE`","inputUtf16":[60,33,68,79,67,84,89,80,69,96],"output":[{"Doctype":{"name":"`","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0862() {
    tokenize(
        r##"{"description":"<!DOCTYPEa","initialState":"Data","input":"<!DOCTYPEa","inputUtf16":[60,33,68,79,67,84,89,80,69,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0863() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u0000","initialState":"Data","input":"<!DOCTYPEa\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,0],"output":[{"Doctype":{"name":"a�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0864() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u0008","initialState":"Data","input":"<!DOCTYPEa\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,8],"output":[{"Doctype":{"name":"a\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0865() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u0009","initialState":"Data","input":"<!DOCTYPEa\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0866() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000A","initialState":"Data","input":"<!DOCTYPEa\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0867() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000B","initialState":"Data","input":"<!DOCTYPEa\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,11],"output":[{"Doctype":{"name":"a\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0868() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000C","initialState":"Data","input":"<!DOCTYPEa\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0869() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000D","initialState":"Data","input":"<!DOCTYPEa\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0870() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u001F","initialState":"Data","input":"<!DOCTYPEa\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,31],"output":[{"Doctype":{"name":"a\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0871() {
    tokenize(
        r##"{"description":"<!DOCTYPEa ","initialState":"Data","input":"<!DOCTYPEa ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0872() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u0000","initialState":"Data","input":"<!DOCTYPEa \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0873() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u0008","initialState":"Data","input":"<!DOCTYPEa \b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0874() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u0009","initialState":"Data","input":"<!DOCTYPEa \t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0875() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000A","initialState":"Data","input":"<!DOCTYPEa \n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0876() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000B","initialState":"Data","input":"<!DOCTYPEa \u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0877() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000C","initialState":"Data","input":"<!DOCTYPEa \f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0878() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000D","initialState":"Data","input":"<!DOCTYPEa \r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0879() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u001F","initialState":"Data","input":"<!DOCTYPEa \u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0880() {
    tokenize(
        r##"{"description":"<!DOCTYPEa  ","initialState":"Data","input":"<!DOCTYPEa  ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0881() {
    tokenize(
        r##"{"description":"<!DOCTYPEa !","initialState":"Data","input":"<!DOCTYPEa !","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0882() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \"","initialState":"Data","input":"<!DOCTYPEa \"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0883() {
    tokenize(
        r##"{"description":"<!DOCTYPEa &","initialState":"Data","input":"<!DOCTYPEa &","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0884() {
    tokenize(
        r##"{"description":"<!DOCTYPEa '","initialState":"Data","input":"<!DOCTYPEa '","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0885() {
    tokenize(
        r##"{"description":"<!DOCTYPEa -","initialState":"Data","input":"<!DOCTYPEa -","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0886() {
    tokenize(
        r##"{"description":"<!DOCTYPEa /","initialState":"Data","input":"<!DOCTYPEa /","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0887() {
    tokenize(
        r##"{"description":"<!DOCTYPEa 0","initialState":"Data","input":"<!DOCTYPEa 0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0888() {
    tokenize(
        r##"{"description":"<!DOCTYPEa 1","initialState":"Data","input":"<!DOCTYPEa 1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0889() {
    tokenize(
        r##"{"description":"<!DOCTYPEa 9","initialState":"Data","input":"<!DOCTYPEa 9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0890() {
    tokenize(
        r##"{"description":"<!DOCTYPEa <","initialState":"Data","input":"<!DOCTYPEa <","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0891() {
    tokenize(
        r##"{"description":"<!DOCTYPEa =","initialState":"Data","input":"<!DOCTYPEa =","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0892() {
    tokenize(
        r##"{"description":"<!DOCTYPEa >","initialState":"Data","input":"<!DOCTYPEa >","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0893() {
    tokenize(
        r##"{"description":"<!DOCTYPEa ?","initialState":"Data","input":"<!DOCTYPEa ?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0894() {
    tokenize(
        r##"{"description":"<!DOCTYPEa @","initialState":"Data","input":"<!DOCTYPEa @","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0895() {
    tokenize(
        r##"{"description":"<!DOCTYPEa A","initialState":"Data","input":"<!DOCTYPEa A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0896() {
    tokenize(
        r##"{"description":"<!DOCTYPEa B","initialState":"Data","input":"<!DOCTYPEa B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_0897() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC","initialState":"Data","input":"<!DOCTYPEa PUBLIC","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0898() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0899() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u0008","initialState":"Data","input":"<!DOCTYPEa PUBLIC\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0900() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0901() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0902() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0903() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0904() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000D","initialState":"Data","input":"<!DOCTYPEa PUBLIC\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0905() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u001F","initialState":"Data","input":"<!DOCTYPEa PUBLIC\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0906() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC ","initialState":"Data","input":"<!DOCTYPEa PUBLIC ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0907() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC!","initialState":"Data","input":"<!DOCTYPEa PUBLIC!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0908() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0909() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0910() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0911() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0912() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0913() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0914() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\" ","initialState":"Data","input":"<!DOCTYPEa PUBLIC\" ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0915() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"!","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0916() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0917() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"#","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,35],"output":[{"Doctype":{"name":"a","public_id":"#","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0918() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"&","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0919() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"'","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,39],"output":[{"Doctype":{"name":"a","public_id":"'","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0920() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"-","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0921() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"/","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0922() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"0","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0923() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"1","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0924() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"9","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0925() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"<","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0926() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"=","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0927() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\">","initialState":"Data","input":"<!DOCTYPEa PUBLIC\">","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0928() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"?","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,63],"output":[{"Doctype":{"name":"a","public_id":"?","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0929() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"@","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,64],"output":[{"Doctype":{"name":"a","public_id":"@","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0930() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"A","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,65],"output":[{"Doctype":{"name":"a","public_id":"A","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0931() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"B","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,66],"output":[{"Doctype":{"name":"a","public_id":"B","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0932() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"Y","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,89],"output":[{"Doctype":{"name":"a","public_id":"Y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0933() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"Z","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,90],"output":[{"Doctype":{"name":"a","public_id":"Z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0934() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"`","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,96],"output":[{"Doctype":{"name":"a","public_id":"`","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0935() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"a","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,97],"output":[{"Doctype":{"name":"a","public_id":"a","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0936() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"b","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,98],"output":[{"Doctype":{"name":"a","public_id":"b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0937() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"y","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,121],"output":[{"Doctype":{"name":"a","public_id":"y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0938() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"z","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,122],"output":[{"Doctype":{"name":"a","public_id":"z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0939() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"{","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,123],"output":[{"Doctype":{"name":"a","public_id":"{","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0940() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"􀀀","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0941() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC#","initialState":"Data","input":"<!DOCTYPEa PUBLIC#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0942() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC&","initialState":"Data","input":"<!DOCTYPEa PUBLIC&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0943() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'","initialState":"Data","input":"<!DOCTYPEa PUBLIC'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0944() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0945() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0946() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0947() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0948() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0949() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC' ","initialState":"Data","input":"<!DOCTYPEa PUBLIC' ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0950() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'!","initialState":"Data","input":"<!DOCTYPEa PUBLIC'!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0951() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,34],"output":[{"Doctype":{"name":"a","public_id":"\"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0952() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'&","initialState":"Data","input":"<!DOCTYPEa PUBLIC'&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0953() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''","initialState":"Data","input":"<!DOCTYPEa PUBLIC''","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0954() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0955() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u0008","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,8],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0956() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,9],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0957() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,10],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0958() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,11],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0959() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,12],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0960() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000D","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,13],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_0961() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u001F","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,31],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0962() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'' ","initialState":"Data","input":"<!DOCTYPEa PUBLIC'' ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,32],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0963() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''!","initialState":"Data","input":"<!DOCTYPEa PUBLIC''!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,33],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0964() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0965() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''#","initialState":"Data","input":"<!DOCTYPEa PUBLIC''#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,35],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0966() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''&","initialState":"Data","input":"<!DOCTYPEa PUBLIC''&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,38],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0967() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'''","initialState":"Data","input":"<!DOCTYPEa PUBLIC'''","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_0968() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''(","initialState":"Data","input":"<!DOCTYPEa PUBLIC''(","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,40],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0969() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''-","initialState":"Data","input":"<!DOCTYPEa PUBLIC''-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,45],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0970() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''/","initialState":"Data","input":"<!DOCTYPEa PUBLIC''/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,47],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0971() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''0","initialState":"Data","input":"<!DOCTYPEa PUBLIC''0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,48],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0972() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''1","initialState":"Data","input":"<!DOCTYPEa PUBLIC''1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,49],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0973() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''9","initialState":"Data","input":"<!DOCTYPEa PUBLIC''9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,57],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0974() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''<","initialState":"Data","input":"<!DOCTYPEa PUBLIC''<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,60],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0975() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''=","initialState":"Data","input":"<!DOCTYPEa PUBLIC''=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,61],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0976() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''>","initialState":"Data","input":"<!DOCTYPEa PUBLIC''>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_0977() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''?","initialState":"Data","input":"<!DOCTYPEa PUBLIC''?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,63],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0978() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''@","initialState":"Data","input":"<!DOCTYPEa PUBLIC''@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,64],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0979() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''A","initialState":"Data","input":"<!DOCTYPEa PUBLIC''A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,65],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0980() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''B","initialState":"Data","input":"<!DOCTYPEa PUBLIC''B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,66],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0981() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''Y","initialState":"Data","input":"<!DOCTYPEa PUBLIC''Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,89],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0982() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''Z","initialState":"Data","input":"<!DOCTYPEa PUBLIC''Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,90],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0983() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''`","initialState":"Data","input":"<!DOCTYPEa PUBLIC''`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,96],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0984() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''a","initialState":"Data","input":"<!DOCTYPEa PUBLIC''a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,97],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0985() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''b","initialState":"Data","input":"<!DOCTYPEa PUBLIC''b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,98],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0986() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''y","initialState":"Data","input":"<!DOCTYPEa PUBLIC''y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,121],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0987() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''z","initialState":"Data","input":"<!DOCTYPEa PUBLIC''z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,122],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0988() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''{","initialState":"Data","input":"<!DOCTYPEa PUBLIC''{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,123],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0989() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa PUBLIC''􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0990() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'(","initialState":"Data","input":"<!DOCTYPEa PUBLIC'(","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,40],"output":[{"Doctype":{"name":"a","public_id":"(","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0991() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'-","initialState":"Data","input":"<!DOCTYPEa PUBLIC'-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0992() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'/","initialState":"Data","input":"<!DOCTYPEa PUBLIC'/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0993() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'0","initialState":"Data","input":"<!DOCTYPEa PUBLIC'0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0994() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'1","initialState":"Data","input":"<!DOCTYPEa PUBLIC'1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0995() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'9","initialState":"Data","input":"<!DOCTYPEa PUBLIC'9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0996() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'<","initialState":"Data","input":"<!DOCTYPEa PUBLIC'<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0997() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'=","initialState":"Data","input":"<!DOCTYPEa PUBLIC'=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_0998() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'>","initialState":"Data","input":"<!DOCTYPEa PUBLIC'>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_0999() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'?","initialState":"Data","input":"<!DOCTYPEa PUBLIC'?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,63],"output":[{"Doctype":{"name":"a","public_id":"?","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1000() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'@","initialState":"Data","input":"<!DOCTYPEa PUBLIC'@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,64],"output":[{"Doctype":{"name":"a","public_id":"@","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1001() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'A","initialState":"Data","input":"<!DOCTYPEa PUBLIC'A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,65],"output":[{"Doctype":{"name":"a","public_id":"A","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1002() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'B","initialState":"Data","input":"<!DOCTYPEa PUBLIC'B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,66],"output":[{"Doctype":{"name":"a","public_id":"B","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1003() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'Y","initialState":"Data","input":"<!DOCTYPEa PUBLIC'Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,89],"output":[{"Doctype":{"name":"a","public_id":"Y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1004() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'Z","initialState":"Data","input":"<!DOCTYPEa PUBLIC'Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,90],"output":[{"Doctype":{"name":"a","public_id":"Z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1005() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'`","initialState":"Data","input":"<!DOCTYPEa PUBLIC'`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,96],"output":[{"Doctype":{"name":"a","public_id":"`","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1006() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'a","initialState":"Data","input":"<!DOCTYPEa PUBLIC'a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,97],"output":[{"Doctype":{"name":"a","public_id":"a","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1007() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'b","initialState":"Data","input":"<!DOCTYPEa PUBLIC'b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,98],"output":[{"Doctype":{"name":"a","public_id":"b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1008() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'y","initialState":"Data","input":"<!DOCTYPEa PUBLIC'y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,121],"output":[{"Doctype":{"name":"a","public_id":"y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1009() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'z","initialState":"Data","input":"<!DOCTYPEa PUBLIC'z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,122],"output":[{"Doctype":{"name":"a","public_id":"z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1010() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'{","initialState":"Data","input":"<!DOCTYPEa PUBLIC'{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,123],"output":[{"Doctype":{"name":"a","public_id":"{","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1011() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa PUBLIC'􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"􀀀","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1012() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC(","initialState":"Data","input":"<!DOCTYPEa PUBLIC(","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1013() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC-","initialState":"Data","input":"<!DOCTYPEa PUBLIC-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1014() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC/","initialState":"Data","input":"<!DOCTYPEa PUBLIC/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1015() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC0","initialState":"Data","input":"<!DOCTYPEa PUBLIC0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1016() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC1","initialState":"Data","input":"<!DOCTYPEa PUBLIC1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1017() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC9","initialState":"Data","input":"<!DOCTYPEa PUBLIC9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1018() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC<","initialState":"Data","input":"<!DOCTYPEa PUBLIC<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1019() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC=","initialState":"Data","input":"<!DOCTYPEa PUBLIC=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1020() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC>","initialState":"Data","input":"<!DOCTYPEa PUBLIC>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1021() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC?","initialState":"Data","input":"<!DOCTYPEa PUBLIC?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1022() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC@","initialState":"Data","input":"<!DOCTYPEa PUBLIC@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1023() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICA","initialState":"Data","input":"<!DOCTYPEa PUBLICA","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1024() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICB","initialState":"Data","input":"<!DOCTYPEa PUBLICB","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1025() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICY","initialState":"Data","input":"<!DOCTYPEa PUBLICY","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1026() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICZ","initialState":"Data","input":"<!DOCTYPEa PUBLICZ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1027() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC`","initialState":"Data","input":"<!DOCTYPEa PUBLIC`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1028() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICa","initialState":"Data","input":"<!DOCTYPEa PUBLICa","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1029() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICb","initialState":"Data","input":"<!DOCTYPEa PUBLICb","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1030() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICy","initialState":"Data","input":"<!DOCTYPEa PUBLICy","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1031() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLICz","initialState":"Data","input":"<!DOCTYPEa PUBLICz","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1032() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC{","initialState":"Data","input":"<!DOCTYPEa PUBLIC{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1033() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa PUBLIC􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1034() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM","initialState":"Data","input":"<!DOCTYPEa SYSTEM","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1035() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u0000","initialState":"Data","input":"<!DOCTYPEa SYSTEM\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1036() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u0008","initialState":"Data","input":"<!DOCTYPEa SYSTEM\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1037() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u0009","initialState":"Data","input":"<!DOCTYPEa SYSTEM\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_1038() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u000A","initialState":"Data","input":"<!DOCTYPEa SYSTEM\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1039() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u000B","initialState":"Data","input":"<!DOCTYPEa SYSTEM\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1040() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u000C","initialState":"Data","input":"<!DOCTYPEa SYSTEM\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_1041() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u000D","initialState":"Data","input":"<!DOCTYPEa SYSTEM\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1042() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\u001F","initialState":"Data","input":"<!DOCTYPEa SYSTEM\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1043() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM ","initialState":"Data","input":"<!DOCTYPEa SYSTEM ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_1044() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM!","initialState":"Data","input":"<!DOCTYPEa SYSTEM!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1045() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_1046() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"\\u0000","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"�","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1047() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"\\u0009","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\t","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1048() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"\\u000A","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\n","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1049() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"\\u000B","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\u000b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1050() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"\\u000C","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\f","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1051() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\" ","initialState":"Data","input":"<!DOCTYPEa SYSTEM\" ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":" ","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1052() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"!","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"!","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1053() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"\"","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1054() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"#","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"#","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1055() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"&","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"&","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1056() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"'","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"'","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1057() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"-","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"-","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1058() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"/","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"/","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1059() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"0","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"0","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1060() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"1","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"1","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1061() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"9","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"9","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1062() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"<","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"<","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1063() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"=","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"=","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1064() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\">","initialState":"Data","input":"<!DOCTYPEa SYSTEM\">","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"AbruptDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_1065() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"?","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"?","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1066() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"@","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"@","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1067() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"A","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"A","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1068() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"B","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"B","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1069() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"Y","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1070() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"Z","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1071() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"`","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"`","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1072() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"a","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"a","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1073() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"b","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1074() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"y","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1075() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"z","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1076() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"{","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"{","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1077() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\"\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa SYSTEM\"􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,34,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"􀀀","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1078() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM#","initialState":"Data","input":"<!DOCTYPEa SYSTEM#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1079() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM&","initialState":"Data","input":"<!DOCTYPEa SYSTEM&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1080() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'","initialState":"Data","input":"<!DOCTYPEa SYSTEM'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_1081() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'\\u0000","initialState":"Data","input":"<!DOCTYPEa SYSTEM'\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"�","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1082() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'\\u0009","initialState":"Data","input":"<!DOCTYPEa SYSTEM'\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\t","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1083() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'\\u000A","initialState":"Data","input":"<!DOCTYPEa SYSTEM'\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\n","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1084() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'\\u000B","initialState":"Data","input":"<!DOCTYPEa SYSTEM'\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\u000b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1085() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'\\u000C","initialState":"Data","input":"<!DOCTYPEa SYSTEM'\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\f","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1086() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM' ","initialState":"Data","input":"<!DOCTYPEa SYSTEM' ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":" ","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1087() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'!","initialState":"Data","input":"<!DOCTYPEa SYSTEM'!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"!","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1088() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'\"","initialState":"Data","input":"<!DOCTYPEa SYSTEM'\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1089() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'&","initialState":"Data","input":"<!DOCTYPEa SYSTEM'&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"&","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1090() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''","initialState":"Data","input":"<!DOCTYPEa SYSTEM''","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1091() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u0000","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1092() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u0008","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1093() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u0009","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_1094() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u000A","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1095() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u000B","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1096() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u000C","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_1097() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u000D","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1098() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\u001F","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1099() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'' ","initialState":"Data","input":"<!DOCTYPEa SYSTEM'' ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_1100() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''!","initialState":"Data","input":"<!DOCTYPEa SYSTEM''!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1101() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\"","initialState":"Data","input":"<!DOCTYPEa SYSTEM''\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1102() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''&","initialState":"Data","input":"<!DOCTYPEa SYSTEM''&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1103() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'''","initialState":"Data","input":"<!DOCTYPEa SYSTEM'''","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1104() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''-","initialState":"Data","input":"<!DOCTYPEa SYSTEM''-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1105() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''/","initialState":"Data","input":"<!DOCTYPEa SYSTEM''/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1106() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''0","initialState":"Data","input":"<!DOCTYPEa SYSTEM''0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1107() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''1","initialState":"Data","input":"<!DOCTYPEa SYSTEM''1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1108() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''9","initialState":"Data","input":"<!DOCTYPEa SYSTEM''9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1109() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''<","initialState":"Data","input":"<!DOCTYPEa SYSTEM''<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1110() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''=","initialState":"Data","input":"<!DOCTYPEa SYSTEM''=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1111() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''>","initialState":"Data","input":"<!DOCTYPEa SYSTEM''>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1112() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''?","initialState":"Data","input":"<!DOCTYPEa SYSTEM''?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1113() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''@","initialState":"Data","input":"<!DOCTYPEa SYSTEM''@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1114() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''A","initialState":"Data","input":"<!DOCTYPEa SYSTEM''A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1115() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''B","initialState":"Data","input":"<!DOCTYPEa SYSTEM''B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1116() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''Y","initialState":"Data","input":"<!DOCTYPEa SYSTEM''Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1117() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''Z","initialState":"Data","input":"<!DOCTYPEa SYSTEM''Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1118() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''`","initialState":"Data","input":"<!DOCTYPEa SYSTEM''`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1119() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''a","initialState":"Data","input":"<!DOCTYPEa SYSTEM''a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1120() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''b","initialState":"Data","input":"<!DOCTYPEa SYSTEM''b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1121() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''y","initialState":"Data","input":"<!DOCTYPEa SYSTEM''y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1122() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''z","initialState":"Data","input":"<!DOCTYPEa SYSTEM''z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1123() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''{","initialState":"Data","input":"<!DOCTYPEa SYSTEM''{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1124() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM''\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa SYSTEM''􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1125() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'(","initialState":"Data","input":"<!DOCTYPEa SYSTEM'(","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"(","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1126() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'-","initialState":"Data","input":"<!DOCTYPEa SYSTEM'-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"-","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1127() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'/","initialState":"Data","input":"<!DOCTYPEa SYSTEM'/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"/","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1128() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'0","initialState":"Data","input":"<!DOCTYPEa SYSTEM'0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"0","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1129() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'1","initialState":"Data","input":"<!DOCTYPEa SYSTEM'1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"1","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1130() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'9","initialState":"Data","input":"<!DOCTYPEa SYSTEM'9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"9","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1131() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'<","initialState":"Data","input":"<!DOCTYPEa SYSTEM'<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"<","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1132() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'=","initialState":"Data","input":"<!DOCTYPEa SYSTEM'=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"=","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1133() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'>","initialState":"Data","input":"<!DOCTYPEa SYSTEM'>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"AbruptDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_1134() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'?","initialState":"Data","input":"<!DOCTYPEa SYSTEM'?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"?","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1135() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'@","initialState":"Data","input":"<!DOCTYPEa SYSTEM'@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"@","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1136() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'A","initialState":"Data","input":"<!DOCTYPEa SYSTEM'A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"A","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1137() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'B","initialState":"Data","input":"<!DOCTYPEa SYSTEM'B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"B","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1138() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'Y","initialState":"Data","input":"<!DOCTYPEa SYSTEM'Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1139() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'Z","initialState":"Data","input":"<!DOCTYPEa SYSTEM'Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1140() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'`","initialState":"Data","input":"<!DOCTYPEa SYSTEM'`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"`","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1141() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'a","initialState":"Data","input":"<!DOCTYPEa SYSTEM'a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"a","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1142() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'b","initialState":"Data","input":"<!DOCTYPEa SYSTEM'b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1143() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'y","initialState":"Data","input":"<!DOCTYPEa SYSTEM'y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1144() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'z","initialState":"Data","input":"<!DOCTYPEa SYSTEM'z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1145() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'{","initialState":"Data","input":"<!DOCTYPEa SYSTEM'{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"{","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1146() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM'\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa SYSTEM'􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"􀀀","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_1147() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM(","initialState":"Data","input":"<!DOCTYPEa SYSTEM(","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1148() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM-","initialState":"Data","input":"<!DOCTYPEa SYSTEM-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1149() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM/","initialState":"Data","input":"<!DOCTYPEa SYSTEM/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1150() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM0","initialState":"Data","input":"<!DOCTYPEa SYSTEM0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1151() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM1","initialState":"Data","input":"<!DOCTYPEa SYSTEM1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1152() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM9","initialState":"Data","input":"<!DOCTYPEa SYSTEM9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1153() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM<","initialState":"Data","input":"<!DOCTYPEa SYSTEM<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1154() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM=","initialState":"Data","input":"<!DOCTYPEa SYSTEM=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1155() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM>","initialState":"Data","input":"<!DOCTYPEa SYSTEM>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1156() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM?","initialState":"Data","input":"<!DOCTYPEa SYSTEM?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1157() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM@","initialState":"Data","input":"<!DOCTYPEa SYSTEM@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1158() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMA","initialState":"Data","input":"<!DOCTYPEa SYSTEMA","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1159() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMB","initialState":"Data","input":"<!DOCTYPEa SYSTEMB","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1160() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMY","initialState":"Data","input":"<!DOCTYPEa SYSTEMY","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1161() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMZ","initialState":"Data","input":"<!DOCTYPEa SYSTEMZ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1162() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM`","initialState":"Data","input":"<!DOCTYPEa SYSTEM`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1163() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMa","initialState":"Data","input":"<!DOCTYPEa SYSTEMa","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1164() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMb","initialState":"Data","input":"<!DOCTYPEa SYSTEMb","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1165() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMy","initialState":"Data","input":"<!DOCTYPEa SYSTEMy","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1166() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEMz","initialState":"Data","input":"<!DOCTYPEa SYSTEMz","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1167() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM{","initialState":"Data","input":"<!DOCTYPEa SYSTEM{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1168() {
    tokenize(
        r##"{"description":"<!DOCTYPEa SYSTEM\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa SYSTEM􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,83,89,83,84,69,77,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_1169() {
    tokenize(
        r##"{"description":"<!DOCTYPEa Y","initialState":"Data","input":"<!DOCTYPEa Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1170() {
    tokenize(
        r##"{"description":"<!DOCTYPEa Z","initialState":"Data","input":"<!DOCTYPEa Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1171() {
    tokenize(
        r##"{"description":"<!DOCTYPEa `","initialState":"Data","input":"<!DOCTYPEa `","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1172() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a","initialState":"Data","input":"<!DOCTYPEa a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1173() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a\\u0000","initialState":"Data","input":"<!DOCTYPEa a\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_1174() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a\\u0009","initialState":"Data","input":"<!DOCTYPEa a\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1175() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a\\u000A","initialState":"Data","input":"<!DOCTYPEa a\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1176() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a\\u000B","initialState":"Data","input":"<!DOCTYPEa a\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_1177() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a\\u000C","initialState":"Data","input":"<!DOCTYPEa a\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1178() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a ","initialState":"Data","input":"<!DOCTYPEa a ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1179() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a!","initialState":"Data","input":"<!DOCTYPEa a!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1180() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a\"","initialState":"Data","input":"<!DOCTYPEa a\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1181() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a&","initialState":"Data","input":"<!DOCTYPEa a&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1182() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a'","initialState":"Data","input":"<!DOCTYPEa a'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1183() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a-","initialState":"Data","input":"<!DOCTYPEa a-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1184() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a/","initialState":"Data","input":"<!DOCTYPEa a/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1185() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a0","initialState":"Data","input":"<!DOCTYPEa a0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1186() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a1","initialState":"Data","input":"<!DOCTYPEa a1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1187() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a9","initialState":"Data","input":"<!DOCTYPEa a9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1188() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a<","initialState":"Data","input":"<!DOCTYPEa a<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1189() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a=","initialState":"Data","input":"<!DOCTYPEa a=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1190() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a>","initialState":"Data","input":"<!DOCTYPEa a>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1191() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a?","initialState":"Data","input":"<!DOCTYPEa a?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1192() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a@","initialState":"Data","input":"<!DOCTYPEa a@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1193() {
    tokenize(
        r##"{"description":"<!DOCTYPEa aA","initialState":"Data","input":"<!DOCTYPEa aA","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1194() {
    tokenize(
        r##"{"description":"<!DOCTYPEa aB","initialState":"Data","input":"<!DOCTYPEa aB","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1195() {
    tokenize(
        r##"{"description":"<!DOCTYPEa aY","initialState":"Data","input":"<!DOCTYPEa aY","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1196() {
    tokenize(
        r##"{"description":"<!DOCTYPEa aZ","initialState":"Data","input":"<!DOCTYPEa aZ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1197() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a`","initialState":"Data","input":"<!DOCTYPEa a`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1198() {
    tokenize(
        r##"{"description":"<!DOCTYPEa aa","initialState":"Data","input":"<!DOCTYPEa aa","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1199() {
    tokenize(
        r##"{"description":"<!DOCTYPEa ab","initialState":"Data","input":"<!DOCTYPEa ab","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1200() {
    tokenize(
        r##"{"description":"<!DOCTYPEa ay","initialState":"Data","input":"<!DOCTYPEa ay","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1201() {
    tokenize(
        r##"{"description":"<!DOCTYPEa az","initialState":"Data","input":"<!DOCTYPEa az","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1202() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a{","initialState":"Data","input":"<!DOCTYPEa a{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1203() {
    tokenize(
        r##"{"description":"<!DOCTYPEa a\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa a􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,97,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1204() {
    tokenize(
        r##"{"description":"<!DOCTYPEa b","initialState":"Data","input":"<!DOCTYPEa b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1205() {
    tokenize(
        r##"{"description":"<!DOCTYPEa y","initialState":"Data","input":"<!DOCTYPEa y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1206() {
    tokenize(
        r##"{"description":"<!DOCTYPEa z","initialState":"Data","input":"<!DOCTYPEa z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1207() {
    tokenize(
        r##"{"description":"<!DOCTYPEa {","initialState":"Data","input":"<!DOCTYPEa {","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1208() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa 􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1209() {
    tokenize(
        r##"{"description":"<!DOCTYPEa!","initialState":"Data","input":"<!DOCTYPEa!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,33],"output":[{"Doctype":{"name":"a!","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1210() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\"","initialState":"Data","input":"<!DOCTYPEa\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,34],"output":[{"Doctype":{"name":"a\"","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1211() {
    tokenize(
        r##"{"description":"<!DOCTYPEa&","initialState":"Data","input":"<!DOCTYPEa&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,38],"output":[{"Doctype":{"name":"a&","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1212() {
    tokenize(
        r##"{"description":"<!DOCTYPEa'","initialState":"Data","input":"<!DOCTYPEa'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,39],"output":[{"Doctype":{"name":"a'","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1213() {
    tokenize(
        r##"{"description":"<!DOCTYPEa-","initialState":"Data","input":"<!DOCTYPEa-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,45],"output":[{"Doctype":{"name":"a-","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1214() {
    tokenize(
        r##"{"description":"<!DOCTYPEa/","initialState":"Data","input":"<!DOCTYPEa/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,47],"output":[{"Doctype":{"name":"a/","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1215() {
    tokenize(
        r##"{"description":"<!DOCTYPEa0","initialState":"Data","input":"<!DOCTYPEa0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,48],"output":[{"Doctype":{"name":"a0","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1216() {
    tokenize(
        r##"{"description":"<!DOCTYPEa1","initialState":"Data","input":"<!DOCTYPEa1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,49],"output":[{"Doctype":{"name":"a1","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1217() {
    tokenize(
        r##"{"description":"<!DOCTYPEa9","initialState":"Data","input":"<!DOCTYPEa9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,57],"output":[{"Doctype":{"name":"a9","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1218() {
    tokenize(
        r##"{"description":"<!DOCTYPEa<","initialState":"Data","input":"<!DOCTYPEa<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,60],"output":[{"Doctype":{"name":"a<","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1219() {
    tokenize(
        r##"{"description":"<!DOCTYPEa=","initialState":"Data","input":"<!DOCTYPEa=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,61],"output":[{"Doctype":{"name":"a=","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1220() {
    tokenize(
        r##"{"description":"<!DOCTYPEa>","initialState":"Data","input":"<!DOCTYPEa>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_1221() {
    tokenize(
        r##"{"description":"<!DOCTYPEa?","initialState":"Data","input":"<!DOCTYPEa?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,63],"output":[{"Doctype":{"name":"a?","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1222() {
    tokenize(
        r##"{"description":"<!DOCTYPEa@","initialState":"Data","input":"<!DOCTYPEa@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,64],"output":[{"Doctype":{"name":"a@","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1223() {
    tokenize(
        r##"{"description":"<!DOCTYPEaA","initialState":"Data","input":"<!DOCTYPEaA","inputUtf16":[60,33,68,79,67,84,89,80,69,97,65],"output":[{"Doctype":{"name":"aa","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1224() {
    tokenize(
        r##"{"description":"<!DOCTYPEaB","initialState":"Data","input":"<!DOCTYPEaB","inputUtf16":[60,33,68,79,67,84,89,80,69,97,66],"output":[{"Doctype":{"name":"ab","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1225() {
    tokenize(
        r##"{"description":"<!DOCTYPEaY","initialState":"Data","input":"<!DOCTYPEaY","inputUtf16":[60,33,68,79,67,84,89,80,69,97,89],"output":[{"Doctype":{"name":"ay","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1226() {
    tokenize(
        r##"{"description":"<!DOCTYPEaZ","initialState":"Data","input":"<!DOCTYPEaZ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,90],"output":[{"Doctype":{"name":"az","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1227() {
    tokenize(
        r##"{"description":"<!DOCTYPEa[","initialState":"Data","input":"<!DOCTYPEa[","inputUtf16":[60,33,68,79,67,84,89,80,69,97,91],"output":[{"Doctype":{"name":"a[","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1228() {
    tokenize(
        r##"{"description":"<!DOCTYPEa`","initialState":"Data","input":"<!DOCTYPEa`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,96],"output":[{"Doctype":{"name":"a`","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1229() {
    tokenize(
        r##"{"description":"<!DOCTYPEaa","initialState":"Data","input":"<!DOCTYPEaa","inputUtf16":[60,33,68,79,67,84,89,80,69,97,97],"output":[{"Doctype":{"name":"aa","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1230() {
    tokenize(
        r##"{"description":"<!DOCTYPEab","initialState":"Data","input":"<!DOCTYPEab","inputUtf16":[60,33,68,79,67,84,89,80,69,97,98],"output":[{"Doctype":{"name":"ab","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1231() {
    tokenize(
        r##"{"description":"<!DOCTYPEay","initialState":"Data","input":"<!DOCTYPEay","inputUtf16":[60,33,68,79,67,84,89,80,69,97,121],"output":[{"Doctype":{"name":"ay","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1232() {
    tokenize(
        r##"{"description":"<!DOCTYPEaz","initialState":"Data","input":"<!DOCTYPEaz","inputUtf16":[60,33,68,79,67,84,89,80,69,97,122],"output":[{"Doctype":{"name":"az","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1233() {
    tokenize(
        r##"{"description":"<!DOCTYPEa{","initialState":"Data","input":"<!DOCTYPEa{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,123],"output":[{"Doctype":{"name":"a{","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1234() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,56256,56320],"output":[{"Doctype":{"name":"a􀀀","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_1235() {
    tokenize(
        r##"{"description":"<!DOCTYPEb","initialState":"Data","input":"<!DOCTYPEb","inputUtf16":[60,33,68,79,67,84,89,80,69,98],"output":[{"Doctype":{"name":"b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_1236() {
    tokenize(
        r##"{"description":"<!DOCTYPEy","initialState":"Data","input":"<!DOCTYPEy","inputUtf16":[60,33,68,79,67,84,89,80,69,121],"output":[{"Doctype":{"name":"y","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_1237() {
    tokenize(
        r##"{"description":"<!DOCTYPEz","initialState":"Data","input":"<!DOCTYPEz","inputUtf16":[60,33,68,79,67,84,89,80,69,122],"output":[{"Doctype":{"name":"z","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_1238() {
    tokenize(
        r##"{"description":"<!DOCTYPE{","initialState":"Data","input":"<!DOCTYPE{","inputUtf16":[60,33,68,79,67,84,89,80,69,123],"output":[{"Doctype":{"name":"{","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_1239() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,56256,56320],"output":[{"Doctype":{"name":"􀀀","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_1240() {
    tokenize(
        r##"{"description":"<!Y","initialState":"Data","input":"<!Y","inputUtf16":[60,33,89],"output":[{"Comment":{"data":"Y"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1241() {
    tokenize(
        r##"{"description":"<!Z","initialState":"Data","input":"<!Z","inputUtf16":[60,33,90],"output":[{"Comment":{"data":"Z"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1242() {
    tokenize(
        r##"{"description":"<!`","initialState":"Data","input":"<!`","inputUtf16":[60,33,96],"output":[{"Comment":{"data":"`"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1243() {
    tokenize(
        r##"{"description":"<!a","initialState":"Data","input":"<!a","inputUtf16":[60,33,97],"output":[{"Comment":{"data":"a"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1244() {
    tokenize(
        r##"{"description":"<!b","initialState":"Data","input":"<!b","inputUtf16":[60,33,98],"output":[{"Comment":{"data":"b"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1245() {
    tokenize(
        r##"{"description":"<!y","initialState":"Data","input":"<!y","inputUtf16":[60,33,121],"output":[{"Comment":{"data":"y"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1246() {
    tokenize(
        r##"{"description":"<!z","initialState":"Data","input":"<!z","inputUtf16":[60,33,122],"output":[{"Comment":{"data":"z"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1247() {
    tokenize(
        r##"{"description":"<!{","initialState":"Data","input":"<!{","inputUtf16":[60,33,123],"output":[{"Comment":{"data":"{"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1248() {
    tokenize(
        r##"{"description":"<!\\uDBC0\\uDC00","initialState":"Data","input":"<!􀀀","inputUtf16":[60,33,56256,56320],"output":[{"Comment":{"data":"􀀀"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1249() {
    tokenize(
        r##"{"description":"<\"","initialState":"Data","input":"<\"","inputUtf16":[60,34],"output":[{"Character":{"data":"<\""}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1250() {
    tokenize(
        r##"{"description":"<&","initialState":"Data","input":"<&","inputUtf16":[60,38],"output":[{"Character":{"data":"<&"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1251() {
    tokenize(
        r##"{"description":"<'","initialState":"Data","input":"<'","inputUtf16":[60,39],"output":[{"Character":{"data":"<'"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1252() {
    tokenize(
        r##"{"description":"<-","initialState":"Data","input":"<-","inputUtf16":[60,45],"output":[{"Character":{"data":"<-"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1253() {
    tokenize(
        r##"{"description":"<.","initialState":"Data","input":"<.","inputUtf16":[60,46],"output":[{"Character":{"data":"<."}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1254() {
    tokenize(
        r##"{"description":"</","initialState":"Data","input":"</","inputUtf16":[60,47],"output":[{"Character":{"data":"</"}}],"errors":[{"code":"EofBeforeTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1255() {
    tokenize(
        r##"{"description":"</\\u0000","initialState":"Data","input":"</\u0000","inputUtf16":[60,47,0],"output":[{"Comment":{"data":"�"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1256() {
    tokenize(
        r##"{"description":"</\\u0009","initialState":"Data","input":"</\t","inputUtf16":[60,47,9],"output":[{"Comment":{"data":"\t"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1257() {
    tokenize(
        r##"{"description":"</\\u000A","initialState":"Data","input":"</\n","inputUtf16":[60,47,10],"output":[{"Comment":{"data":"\n"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1258() {
    tokenize(
        r##"{"description":"</\\u000B","initialState":"Data","input":"</\u000b","inputUtf16":[60,47,11],"output":[{"Comment":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":3}},{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1259() {
    tokenize(
        r##"{"description":"</\\u000C","initialState":"Data","input":"</\f","inputUtf16":[60,47,12],"output":[{"Comment":{"data":"\f"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1260() {
    tokenize(
        r##"{"description":"</ ","initialState":"Data","input":"</ ","inputUtf16":[60,47,32],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1261() {
    tokenize(
        r##"{"description":"</ \\u0000","initialState":"Data","input":"</ \u0000","inputUtf16":[60,47,32,0],"output":[{"Comment":{"data":" �"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1262() {
    tokenize(
        r##"{"description":"</!","initialState":"Data","input":"</!","inputUtf16":[60,47,33],"output":[{"Comment":{"data":"!"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1263() {
    tokenize(
        r##"{"description":"</\"","initialState":"Data","input":"</\"","inputUtf16":[60,47,34],"output":[{"Comment":{"data":"\""}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1264() {
    tokenize(
        r##"{"description":"</&","initialState":"Data","input":"</&","inputUtf16":[60,47,38],"output":[{"Comment":{"data":"&"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1265() {
    tokenize(
        r##"{"description":"</'","initialState":"Data","input":"</'","inputUtf16":[60,47,39],"output":[{"Comment":{"data":"'"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1266() {
    tokenize(
        r##"{"description":"</-","initialState":"Data","input":"</-","inputUtf16":[60,47,45],"output":[{"Comment":{"data":"-"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1267() {
    tokenize(
        r##"{"description":"<//","initialState":"Data","input":"<//","inputUtf16":[60,47,47],"output":[{"Comment":{"data":"/"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1268() {
    tokenize(
        r##"{"description":"</0","initialState":"Data","input":"</0","inputUtf16":[60,47,48],"output":[{"Comment":{"data":"0"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1269() {
    tokenize(
        r##"{"description":"</1","initialState":"Data","input":"</1","inputUtf16":[60,47,49],"output":[{"Comment":{"data":"1"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1270() {
    tokenize(
        r##"{"description":"</9","initialState":"Data","input":"</9","inputUtf16":[60,47,57],"output":[{"Comment":{"data":"9"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1271() {
    tokenize(
        r##"{"description":"</<","initialState":"Data","input":"</<","inputUtf16":[60,47,60],"output":[{"Comment":{"data":"<"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1272() {
    tokenize(
        r##"{"description":"</=","initialState":"Data","input":"</=","inputUtf16":[60,47,61],"output":[{"Comment":{"data":"="}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1273() {
    tokenize(
        r##"{"description":"</>","initialState":"Data","input":"</>","inputUtf16":[60,47,62],"output":[],"errors":[{"code":"MissingEndTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1274() {
    tokenize(
        r##"{"description":"</?","initialState":"Data","input":"</?","inputUtf16":[60,47,63],"output":[{"Comment":{"data":"?"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1275() {
    tokenize(
        r##"{"description":"</@","initialState":"Data","input":"</@","inputUtf16":[60,47,64],"output":[{"Comment":{"data":"@"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1276() {
    tokenize(
        r##"{"description":"</A>","initialState":"Data","input":"</A>","inputUtf16":[60,47,65,62],"output":[{"EndTag":{"name":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1277() {
    tokenize(
        r##"{"description":"</B>","initialState":"Data","input":"</B>","inputUtf16":[60,47,66,62],"output":[{"EndTag":{"name":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1278() {
    tokenize(
        r##"{"description":"</Y>","initialState":"Data","input":"</Y>","inputUtf16":[60,47,89,62],"output":[{"EndTag":{"name":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1279() {
    tokenize(
        r##"{"description":"</Z>","initialState":"Data","input":"</Z>","inputUtf16":[60,47,90,62],"output":[{"EndTag":{"name":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1280() {
    tokenize(
        r##"{"description":"</[","initialState":"Data","input":"</[","inputUtf16":[60,47,91],"output":[{"Comment":{"data":"["}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1281() {
    tokenize(
        r##"{"description":"</`","initialState":"Data","input":"</`","inputUtf16":[60,47,96],"output":[{"Comment":{"data":"`"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1282() {
    tokenize(
        r##"{"description":"</a>","initialState":"Data","input":"</a>","inputUtf16":[60,47,97,62],"output":[{"EndTag":{"name":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1283() {
    tokenize(
        r##"{"description":"</b>","initialState":"Data","input":"</b>","inputUtf16":[60,47,98,62],"output":[{"EndTag":{"name":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1284() {
    tokenize(
        r##"{"description":"</y>","initialState":"Data","input":"</y>","inputUtf16":[60,47,121,62],"output":[{"EndTag":{"name":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1285() {
    tokenize(
        r##"{"description":"</z>","initialState":"Data","input":"</z>","inputUtf16":[60,47,122,62],"output":[{"EndTag":{"name":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1286() {
    tokenize(
        r##"{"description":"</{","initialState":"Data","input":"</{","inputUtf16":[60,47,123],"output":[{"Comment":{"data":"{"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1287() {
    tokenize(
        r##"{"description":"</\\uDBC0\\uDC00","initialState":"Data","input":"</􀀀","inputUtf16":[60,47,56256,56320],"output":[{"Comment":{"data":"􀀀"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1288() {
    tokenize(
        r##"{"description":"<0","initialState":"Data","input":"<0","inputUtf16":[60,48],"output":[{"Character":{"data":"<0"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1289() {
    tokenize(
        r##"{"description":"<1","initialState":"Data","input":"<1","inputUtf16":[60,49],"output":[{"Character":{"data":"<1"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1290() {
    tokenize(
        r##"{"description":"<9","initialState":"Data","input":"<9","inputUtf16":[60,57],"output":[{"Character":{"data":"<9"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1291() {
    tokenize(
        r##"{"description":"<<","initialState":"Data","input":"<<","inputUtf16":[60,60],"output":[{"Character":{"data":"<<"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}},{"code":"EofBeforeTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1292() {
    tokenize(
        r##"{"description":"<=","initialState":"Data","input":"<=","inputUtf16":[60,61],"output":[{"Character":{"data":"<="}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1293() {
    tokenize(
        r##"{"description":"<>","initialState":"Data","input":"<>","inputUtf16":[60,62],"output":[{"Character":{"data":"<>"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1294() {
    tokenize(
        r##"{"description":"<?","initialState":"Data","input":"<?","inputUtf16":[60,63],"output":[{"Comment":{"data":"?"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1295() {
    tokenize(
        r##"{"description":"<?\\u0000","initialState":"Data","input":"<?\u0000","inputUtf16":[60,63,0],"output":[{"Comment":{"data":"?�"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1296() {
    tokenize(
        r##"{"description":"<?\\u0009","initialState":"Data","input":"<?\t","inputUtf16":[60,63,9],"output":[{"Comment":{"data":"?\t"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1297() {
    tokenize(
        r##"{"description":"<?\\u000A","initialState":"Data","input":"<?\n","inputUtf16":[60,63,10],"output":[{"Comment":{"data":"?\n"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1298() {
    tokenize(
        r##"{"description":"<?\\u000B","initialState":"Data","input":"<?\u000b","inputUtf16":[60,63,11],"output":[{"Comment":{"data":"?\u000b"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1299() {
    tokenize(
        r##"{"description":"<?\\u000C","initialState":"Data","input":"<?\f","inputUtf16":[60,63,12],"output":[{"Comment":{"data":"?\f"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1300() {
    tokenize(
        r##"{"description":"<? ","initialState":"Data","input":"<? ","inputUtf16":[60,63,32],"output":[{"Comment":{"data":"? "}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1301() {
    tokenize(
        r##"{"description":"<? \\u0000","initialState":"Data","input":"<? \u0000","inputUtf16":[60,63,32,0],"output":[{"Comment":{"data":"? �"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1302() {
    tokenize(
        r##"{"description":"<?!","initialState":"Data","input":"<?!","inputUtf16":[60,63,33],"output":[{"Comment":{"data":"?!"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1303() {
    tokenize(
        r##"{"description":"<?\"","initialState":"Data","input":"<?\"","inputUtf16":[60,63,34],"output":[{"Comment":{"data":"?\""}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1304() {
    tokenize(
        r##"{"description":"<?&","initialState":"Data","input":"<?&","inputUtf16":[60,63,38],"output":[{"Comment":{"data":"?&"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1305() {
    tokenize(
        r##"{"description":"<?'","initialState":"Data","input":"<?'","inputUtf16":[60,63,39],"output":[{"Comment":{"data":"?'"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1306() {
    tokenize(
        r##"{"description":"<?-","initialState":"Data","input":"<?-","inputUtf16":[60,63,45],"output":[{"Comment":{"data":"?-"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1307() {
    tokenize(
        r##"{"description":"<?/","initialState":"Data","input":"<?/","inputUtf16":[60,63,47],"output":[{"Comment":{"data":"?/"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1308() {
    tokenize(
        r##"{"description":"<?0","initialState":"Data","input":"<?0","inputUtf16":[60,63,48],"output":[{"Comment":{"data":"?0"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1309() {
    tokenize(
        r##"{"description":"<?1","initialState":"Data","input":"<?1","inputUtf16":[60,63,49],"output":[{"Comment":{"data":"?1"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1310() {
    tokenize(
        r##"{"description":"<?9","initialState":"Data","input":"<?9","inputUtf16":[60,63,57],"output":[{"Comment":{"data":"?9"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1311() {
    tokenize(
        r##"{"description":"<?<","initialState":"Data","input":"<?<","inputUtf16":[60,63,60],"output":[{"Comment":{"data":"?<"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1312() {
    tokenize(
        r##"{"description":"<?=","initialState":"Data","input":"<?=","inputUtf16":[60,63,61],"output":[{"Comment":{"data":"?="}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1313() {
    tokenize(
        r##"{"description":"<?>","initialState":"Data","input":"<?>","inputUtf16":[60,63,62],"output":[{"Comment":{"data":"?"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1314() {
    tokenize(
        r##"{"description":"<??","initialState":"Data","input":"<??","inputUtf16":[60,63,63],"output":[{"Comment":{"data":"??"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1315() {
    tokenize(
        r##"{"description":"<?@","initialState":"Data","input":"<?@","inputUtf16":[60,63,64],"output":[{"Comment":{"data":"?@"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1316() {
    tokenize(
        r##"{"description":"<?A","initialState":"Data","input":"<?A","inputUtf16":[60,63,65],"output":[{"Comment":{"data":"?A"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1317() {
    tokenize(
        r##"{"description":"<?B","initialState":"Data","input":"<?B","inputUtf16":[60,63,66],"output":[{"Comment":{"data":"?B"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1318() {
    tokenize(
        r##"{"description":"<?Y","initialState":"Data","input":"<?Y","inputUtf16":[60,63,89],"output":[{"Comment":{"data":"?Y"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1319() {
    tokenize(
        r##"{"description":"<?Z","initialState":"Data","input":"<?Z","inputUtf16":[60,63,90],"output":[{"Comment":{"data":"?Z"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1320() {
    tokenize(
        r##"{"description":"<?`","initialState":"Data","input":"<?`","inputUtf16":[60,63,96],"output":[{"Comment":{"data":"?`"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1321() {
    tokenize(
        r##"{"description":"<?a","initialState":"Data","input":"<?a","inputUtf16":[60,63,97],"output":[{"Comment":{"data":"?a"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1322() {
    tokenize(
        r##"{"description":"<?b","initialState":"Data","input":"<?b","inputUtf16":[60,63,98],"output":[{"Comment":{"data":"?b"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1323() {
    tokenize(
        r##"{"description":"<?y","initialState":"Data","input":"<?y","inputUtf16":[60,63,121],"output":[{"Comment":{"data":"?y"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1324() {
    tokenize(
        r##"{"description":"<?z","initialState":"Data","input":"<?z","inputUtf16":[60,63,122],"output":[{"Comment":{"data":"?z"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1325() {
    tokenize(
        r##"{"description":"<?{","initialState":"Data","input":"<?{","inputUtf16":[60,63,123],"output":[{"Comment":{"data":"?{"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1326() {
    tokenize(
        r##"{"description":"<?\\uDBC0\\uDC00","initialState":"Data","input":"<?􀀀","inputUtf16":[60,63,56256,56320],"output":[{"Comment":{"data":"?􀀀"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1327() {
    tokenize(
        r##"{"description":"<@","initialState":"Data","input":"<@","inputUtf16":[60,64],"output":[{"Character":{"data":"<@"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1328() {
    tokenize(
        r##"{"description":"<A>","initialState":"Data","input":"<A>","inputUtf16":[60,65,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1329() {
    tokenize(
        r##"{"description":"<B>","initialState":"Data","input":"<B>","inputUtf16":[60,66,62],"output":[{"StartTag":{"name":"b","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1330() {
    tokenize(
        r##"{"description":"<Y>","initialState":"Data","input":"<Y>","inputUtf16":[60,89,62],"output":[{"StartTag":{"name":"y","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1331() {
    tokenize(
        r##"{"description":"<Z>","initialState":"Data","input":"<Z>","inputUtf16":[60,90,62],"output":[{"StartTag":{"name":"z","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1332() {
    tokenize(
        r##"{"description":"<[","initialState":"Data","input":"<[","inputUtf16":[60,91],"output":[{"Character":{"data":"<["}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1333() {
    tokenize(
        r##"{"description":"<`","initialState":"Data","input":"<`","inputUtf16":[60,96],"output":[{"Character":{"data":"<`"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1334() {
    tokenize(
        r##"{"description":"<a>","initialState":"Data","input":"<a>","inputUtf16":[60,97,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1335() {
    tokenize(
        r##"{"description":"<a\\u0000>","initialState":"Data","input":"<a\u0000>","inputUtf16":[60,97,0,62],"output":[{"StartTag":{"name":"a�","attrs":{},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1336() {
    tokenize(
        r##"{"description":"<a\\u0008>","initialState":"Data","input":"<a\b>","inputUtf16":[60,97,8,62],"output":[{"StartTag":{"name":"a\b","attrs":{},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1337() {
    tokenize(
        r##"{"description":"<a\\u0009>","initialState":"Data","input":"<a\t>","inputUtf16":[60,97,9,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1338() {
    tokenize(
        r##"{"description":"<a\\u000A>","initialState":"Data","input":"<a\n>","inputUtf16":[60,97,10,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1339() {
    tokenize(
        r##"{"description":"<a\\u000B>","initialState":"Data","input":"<a\u000b>","inputUtf16":[60,97,11,62],"output":[{"StartTag":{"name":"a\u000b","attrs":{},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1340() {
    tokenize(
        r##"{"description":"<a\\u000C>","initialState":"Data","input":"<a\f>","inputUtf16":[60,97,12,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1341() {
    tokenize(
        r##"{"description":"<a\\u000D>","initialState":"Data","input":"<a\r>","inputUtf16":[60,97,13,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1342() {
    tokenize(
        r##"{"description":"<a\\u001F>","initialState":"Data","input":"<a\u001f>","inputUtf16":[60,97,31,62],"output":[{"StartTag":{"name":"a\u001f","attrs":{},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_1343() {
    tokenize(
        r##"{"description":"<a >","initialState":"Data","input":"<a >","inputUtf16":[60,97,32,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1344() {
    tokenize(
        r##"{"description":"<a \\u0000>","initialState":"Data","input":"<a \u0000>","inputUtf16":[60,97,32,0,62],"output":[{"StartTag":{"name":"a","attrs":{"�":""},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1345() {
    tokenize(
        r##"{"description":"<a \\u0008>","initialState":"Data","input":"<a \b>","inputUtf16":[60,97,32,8,62],"output":[{"StartTag":{"name":"a","attrs":{"\b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1346() {
    tokenize(
        r##"{"description":"<a \\u0009>","initialState":"Data","input":"<a \t>","inputUtf16":[60,97,32,9,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1347() {
    tokenize(
        r##"{"description":"<a \\u000A>","initialState":"Data","input":"<a \n>","inputUtf16":[60,97,32,10,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1348() {
    tokenize(
        r##"{"description":"<a \\u000B>","initialState":"Data","input":"<a \u000b>","inputUtf16":[60,97,32,11,62],"output":[{"StartTag":{"name":"a","attrs":{"\u000b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1349() {
    tokenize(
        r##"{"description":"<a \\u000C>","initialState":"Data","input":"<a \f>","inputUtf16":[60,97,32,12,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1350() {
    tokenize(
        r##"{"description":"<a \\u000D>","initialState":"Data","input":"<a \r>","inputUtf16":[60,97,32,13,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1351() {
    tokenize(
        r##"{"description":"<a \\u001F>","initialState":"Data","input":"<a \u001f>","inputUtf16":[60,97,32,31,62],"output":[{"StartTag":{"name":"a","attrs":{"\u001f":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1352() {
    tokenize(
        r##"{"description":"<a  >","initialState":"Data","input":"<a  >","inputUtf16":[60,97,32,32,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1353() {
    tokenize(
        r##"{"description":"<a !>","initialState":"Data","input":"<a !>","inputUtf16":[60,97,32,33,62],"output":[{"StartTag":{"name":"a","attrs":{"!":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1354() {
    tokenize(
        r##"{"description":"<a \">","initialState":"Data","input":"<a \">","inputUtf16":[60,97,32,34,62],"output":[{"StartTag":{"name":"a","attrs":{"\"":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1355() {
    tokenize(
        r##"{"description":"<a #>","initialState":"Data","input":"<a #>","inputUtf16":[60,97,32,35,62],"output":[{"StartTag":{"name":"a","attrs":{"#":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1356() {
    tokenize(
        r##"{"description":"<a &>","initialState":"Data","input":"<a &>","inputUtf16":[60,97,32,38,62],"output":[{"StartTag":{"name":"a","attrs":{"&":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1357() {
    tokenize(
        r##"{"description":"<a '>","initialState":"Data","input":"<a '>","inputUtf16":[60,97,32,39,62],"output":[{"StartTag":{"name":"a","attrs":{"'":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1358() {
    tokenize(
        r##"{"description":"<a (>","initialState":"Data","input":"<a (>","inputUtf16":[60,97,32,40,62],"output":[{"StartTag":{"name":"a","attrs":{"(":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1359() {
    tokenize(
        r##"{"description":"<a ->","initialState":"Data","input":"<a ->","inputUtf16":[60,97,32,45,62],"output":[{"StartTag":{"name":"a","attrs":{"-":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1360() {
    tokenize(
        r##"{"description":"<a .>","initialState":"Data","input":"<a .>","inputUtf16":[60,97,32,46,62],"output":[{"StartTag":{"name":"a","attrs":{".":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1361() {
    tokenize(
        r##"{"description":"<a />","initialState":"Data","input":"<a />","inputUtf16":[60,97,32,47,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_1362() {
    tokenize(
        r##"{"description":"<a 0>","initialState":"Data","input":"<a 0>","inputUtf16":[60,97,32,48,62],"output":[{"StartTag":{"name":"a","attrs":{"0":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1363() {
    tokenize(
        r##"{"description":"<a 1>","initialState":"Data","input":"<a 1>","inputUtf16":[60,97,32,49,62],"output":[{"StartTag":{"name":"a","attrs":{"1":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1364() {
    tokenize(
        r##"{"description":"<a 9>","initialState":"Data","input":"<a 9>","inputUtf16":[60,97,32,57,62],"output":[{"StartTag":{"name":"a","attrs":{"9":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1365() {
    tokenize(
        r##"{"description":"<a <>","initialState":"Data","input":"<a <>","inputUtf16":[60,97,32,60,62],"output":[{"StartTag":{"name":"a","attrs":{"<":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1366() {
    tokenize(
        r##"{"description":"<a =>","initialState":"Data","input":"<a =>","inputUtf16":[60,97,32,61,62],"output":[{"StartTag":{"name":"a","attrs":{"=":""},"self_closing":false}}],"errors":[{"code":"UnexpectedEqualsSignBeforeAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1367() {
    tokenize(
        r##"{"description":"<a >","initialState":"Data","input":"<a >","inputUtf16":[60,97,32,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1368() {
    tokenize(
        r##"{"description":"<a ?>","initialState":"Data","input":"<a ?>","inputUtf16":[60,97,32,63,62],"output":[{"StartTag":{"name":"a","attrs":{"?":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1369() {
    tokenize(
        r##"{"description":"<a @>","initialState":"Data","input":"<a @>","inputUtf16":[60,97,32,64,62],"output":[{"StartTag":{"name":"a","attrs":{"@":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1370() {
    tokenize(
        r##"{"description":"<a A>","initialState":"Data","input":"<a A>","inputUtf16":[60,97,32,65,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1371() {
    tokenize(
        r##"{"description":"<a B>","initialState":"Data","input":"<a B>","inputUtf16":[60,97,32,66,62],"output":[{"StartTag":{"name":"a","attrs":{"b":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1372() {
    tokenize(
        r##"{"description":"<a Y>","initialState":"Data","input":"<a Y>","inputUtf16":[60,97,32,89,62],"output":[{"StartTag":{"name":"a","attrs":{"y":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1373() {
    tokenize(
        r##"{"description":"<a Z>","initialState":"Data","input":"<a Z>","inputUtf16":[60,97,32,90,62],"output":[{"StartTag":{"name":"a","attrs":{"z":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1374() {
    tokenize(
        r##"{"description":"<a [>","initialState":"Data","input":"<a [>","inputUtf16":[60,97,32,91,62],"output":[{"StartTag":{"name":"a","attrs":{"[":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1375() {
    tokenize(
        r##"{"description":"<a `>","initialState":"Data","input":"<a `>","inputUtf16":[60,97,32,96,62],"output":[{"StartTag":{"name":"a","attrs":{"`":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1376() {
    tokenize(
        r##"{"description":"<a a>","initialState":"Data","input":"<a a>","inputUtf16":[60,97,32,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1377() {
    tokenize(
        r##"{"description":"<a a\\u0000>","initialState":"Data","input":"<a a\u0000>","inputUtf16":[60,97,32,97,0,62],"output":[{"StartTag":{"name":"a","attrs":{"a�":""},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_1378() {
    tokenize(
        r##"{"description":"<a a\\u0008>","initialState":"Data","input":"<a a\b>","inputUtf16":[60,97,32,97,8,62],"output":[{"StartTag":{"name":"a","attrs":{"a\b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_1379() {
    tokenize(
        r##"{"description":"<a a\\u0009>","initialState":"Data","input":"<a a\t>","inputUtf16":[60,97,32,97,9,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1380() {
    tokenize(
        r##"{"description":"<a a\\u000A>","initialState":"Data","input":"<a a\n>","inputUtf16":[60,97,32,97,10,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1381() {
    tokenize(
        r##"{"description":"<a a\\u000B>","initialState":"Data","input":"<a a\u000b>","inputUtf16":[60,97,32,97,11,62],"output":[{"StartTag":{"name":"a","attrs":{"a\u000b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_1382() {
    tokenize(
        r##"{"description":"<a a\\u000C>","initialState":"Data","input":"<a a\f>","inputUtf16":[60,97,32,97,12,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1383() {
    tokenize(
        r##"{"description":"<a a\\u000D>","initialState":"Data","input":"<a a\r>","inputUtf16":[60,97,32,97,13,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1384() {
    tokenize(
        r##"{"description":"<a a\\u001F>","initialState":"Data","input":"<a a\u001f>","inputUtf16":[60,97,32,97,31,62],"output":[{"StartTag":{"name":"a","attrs":{"a\u001f":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_1385() {
    tokenize(
        r##"{"description":"<a a >","initialState":"Data","input":"<a a >","inputUtf16":[60,97,32,97,32,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1386() {
    tokenize(
        r##"{"description":"<a a \\u0000>","initialState":"Data","input":"<a a \u0000>","inputUtf16":[60,97,32,97,32,0,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","�":""},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1387() {
    tokenize(
        r##"{"description":"<a a \\u0008>","initialState":"Data","input":"<a a \b>","inputUtf16":[60,97,32,97,32,8,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1388() {
    tokenize(
        r##"{"description":"<a a \\u0009>","initialState":"Data","input":"<a a \t>","inputUtf16":[60,97,32,97,32,9,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1389() {
    tokenize(
        r##"{"description":"<a a \\u000A>","initialState":"Data","input":"<a a \n>","inputUtf16":[60,97,32,97,32,10,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1390() {
    tokenize(
        r##"{"description":"<a a \\u000B>","initialState":"Data","input":"<a a \u000b>","inputUtf16":[60,97,32,97,32,11,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\u000b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1391() {
    tokenize(
        r##"{"description":"<a a \\u000C>","initialState":"Data","input":"<a a \f>","inputUtf16":[60,97,32,97,32,12,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1392() {
    tokenize(
        r##"{"description":"<a a \\u000D>","initialState":"Data","input":"<a a \r>","inputUtf16":[60,97,32,97,32,13,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1393() {
    tokenize(
        r##"{"description":"<a a \\u001F>","initialState":"Data","input":"<a a \u001f>","inputUtf16":[60,97,32,97,32,31,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\u001f":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1394() {
    tokenize(
        r##"{"description":"<a a  >","initialState":"Data","input":"<a a  >","inputUtf16":[60,97,32,97,32,32,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1395() {
    tokenize(
        r##"{"description":"<a a !>","initialState":"Data","input":"<a a !>","inputUtf16":[60,97,32,97,32,33,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","!":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1396() {
    tokenize(
        r##"{"description":"<a a \">","initialState":"Data","input":"<a a \">","inputUtf16":[60,97,32,97,32,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\"":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1397() {
    tokenize(
        r##"{"description":"<a a #>","initialState":"Data","input":"<a a #>","inputUtf16":[60,97,32,97,32,35,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","#":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1398() {
    tokenize(
        r##"{"description":"<a a &>","initialState":"Data","input":"<a a &>","inputUtf16":[60,97,32,97,32,38,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","&":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1399() {
    tokenize(
        r##"{"description":"<a a '>","initialState":"Data","input":"<a a '>","inputUtf16":[60,97,32,97,32,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","'":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1400() {
    tokenize(
        r##"{"description":"<a a (>","initialState":"Data","input":"<a a (>","inputUtf16":[60,97,32,97,32,40,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","(":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1401() {
    tokenize(
        r##"{"description":"<a a ->","initialState":"Data","input":"<a a ->","inputUtf16":[60,97,32,97,32,45,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","-":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1402() {
    tokenize(
        r##"{"description":"<a a .>","initialState":"Data","input":"<a a .>","inputUtf16":[60,97,32,97,32,46,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"",".":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1403() {
    tokenize(
        r##"{"description":"<a a />","initialState":"Data","input":"<a a />","inputUtf16":[60,97,32,97,32,47,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_1404() {
    tokenize(
        r##"{"description":"<a a 0>","initialState":"Data","input":"<a a 0>","inputUtf16":[60,97,32,97,32,48,62],"output":[{"StartTag":{"name":"a","attrs":{"0":"","a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1405() {
    tokenize(
        r##"{"description":"<a a 1>","initialState":"Data","input":"<a a 1>","inputUtf16":[60,97,32,97,32,49,62],"output":[{"StartTag":{"name":"a","attrs":{"1":"","a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1406() {
    tokenize(
        r##"{"description":"<a a 9>","initialState":"Data","input":"<a a 9>","inputUtf16":[60,97,32,97,32,57,62],"output":[{"StartTag":{"name":"a","attrs":{"9":"","a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1407() {
    tokenize(
        r##"{"description":"<a a <>","initialState":"Data","input":"<a a <>","inputUtf16":[60,97,32,97,32,60,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","<":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1408() {
    tokenize(
        r##"{"description":"<a a =>","initialState":"Data","input":"<a a =>","inputUtf16":[60,97,32,97,32,61,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1409() {
    tokenize(
        r##"{"description":"<a a >","initialState":"Data","input":"<a a >","inputUtf16":[60,97,32,97,32,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1410() {
    tokenize(
        r##"{"description":"<a a ?>","initialState":"Data","input":"<a a ?>","inputUtf16":[60,97,32,97,32,63,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","?":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1411() {
    tokenize(
        r##"{"description":"<a a @>","initialState":"Data","input":"<a a @>","inputUtf16":[60,97,32,97,32,64,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","@":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1412() {
    tokenize(
        r##"{"description":"<a a A>","initialState":"Data","input":"<a a A>","inputUtf16":[60,97,32,97,32,65,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"DuplicateAttribute","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1413() {
    tokenize(
        r##"{"description":"<a a B>","initialState":"Data","input":"<a a B>","inputUtf16":[60,97,32,97,32,66,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","b":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1414() {
    tokenize(
        r##"{"description":"<a a Y>","initialState":"Data","input":"<a a Y>","inputUtf16":[60,97,32,97,32,89,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","y":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1415() {
    tokenize(
        r##"{"description":"<a a Z>","initialState":"Data","input":"<a a Z>","inputUtf16":[60,97,32,97,32,90,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","z":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1416() {
    tokenize(
        r##"{"description":"<a a [>","initialState":"Data","input":"<a a [>","inputUtf16":[60,97,32,97,32,91,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","[":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1417() {
    tokenize(
        r##"{"description":"<a a `>","initialState":"Data","input":"<a a `>","inputUtf16":[60,97,32,97,32,96,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","`":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1418() {
    tokenize(
        r##"{"description":"<a a a>","initialState":"Data","input":"<a a a>","inputUtf16":[60,97,32,97,32,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"DuplicateAttribute","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1419() {
    tokenize(
        r##"{"description":"<a a b>","initialState":"Data","input":"<a a b>","inputUtf16":[60,97,32,97,32,98,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","b":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1420() {
    tokenize(
        r##"{"description":"<a a y>","initialState":"Data","input":"<a a y>","inputUtf16":[60,97,32,97,32,121,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","y":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1421() {
    tokenize(
        r##"{"description":"<a a z>","initialState":"Data","input":"<a a z>","inputUtf16":[60,97,32,97,32,122,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","z":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1422() {
    tokenize(
        r##"{"description":"<a a {>","initialState":"Data","input":"<a a {>","inputUtf16":[60,97,32,97,32,123,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","{":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1423() {
    tokenize(
        r##"{"description":"<a a \\uDBC0\\uDC00>","initialState":"Data","input":"<a a 􀀀>","inputUtf16":[60,97,32,97,32,56256,56320,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","􀀀":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1424() {
    tokenize(
        r##"{"description":"<a a!>","initialState":"Data","input":"<a a!>","inputUtf16":[60,97,32,97,33,62],"output":[{"StartTag":{"name":"a","attrs":{"a!":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1425() {
    tokenize(
        r##"{"description":"<a a\">","initialState":"Data","input":"<a a\">","inputUtf16":[60,97,32,97,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a\"":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_1426() {
    tokenize(
        r##"{"description":"<a a#>","initialState":"Data","input":"<a a#>","inputUtf16":[60,97,32,97,35,62],"output":[{"StartTag":{"name":"a","attrs":{"a#":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1427() {
    tokenize(
        r##"{"description":"<a a&>","initialState":"Data","input":"<a a&>","inputUtf16":[60,97,32,97,38,62],"output":[{"StartTag":{"name":"a","attrs":{"a&":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1428() {
    tokenize(
        r##"{"description":"<a a'>","initialState":"Data","input":"<a a'>","inputUtf16":[60,97,32,97,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a'":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_1429() {
    tokenize(
        r##"{"description":"<a a(>","initialState":"Data","input":"<a a(>","inputUtf16":[60,97,32,97,40,62],"output":[{"StartTag":{"name":"a","attrs":{"a(":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1430() {
    tokenize(
        r##"{"description":"<a a->","initialState":"Data","input":"<a a->","inputUtf16":[60,97,32,97,45,62],"output":[{"StartTag":{"name":"a","attrs":{"a-":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1431() {
    tokenize(
        r##"{"description":"<a a.>","initialState":"Data","input":"<a a.>","inputUtf16":[60,97,32,97,46,62],"output":[{"StartTag":{"name":"a","attrs":{"a.":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1432() {
    tokenize(
        r##"{"description":"<a a/>","initialState":"Data","input":"<a a/>","inputUtf16":[60,97,32,97,47,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_1433() {
    tokenize(
        r##"{"description":"<a a0>","initialState":"Data","input":"<a a0>","inputUtf16":[60,97,32,97,48,62],"output":[{"StartTag":{"name":"a","attrs":{"a0":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1434() {
    tokenize(
        r##"{"description":"<a a1>","initialState":"Data","input":"<a a1>","inputUtf16":[60,97,32,97,49,62],"output":[{"StartTag":{"name":"a","attrs":{"a1":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1435() {
    tokenize(
        r##"{"description":"<a a9>","initialState":"Data","input":"<a a9>","inputUtf16":[60,97,32,97,57,62],"output":[{"StartTag":{"name":"a","attrs":{"a9":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1436() {
    tokenize(
        r##"{"description":"<a a<>","initialState":"Data","input":"<a a<>","inputUtf16":[60,97,32,97,60,62],"output":[{"StartTag":{"name":"a","attrs":{"a<":""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_1437() {
    tokenize(
        r##"{"description":"<a a=>","initialState":"Data","input":"<a a=>","inputUtf16":[60,97,32,97,61,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1438() {
    tokenize(
        r##"{"description":"<a a=\\u0000>","initialState":"Data","input":"<a a=\u0000>","inputUtf16":[60,97,32,97,61,0,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"�"},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1439() {
    tokenize(
        r##"{"description":"<a a=\\u0008>","initialState":"Data","input":"<a a=\b>","inputUtf16":[60,97,32,97,61,8,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\b"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1440() {
    tokenize(
        r##"{"description":"<a a=\\u0009>","initialState":"Data","input":"<a a=\t>","inputUtf16":[60,97,32,97,61,9,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1441() {
    tokenize(
        r##"{"description":"<a a=\\u000A>","initialState":"Data","input":"<a a=\n>","inputUtf16":[60,97,32,97,61,10,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1442() {
    tokenize(
        r##"{"description":"<a a=\\u000B>","initialState":"Data","input":"<a a=\u000b>","inputUtf16":[60,97,32,97,61,11,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\u000b"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1443() {
    tokenize(
        r##"{"description":"<a a=\\u000C>","initialState":"Data","input":"<a a=\f>","inputUtf16":[60,97,32,97,61,12,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1444() {
    tokenize(
        r##"{"description":"<a a=\\u000D>","initialState":"Data","input":"<a a=\r>","inputUtf16":[60,97,32,97,61,13,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_1445() {
    tokenize(
        r##"{"description":"<a a=\\u001F>","initialState":"Data","input":"<a a=\u001f>","inputUtf16":[60,97,32,97,61,31,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\u001f"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1446() {
    tokenize(
        r##"{"description":"<a a= >","initialState":"Data","input":"<a a= >","inputUtf16":[60,97,32,97,61,32,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1447() {
    tokenize(
        r##"{"description":"<a a=!>","initialState":"Data","input":"<a a=!>","inputUtf16":[60,97,32,97,61,33,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"!"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1448() {
    tokenize(
        r##"{"description":"<a a=\"\">","initialState":"Data","input":"<a a=\"\">","inputUtf16":[60,97,32,97,61,34,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1449() {
    tokenize(
        r##"{"description":"<a a=\"\\u0000\">","initialState":"Data","input":"<a a=\"\u0000\">","inputUtf16":[60,97,32,97,61,34,0,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"�"},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1450() {
    tokenize(
        r##"{"description":"<a a=\"\\u0009\">","initialState":"Data","input":"<a a=\"\t\">","inputUtf16":[60,97,32,97,61,34,9,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\t"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1451() {
    tokenize(
        r##"{"description":"<a a=\"\\u000A\">","initialState":"Data","input":"<a a=\"\n\">","inputUtf16":[60,97,32,97,61,34,10,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\n"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1452() {
    tokenize(
        r##"{"description":"<a a=\"\\u000B\">","initialState":"Data","input":"<a a=\"\u000b\">","inputUtf16":[60,97,32,97,61,34,11,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\u000b"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1453() {
    tokenize(
        r##"{"description":"<a a=\"\\u000C\">","initialState":"Data","input":"<a a=\"\f\">","inputUtf16":[60,97,32,97,61,34,12,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\f"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1454() {
    tokenize(
        r##"{"description":"<a a=\" \">","initialState":"Data","input":"<a a=\" \">","inputUtf16":[60,97,32,97,61,34,32,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":" "},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1455() {
    tokenize(
        r##"{"description":"<a a=\"!\">","initialState":"Data","input":"<a a=\"!\">","inputUtf16":[60,97,32,97,61,34,33,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"!"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1456() {
    tokenize(
        r##"{"description":"<a a=\"\">","initialState":"Data","input":"<a a=\"\">","inputUtf16":[60,97,32,97,61,34,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1457() {
    tokenize(
        r##"{"description":"<a a=\"#\">","initialState":"Data","input":"<a a=\"#\">","inputUtf16":[60,97,32,97,61,34,35,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"#"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1458() {
    tokenize(
        r##"{"description":"<a a=\"%\">","initialState":"Data","input":"<a a=\"%\">","inputUtf16":[60,97,32,97,61,34,37,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"%"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1459() {
    tokenize(
        r##"{"description":"<a a=\"&\">","initialState":"Data","input":"<a a=\"&\">","inputUtf16":[60,97,32,97,61,34,38,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"&"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1460() {
    tokenize(
        r##"{"description":"<a a=\"'\">","initialState":"Data","input":"<a a=\"'\">","inputUtf16":[60,97,32,97,61,34,39,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"'"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1461() {
    tokenize(
        r##"{"description":"<a a=\"-\">","initialState":"Data","input":"<a a=\"-\">","inputUtf16":[60,97,32,97,61,34,45,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"-"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1462() {
    tokenize(
        r##"{"description":"<a a=\"/\">","initialState":"Data","input":"<a a=\"/\">","inputUtf16":[60,97,32,97,61,34,47,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"/"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1463() {
    tokenize(
        r##"{"description":"<a a=\"0\">","initialState":"Data","input":"<a a=\"0\">","inputUtf16":[60,97,32,97,61,34,48,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"0"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1464() {
    tokenize(
        r##"{"description":"<a a=\"1\">","initialState":"Data","input":"<a a=\"1\">","inputUtf16":[60,97,32,97,61,34,49,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"1"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1465() {
    tokenize(
        r##"{"description":"<a a=\"9\">","initialState":"Data","input":"<a a=\"9\">","inputUtf16":[60,97,32,97,61,34,57,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"9"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1466() {
    tokenize(
        r##"{"description":"<a a=\"<\">","initialState":"Data","input":"<a a=\"<\">","inputUtf16":[60,97,32,97,61,34,60,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"<"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1467() {
    tokenize(
        r##"{"description":"<a a=\"=\">","initialState":"Data","input":"<a a=\"=\">","inputUtf16":[60,97,32,97,61,34,61,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"="},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1468() {
    tokenize(
        r##"{"description":"<a a=\">\">","initialState":"Data","input":"<a a=\">\">","inputUtf16":[60,97,32,97,61,34,62,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":">"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1469() {
    tokenize(
        r##"{"description":"<a a=\"?\">","initialState":"Data","input":"<a a=\"?\">","inputUtf16":[60,97,32,97,61,34,63,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"?"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1470() {
    tokenize(
        r##"{"description":"<a a=\"@\">","initialState":"Data","input":"<a a=\"@\">","inputUtf16":[60,97,32,97,61,34,64,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"@"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1471() {
    tokenize(
        r##"{"description":"<a a=\"A\">","initialState":"Data","input":"<a a=\"A\">","inputUtf16":[60,97,32,97,61,34,65,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"A"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1472() {
    tokenize(
        r##"{"description":"<a a=\"B\">","initialState":"Data","input":"<a a=\"B\">","inputUtf16":[60,97,32,97,61,34,66,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"B"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1473() {
    tokenize(
        r##"{"description":"<a a=\"Y\">","initialState":"Data","input":"<a a=\"Y\">","inputUtf16":[60,97,32,97,61,34,89,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"Y"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1474() {
    tokenize(
        r##"{"description":"<a a=\"Z\">","initialState":"Data","input":"<a a=\"Z\">","inputUtf16":[60,97,32,97,61,34,90,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"Z"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1475() {
    tokenize(
        r##"{"description":"<a a=\"`\">","initialState":"Data","input":"<a a=\"`\">","inputUtf16":[60,97,32,97,61,34,96,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"`"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1476() {
    tokenize(
        r##"{"description":"<a a=\"a\">","initialState":"Data","input":"<a a=\"a\">","inputUtf16":[60,97,32,97,61,34,97,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1477() {
    tokenize(
        r##"{"description":"<a a=\"b\">","initialState":"Data","input":"<a a=\"b\">","inputUtf16":[60,97,32,97,61,34,98,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"b"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1478() {
    tokenize(
        r##"{"description":"<a a=\"y\">","initialState":"Data","input":"<a a=\"y\">","inputUtf16":[60,97,32,97,61,34,121,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"y"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1479() {
    tokenize(
        r##"{"description":"<a a=\"z\">","initialState":"Data","input":"<a a=\"z\">","inputUtf16":[60,97,32,97,61,34,122,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"z"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1480() {
    tokenize(
        r##"{"description":"<a a=\"{\">","initialState":"Data","input":"<a a=\"{\">","inputUtf16":[60,97,32,97,61,34,123,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"{"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1481() {
    tokenize(
        r##"{"description":"<a a=\"\\uDBC0\\uDC00\">","initialState":"Data","input":"<a a=\"􀀀\">","inputUtf16":[60,97,32,97,61,34,56256,56320,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"􀀀"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1482() {
    tokenize(
        r##"{"description":"<a a=#>","initialState":"Data","input":"<a a=#>","inputUtf16":[60,97,32,97,61,35,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"#"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1483() {
    tokenize(
        r##"{"description":"<a a=%>","initialState":"Data","input":"<a a=%>","inputUtf16":[60,97,32,97,61,37,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"%"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1484() {
    tokenize(
        r##"{"description":"<a a=&>","initialState":"Data","input":"<a a=&>","inputUtf16":[60,97,32,97,61,38,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"&"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1485() {
    tokenize(
        r##"{"description":"<a a=''>","initialState":"Data","input":"<a a=''>","inputUtf16":[60,97,32,97,61,39,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1486() {
    tokenize(
        r##"{"description":"<a a='\\u0000'>","initialState":"Data","input":"<a a='\u0000'>","inputUtf16":[60,97,32,97,61,39,0,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"�"},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1487() {
    tokenize(
        r##"{"description":"<a a='\\u0009'>","initialState":"Data","input":"<a a='\t'>","inputUtf16":[60,97,32,97,61,39,9,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\t"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1488() {
    tokenize(
        r##"{"description":"<a a='\\u000A'>","initialState":"Data","input":"<a a='\n'>","inputUtf16":[60,97,32,97,61,39,10,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\n"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1489() {
    tokenize(
        r##"{"description":"<a a='\\u000B'>","initialState":"Data","input":"<a a='\u000b'>","inputUtf16":[60,97,32,97,61,39,11,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\u000b"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1490() {
    tokenize(
        r##"{"description":"<a a='\\u000C'>","initialState":"Data","input":"<a a='\f'>","inputUtf16":[60,97,32,97,61,39,12,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\f"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1491() {
    tokenize(
        r##"{"description":"<a a=' '>","initialState":"Data","input":"<a a=' '>","inputUtf16":[60,97,32,97,61,39,32,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":" "},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1492() {
    tokenize(
        r##"{"description":"<a a='!'>","initialState":"Data","input":"<a a='!'>","inputUtf16":[60,97,32,97,61,39,33,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"!"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1493() {
    tokenize(
        r##"{"description":"<a a='\"'>","initialState":"Data","input":"<a a='\"'>","inputUtf16":[60,97,32,97,61,39,34,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"\""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1494() {
    tokenize(
        r##"{"description":"<a a='%'>","initialState":"Data","input":"<a a='%'>","inputUtf16":[60,97,32,97,61,39,37,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"%"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1495() {
    tokenize(
        r##"{"description":"<a a='&'>","initialState":"Data","input":"<a a='&'>","inputUtf16":[60,97,32,97,61,39,38,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"&"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1496() {
    tokenize(
        r##"{"description":"<a a=''>","initialState":"Data","input":"<a a=''>","inputUtf16":[60,97,32,97,61,39,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1497() {
    tokenize(
        r##"{"description":"<a a=''\\u0000>","initialState":"Data","input":"<a a=''\u0000>","inputUtf16":[60,97,32,97,61,39,39,0,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","�":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1498() {
    tokenize(
        r##"{"description":"<a a=''\\u0008>","initialState":"Data","input":"<a a=''\b>","inputUtf16":[60,97,32,97,61,39,39,8,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":8}},{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1499() {
    tokenize(
        r##"{"description":"<a a=''\\u0009>","initialState":"Data","input":"<a a=''\t>","inputUtf16":[60,97,32,97,61,39,39,9,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1500() {
    tokenize(
        r##"{"description":"<a a=''\\u000A>","initialState":"Data","input":"<a a=''\n>","inputUtf16":[60,97,32,97,61,39,39,10,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1501() {
    tokenize(
        r##"{"description":"<a a=''\\u000B>","initialState":"Data","input":"<a a=''\u000b>","inputUtf16":[60,97,32,97,61,39,39,11,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\u000b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":8}},{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1502() {
    tokenize(
        r##"{"description":"<a a=''\\u000C>","initialState":"Data","input":"<a a=''\f>","inputUtf16":[60,97,32,97,61,39,39,12,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1503() {
    tokenize(
        r##"{"description":"<a a=''\\u000D>","initialState":"Data","input":"<a a=''\r>","inputUtf16":[60,97,32,97,61,39,39,13,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1504() {
    tokenize(
        r##"{"description":"<a a=''\\u001F>","initialState":"Data","input":"<a a=''\u001f>","inputUtf16":[60,97,32,97,61,39,39,31,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\u001f":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":8}},{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1505() {
    tokenize(
        r##"{"description":"<a a='' >","initialState":"Data","input":"<a a='' >","inputUtf16":[60,97,32,97,61,39,39,32,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1506() {
    tokenize(
        r##"{"description":"<a a=''!>","initialState":"Data","input":"<a a=''!>","inputUtf16":[60,97,32,97,61,39,39,33,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","!":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1507() {
    tokenize(
        r##"{"description":"<a a=''\">","initialState":"Data","input":"<a a=''\">","inputUtf16":[60,97,32,97,61,39,39,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","\"":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}},{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1508() {
    tokenize(
        r##"{"description":"<a a=''&>","initialState":"Data","input":"<a a=''&>","inputUtf16":[60,97,32,97,61,39,39,38,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","&":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1509() {
    tokenize(
        r##"{"description":"<a a='''>","initialState":"Data","input":"<a a='''>","inputUtf16":[60,97,32,97,61,39,39,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","'":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}},{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1510() {
    tokenize(
        r##"{"description":"<a a=''->","initialState":"Data","input":"<a a=''->","inputUtf16":[60,97,32,97,61,39,39,45,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","-":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1511() {
    tokenize(
        r##"{"description":"<a a=''.>","initialState":"Data","input":"<a a=''.>","inputUtf16":[60,97,32,97,61,39,39,46,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"",".":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1512() {
    tokenize(
        r##"{"description":"<a a=''/>","initialState":"Data","input":"<a a=''/>","inputUtf16":[60,97,32,97,61,39,39,47,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_1513() {
    tokenize(
        r##"{"description":"<a a=''0>","initialState":"Data","input":"<a a=''0>","inputUtf16":[60,97,32,97,61,39,39,48,62],"output":[{"StartTag":{"name":"a","attrs":{"0":"","a":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1514() {
    tokenize(
        r##"{"description":"<a a=''1>","initialState":"Data","input":"<a a=''1>","inputUtf16":[60,97,32,97,61,39,39,49,62],"output":[{"StartTag":{"name":"a","attrs":{"1":"","a":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1515() {
    tokenize(
        r##"{"description":"<a a=''9>","initialState":"Data","input":"<a a=''9>","inputUtf16":[60,97,32,97,61,39,39,57,62],"output":[{"StartTag":{"name":"a","attrs":{"9":"","a":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1516() {
    tokenize(
        r##"{"description":"<a a=''<>","initialState":"Data","input":"<a a=''<>","inputUtf16":[60,97,32,97,61,39,39,60,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","<":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}},{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1517() {
    tokenize(
        r##"{"description":"<a a=''=>","initialState":"Data","input":"<a a=''=>","inputUtf16":[60,97,32,97,61,39,39,61,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","=":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}},{"code":"UnexpectedEqualsSignBeforeAttributeName","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1518() {
    tokenize(
        r##"{"description":"<a a=''>","initialState":"Data","input":"<a a=''>","inputUtf16":[60,97,32,97,61,39,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1519() {
    tokenize(
        r##"{"description":"<a a=''?>","initialState":"Data","input":"<a a=''?>","inputUtf16":[60,97,32,97,61,39,39,63,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","?":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1520() {
    tokenize(
        r##"{"description":"<a a=''@>","initialState":"Data","input":"<a a=''@>","inputUtf16":[60,97,32,97,61,39,39,64,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","@":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1521() {
    tokenize(
        r##"{"description":"<a a=''A>","initialState":"Data","input":"<a a=''A>","inputUtf16":[60,97,32,97,61,39,39,65,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}},{"code":"DuplicateAttribute","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_1522() {
    tokenize(
        r##"{"description":"<a a=''B>","initialState":"Data","input":"<a a=''B>","inputUtf16":[60,97,32,97,61,39,39,66,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","b":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1523() {
    tokenize(
        r##"{"description":"<a a=''Y>","initialState":"Data","input":"<a a=''Y>","inputUtf16":[60,97,32,97,61,39,39,89,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","y":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1524() {
    tokenize(
        r##"{"description":"<a a=''Z>","initialState":"Data","input":"<a a=''Z>","inputUtf16":[60,97,32,97,61,39,39,90,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","z":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1525() {
    tokenize(
        r##"{"description":"<a a=''`>","initialState":"Data","input":"<a a=''`>","inputUtf16":[60,97,32,97,61,39,39,96,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","`":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1526() {
    tokenize(
        r##"{"description":"<a a=''a>","initialState":"Data","input":"<a a=''a>","inputUtf16":[60,97,32,97,61,39,39,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}},{"code":"DuplicateAttribute","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_1527() {
    tokenize(
        r##"{"description":"<a a=''b>","initialState":"Data","input":"<a a=''b>","inputUtf16":[60,97,32,97,61,39,39,98,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","b":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1528() {
    tokenize(
        r##"{"description":"<a a=''y>","initialState":"Data","input":"<a a=''y>","inputUtf16":[60,97,32,97,61,39,39,121,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","y":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1529() {
    tokenize(
        r##"{"description":"<a a=''z>","initialState":"Data","input":"<a a=''z>","inputUtf16":[60,97,32,97,61,39,39,122,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","z":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1530() {
    tokenize(
        r##"{"description":"<a a=''{>","initialState":"Data","input":"<a a=''{>","inputUtf16":[60,97,32,97,61,39,39,123,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","{":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1531() {
    tokenize(
        r##"{"description":"<a a=''\\uDBC0\\uDC00>","initialState":"Data","input":"<a a=''􀀀>","inputUtf16":[60,97,32,97,61,39,39,56256,56320,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"","􀀀":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_1532() {
    tokenize(
        r##"{"description":"<a a='('>","initialState":"Data","input":"<a a='('>","inputUtf16":[60,97,32,97,61,39,40,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"("},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1533() {
    tokenize(
        r##"{"description":"<a a='-'>","initialState":"Data","input":"<a a='-'>","inputUtf16":[60,97,32,97,61,39,45,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"-"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1534() {
    tokenize(
        r##"{"description":"<a a='/'>","initialState":"Data","input":"<a a='/'>","inputUtf16":[60,97,32,97,61,39,47,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"/"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1535() {
    tokenize(
        r##"{"description":"<a a='0'>","initialState":"Data","input":"<a a='0'>","inputUtf16":[60,97,32,97,61,39,48,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"0"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1536() {
    tokenize(
        r##"{"description":"<a a='1'>","initialState":"Data","input":"<a a='1'>","inputUtf16":[60,97,32,97,61,39,49,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"1"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1537() {
    tokenize(
        r##"{"description":"<a a='9'>","initialState":"Data","input":"<a a='9'>","inputUtf16":[60,97,32,97,61,39,57,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"9"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1538() {
    tokenize(
        r##"{"description":"<a a='<'>","initialState":"Data","input":"<a a='<'>","inputUtf16":[60,97,32,97,61,39,60,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"<"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1539() {
    tokenize(
        r##"{"description":"<a a='='>","initialState":"Data","input":"<a a='='>","inputUtf16":[60,97,32,97,61,39,61,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"="},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1540() {
    tokenize(
        r##"{"description":"<a a='>'>","initialState":"Data","input":"<a a='>'>","inputUtf16":[60,97,32,97,61,39,62,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":">"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1541() {
    tokenize(
        r##"{"description":"<a a='?'>","initialState":"Data","input":"<a a='?'>","inputUtf16":[60,97,32,97,61,39,63,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"?"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1542() {
    tokenize(
        r##"{"description":"<a a='@'>","initialState":"Data","input":"<a a='@'>","inputUtf16":[60,97,32,97,61,39,64,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"@"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1543() {
    tokenize(
        r##"{"description":"<a a='A'>","initialState":"Data","input":"<a a='A'>","inputUtf16":[60,97,32,97,61,39,65,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"A"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1544() {
    tokenize(
        r##"{"description":"<a a='B'>","initialState":"Data","input":"<a a='B'>","inputUtf16":[60,97,32,97,61,39,66,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"B"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1545() {
    tokenize(
        r##"{"description":"<a a='Y'>","initialState":"Data","input":"<a a='Y'>","inputUtf16":[60,97,32,97,61,39,89,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"Y"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1546() {
    tokenize(
        r##"{"description":"<a a='Z'>","initialState":"Data","input":"<a a='Z'>","inputUtf16":[60,97,32,97,61,39,90,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"Z"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1547() {
    tokenize(
        r##"{"description":"<a a='`'>","initialState":"Data","input":"<a a='`'>","inputUtf16":[60,97,32,97,61,39,96,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"`"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1548() {
    tokenize(
        r##"{"description":"<a a='a'>","initialState":"Data","input":"<a a='a'>","inputUtf16":[60,97,32,97,61,39,97,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1549() {
    tokenize(
        r##"{"description":"<a a='b'>","initialState":"Data","input":"<a a='b'>","inputUtf16":[60,97,32,97,61,39,98,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"b"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1550() {
    tokenize(
        r##"{"description":"<a a='y'>","initialState":"Data","input":"<a a='y'>","inputUtf16":[60,97,32,97,61,39,121,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"y"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1551() {
    tokenize(
        r##"{"description":"<a a='z'>","initialState":"Data","input":"<a a='z'>","inputUtf16":[60,97,32,97,61,39,122,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"z"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1552() {
    tokenize(
        r##"{"description":"<a a='{'>","initialState":"Data","input":"<a a='{'>","inputUtf16":[60,97,32,97,61,39,123,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"{"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1553() {
    tokenize(
        r##"{"description":"<a a='\\uDBC0\\uDC00'>","initialState":"Data","input":"<a a='􀀀'>","inputUtf16":[60,97,32,97,61,39,56256,56320,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"􀀀"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1554() {
    tokenize(
        r##"{"description":"<a a=(>","initialState":"Data","input":"<a a=(>","inputUtf16":[60,97,32,97,61,40,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"("},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1555() {
    tokenize(
        r##"{"description":"<a a=->","initialState":"Data","input":"<a a=->","inputUtf16":[60,97,32,97,61,45,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"-"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1556() {
    tokenize(
        r##"{"description":"<a a=/>","initialState":"Data","input":"<a a=/>","inputUtf16":[60,97,32,97,61,47,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"/"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1557() {
    tokenize(
        r##"{"description":"<a a=0>","initialState":"Data","input":"<a a=0>","inputUtf16":[60,97,32,97,61,48,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"0"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1558() {
    tokenize(
        r##"{"description":"<a a=1>","initialState":"Data","input":"<a a=1>","inputUtf16":[60,97,32,97,61,49,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"1"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1559() {
    tokenize(
        r##"{"description":"<a a=9>","initialState":"Data","input":"<a a=9>","inputUtf16":[60,97,32,97,61,57,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"9"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1560() {
    tokenize(
        r##"{"description":"<a a=<>","initialState":"Data","input":"<a a=<>","inputUtf16":[60,97,32,97,61,60,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"<"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1561() {
    tokenize(
        r##"{"description":"<a a==>","initialState":"Data","input":"<a a==>","inputUtf16":[60,97,32,97,61,61,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"="},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1562() {
    tokenize(
        r##"{"description":"<a a=>","initialState":"Data","input":"<a a=>","inputUtf16":[60,97,32,97,61,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"MissingAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1563() {
    tokenize(
        r##"{"description":"<a a=?>","initialState":"Data","input":"<a a=?>","inputUtf16":[60,97,32,97,61,63,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"?"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1564() {
    tokenize(
        r##"{"description":"<a a=@>","initialState":"Data","input":"<a a=@>","inputUtf16":[60,97,32,97,61,64,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"@"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1565() {
    tokenize(
        r##"{"description":"<a a=A>","initialState":"Data","input":"<a a=A>","inputUtf16":[60,97,32,97,61,65,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"A"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1566() {
    tokenize(
        r##"{"description":"<a a=B>","initialState":"Data","input":"<a a=B>","inputUtf16":[60,97,32,97,61,66,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"B"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1567() {
    tokenize(
        r##"{"description":"<a a=Y>","initialState":"Data","input":"<a a=Y>","inputUtf16":[60,97,32,97,61,89,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"Y"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1568() {
    tokenize(
        r##"{"description":"<a a=Z>","initialState":"Data","input":"<a a=Z>","inputUtf16":[60,97,32,97,61,90,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"Z"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1569() {
    tokenize(
        r##"{"description":"<a a=`>","initialState":"Data","input":"<a a=`>","inputUtf16":[60,97,32,97,61,96,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"`"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_1570() {
    tokenize(
        r##"{"description":"<a a=a>","initialState":"Data","input":"<a a=a>","inputUtf16":[60,97,32,97,61,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1571() {
    tokenize(
        r##"{"description":"<a a=a\\u0000>","initialState":"Data","input":"<a a=a\u0000>","inputUtf16":[60,97,32,97,61,97,0,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a�"},"self_closing":false}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1572() {
    tokenize(
        r##"{"description":"<a a=a\\u0008>","initialState":"Data","input":"<a a=a\b>","inputUtf16":[60,97,32,97,61,97,8,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a\b"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1573() {
    tokenize(
        r##"{"description":"<a a=a\\u0009>","initialState":"Data","input":"<a a=a\t>","inputUtf16":[60,97,32,97,61,97,9,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1574() {
    tokenize(
        r##"{"description":"<a a=a\\u000A>","initialState":"Data","input":"<a a=a\n>","inputUtf16":[60,97,32,97,61,97,10,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1575() {
    tokenize(
        r##"{"description":"<a a=a\\u000B>","initialState":"Data","input":"<a a=a\u000b>","inputUtf16":[60,97,32,97,61,97,11,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a\u000b"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1576() {
    tokenize(
        r##"{"description":"<a a=a\\u000C>","initialState":"Data","input":"<a a=a\f>","inputUtf16":[60,97,32,97,61,97,12,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1577() {
    tokenize(
        r##"{"description":"<a a=a\\u000D>","initialState":"Data","input":"<a a=a\r>","inputUtf16":[60,97,32,97,61,97,13,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1578() {
    tokenize(
        r##"{"description":"<a a=a\\u001F>","initialState":"Data","input":"<a a=a\u001f>","inputUtf16":[60,97,32,97,61,97,31,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a\u001f"},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1579() {
    tokenize(
        r##"{"description":"<a a=a >","initialState":"Data","input":"<a a=a >","inputUtf16":[60,97,32,97,61,97,32,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1580() {
    tokenize(
        r##"{"description":"<a a=a!>","initialState":"Data","input":"<a a=a!>","inputUtf16":[60,97,32,97,61,97,33,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a!"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1581() {
    tokenize(
        r##"{"description":"<a a=a\">","initialState":"Data","input":"<a a=a\">","inputUtf16":[60,97,32,97,61,97,34,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a\""},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1582() {
    tokenize(
        r##"{"description":"<a a=a#>","initialState":"Data","input":"<a a=a#>","inputUtf16":[60,97,32,97,61,97,35,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a#"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1583() {
    tokenize(
        r##"{"description":"<a a=a%>","initialState":"Data","input":"<a a=a%>","inputUtf16":[60,97,32,97,61,97,37,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a%"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1584() {
    tokenize(
        r##"{"description":"<a a=a&>","initialState":"Data","input":"<a a=a&>","inputUtf16":[60,97,32,97,61,97,38,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a&"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1585() {
    tokenize(
        r##"{"description":"<a a=a'>","initialState":"Data","input":"<a a=a'>","inputUtf16":[60,97,32,97,61,97,39,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a'"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1586() {
    tokenize(
        r##"{"description":"<a a=a(>","initialState":"Data","input":"<a a=a(>","inputUtf16":[60,97,32,97,61,97,40,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a("},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1587() {
    tokenize(
        r##"{"description":"<a a=a->","initialState":"Data","input":"<a a=a->","inputUtf16":[60,97,32,97,61,97,45,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a-"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1588() {
    tokenize(
        r##"{"description":"<a a=a/>","initialState":"Data","input":"<a a=a/>","inputUtf16":[60,97,32,97,61,97,47,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a/"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1589() {
    tokenize(
        r##"{"description":"<a a=a0>","initialState":"Data","input":"<a a=a0>","inputUtf16":[60,97,32,97,61,97,48,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a0"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1590() {
    tokenize(
        r##"{"description":"<a a=a1>","initialState":"Data","input":"<a a=a1>","inputUtf16":[60,97,32,97,61,97,49,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a1"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1591() {
    tokenize(
        r##"{"description":"<a a=a9>","initialState":"Data","input":"<a a=a9>","inputUtf16":[60,97,32,97,61,97,57,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a9"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1592() {
    tokenize(
        r##"{"description":"<a a=a<>","initialState":"Data","input":"<a a=a<>","inputUtf16":[60,97,32,97,61,97,60,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a<"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1593() {
    tokenize(
        r##"{"description":"<a a=a=>","initialState":"Data","input":"<a a=a=>","inputUtf16":[60,97,32,97,61,97,61,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a="},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1594() {
    tokenize(
        r##"{"description":"<a a=a>","initialState":"Data","input":"<a a=a>","inputUtf16":[60,97,32,97,61,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1595() {
    tokenize(
        r##"{"description":"<a a=a?>","initialState":"Data","input":"<a a=a?>","inputUtf16":[60,97,32,97,61,97,63,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a?"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1596() {
    tokenize(
        r##"{"description":"<a a=a@>","initialState":"Data","input":"<a a=a@>","inputUtf16":[60,97,32,97,61,97,64,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a@"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1597() {
    tokenize(
        r##"{"description":"<a a=aA>","initialState":"Data","input":"<a a=aA>","inputUtf16":[60,97,32,97,61,97,65,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"aA"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1598() {
    tokenize(
        r##"{"description":"<a a=aB>","initialState":"Data","input":"<a a=aB>","inputUtf16":[60,97,32,97,61,97,66,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"aB"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1599() {
    tokenize(
        r##"{"description":"<a a=aY>","initialState":"Data","input":"<a a=aY>","inputUtf16":[60,97,32,97,61,97,89,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"aY"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1600() {
    tokenize(
        r##"{"description":"<a a=aZ>","initialState":"Data","input":"<a a=aZ>","inputUtf16":[60,97,32,97,61,97,90,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"aZ"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1601() {
    tokenize(
        r##"{"description":"<a a=a`>","initialState":"Data","input":"<a a=a`>","inputUtf16":[60,97,32,97,61,97,96,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a`"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1602() {
    tokenize(
        r##"{"description":"<a a=aa>","initialState":"Data","input":"<a a=aa>","inputUtf16":[60,97,32,97,61,97,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"aa"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1603() {
    tokenize(
        r##"{"description":"<a a=ab>","initialState":"Data","input":"<a a=ab>","inputUtf16":[60,97,32,97,61,97,98,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"ab"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1604() {
    tokenize(
        r##"{"description":"<a a=ay>","initialState":"Data","input":"<a a=ay>","inputUtf16":[60,97,32,97,61,97,121,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"ay"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1605() {
    tokenize(
        r##"{"description":"<a a=az>","initialState":"Data","input":"<a a=az>","inputUtf16":[60,97,32,97,61,97,122,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"az"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1606() {
    tokenize(
        r##"{"description":"<a a=a{>","initialState":"Data","input":"<a a=a{>","inputUtf16":[60,97,32,97,61,97,123,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a{"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1607() {
    tokenize(
        r##"{"description":"<a a=a\\uDBC0\\uDC00>","initialState":"Data","input":"<a a=a􀀀>","inputUtf16":[60,97,32,97,61,97,56256,56320,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"a􀀀"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1608() {
    tokenize(
        r##"{"description":"<a a=b>","initialState":"Data","input":"<a a=b>","inputUtf16":[60,97,32,97,61,98,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"b"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1609() {
    tokenize(
        r##"{"description":"<a a=y>","initialState":"Data","input":"<a a=y>","inputUtf16":[60,97,32,97,61,121,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"y"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1610() {
    tokenize(
        r##"{"description":"<a a=z>","initialState":"Data","input":"<a a=z>","inputUtf16":[60,97,32,97,61,122,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"z"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1611() {
    tokenize(
        r##"{"description":"<a a={>","initialState":"Data","input":"<a a={>","inputUtf16":[60,97,32,97,61,123,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"{"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1612() {
    tokenize(
        r##"{"description":"<a a=\\uDBC0\\uDC00>","initialState":"Data","input":"<a a=􀀀>","inputUtf16":[60,97,32,97,61,56256,56320,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"􀀀"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1613() {
    tokenize(
        r##"{"description":"<a a>","initialState":"Data","input":"<a a>","inputUtf16":[60,97,32,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1614() {
    tokenize(
        r##"{"description":"<a a?>","initialState":"Data","input":"<a a?>","inputUtf16":[60,97,32,97,63,62],"output":[{"StartTag":{"name":"a","attrs":{"a?":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1615() {
    tokenize(
        r##"{"description":"<a a@>","initialState":"Data","input":"<a a@>","inputUtf16":[60,97,32,97,64,62],"output":[{"StartTag":{"name":"a","attrs":{"a@":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1616() {
    tokenize(
        r##"{"description":"<a aA>","initialState":"Data","input":"<a aA>","inputUtf16":[60,97,32,97,65,62],"output":[{"StartTag":{"name":"a","attrs":{"aa":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1617() {
    tokenize(
        r##"{"description":"<a aB>","initialState":"Data","input":"<a aB>","inputUtf16":[60,97,32,97,66,62],"output":[{"StartTag":{"name":"a","attrs":{"ab":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1618() {
    tokenize(
        r##"{"description":"<a aY>","initialState":"Data","input":"<a aY>","inputUtf16":[60,97,32,97,89,62],"output":[{"StartTag":{"name":"a","attrs":{"ay":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1619() {
    tokenize(
        r##"{"description":"<a aZ>","initialState":"Data","input":"<a aZ>","inputUtf16":[60,97,32,97,90,62],"output":[{"StartTag":{"name":"a","attrs":{"az":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1620() {
    tokenize(
        r##"{"description":"<a a[>","initialState":"Data","input":"<a a[>","inputUtf16":[60,97,32,97,91,62],"output":[{"StartTag":{"name":"a","attrs":{"a[":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1621() {
    tokenize(
        r##"{"description":"<a a`>","initialState":"Data","input":"<a a`>","inputUtf16":[60,97,32,97,96,62],"output":[{"StartTag":{"name":"a","attrs":{"a`":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1622() {
    tokenize(
        r##"{"description":"<a aa>","initialState":"Data","input":"<a aa>","inputUtf16":[60,97,32,97,97,62],"output":[{"StartTag":{"name":"a","attrs":{"aa":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1623() {
    tokenize(
        r##"{"description":"<a ab>","initialState":"Data","input":"<a ab>","inputUtf16":[60,97,32,97,98,62],"output":[{"StartTag":{"name":"a","attrs":{"ab":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1624() {
    tokenize(
        r##"{"description":"<a ay>","initialState":"Data","input":"<a ay>","inputUtf16":[60,97,32,97,121,62],"output":[{"StartTag":{"name":"a","attrs":{"ay":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1625() {
    tokenize(
        r##"{"description":"<a az>","initialState":"Data","input":"<a az>","inputUtf16":[60,97,32,97,122,62],"output":[{"StartTag":{"name":"a","attrs":{"az":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1626() {
    tokenize(
        r##"{"description":"<a a{>","initialState":"Data","input":"<a a{>","inputUtf16":[60,97,32,97,123,62],"output":[{"StartTag":{"name":"a","attrs":{"a{":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1627() {
    tokenize(
        r##"{"description":"<a a\\uDBC0\\uDC00>","initialState":"Data","input":"<a a􀀀>","inputUtf16":[60,97,32,97,56256,56320,62],"output":[{"StartTag":{"name":"a","attrs":{"a􀀀":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1628() {
    tokenize(
        r##"{"description":"<a b>","initialState":"Data","input":"<a b>","inputUtf16":[60,97,32,98,62],"output":[{"StartTag":{"name":"a","attrs":{"b":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1629() {
    tokenize(
        r##"{"description":"<a y>","initialState":"Data","input":"<a y>","inputUtf16":[60,97,32,121,62],"output":[{"StartTag":{"name":"a","attrs":{"y":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1630() {
    tokenize(
        r##"{"description":"<a z>","initialState":"Data","input":"<a z>","inputUtf16":[60,97,32,122,62],"output":[{"StartTag":{"name":"a","attrs":{"z":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1631() {
    tokenize(
        r##"{"description":"<a {>","initialState":"Data","input":"<a {>","inputUtf16":[60,97,32,123,62],"output":[{"StartTag":{"name":"a","attrs":{"{":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1632() {
    tokenize(
        r##"{"description":"<a \\uDBC0\\uDC00>","initialState":"Data","input":"<a 􀀀>","inputUtf16":[60,97,32,56256,56320,62],"output":[{"StartTag":{"name":"a","attrs":{"􀀀":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1633() {
    tokenize(
        r##"{"description":"<a!>","initialState":"Data","input":"<a!>","inputUtf16":[60,97,33,62],"output":[{"StartTag":{"name":"a!","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1634() {
    tokenize(
        r##"{"description":"<a\">","initialState":"Data","input":"<a\">","inputUtf16":[60,97,34,62],"output":[{"StartTag":{"name":"a\"","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1635() {
    tokenize(
        r##"{"description":"<a&>","initialState":"Data","input":"<a&>","inputUtf16":[60,97,38,62],"output":[{"StartTag":{"name":"a&","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1636() {
    tokenize(
        r##"{"description":"<a'>","initialState":"Data","input":"<a'>","inputUtf16":[60,97,39,62],"output":[{"StartTag":{"name":"a'","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1637() {
    tokenize(
        r##"{"description":"<a->","initialState":"Data","input":"<a->","inputUtf16":[60,97,45,62],"output":[{"StartTag":{"name":"a-","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1638() {
    tokenize(
        r##"{"description":"<a.>","initialState":"Data","input":"<a.>","inputUtf16":[60,97,46,62],"output":[{"StartTag":{"name":"a.","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1639() {
    tokenize(
        r##"{"description":"<a/>","initialState":"Data","input":"<a/>","inputUtf16":[60,97,47,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_1640() {
    tokenize(
        r##"{"description":"<a/\\u0000>","initialState":"Data","input":"<a/\u0000>","inputUtf16":[60,97,47,0,62],"output":[{"StartTag":{"name":"a","attrs":{"�":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1641() {
    tokenize(
        r##"{"description":"<a/\\u0009>","initialState":"Data","input":"<a/\t>","inputUtf16":[60,97,47,9,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1642() {
    tokenize(
        r##"{"description":"<a/\\u000A>","initialState":"Data","input":"<a/\n>","inputUtf16":[60,97,47,10,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1643() {
    tokenize(
        r##"{"description":"<a/\\u000B>","initialState":"Data","input":"<a/\u000b>","inputUtf16":[60,97,47,11,62],"output":[{"StartTag":{"name":"a","attrs":{"\u000b":""},"self_closing":false}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":4}},{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1644() {
    tokenize(
        r##"{"description":"<a/\\u000C>","initialState":"Data","input":"<a/\f>","inputUtf16":[60,97,47,12,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1645() {
    tokenize(
        r##"{"description":"<a/ >","initialState":"Data","input":"<a/ >","inputUtf16":[60,97,47,32,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1646() {
    tokenize(
        r##"{"description":"<a/!>","initialState":"Data","input":"<a/!>","inputUtf16":[60,97,47,33,62],"output":[{"StartTag":{"name":"a","attrs":{"!":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1647() {
    tokenize(
        r##"{"description":"<a/\">","initialState":"Data","input":"<a/\">","inputUtf16":[60,97,47,34,62],"output":[{"StartTag":{"name":"a","attrs":{"\"":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}},{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1648() {
    tokenize(
        r##"{"description":"<a/&>","initialState":"Data","input":"<a/&>","inputUtf16":[60,97,47,38,62],"output":[{"StartTag":{"name":"a","attrs":{"&":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1649() {
    tokenize(
        r##"{"description":"<a/'>","initialState":"Data","input":"<a/'>","inputUtf16":[60,97,47,39,62],"output":[{"StartTag":{"name":"a","attrs":{"'":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}},{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1650() {
    tokenize(
        r##"{"description":"<a/->","initialState":"Data","input":"<a/->","inputUtf16":[60,97,47,45,62],"output":[{"StartTag":{"name":"a","attrs":{"-":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1651() {
    tokenize(
        r##"{"description":"<a//>","initialState":"Data","input":"<a//>","inputUtf16":[60,97,47,47,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":true}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1652() {
    tokenize(
        r##"{"description":"<a/0>","initialState":"Data","input":"<a/0>","inputUtf16":[60,97,47,48,62],"output":[{"StartTag":{"name":"a","attrs":{"0":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1653() {
    tokenize(
        r##"{"description":"<a/1>","initialState":"Data","input":"<a/1>","inputUtf16":[60,97,47,49,62],"output":[{"StartTag":{"name":"a","attrs":{"1":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1654() {
    tokenize(
        r##"{"description":"<a/9>","initialState":"Data","input":"<a/9>","inputUtf16":[60,97,47,57,62],"output":[{"StartTag":{"name":"a","attrs":{"9":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1655() {
    tokenize(
        r##"{"description":"<a/<>","initialState":"Data","input":"<a/<>","inputUtf16":[60,97,47,60,62],"output":[{"StartTag":{"name":"a","attrs":{"<":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}},{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1656() {
    tokenize(
        r##"{"description":"<a/=>","initialState":"Data","input":"<a/=>","inputUtf16":[60,97,47,61,62],"output":[{"StartTag":{"name":"a","attrs":{"=":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}},{"code":"UnexpectedEqualsSignBeforeAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1657() {
    tokenize(
        r##"{"description":"<a/>","initialState":"Data","input":"<a/>","inputUtf16":[60,97,47,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_1658() {
    tokenize(
        r##"{"description":"<a/?>","initialState":"Data","input":"<a/?>","inputUtf16":[60,97,47,63,62],"output":[{"StartTag":{"name":"a","attrs":{"?":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1659() {
    tokenize(
        r##"{"description":"<a/@>","initialState":"Data","input":"<a/@>","inputUtf16":[60,97,47,64,62],"output":[{"StartTag":{"name":"a","attrs":{"@":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1660() {
    tokenize(
        r##"{"description":"<a/A>","initialState":"Data","input":"<a/A>","inputUtf16":[60,97,47,65,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1661() {
    tokenize(
        r##"{"description":"<a/B>","initialState":"Data","input":"<a/B>","inputUtf16":[60,97,47,66,62],"output":[{"StartTag":{"name":"a","attrs":{"b":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1662() {
    tokenize(
        r##"{"description":"<a/Y>","initialState":"Data","input":"<a/Y>","inputUtf16":[60,97,47,89,62],"output":[{"StartTag":{"name":"a","attrs":{"y":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1663() {
    tokenize(
        r##"{"description":"<a/Z>","initialState":"Data","input":"<a/Z>","inputUtf16":[60,97,47,90,62],"output":[{"StartTag":{"name":"a","attrs":{"z":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1664() {
    tokenize(
        r##"{"description":"<a/`>","initialState":"Data","input":"<a/`>","inputUtf16":[60,97,47,96,62],"output":[{"StartTag":{"name":"a","attrs":{"`":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1665() {
    tokenize(
        r##"{"description":"<a/a>","initialState":"Data","input":"<a/a>","inputUtf16":[60,97,47,97,62],"output":[{"StartTag":{"name":"a","attrs":{"a":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1666() {
    tokenize(
        r##"{"description":"<a/b>","initialState":"Data","input":"<a/b>","inputUtf16":[60,97,47,98,62],"output":[{"StartTag":{"name":"a","attrs":{"b":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1667() {
    tokenize(
        r##"{"description":"<a/y>","initialState":"Data","input":"<a/y>","inputUtf16":[60,97,47,121,62],"output":[{"StartTag":{"name":"a","attrs":{"y":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1668() {
    tokenize(
        r##"{"description":"<a/z>","initialState":"Data","input":"<a/z>","inputUtf16":[60,97,47,122,62],"output":[{"StartTag":{"name":"a","attrs":{"z":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1669() {
    tokenize(
        r##"{"description":"<a/{>","initialState":"Data","input":"<a/{>","inputUtf16":[60,97,47,123,62],"output":[{"StartTag":{"name":"a","attrs":{"{":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1670() {
    tokenize(
        r##"{"description":"<a/\\uDBC0\\uDC00>","initialState":"Data","input":"<a/􀀀>","inputUtf16":[60,97,47,56256,56320,62],"output":[{"StartTag":{"name":"a","attrs":{"􀀀":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_1671() {
    tokenize(
        r##"{"description":"<a0>","initialState":"Data","input":"<a0>","inputUtf16":[60,97,48,62],"output":[{"StartTag":{"name":"a0","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1672() {
    tokenize(
        r##"{"description":"<a1>","initialState":"Data","input":"<a1>","inputUtf16":[60,97,49,62],"output":[{"StartTag":{"name":"a1","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1673() {
    tokenize(
        r##"{"description":"<a9>","initialState":"Data","input":"<a9>","inputUtf16":[60,97,57,62],"output":[{"StartTag":{"name":"a9","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1674() {
    tokenize(
        r##"{"description":"<a<>","initialState":"Data","input":"<a<>","inputUtf16":[60,97,60,62],"output":[{"StartTag":{"name":"a<","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1675() {
    tokenize(
        r##"{"description":"<a=>","initialState":"Data","input":"<a=>","inputUtf16":[60,97,61,62],"output":[{"StartTag":{"name":"a=","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1676() {
    tokenize(
        r##"{"description":"<a>","initialState":"Data","input":"<a>","inputUtf16":[60,97,62],"output":[{"StartTag":{"name":"a","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1677() {
    tokenize(
        r##"{"description":"<a?>","initialState":"Data","input":"<a?>","inputUtf16":[60,97,63,62],"output":[{"StartTag":{"name":"a?","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1678() {
    tokenize(
        r##"{"description":"<a@>","initialState":"Data","input":"<a@>","inputUtf16":[60,97,64,62],"output":[{"StartTag":{"name":"a@","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1679() {
    tokenize(
        r##"{"description":"<aA>","initialState":"Data","input":"<aA>","inputUtf16":[60,97,65,62],"output":[{"StartTag":{"name":"aa","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1680() {
    tokenize(
        r##"{"description":"<aB>","initialState":"Data","input":"<aB>","inputUtf16":[60,97,66,62],"output":[{"StartTag":{"name":"ab","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1681() {
    tokenize(
        r##"{"description":"<aY>","initialState":"Data","input":"<aY>","inputUtf16":[60,97,89,62],"output":[{"StartTag":{"name":"ay","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1682() {
    tokenize(
        r##"{"description":"<aZ>","initialState":"Data","input":"<aZ>","inputUtf16":[60,97,90,62],"output":[{"StartTag":{"name":"az","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1683() {
    tokenize(
        r##"{"description":"<a[>","initialState":"Data","input":"<a[>","inputUtf16":[60,97,91,62],"output":[{"StartTag":{"name":"a[","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1684() {
    tokenize(
        r##"{"description":"<a`>","initialState":"Data","input":"<a`>","inputUtf16":[60,97,96,62],"output":[{"StartTag":{"name":"a`","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1685() {
    tokenize(
        r##"{"description":"<aa>","initialState":"Data","input":"<aa>","inputUtf16":[60,97,97,62],"output":[{"StartTag":{"name":"aa","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1686() {
    tokenize(
        r##"{"description":"<ab>","initialState":"Data","input":"<ab>","inputUtf16":[60,97,98,62],"output":[{"StartTag":{"name":"ab","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1687() {
    tokenize(
        r##"{"description":"<ay>","initialState":"Data","input":"<ay>","inputUtf16":[60,97,121,62],"output":[{"StartTag":{"name":"ay","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1688() {
    tokenize(
        r##"{"description":"<az>","initialState":"Data","input":"<az>","inputUtf16":[60,97,122,62],"output":[{"StartTag":{"name":"az","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1689() {
    tokenize(
        r##"{"description":"<a{>","initialState":"Data","input":"<a{>","inputUtf16":[60,97,123,62],"output":[{"StartTag":{"name":"a{","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1690() {
    tokenize(
        r##"{"description":"<a\\uDBC0\\uDC00>","initialState":"Data","input":"<a􀀀>","inputUtf16":[60,97,56256,56320,62],"output":[{"StartTag":{"name":"a􀀀","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1691() {
    tokenize(
        r##"{"description":"<b>","initialState":"Data","input":"<b>","inputUtf16":[60,98,62],"output":[{"StartTag":{"name":"b","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1692() {
    tokenize(
        r##"{"description":"<y>","initialState":"Data","input":"<y>","inputUtf16":[60,121,62],"output":[{"StartTag":{"name":"y","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1693() {
    tokenize(
        r##"{"description":"<z>","initialState":"Data","input":"<z>","inputUtf16":[60,122,62],"output":[{"StartTag":{"name":"z","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1694() {
    tokenize(
        r##"{"description":"<{","initialState":"Data","input":"<{","inputUtf16":[60,123],"output":[{"Character":{"data":"<{"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1695() {
    tokenize(
        r##"{"description":"<\\uDBC0\\uDC00","initialState":"Data","input":"<􀀀","inputUtf16":[60,56256,56320],"output":[{"Character":{"data":"<􀀀"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1696() {
    tokenize(
        r##"{"description":"=","initialState":"Data","input":"=","inputUtf16":[61],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_1697() {
    tokenize(
        r##"{"description":"=","initialState":"Plaintext","input":"=","inputUtf16":[61],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_1698() {
    tokenize(
        r##"{"description":"=","initialState":"Rcdata","input":"=","inputUtf16":[61],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_1699() {
    tokenize(
        r##"{"description":"=","initialState":"Rawtext","input":"=","inputUtf16":[61],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_1700() {
    tokenize(
        r##"{"description":"=","initialState":"ScriptData","input":"=","inputUtf16":[61],"output":[{"Character":{"data":"="}}],"errors":[]}"##,
    );
}

#[test]
fn test_1701() {
    tokenize(
        r##"{"description":"=","initialState":"CdataSection","input":"=","inputUtf16":[61],"output":[{"Character":{"data":"="}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1702() {
    tokenize(
        r##"{"description":">","initialState":"Data","input":">","inputUtf16":[62],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1703() {
    tokenize(
        r##"{"description":">","initialState":"Plaintext","input":">","inputUtf16":[62],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1704() {
    tokenize(
        r##"{"description":">","initialState":"Rcdata","input":">","inputUtf16":[62],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1705() {
    tokenize(
        r##"{"description":">","initialState":"Rawtext","input":">","inputUtf16":[62],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1706() {
    tokenize(
        r##"{"description":">","initialState":"ScriptData","input":">","inputUtf16":[62],"output":[{"Character":{"data":">"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1707() {
    tokenize(
        r##"{"description":">","initialState":"CdataSection","input":">","inputUtf16":[62],"output":[{"Character":{"data":">"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1708() {
    tokenize(
        r##"{"description":"?","initialState":"Data","input":"?","inputUtf16":[63],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1709() {
    tokenize(
        r##"{"description":"?","initialState":"Plaintext","input":"?","inputUtf16":[63],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1710() {
    tokenize(
        r##"{"description":"?","initialState":"Rcdata","input":"?","inputUtf16":[63],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1711() {
    tokenize(
        r##"{"description":"?","initialState":"Rawtext","input":"?","inputUtf16":[63],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1712() {
    tokenize(
        r##"{"description":"?","initialState":"ScriptData","input":"?","inputUtf16":[63],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1713() {
    tokenize(
        r##"{"description":"?","initialState":"CdataSection","input":"?","inputUtf16":[63],"output":[{"Character":{"data":"?"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1714() {
    tokenize(
        r##"{"description":"@","initialState":"Data","input":"@","inputUtf16":[64],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1715() {
    tokenize(
        r##"{"description":"@","initialState":"Plaintext","input":"@","inputUtf16":[64],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1716() {
    tokenize(
        r##"{"description":"@","initialState":"Rcdata","input":"@","inputUtf16":[64],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1717() {
    tokenize(
        r##"{"description":"@","initialState":"Rawtext","input":"@","inputUtf16":[64],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1718() {
    tokenize(
        r##"{"description":"@","initialState":"ScriptData","input":"@","inputUtf16":[64],"output":[{"Character":{"data":"@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1719() {
    tokenize(
        r##"{"description":"@","initialState":"CdataSection","input":"@","inputUtf16":[64],"output":[{"Character":{"data":"@"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1720() {
    tokenize(
        r##"{"description":"A","initialState":"Data","input":"A","inputUtf16":[65],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1721() {
    tokenize(
        r##"{"description":"A","initialState":"Plaintext","input":"A","inputUtf16":[65],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1722() {
    tokenize(
        r##"{"description":"A","initialState":"Rcdata","input":"A","inputUtf16":[65],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1723() {
    tokenize(
        r##"{"description":"A","initialState":"Rawtext","input":"A","inputUtf16":[65],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1724() {
    tokenize(
        r##"{"description":"A","initialState":"ScriptData","input":"A","inputUtf16":[65],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1725() {
    tokenize(
        r##"{"description":"A","initialState":"CdataSection","input":"A","inputUtf16":[65],"output":[{"Character":{"data":"A"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1726() {
    tokenize(
        r##"{"description":"B","initialState":"Data","input":"B","inputUtf16":[66],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1727() {
    tokenize(
        r##"{"description":"B","initialState":"Plaintext","input":"B","inputUtf16":[66],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1728() {
    tokenize(
        r##"{"description":"B","initialState":"Rcdata","input":"B","inputUtf16":[66],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1729() {
    tokenize(
        r##"{"description":"B","initialState":"Rawtext","input":"B","inputUtf16":[66],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1730() {
    tokenize(
        r##"{"description":"B","initialState":"ScriptData","input":"B","inputUtf16":[66],"output":[{"Character":{"data":"B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1731() {
    tokenize(
        r##"{"description":"B","initialState":"CdataSection","input":"B","inputUtf16":[66],"output":[{"Character":{"data":"B"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1732() {
    tokenize(
        r##"{"description":"Y","initialState":"Data","input":"Y","inputUtf16":[89],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1733() {
    tokenize(
        r##"{"description":"Y","initialState":"Plaintext","input":"Y","inputUtf16":[89],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1734() {
    tokenize(
        r##"{"description":"Y","initialState":"Rcdata","input":"Y","inputUtf16":[89],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1735() {
    tokenize(
        r##"{"description":"Y","initialState":"Rawtext","input":"Y","inputUtf16":[89],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1736() {
    tokenize(
        r##"{"description":"Y","initialState":"ScriptData","input":"Y","inputUtf16":[89],"output":[{"Character":{"data":"Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1737() {
    tokenize(
        r##"{"description":"Y","initialState":"CdataSection","input":"Y","inputUtf16":[89],"output":[{"Character":{"data":"Y"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1738() {
    tokenize(
        r##"{"description":"Z","initialState":"Data","input":"Z","inputUtf16":[90],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1739() {
    tokenize(
        r##"{"description":"Z","initialState":"Plaintext","input":"Z","inputUtf16":[90],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1740() {
    tokenize(
        r##"{"description":"Z","initialState":"Rcdata","input":"Z","inputUtf16":[90],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1741() {
    tokenize(
        r##"{"description":"Z","initialState":"Rawtext","input":"Z","inputUtf16":[90],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1742() {
    tokenize(
        r##"{"description":"Z","initialState":"ScriptData","input":"Z","inputUtf16":[90],"output":[{"Character":{"data":"Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1743() {
    tokenize(
        r##"{"description":"Z","initialState":"CdataSection","input":"Z","inputUtf16":[90],"output":[{"Character":{"data":"Z"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1744() {
    tokenize(
        r##"{"description":"`","initialState":"Data","input":"`","inputUtf16":[96],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1745() {
    tokenize(
        r##"{"description":"`","initialState":"Plaintext","input":"`","inputUtf16":[96],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1746() {
    tokenize(
        r##"{"description":"`","initialState":"Rcdata","input":"`","inputUtf16":[96],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1747() {
    tokenize(
        r##"{"description":"`","initialState":"Rawtext","input":"`","inputUtf16":[96],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1748() {
    tokenize(
        r##"{"description":"`","initialState":"ScriptData","input":"`","inputUtf16":[96],"output":[{"Character":{"data":"`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1749() {
    tokenize(
        r##"{"description":"`","initialState":"CdataSection","input":"`","inputUtf16":[96],"output":[{"Character":{"data":"`"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1750() {
    tokenize(
        r##"{"description":"a","initialState":"Data","input":"a","inputUtf16":[97],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1751() {
    tokenize(
        r##"{"description":"a","initialState":"Plaintext","input":"a","inputUtf16":[97],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1752() {
    tokenize(
        r##"{"description":"a","initialState":"Rcdata","input":"a","inputUtf16":[97],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1753() {
    tokenize(
        r##"{"description":"a","initialState":"Rawtext","input":"a","inputUtf16":[97],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1754() {
    tokenize(
        r##"{"description":"a","initialState":"ScriptData","input":"a","inputUtf16":[97],"output":[{"Character":{"data":"a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1755() {
    tokenize(
        r##"{"description":"a","initialState":"CdataSection","input":"a","inputUtf16":[97],"output":[{"Character":{"data":"a"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1756() {
    tokenize(
        r##"{"description":"b","initialState":"Data","input":"b","inputUtf16":[98],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1757() {
    tokenize(
        r##"{"description":"b","initialState":"Plaintext","input":"b","inputUtf16":[98],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1758() {
    tokenize(
        r##"{"description":"b","initialState":"Rcdata","input":"b","inputUtf16":[98],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1759() {
    tokenize(
        r##"{"description":"b","initialState":"Rawtext","input":"b","inputUtf16":[98],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1760() {
    tokenize(
        r##"{"description":"b","initialState":"ScriptData","input":"b","inputUtf16":[98],"output":[{"Character":{"data":"b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1761() {
    tokenize(
        r##"{"description":"b","initialState":"CdataSection","input":"b","inputUtf16":[98],"output":[{"Character":{"data":"b"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1762() {
    tokenize(
        r##"{"description":"y","initialState":"Data","input":"y","inputUtf16":[121],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1763() {
    tokenize(
        r##"{"description":"y","initialState":"Plaintext","input":"y","inputUtf16":[121],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1764() {
    tokenize(
        r##"{"description":"y","initialState":"Rcdata","input":"y","inputUtf16":[121],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1765() {
    tokenize(
        r##"{"description":"y","initialState":"Rawtext","input":"y","inputUtf16":[121],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1766() {
    tokenize(
        r##"{"description":"y","initialState":"ScriptData","input":"y","inputUtf16":[121],"output":[{"Character":{"data":"y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1767() {
    tokenize(
        r##"{"description":"y","initialState":"CdataSection","input":"y","inputUtf16":[121],"output":[{"Character":{"data":"y"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1768() {
    tokenize(
        r##"{"description":"z","initialState":"Data","input":"z","inputUtf16":[122],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1769() {
    tokenize(
        r##"{"description":"z","initialState":"Plaintext","input":"z","inputUtf16":[122],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1770() {
    tokenize(
        r##"{"description":"z","initialState":"Rcdata","input":"z","inputUtf16":[122],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1771() {
    tokenize(
        r##"{"description":"z","initialState":"Rawtext","input":"z","inputUtf16":[122],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1772() {
    tokenize(
        r##"{"description":"z","initialState":"ScriptData","input":"z","inputUtf16":[122],"output":[{"Character":{"data":"z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1773() {
    tokenize(
        r##"{"description":"z","initialState":"CdataSection","input":"z","inputUtf16":[122],"output":[{"Character":{"data":"z"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1774() {
    tokenize(
        r##"{"description":"{","initialState":"Data","input":"{","inputUtf16":[123],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1775() {
    tokenize(
        r##"{"description":"{","initialState":"Plaintext","input":"{","inputUtf16":[123],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1776() {
    tokenize(
        r##"{"description":"{","initialState":"Rcdata","input":"{","inputUtf16":[123],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1777() {
    tokenize(
        r##"{"description":"{","initialState":"Rawtext","input":"{","inputUtf16":[123],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1778() {
    tokenize(
        r##"{"description":"{","initialState":"ScriptData","input":"{","inputUtf16":[123],"output":[{"Character":{"data":"{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1779() {
    tokenize(
        r##"{"description":"{","initialState":"CdataSection","input":"{","inputUtf16":[123],"output":[{"Character":{"data":"{"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1780() {
    tokenize(
        r##"{"description":"\\uDBC0\\uDC00","initialState":"Data","input":"􀀀","inputUtf16":[56256,56320],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1781() {
    tokenize(
        r##"{"description":"\\uDBC0\\uDC00","initialState":"Plaintext","input":"􀀀","inputUtf16":[56256,56320],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1782() {
    tokenize(
        r##"{"description":"\\uDBC0\\uDC00","initialState":"Rcdata","input":"􀀀","inputUtf16":[56256,56320],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1783() {
    tokenize(
        r##"{"description":"\\uDBC0\\uDC00","initialState":"Rawtext","input":"􀀀","inputUtf16":[56256,56320],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1784() {
    tokenize(
        r##"{"description":"\\uDBC0\\uDC00","initialState":"ScriptData","input":"􀀀","inputUtf16":[56256,56320],"output":[{"Character":{"data":"􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1785() {
    tokenize(
        r##"{"description":"\\uDBC0\\uDC00","initialState":"CdataSection","input":"􀀀","inputUtf16":[56256,56320],"output":[{"Character":{"data":"􀀀"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}
//</coverage:exclude>
