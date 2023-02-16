//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Data","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Plaintext","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Rcdata","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"[empty]","initialState":"Rawtext","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"[empty]","initialState":"ScriptData","input":"","inputUtf16":[],"output":[],"errors":[]}"##,
    );
}

#[test]
fn test_5() {
    tokenize(
        r##"{"description":"[empty]","initialState":"CdataSection","input":"","inputUtf16":[],"output":[],"errors":[{"code":"EofInCdata","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_6() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Data","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_7() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Plaintext","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_8() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Rcdata","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_9() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"Rawtext","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_10() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"ScriptData","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[]}"##,
    );
}

#[test]
fn test_11() {
    tokenize(
        r##"{"description":"\\u0009","initialState":"CdataSection","input":"\t","inputUtf16":[9],"output":[{"Character":{"data":"\t"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_12() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Data","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_13() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Plaintext","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_14() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Rcdata","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_15() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"Rawtext","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_16() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"ScriptData","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_17() {
    tokenize(
        r##"{"description":"\\u000A","initialState":"CdataSection","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[{"code":"EofInCdata","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_18() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Data","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_19() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Plaintext","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_20() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Rcdata","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_21() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"Rawtext","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_22() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"ScriptData","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_23() {
    tokenize(
        r##"{"description":"\\u000B","initialState":"CdataSection","input":"\u000b","inputUtf16":[11],"output":[{"Character":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":1}},{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_24() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Data","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_25() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Plaintext","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_26() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Rcdata","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_27() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"Rawtext","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_28() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"ScriptData","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_29() {
    tokenize(
        r##"{"description":"\\u000C","initialState":"CdataSection","input":"\f","inputUtf16":[12],"output":[{"Character":{"data":"\f"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_30() {
    tokenize(
        r##"{"description":" ","initialState":"Data","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_31() {
    tokenize(
        r##"{"description":" ","initialState":"Plaintext","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_32() {
    tokenize(
        r##"{"description":" ","initialState":"Rcdata","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_33() {
    tokenize(
        r##"{"description":" ","initialState":"Rawtext","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_34() {
    tokenize(
        r##"{"description":" ","initialState":"ScriptData","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[]}"##,
    );
}

#[test]
fn test_35() {
    tokenize(
        r##"{"description":" ","initialState":"CdataSection","input":" ","inputUtf16":[32],"output":[{"Character":{"data":" "}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_36() {
    tokenize(
        r##"{"description":"!","initialState":"Data","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_37() {
    tokenize(
        r##"{"description":"!","initialState":"Plaintext","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_38() {
    tokenize(
        r##"{"description":"!","initialState":"Rcdata","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_39() {
    tokenize(
        r##"{"description":"!","initialState":"Rawtext","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_40() {
    tokenize(
        r##"{"description":"!","initialState":"ScriptData","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_41() {
    tokenize(
        r##"{"description":"!","initialState":"CdataSection","input":"!","inputUtf16":[33],"output":[{"Character":{"data":"!"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_42() {
    tokenize(
        r##"{"description":"\"","initialState":"Data","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_43() {
    tokenize(
        r##"{"description":"\"","initialState":"Plaintext","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_44() {
    tokenize(
        r##"{"description":"\"","initialState":"Rcdata","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_45() {
    tokenize(
        r##"{"description":"\"","initialState":"Rawtext","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_46() {
    tokenize(
        r##"{"description":"\"","initialState":"ScriptData","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[]}"##,
    );
}

#[test]
fn test_47() {
    tokenize(
        r##"{"description":"\"","initialState":"CdataSection","input":"\"","inputUtf16":[34],"output":[{"Character":{"data":"\""}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_48() {
    tokenize(
        r##"{"description":"%","initialState":"Data","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_49() {
    tokenize(
        r##"{"description":"%","initialState":"Plaintext","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_50() {
    tokenize(
        r##"{"description":"%","initialState":"Rcdata","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_51() {
    tokenize(
        r##"{"description":"%","initialState":"Rawtext","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_52() {
    tokenize(
        r##"{"description":"%","initialState":"ScriptData","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[]}"##,
    );
}

#[test]
fn test_53() {
    tokenize(
        r##"{"description":"%","initialState":"CdataSection","input":"%","inputUtf16":[37],"output":[{"Character":{"data":"%"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_54() {
    tokenize(
        r##"{"description":"&","initialState":"Data","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_55() {
    tokenize(
        r##"{"description":"&","initialState":"Plaintext","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_56() {
    tokenize(
        r##"{"description":"&","initialState":"Rcdata","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_57() {
    tokenize(
        r##"{"description":"&","initialState":"Rawtext","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_58() {
    tokenize(
        r##"{"description":"&","initialState":"ScriptData","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_59() {
    tokenize(
        r##"{"description":"&","initialState":"CdataSection","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_60() {
    tokenize(
        r##"{"description":"'","initialState":"Data","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_61() {
    tokenize(
        r##"{"description":"'","initialState":"Plaintext","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_62() {
    tokenize(
        r##"{"description":"'","initialState":"Rcdata","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_63() {
    tokenize(
        r##"{"description":"'","initialState":"Rawtext","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_64() {
    tokenize(
        r##"{"description":"'","initialState":"ScriptData","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[]}"##,
    );
}

#[test]
fn test_65() {
    tokenize(
        r##"{"description":"'","initialState":"CdataSection","input":"'","inputUtf16":[39],"output":[{"Character":{"data":"'"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_66() {
    tokenize(
        r##"{"description":",","initialState":"Data","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_67() {
    tokenize(
        r##"{"description":",","initialState":"Plaintext","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_68() {
    tokenize(
        r##"{"description":",","initialState":"Rcdata","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_69() {
    tokenize(
        r##"{"description":",","initialState":"Rawtext","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_70() {
    tokenize(
        r##"{"description":",","initialState":"ScriptData","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[]}"##,
    );
}

#[test]
fn test_71() {
    tokenize(
        r##"{"description":",","initialState":"CdataSection","input":",","inputUtf16":[44],"output":[{"Character":{"data":","}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_72() {
    tokenize(
        r##"{"description":"-","initialState":"Data","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_73() {
    tokenize(
        r##"{"description":"-","initialState":"Plaintext","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_74() {
    tokenize(
        r##"{"description":"-","initialState":"Rcdata","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_75() {
    tokenize(
        r##"{"description":"-","initialState":"Rawtext","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_76() {
    tokenize(
        r##"{"description":"-","initialState":"ScriptData","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_77() {
    tokenize(
        r##"{"description":"-","initialState":"CdataSection","input":"-","inputUtf16":[45],"output":[{"Character":{"data":"-"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_78() {
    tokenize(
        r##"{"description":".","initialState":"Data","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_79() {
    tokenize(
        r##"{"description":".","initialState":"Plaintext","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_80() {
    tokenize(
        r##"{"description":".","initialState":"Rcdata","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_81() {
    tokenize(
        r##"{"description":".","initialState":"Rawtext","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_82() {
    tokenize(
        r##"{"description":".","initialState":"ScriptData","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[]}"##,
    );
}

#[test]
fn test_83() {
    tokenize(
        r##"{"description":".","initialState":"CdataSection","input":".","inputUtf16":[46],"output":[{"Character":{"data":"."}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_84() {
    tokenize(
        r##"{"description":"/","initialState":"Data","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_85() {
    tokenize(
        r##"{"description":"/","initialState":"Plaintext","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_86() {
    tokenize(
        r##"{"description":"/","initialState":"Rcdata","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_87() {
    tokenize(
        r##"{"description":"/","initialState":"Rawtext","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_88() {
    tokenize(
        r##"{"description":"/","initialState":"ScriptData","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_89() {
    tokenize(
        r##"{"description":"/","initialState":"CdataSection","input":"/","inputUtf16":[47],"output":[{"Character":{"data":"/"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_90() {
    tokenize(
        r##"{"description":"0","initialState":"Data","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_91() {
    tokenize(
        r##"{"description":"0","initialState":"Plaintext","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_92() {
    tokenize(
        r##"{"description":"0","initialState":"Rcdata","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_93() {
    tokenize(
        r##"{"description":"0","initialState":"Rawtext","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_94() {
    tokenize(
        r##"{"description":"0","initialState":"ScriptData","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[]}"##,
    );
}

#[test]
fn test_95() {
    tokenize(
        r##"{"description":"0","initialState":"CdataSection","input":"0","inputUtf16":[48],"output":[{"Character":{"data":"0"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_96() {
    tokenize(
        r##"{"description":"1","initialState":"Data","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_97() {
    tokenize(
        r##"{"description":"1","initialState":"Plaintext","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_98() {
    tokenize(
        r##"{"description":"1","initialState":"Rcdata","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_99() {
    tokenize(
        r##"{"description":"1","initialState":"Rawtext","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_100() {
    tokenize(
        r##"{"description":"1","initialState":"ScriptData","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[]}"##,
    );
}

#[test]
fn test_101() {
    tokenize(
        r##"{"description":"1","initialState":"CdataSection","input":"1","inputUtf16":[49],"output":[{"Character":{"data":"1"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_102() {
    tokenize(
        r##"{"description":"9","initialState":"Data","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_103() {
    tokenize(
        r##"{"description":"9","initialState":"Plaintext","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_104() {
    tokenize(
        r##"{"description":"9","initialState":"Rcdata","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_105() {
    tokenize(
        r##"{"description":"9","initialState":"Rawtext","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_106() {
    tokenize(
        r##"{"description":"9","initialState":"ScriptData","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[]}"##,
    );
}

#[test]
fn test_107() {
    tokenize(
        r##"{"description":"9","initialState":"CdataSection","input":"9","inputUtf16":[57],"output":[{"Character":{"data":"9"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_108() {
    tokenize(
        r##"{"description":";","initialState":"Data","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_109() {
    tokenize(
        r##"{"description":";","initialState":"Plaintext","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_110() {
    tokenize(
        r##"{"description":";","initialState":"Rcdata","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_111() {
    tokenize(
        r##"{"description":";","initialState":"Rawtext","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_112() {
    tokenize(
        r##"{"description":";","initialState":"ScriptData","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[]}"##,
    );
}

#[test]
fn test_113() {
    tokenize(
        r##"{"description":";","initialState":"CdataSection","input":";","inputUtf16":[59],"output":[{"Character":{"data":";"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_114() {
    tokenize(
        r##"{"description":";=","initialState":"Data","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_115() {
    tokenize(
        r##"{"description":";=","initialState":"Plaintext","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_116() {
    tokenize(
        r##"{"description":";=","initialState":"Rcdata","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_117() {
    tokenize(
        r##"{"description":";=","initialState":"Rawtext","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_118() {
    tokenize(
        r##"{"description":";=","initialState":"ScriptData","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[]}"##,
    );
}

#[test]
fn test_119() {
    tokenize(
        r##"{"description":";=","initialState":"CdataSection","input":";=","inputUtf16":[59,61],"output":[{"Character":{"data":";="}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_120() {
    tokenize(
        r##"{"description":";>","initialState":"Data","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_121() {
    tokenize(
        r##"{"description":";>","initialState":"Plaintext","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_122() {
    tokenize(
        r##"{"description":";>","initialState":"Rcdata","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_123() {
    tokenize(
        r##"{"description":";>","initialState":"Rawtext","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_124() {
    tokenize(
        r##"{"description":";>","initialState":"ScriptData","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_125() {
    tokenize(
        r##"{"description":";>","initialState":"CdataSection","input":";>","inputUtf16":[59,62],"output":[{"Character":{"data":";>"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_126() {
    tokenize(
        r##"{"description":";?","initialState":"Data","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_127() {
    tokenize(
        r##"{"description":";?","initialState":"Plaintext","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_128() {
    tokenize(
        r##"{"description":";?","initialState":"Rcdata","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_129() {
    tokenize(
        r##"{"description":";?","initialState":"Rawtext","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_130() {
    tokenize(
        r##"{"description":";?","initialState":"ScriptData","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_131() {
    tokenize(
        r##"{"description":";?","initialState":"CdataSection","input":";?","inputUtf16":[59,63],"output":[{"Character":{"data":";?"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_132() {
    tokenize(
        r##"{"description":";@","initialState":"Data","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_133() {
    tokenize(
        r##"{"description":";@","initialState":"Plaintext","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_134() {
    tokenize(
        r##"{"description":";@","initialState":"Rcdata","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_135() {
    tokenize(
        r##"{"description":";@","initialState":"Rawtext","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_136() {
    tokenize(
        r##"{"description":";@","initialState":"ScriptData","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[]}"##,
    );
}

#[test]
fn test_137() {
    tokenize(
        r##"{"description":";@","initialState":"CdataSection","input":";@","inputUtf16":[59,64],"output":[{"Character":{"data":";@"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_138() {
    tokenize(
        r##"{"description":";A","initialState":"Data","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_139() {
    tokenize(
        r##"{"description":";A","initialState":"Plaintext","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_140() {
    tokenize(
        r##"{"description":";A","initialState":"Rcdata","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_141() {
    tokenize(
        r##"{"description":";A","initialState":"Rawtext","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_142() {
    tokenize(
        r##"{"description":";A","initialState":"ScriptData","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_143() {
    tokenize(
        r##"{"description":";A","initialState":"CdataSection","input":";A","inputUtf16":[59,65],"output":[{"Character":{"data":";A"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_144() {
    tokenize(
        r##"{"description":";B","initialState":"Data","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_145() {
    tokenize(
        r##"{"description":";B","initialState":"Plaintext","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_146() {
    tokenize(
        r##"{"description":";B","initialState":"Rcdata","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_147() {
    tokenize(
        r##"{"description":";B","initialState":"Rawtext","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_148() {
    tokenize(
        r##"{"description":";B","initialState":"ScriptData","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[]}"##,
    );
}

#[test]
fn test_149() {
    tokenize(
        r##"{"description":";B","initialState":"CdataSection","input":";B","inputUtf16":[59,66],"output":[{"Character":{"data":";B"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_150() {
    tokenize(
        r##"{"description":";Y","initialState":"Data","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_151() {
    tokenize(
        r##"{"description":";Y","initialState":"Plaintext","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_152() {
    tokenize(
        r##"{"description":";Y","initialState":"Rcdata","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_153() {
    tokenize(
        r##"{"description":";Y","initialState":"Rawtext","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_154() {
    tokenize(
        r##"{"description":";Y","initialState":"ScriptData","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_155() {
    tokenize(
        r##"{"description":";Y","initialState":"CdataSection","input":";Y","inputUtf16":[59,89],"output":[{"Character":{"data":";Y"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_156() {
    tokenize(
        r##"{"description":";Z","initialState":"Data","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_157() {
    tokenize(
        r##"{"description":";Z","initialState":"Plaintext","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_158() {
    tokenize(
        r##"{"description":";Z","initialState":"Rcdata","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_159() {
    tokenize(
        r##"{"description":";Z","initialState":"Rawtext","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_160() {
    tokenize(
        r##"{"description":";Z","initialState":"ScriptData","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_161() {
    tokenize(
        r##"{"description":";Z","initialState":"CdataSection","input":";Z","inputUtf16":[59,90],"output":[{"Character":{"data":";Z"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_162() {
    tokenize(
        r##"{"description":";`","initialState":"Data","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_163() {
    tokenize(
        r##"{"description":";`","initialState":"Plaintext","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_164() {
    tokenize(
        r##"{"description":";`","initialState":"Rcdata","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_165() {
    tokenize(
        r##"{"description":";`","initialState":"Rawtext","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_166() {
    tokenize(
        r##"{"description":";`","initialState":"ScriptData","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[]}"##,
    );
}

#[test]
fn test_167() {
    tokenize(
        r##"{"description":";`","initialState":"CdataSection","input":";`","inputUtf16":[59,96],"output":[{"Character":{"data":";`"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_168() {
    tokenize(
        r##"{"description":";a","initialState":"Data","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_169() {
    tokenize(
        r##"{"description":";a","initialState":"Plaintext","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_170() {
    tokenize(
        r##"{"description":";a","initialState":"Rcdata","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_171() {
    tokenize(
        r##"{"description":";a","initialState":"Rawtext","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_172() {
    tokenize(
        r##"{"description":";a","initialState":"ScriptData","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_173() {
    tokenize(
        r##"{"description":";a","initialState":"CdataSection","input":";a","inputUtf16":[59,97],"output":[{"Character":{"data":";a"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_174() {
    tokenize(
        r##"{"description":";b","initialState":"Data","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_175() {
    tokenize(
        r##"{"description":";b","initialState":"Plaintext","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_176() {
    tokenize(
        r##"{"description":";b","initialState":"Rcdata","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_177() {
    tokenize(
        r##"{"description":";b","initialState":"Rawtext","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_178() {
    tokenize(
        r##"{"description":";b","initialState":"ScriptData","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[]}"##,
    );
}

#[test]
fn test_179() {
    tokenize(
        r##"{"description":";b","initialState":"CdataSection","input":";b","inputUtf16":[59,98],"output":[{"Character":{"data":";b"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_180() {
    tokenize(
        r##"{"description":";y","initialState":"Data","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_181() {
    tokenize(
        r##"{"description":";y","initialState":"Plaintext","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_182() {
    tokenize(
        r##"{"description":";y","initialState":"Rcdata","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_183() {
    tokenize(
        r##"{"description":";y","initialState":"Rawtext","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_184() {
    tokenize(
        r##"{"description":";y","initialState":"ScriptData","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[]}"##,
    );
}

#[test]
fn test_185() {
    tokenize(
        r##"{"description":";y","initialState":"CdataSection","input":";y","inputUtf16":[59,121],"output":[{"Character":{"data":";y"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_186() {
    tokenize(
        r##"{"description":";z","initialState":"Data","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_187() {
    tokenize(
        r##"{"description":";z","initialState":"Plaintext","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_188() {
    tokenize(
        r##"{"description":";z","initialState":"Rcdata","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_189() {
    tokenize(
        r##"{"description":";z","initialState":"Rawtext","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_190() {
    tokenize(
        r##"{"description":";z","initialState":"ScriptData","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[]}"##,
    );
}

#[test]
fn test_191() {
    tokenize(
        r##"{"description":";z","initialState":"CdataSection","input":";z","inputUtf16":[59,122],"output":[{"Character":{"data":";z"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_192() {
    tokenize(
        r##"{"description":";{","initialState":"Data","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_193() {
    tokenize(
        r##"{"description":";{","initialState":"Plaintext","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_194() {
    tokenize(
        r##"{"description":";{","initialState":"Rcdata","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_195() {
    tokenize(
        r##"{"description":";{","initialState":"Rawtext","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_196() {
    tokenize(
        r##"{"description":";{","initialState":"ScriptData","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[]}"##,
    );
}

#[test]
fn test_197() {
    tokenize(
        r##"{"description":";{","initialState":"CdataSection","input":";{","inputUtf16":[59,123],"output":[{"Character":{"data":";{"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_198() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Data","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_199() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Plaintext","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_200() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Rcdata","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_201() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"Rawtext","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_202() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"ScriptData","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_203() {
    tokenize(
        r##"{"description":";\\uDBC0\\uDC00","initialState":"CdataSection","input":";􀀀","inputUtf16":[59,56256,56320],"output":[{"Character":{"data":";􀀀"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_204() {
    tokenize(
        r##"{"description":"<","initialState":"Data","input":"<","inputUtf16":[60],"output":[{"Character":{"data":"<"}}],"errors":[{"code":"EofBeforeTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_205() {
    tokenize(
        r##"{"description":"<\\u0000","initialState":"Data","input":"<\u0000","inputUtf16":[60,0],"output":[{"Character":{"data":"<\u0000"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_206() {
    tokenize(
        r##"{"description":"<\\u0009","initialState":"Data","input":"<\t","inputUtf16":[60,9],"output":[{"Character":{"data":"<\t"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_207() {
    tokenize(
        r##"{"description":"<\\u000A","initialState":"Data","input":"<\n","inputUtf16":[60,10],"output":[{"Character":{"data":"<\n"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_208() {
    tokenize(
        r##"{"description":"<\\u000B","initialState":"Data","input":"<\u000b","inputUtf16":[60,11],"output":[{"Character":{"data":"<\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":2}},{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_209() {
    tokenize(
        r##"{"description":"<\\u000C","initialState":"Data","input":"<\f","inputUtf16":[60,12],"output":[{"Character":{"data":"<\f"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_210() {
    tokenize(
        r##"{"description":"< ","initialState":"Data","input":"< ","inputUtf16":[60,32],"output":[{"Character":{"data":"< "}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_211() {
    tokenize(
        r##"{"description":"<!","initialState":"Data","input":"<!","inputUtf16":[60,33],"output":[{"Comment":{"data":""}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_212() {
    tokenize(
        r##"{"description":"<!\\u0000","initialState":"Data","input":"<!\u0000","inputUtf16":[60,33,0],"output":[{"Comment":{"data":"�"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_213() {
    tokenize(
        r##"{"description":"<!\\u0009","initialState":"Data","input":"<!\t","inputUtf16":[60,33,9],"output":[{"Comment":{"data":"\t"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_214() {
    tokenize(
        r##"{"description":"<!\\u000A","initialState":"Data","input":"<!\n","inputUtf16":[60,33,10],"output":[{"Comment":{"data":"\n"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_215() {
    tokenize(
        r##"{"description":"<!\\u000B","initialState":"Data","input":"<!\u000b","inputUtf16":[60,33,11],"output":[{"Comment":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":3}},{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_216() {
    tokenize(
        r##"{"description":"<!\\u000C","initialState":"Data","input":"<!\f","inputUtf16":[60,33,12],"output":[{"Comment":{"data":"\f"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_217() {
    tokenize(
        r##"{"description":"<! ","initialState":"Data","input":"<! ","inputUtf16":[60,33,32],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_218() {
    tokenize(
        r##"{"description":"<! \\u0000","initialState":"Data","input":"<! \u0000","inputUtf16":[60,33,32,0],"output":[{"Comment":{"data":" �"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_219() {
    tokenize(
        r##"{"description":"<!!","initialState":"Data","input":"<!!","inputUtf16":[60,33,33],"output":[{"Comment":{"data":"!"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_220() {
    tokenize(
        r##"{"description":"<!\"","initialState":"Data","input":"<!\"","inputUtf16":[60,33,34],"output":[{"Comment":{"data":"\""}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_221() {
    tokenize(
        r##"{"description":"<!&","initialState":"Data","input":"<!&","inputUtf16":[60,33,38],"output":[{"Comment":{"data":"&"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_222() {
    tokenize(
        r##"{"description":"<!'","initialState":"Data","input":"<!'","inputUtf16":[60,33,39],"output":[{"Comment":{"data":"'"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_223() {
    tokenize(
        r##"{"description":"<!-","initialState":"Data","input":"<!-","inputUtf16":[60,33,45],"output":[{"Comment":{"data":"-"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_224() {
    tokenize(
        r##"{"description":"<!--","initialState":"Data","input":"<!--","inputUtf16":[60,33,45,45],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_225() {
    tokenize(
        r##"{"description":"<!--\\u0000","initialState":"Data","input":"<!--\u0000","inputUtf16":[60,33,45,45,0],"output":[{"Comment":{"data":"�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":5}},{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_226() {
    tokenize(
        r##"{"description":"<!--\\u0009","initialState":"Data","input":"<!--\t","inputUtf16":[60,33,45,45,9],"output":[{"Comment":{"data":"\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_227() {
    tokenize(
        r##"{"description":"<!--\\u000A","initialState":"Data","input":"<!--\n","inputUtf16":[60,33,45,45,10],"output":[{"Comment":{"data":"\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_228() {
    tokenize(
        r##"{"description":"<!--\\u000B","initialState":"Data","input":"<!--\u000b","inputUtf16":[60,33,45,45,11],"output":[{"Comment":{"data":"\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":5}},{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_229() {
    tokenize(
        r##"{"description":"<!--\\u000C","initialState":"Data","input":"<!--\f","inputUtf16":[60,33,45,45,12],"output":[{"Comment":{"data":"\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_230() {
    tokenize(
        r##"{"description":"<!-- ","initialState":"Data","input":"<!-- ","inputUtf16":[60,33,45,45,32],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_231() {
    tokenize(
        r##"{"description":"<!-- \\u0000","initialState":"Data","input":"<!-- \u0000","inputUtf16":[60,33,45,45,32,0],"output":[{"Comment":{"data":" �"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_232() {
    tokenize(
        r##"{"description":"<!-- \\u0009","initialState":"Data","input":"<!-- \t","inputUtf16":[60,33,45,45,32,9],"output":[{"Comment":{"data":" \t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_233() {
    tokenize(
        r##"{"description":"<!-- \\u000A","initialState":"Data","input":"<!-- \n","inputUtf16":[60,33,45,45,32,10],"output":[{"Comment":{"data":" \n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_234() {
    tokenize(
        r##"{"description":"<!-- \\u000B","initialState":"Data","input":"<!-- \u000b","inputUtf16":[60,33,45,45,32,11],"output":[{"Comment":{"data":" \u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_235() {
    tokenize(
        r##"{"description":"<!-- \\u000C","initialState":"Data","input":"<!-- \f","inputUtf16":[60,33,45,45,32,12],"output":[{"Comment":{"data":" \f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_236() {
    tokenize(
        r##"{"description":"<!--  ","initialState":"Data","input":"<!--  ","inputUtf16":[60,33,45,45,32,32],"output":[{"Comment":{"data":"  "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_237() {
    tokenize(
        r##"{"description":"<!-- !","initialState":"Data","input":"<!-- !","inputUtf16":[60,33,45,45,32,33],"output":[{"Comment":{"data":" !"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_238() {
    tokenize(
        r##"{"description":"<!-- \"","initialState":"Data","input":"<!-- \"","inputUtf16":[60,33,45,45,32,34],"output":[{"Comment":{"data":" \""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_239() {
    tokenize(
        r##"{"description":"<!-- &","initialState":"Data","input":"<!-- &","inputUtf16":[60,33,45,45,32,38],"output":[{"Comment":{"data":" &"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_240() {
    tokenize(
        r##"{"description":"<!-- '","initialState":"Data","input":"<!-- '","inputUtf16":[60,33,45,45,32,39],"output":[{"Comment":{"data":" '"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_241() {
    tokenize(
        r##"{"description":"<!-- ,","initialState":"Data","input":"<!-- ,","inputUtf16":[60,33,45,45,32,44],"output":[{"Comment":{"data":" ,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_242() {
    tokenize(
        r##"{"description":"<!-- -","initialState":"Data","input":"<!-- -","inputUtf16":[60,33,45,45,32,45],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_243() {
    tokenize(
        r##"{"description":"<!-- -\\u0000","initialState":"Data","input":"<!-- -\u0000","inputUtf16":[60,33,45,45,32,45,0],"output":[{"Comment":{"data":" -�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_244() {
    tokenize(
        r##"{"description":"<!-- -\\u0009","initialState":"Data","input":"<!-- -\t","inputUtf16":[60,33,45,45,32,45,9],"output":[{"Comment":{"data":" -\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_245() {
    tokenize(
        r##"{"description":"<!-- -\\u000A","initialState":"Data","input":"<!-- -\n","inputUtf16":[60,33,45,45,32,45,10],"output":[{"Comment":{"data":" -\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_246() {
    tokenize(
        r##"{"description":"<!-- -\\u000B","initialState":"Data","input":"<!-- -\u000b","inputUtf16":[60,33,45,45,32,45,11],"output":[{"Comment":{"data":" -\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_247() {
    tokenize(
        r##"{"description":"<!-- -\\u000C","initialState":"Data","input":"<!-- -\f","inputUtf16":[60,33,45,45,32,45,12],"output":[{"Comment":{"data":" -\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_248() {
    tokenize(
        r##"{"description":"<!-- - ","initialState":"Data","input":"<!-- - ","inputUtf16":[60,33,45,45,32,45,32],"output":[{"Comment":{"data":" - "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_249() {
    tokenize(
        r##"{"description":"<!-- -!","initialState":"Data","input":"<!-- -!","inputUtf16":[60,33,45,45,32,45,33],"output":[{"Comment":{"data":" -!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_250() {
    tokenize(
        r##"{"description":"<!-- -\"","initialState":"Data","input":"<!-- -\"","inputUtf16":[60,33,45,45,32,45,34],"output":[{"Comment":{"data":" -\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_251() {
    tokenize(
        r##"{"description":"<!-- -&","initialState":"Data","input":"<!-- -&","inputUtf16":[60,33,45,45,32,45,38],"output":[{"Comment":{"data":" -&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_252() {
    tokenize(
        r##"{"description":"<!-- -'","initialState":"Data","input":"<!-- -'","inputUtf16":[60,33,45,45,32,45,39],"output":[{"Comment":{"data":" -'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_253() {
    tokenize(
        r##"{"description":"<!-- -,","initialState":"Data","input":"<!-- -,","inputUtf16":[60,33,45,45,32,45,44],"output":[{"Comment":{"data":" -,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_254() {
    tokenize(
        r##"{"description":"<!-- --","initialState":"Data","input":"<!-- --","inputUtf16":[60,33,45,45,32,45,45],"output":[{"Comment":{"data":" "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_255() {
    tokenize(
        r##"{"description":"<!-- -.","initialState":"Data","input":"<!-- -.","inputUtf16":[60,33,45,45,32,45,46],"output":[{"Comment":{"data":" -."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_256() {
    tokenize(
        r##"{"description":"<!-- -/","initialState":"Data","input":"<!-- -/","inputUtf16":[60,33,45,45,32,45,47],"output":[{"Comment":{"data":" -/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_257() {
    tokenize(
        r##"{"description":"<!-- -0","initialState":"Data","input":"<!-- -0","inputUtf16":[60,33,45,45,32,45,48],"output":[{"Comment":{"data":" -0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_258() {
    tokenize(
        r##"{"description":"<!-- -1","initialState":"Data","input":"<!-- -1","inputUtf16":[60,33,45,45,32,45,49],"output":[{"Comment":{"data":" -1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_259() {
    tokenize(
        r##"{"description":"<!-- -9","initialState":"Data","input":"<!-- -9","inputUtf16":[60,33,45,45,32,45,57],"output":[{"Comment":{"data":" -9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_260() {
    tokenize(
        r##"{"description":"<!-- -<","initialState":"Data","input":"<!-- -<","inputUtf16":[60,33,45,45,32,45,60],"output":[{"Comment":{"data":" -<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_261() {
    tokenize(
        r##"{"description":"<!-- -=","initialState":"Data","input":"<!-- -=","inputUtf16":[60,33,45,45,32,45,61],"output":[{"Comment":{"data":" -="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_262() {
    tokenize(
        r##"{"description":"<!-- ->","initialState":"Data","input":"<!-- ->","inputUtf16":[60,33,45,45,32,45,62],"output":[{"Comment":{"data":" ->"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_263() {
    tokenize(
        r##"{"description":"<!-- -?","initialState":"Data","input":"<!-- -?","inputUtf16":[60,33,45,45,32,45,63],"output":[{"Comment":{"data":" -?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_264() {
    tokenize(
        r##"{"description":"<!-- -@","initialState":"Data","input":"<!-- -@","inputUtf16":[60,33,45,45,32,45,64],"output":[{"Comment":{"data":" -@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_265() {
    tokenize(
        r##"{"description":"<!-- -A","initialState":"Data","input":"<!-- -A","inputUtf16":[60,33,45,45,32,45,65],"output":[{"Comment":{"data":" -A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_266() {
    tokenize(
        r##"{"description":"<!-- -B","initialState":"Data","input":"<!-- -B","inputUtf16":[60,33,45,45,32,45,66],"output":[{"Comment":{"data":" -B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_267() {
    tokenize(
        r##"{"description":"<!-- -Y","initialState":"Data","input":"<!-- -Y","inputUtf16":[60,33,45,45,32,45,89],"output":[{"Comment":{"data":" -Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_268() {
    tokenize(
        r##"{"description":"<!-- -Z","initialState":"Data","input":"<!-- -Z","inputUtf16":[60,33,45,45,32,45,90],"output":[{"Comment":{"data":" -Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_269() {
    tokenize(
        r##"{"description":"<!-- -`","initialState":"Data","input":"<!-- -`","inputUtf16":[60,33,45,45,32,45,96],"output":[{"Comment":{"data":" -`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_270() {
    tokenize(
        r##"{"description":"<!-- -a","initialState":"Data","input":"<!-- -a","inputUtf16":[60,33,45,45,32,45,97],"output":[{"Comment":{"data":" -a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_271() {
    tokenize(
        r##"{"description":"<!-- -b","initialState":"Data","input":"<!-- -b","inputUtf16":[60,33,45,45,32,45,98],"output":[{"Comment":{"data":" -b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_272() {
    tokenize(
        r##"{"description":"<!-- -y","initialState":"Data","input":"<!-- -y","inputUtf16":[60,33,45,45,32,45,121],"output":[{"Comment":{"data":" -y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_273() {
    tokenize(
        r##"{"description":"<!-- -z","initialState":"Data","input":"<!-- -z","inputUtf16":[60,33,45,45,32,45,122],"output":[{"Comment":{"data":" -z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_274() {
    tokenize(
        r##"{"description":"<!-- -{","initialState":"Data","input":"<!-- -{","inputUtf16":[60,33,45,45,32,45,123],"output":[{"Comment":{"data":" -{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_275() {
    tokenize(
        r##"{"description":"<!-- -\\uDBC0\\uDC00","initialState":"Data","input":"<!-- -􀀀","inputUtf16":[60,33,45,45,32,45,56256,56320],"output":[{"Comment":{"data":" -􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_276() {
    tokenize(
        r##"{"description":"<!-- .","initialState":"Data","input":"<!-- .","inputUtf16":[60,33,45,45,32,46],"output":[{"Comment":{"data":" ."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_277() {
    tokenize(
        r##"{"description":"<!-- /","initialState":"Data","input":"<!-- /","inputUtf16":[60,33,45,45,32,47],"output":[{"Comment":{"data":" /"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_278() {
    tokenize(
        r##"{"description":"<!-- 0","initialState":"Data","input":"<!-- 0","inputUtf16":[60,33,45,45,32,48],"output":[{"Comment":{"data":" 0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_279() {
    tokenize(
        r##"{"description":"<!-- 1","initialState":"Data","input":"<!-- 1","inputUtf16":[60,33,45,45,32,49],"output":[{"Comment":{"data":" 1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_280() {
    tokenize(
        r##"{"description":"<!-- 9","initialState":"Data","input":"<!-- 9","inputUtf16":[60,33,45,45,32,57],"output":[{"Comment":{"data":" 9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_281() {
    tokenize(
        r##"{"description":"<!-- <","initialState":"Data","input":"<!-- <","inputUtf16":[60,33,45,45,32,60],"output":[{"Comment":{"data":" <"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_282() {
    tokenize(
        r##"{"description":"<!-- =","initialState":"Data","input":"<!-- =","inputUtf16":[60,33,45,45,32,61],"output":[{"Comment":{"data":" ="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_283() {
    tokenize(
        r##"{"description":"<!-- >","initialState":"Data","input":"<!-- >","inputUtf16":[60,33,45,45,32,62],"output":[{"Comment":{"data":" >"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_284() {
    tokenize(
        r##"{"description":"<!-- ?","initialState":"Data","input":"<!-- ?","inputUtf16":[60,33,45,45,32,63],"output":[{"Comment":{"data":" ?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_285() {
    tokenize(
        r##"{"description":"<!-- @","initialState":"Data","input":"<!-- @","inputUtf16":[60,33,45,45,32,64],"output":[{"Comment":{"data":" @"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_286() {
    tokenize(
        r##"{"description":"<!-- A","initialState":"Data","input":"<!-- A","inputUtf16":[60,33,45,45,32,65],"output":[{"Comment":{"data":" A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_287() {
    tokenize(
        r##"{"description":"<!-- B","initialState":"Data","input":"<!-- B","inputUtf16":[60,33,45,45,32,66],"output":[{"Comment":{"data":" B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_288() {
    tokenize(
        r##"{"description":"<!-- Y","initialState":"Data","input":"<!-- Y","inputUtf16":[60,33,45,45,32,89],"output":[{"Comment":{"data":" Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_289() {
    tokenize(
        r##"{"description":"<!-- Z","initialState":"Data","input":"<!-- Z","inputUtf16":[60,33,45,45,32,90],"output":[{"Comment":{"data":" Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_290() {
    tokenize(
        r##"{"description":"<!-- `","initialState":"Data","input":"<!-- `","inputUtf16":[60,33,45,45,32,96],"output":[{"Comment":{"data":" `"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_291() {
    tokenize(
        r##"{"description":"<!-- a","initialState":"Data","input":"<!-- a","inputUtf16":[60,33,45,45,32,97],"output":[{"Comment":{"data":" a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_292() {
    tokenize(
        r##"{"description":"<!-- b","initialState":"Data","input":"<!-- b","inputUtf16":[60,33,45,45,32,98],"output":[{"Comment":{"data":" b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_293() {
    tokenize(
        r##"{"description":"<!-- y","initialState":"Data","input":"<!-- y","inputUtf16":[60,33,45,45,32,121],"output":[{"Comment":{"data":" y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_294() {
    tokenize(
        r##"{"description":"<!-- z","initialState":"Data","input":"<!-- z","inputUtf16":[60,33,45,45,32,122],"output":[{"Comment":{"data":" z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_295() {
    tokenize(
        r##"{"description":"<!-- {","initialState":"Data","input":"<!-- {","inputUtf16":[60,33,45,45,32,123],"output":[{"Comment":{"data":" {"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_296() {
    tokenize(
        r##"{"description":"<!-- \\uDBC0\\uDC00","initialState":"Data","input":"<!-- 􀀀","inputUtf16":[60,33,45,45,32,56256,56320],"output":[{"Comment":{"data":" 􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_297() {
    tokenize(
        r##"{"description":"<!--!","initialState":"Data","input":"<!--!","inputUtf16":[60,33,45,45,33],"output":[{"Comment":{"data":"!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_298() {
    tokenize(
        r##"{"description":"<!--\"","initialState":"Data","input":"<!--\"","inputUtf16":[60,33,45,45,34],"output":[{"Comment":{"data":"\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_299() {
    tokenize(
        r##"{"description":"<!--&","initialState":"Data","input":"<!--&","inputUtf16":[60,33,45,45,38],"output":[{"Comment":{"data":"&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_300() {
    tokenize(
        r##"{"description":"<!--'","initialState":"Data","input":"<!--'","inputUtf16":[60,33,45,45,39],"output":[{"Comment":{"data":"'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_301() {
    tokenize(
        r##"{"description":"<!--,","initialState":"Data","input":"<!--,","inputUtf16":[60,33,45,45,44],"output":[{"Comment":{"data":","}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_302() {
    tokenize(
        r##"{"description":"<!---","initialState":"Data","input":"<!---","inputUtf16":[60,33,45,45,45],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_303() {
    tokenize(
        r##"{"description":"<!---\\u0000","initialState":"Data","input":"<!---\u0000","inputUtf16":[60,33,45,45,45,0],"output":[{"Comment":{"data":"-�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_304() {
    tokenize(
        r##"{"description":"<!---\\u0009","initialState":"Data","input":"<!---\t","inputUtf16":[60,33,45,45,45,9],"output":[{"Comment":{"data":"-\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_305() {
    tokenize(
        r##"{"description":"<!---\\u000A","initialState":"Data","input":"<!---\n","inputUtf16":[60,33,45,45,45,10],"output":[{"Comment":{"data":"-\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_306() {
    tokenize(
        r##"{"description":"<!---\\u000B","initialState":"Data","input":"<!---\u000b","inputUtf16":[60,33,45,45,45,11],"output":[{"Comment":{"data":"-\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}},{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_307() {
    tokenize(
        r##"{"description":"<!---\\u000C","initialState":"Data","input":"<!---\f","inputUtf16":[60,33,45,45,45,12],"output":[{"Comment":{"data":"-\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_308() {
    tokenize(
        r##"{"description":"<!--- ","initialState":"Data","input":"<!--- ","inputUtf16":[60,33,45,45,45,32],"output":[{"Comment":{"data":"- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_309() {
    tokenize(
        r##"{"description":"<!---!","initialState":"Data","input":"<!---!","inputUtf16":[60,33,45,45,45,33],"output":[{"Comment":{"data":"-!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_310() {
    tokenize(
        r##"{"description":"<!---\"","initialState":"Data","input":"<!---\"","inputUtf16":[60,33,45,45,45,34],"output":[{"Comment":{"data":"-\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_311() {
    tokenize(
        r##"{"description":"<!---&","initialState":"Data","input":"<!---&","inputUtf16":[60,33,45,45,45,38],"output":[{"Comment":{"data":"-&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_312() {
    tokenize(
        r##"{"description":"<!---'","initialState":"Data","input":"<!---'","inputUtf16":[60,33,45,45,45,39],"output":[{"Comment":{"data":"-'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_313() {
    tokenize(
        r##"{"description":"<!---,","initialState":"Data","input":"<!---,","inputUtf16":[60,33,45,45,45,44],"output":[{"Comment":{"data":"-,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_314() {
    tokenize(
        r##"{"description":"<!----","initialState":"Data","input":"<!----","inputUtf16":[60,33,45,45,45,45],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_315() {
    tokenize(
        r##"{"description":"<!----\\u0000","initialState":"Data","input":"<!----\u0000","inputUtf16":[60,33,45,45,45,45,0],"output":[{"Comment":{"data":"--�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_316() {
    tokenize(
        r##"{"description":"<!----\\u0009","initialState":"Data","input":"<!----\t","inputUtf16":[60,33,45,45,45,45,9],"output":[{"Comment":{"data":"--\t"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_317() {
    tokenize(
        r##"{"description":"<!----\\u000A","initialState":"Data","input":"<!----\n","inputUtf16":[60,33,45,45,45,45,10],"output":[{"Comment":{"data":"--\n"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_318() {
    tokenize(
        r##"{"description":"<!----\\u000B","initialState":"Data","input":"<!----\u000b","inputUtf16":[60,33,45,45,45,45,11],"output":[{"Comment":{"data":"--\u000b"}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":7}},{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_319() {
    tokenize(
        r##"{"description":"<!----\\u000C","initialState":"Data","input":"<!----\f","inputUtf16":[60,33,45,45,45,45,12],"output":[{"Comment":{"data":"--\f"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_320() {
    tokenize(
        r##"{"description":"<!---- ","initialState":"Data","input":"<!---- ","inputUtf16":[60,33,45,45,45,45,32],"output":[{"Comment":{"data":"-- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_321() {
    tokenize(
        r##"{"description":"<!---- -","initialState":"Data","input":"<!---- -","inputUtf16":[60,33,45,45,45,45,32,45],"output":[{"Comment":{"data":"-- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_322() {
    tokenize(
        r##"{"description":"<!---- --","initialState":"Data","input":"<!---- --","inputUtf16":[60,33,45,45,45,45,32,45,45],"output":[{"Comment":{"data":"-- "}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_323() {
    tokenize(
        r##"{"description":"<!---- -->","initialState":"Data","input":"<!---- -->","inputUtf16":[60,33,45,45,45,45,32,45,45,62],"output":[{"Comment":{"data":"-- "}}],"errors":[]}"##,
    );
}

#[test]
fn test_324() {
    tokenize(
        r##"{"description":"<!----  -->","initialState":"Data","input":"<!----  -->","inputUtf16":[60,33,45,45,45,45,32,32,45,45,62],"output":[{"Comment":{"data":"--  "}}],"errors":[]}"##,
    );
}

#[test]
fn test_325() {
    tokenize(
        r##"{"description":"<!---- a-->","initialState":"Data","input":"<!---- a-->","inputUtf16":[60,33,45,45,45,45,32,97,45,45,62],"output":[{"Comment":{"data":"-- a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_326() {
    tokenize(
        r##"{"description":"<!----!","initialState":"Data","input":"<!----!","inputUtf16":[60,33,45,45,45,45,33],"output":[{"Comment":{"data":""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_327() {
    tokenize(
        r##"{"description":"<!----!>","initialState":"Data","input":"<!----!>","inputUtf16":[60,33,45,45,45,45,33,62],"output":[{"Comment":{"data":""}}],"errors":[{"code":"IncorrectlyClosedComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_328() {
    tokenize(
        r##"{"description":"<!----! >","initialState":"Data","input":"<!----! >","inputUtf16":[60,33,45,45,45,45,33,32,62],"output":[{"Comment":{"data":"--! >"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_329() {
    tokenize(
        r##"{"description":"<!----!LF>","initialState":"Data","input":"<!----!\n>","inputUtf16":[60,33,45,45,45,45,33,10,62],"output":[{"Comment":{"data":"--!\n>"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":2}}]}"##,
    );
}

#[test]
fn test_330() {
    tokenize(
        r##"{"description":"<!----!CR>","initialState":"Data","input":"<!----!\r>","inputUtf16":[60,33,45,45,45,45,33,13,62],"output":[{"Comment":{"data":"--!\n>"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":2}}]}"##,
    );
}

#[test]
fn test_331() {
    tokenize(
        r##"{"description":"<!----!CRLF>","initialState":"Data","input":"<!----!\r\n>","inputUtf16":[60,33,45,45,45,45,33,13,10,62],"output":[{"Comment":{"data":"--!\n>"}}],"errors":[{"code":"EofInComment","location":{"line":2,"column":2}}]}"##,
    );
}

#[test]
fn test_332() {
    tokenize(
        r##"{"description":"<!----!a","initialState":"Data","input":"<!----!a","inputUtf16":[60,33,45,45,45,45,33,97],"output":[{"Comment":{"data":"--!a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_333() {
    tokenize(
        r##"{"description":"<!----!a-","initialState":"Data","input":"<!----!a-","inputUtf16":[60,33,45,45,45,45,33,97,45],"output":[{"Comment":{"data":"--!a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_334() {
    tokenize(
        r##"{"description":"<!----!a--","initialState":"Data","input":"<!----!a--","inputUtf16":[60,33,45,45,45,45,33,97,45,45],"output":[{"Comment":{"data":"--!a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_335() {
    tokenize(
        r##"{"description":"<!----!a-->","initialState":"Data","input":"<!----!a-->","inputUtf16":[60,33,45,45,45,45,33,97,45,45,62],"output":[{"Comment":{"data":"--!a"}}],"errors":[]}"##,
    );
}

#[test]
fn test_336() {
    tokenize(
        r##"{"description":"<!----!-","initialState":"Data","input":"<!----!-","inputUtf16":[60,33,45,45,45,45,33,45],"output":[{"Comment":{"data":"--!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_337() {
    tokenize(
        r##"{"description":"<!----!--","initialState":"Data","input":"<!----!--","inputUtf16":[60,33,45,45,45,45,33,45,45],"output":[{"Comment":{"data":"--!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_338() {
    tokenize(
        r##"{"description":"<!----!-->","initialState":"Data","input":"<!----!-->","inputUtf16":[60,33,45,45,45,45,33,45,45,62],"output":[{"Comment":{"data":"--!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_339() {
    tokenize(
        r##"{"description":"<!----\"","initialState":"Data","input":"<!----\"","inputUtf16":[60,33,45,45,45,45,34],"output":[{"Comment":{"data":"--\""}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_340() {
    tokenize(
        r##"{"description":"<!----&","initialState":"Data","input":"<!----&","inputUtf16":[60,33,45,45,45,45,38],"output":[{"Comment":{"data":"--&"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_341() {
    tokenize(
        r##"{"description":"<!----'","initialState":"Data","input":"<!----'","inputUtf16":[60,33,45,45,45,45,39],"output":[{"Comment":{"data":"--'"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_342() {
    tokenize(
        r##"{"description":"<!----,","initialState":"Data","input":"<!----,","inputUtf16":[60,33,45,45,45,45,44],"output":[{"Comment":{"data":"--,"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_343() {
    tokenize(
        r##"{"description":"<!-----","initialState":"Data","input":"<!-----","inputUtf16":[60,33,45,45,45,45,45],"output":[{"Comment":{"data":"-"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_344() {
    tokenize(
        r##"{"description":"<!----.","initialState":"Data","input":"<!----.","inputUtf16":[60,33,45,45,45,45,46],"output":[{"Comment":{"data":"--."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_345() {
    tokenize(
        r##"{"description":"<!----/","initialState":"Data","input":"<!----/","inputUtf16":[60,33,45,45,45,45,47],"output":[{"Comment":{"data":"--/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_346() {
    tokenize(
        r##"{"description":"<!----0","initialState":"Data","input":"<!----0","inputUtf16":[60,33,45,45,45,45,48],"output":[{"Comment":{"data":"--0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_347() {
    tokenize(
        r##"{"description":"<!----1","initialState":"Data","input":"<!----1","inputUtf16":[60,33,45,45,45,45,49],"output":[{"Comment":{"data":"--1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_348() {
    tokenize(
        r##"{"description":"<!----9","initialState":"Data","input":"<!----9","inputUtf16":[60,33,45,45,45,45,57],"output":[{"Comment":{"data":"--9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_349() {
    tokenize(
        r##"{"description":"<!----<","initialState":"Data","input":"<!----<","inputUtf16":[60,33,45,45,45,45,60],"output":[{"Comment":{"data":"--<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_350() {
    tokenize(
        r##"{"description":"<!----=","initialState":"Data","input":"<!----=","inputUtf16":[60,33,45,45,45,45,61],"output":[{"Comment":{"data":"--="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_351() {
    tokenize(
        r##"{"description":"<!---->","initialState":"Data","input":"<!---->","inputUtf16":[60,33,45,45,45,45,62],"output":[{"Comment":{"data":""}}],"errors":[]}"##,
    );
}

#[test]
fn test_352() {
    tokenize(
        r##"{"description":"<!----?","initialState":"Data","input":"<!----?","inputUtf16":[60,33,45,45,45,45,63],"output":[{"Comment":{"data":"--?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_353() {
    tokenize(
        r##"{"description":"<!----@","initialState":"Data","input":"<!----@","inputUtf16":[60,33,45,45,45,45,64],"output":[{"Comment":{"data":"--@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_354() {
    tokenize(
        r##"{"description":"<!----A","initialState":"Data","input":"<!----A","inputUtf16":[60,33,45,45,45,45,65],"output":[{"Comment":{"data":"--A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_355() {
    tokenize(
        r##"{"description":"<!----B","initialState":"Data","input":"<!----B","inputUtf16":[60,33,45,45,45,45,66],"output":[{"Comment":{"data":"--B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_356() {
    tokenize(
        r##"{"description":"<!----Y","initialState":"Data","input":"<!----Y","inputUtf16":[60,33,45,45,45,45,89],"output":[{"Comment":{"data":"--Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_357() {
    tokenize(
        r##"{"description":"<!----Z","initialState":"Data","input":"<!----Z","inputUtf16":[60,33,45,45,45,45,90],"output":[{"Comment":{"data":"--Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_358() {
    tokenize(
        r##"{"description":"<!----`","initialState":"Data","input":"<!----`","inputUtf16":[60,33,45,45,45,45,96],"output":[{"Comment":{"data":"--`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_359() {
    tokenize(
        r##"{"description":"<!----a","initialState":"Data","input":"<!----a","inputUtf16":[60,33,45,45,45,45,97],"output":[{"Comment":{"data":"--a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_360() {
    tokenize(
        r##"{"description":"<!----b","initialState":"Data","input":"<!----b","inputUtf16":[60,33,45,45,45,45,98],"output":[{"Comment":{"data":"--b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_361() {
    tokenize(
        r##"{"description":"<!----y","initialState":"Data","input":"<!----y","inputUtf16":[60,33,45,45,45,45,121],"output":[{"Comment":{"data":"--y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_362() {
    tokenize(
        r##"{"description":"<!----z","initialState":"Data","input":"<!----z","inputUtf16":[60,33,45,45,45,45,122],"output":[{"Comment":{"data":"--z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_363() {
    tokenize(
        r##"{"description":"<!----{","initialState":"Data","input":"<!----{","inputUtf16":[60,33,45,45,45,45,123],"output":[{"Comment":{"data":"--{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_364() {
    tokenize(
        r##"{"description":"<!----\\uDBC0\\uDC00","initialState":"Data","input":"<!----􀀀","inputUtf16":[60,33,45,45,45,45,56256,56320],"output":[{"Comment":{"data":"--􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_365() {
    tokenize(
        r##"{"description":"<!---.","initialState":"Data","input":"<!---.","inputUtf16":[60,33,45,45,45,46],"output":[{"Comment":{"data":"-."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_366() {
    tokenize(
        r##"{"description":"<!---/","initialState":"Data","input":"<!---/","inputUtf16":[60,33,45,45,45,47],"output":[{"Comment":{"data":"-/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_367() {
    tokenize(
        r##"{"description":"<!---0","initialState":"Data","input":"<!---0","inputUtf16":[60,33,45,45,45,48],"output":[{"Comment":{"data":"-0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_368() {
    tokenize(
        r##"{"description":"<!---1","initialState":"Data","input":"<!---1","inputUtf16":[60,33,45,45,45,49],"output":[{"Comment":{"data":"-1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_369() {
    tokenize(
        r##"{"description":"<!---9","initialState":"Data","input":"<!---9","inputUtf16":[60,33,45,45,45,57],"output":[{"Comment":{"data":"-9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_370() {
    tokenize(
        r##"{"description":"<!---<","initialState":"Data","input":"<!---<","inputUtf16":[60,33,45,45,45,60],"output":[{"Comment":{"data":"-<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_371() {
    tokenize(
        r##"{"description":"<!---=","initialState":"Data","input":"<!---=","inputUtf16":[60,33,45,45,45,61],"output":[{"Comment":{"data":"-="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_372() {
    tokenize(
        r##"{"description":"<!---?","initialState":"Data","input":"<!---?","inputUtf16":[60,33,45,45,45,63],"output":[{"Comment":{"data":"-?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_373() {
    tokenize(
        r##"{"description":"<!---@","initialState":"Data","input":"<!---@","inputUtf16":[60,33,45,45,45,64],"output":[{"Comment":{"data":"-@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_374() {
    tokenize(
        r##"{"description":"<!---A","initialState":"Data","input":"<!---A","inputUtf16":[60,33,45,45,45,65],"output":[{"Comment":{"data":"-A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_375() {
    tokenize(
        r##"{"description":"<!---B","initialState":"Data","input":"<!---B","inputUtf16":[60,33,45,45,45,66],"output":[{"Comment":{"data":"-B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_376() {
    tokenize(
        r##"{"description":"<!---Y","initialState":"Data","input":"<!---Y","inputUtf16":[60,33,45,45,45,89],"output":[{"Comment":{"data":"-Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_377() {
    tokenize(
        r##"{"description":"<!---Z","initialState":"Data","input":"<!---Z","inputUtf16":[60,33,45,45,45,90],"output":[{"Comment":{"data":"-Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_378() {
    tokenize(
        r##"{"description":"<!---`","initialState":"Data","input":"<!---`","inputUtf16":[60,33,45,45,45,96],"output":[{"Comment":{"data":"-`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_379() {
    tokenize(
        r##"{"description":"<!---a","initialState":"Data","input":"<!---a","inputUtf16":[60,33,45,45,45,97],"output":[{"Comment":{"data":"-a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_380() {
    tokenize(
        r##"{"description":"<!---b","initialState":"Data","input":"<!---b","inputUtf16":[60,33,45,45,45,98],"output":[{"Comment":{"data":"-b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_381() {
    tokenize(
        r##"{"description":"<!---y","initialState":"Data","input":"<!---y","inputUtf16":[60,33,45,45,45,121],"output":[{"Comment":{"data":"-y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_382() {
    tokenize(
        r##"{"description":"<!---z","initialState":"Data","input":"<!---z","inputUtf16":[60,33,45,45,45,122],"output":[{"Comment":{"data":"-z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_383() {
    tokenize(
        r##"{"description":"<!---{","initialState":"Data","input":"<!---{","inputUtf16":[60,33,45,45,45,123],"output":[{"Comment":{"data":"-{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_384() {
    tokenize(
        r##"{"description":"<!---\\uDBC0\\uDC00","initialState":"Data","input":"<!---􀀀","inputUtf16":[60,33,45,45,45,56256,56320],"output":[{"Comment":{"data":"-􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_385() {
    tokenize(
        r##"{"description":"<!--.","initialState":"Data","input":"<!--.","inputUtf16":[60,33,45,45,46],"output":[{"Comment":{"data":"."}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_386() {
    tokenize(
        r##"{"description":"<!--/","initialState":"Data","input":"<!--/","inputUtf16":[60,33,45,45,47],"output":[{"Comment":{"data":"/"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_387() {
    tokenize(
        r##"{"description":"<!--0","initialState":"Data","input":"<!--0","inputUtf16":[60,33,45,45,48],"output":[{"Comment":{"data":"0"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_388() {
    tokenize(
        r##"{"description":"<!--1","initialState":"Data","input":"<!--1","inputUtf16":[60,33,45,45,49],"output":[{"Comment":{"data":"1"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_389() {
    tokenize(
        r##"{"description":"<!--9","initialState":"Data","input":"<!--9","inputUtf16":[60,33,45,45,57],"output":[{"Comment":{"data":"9"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_390() {
    tokenize(
        r##"{"description":"<!--<","initialState":"Data","input":"<!--<","inputUtf16":[60,33,45,45,60],"output":[{"Comment":{"data":"<"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_391() {
    tokenize(
        r##"{"description":"<!--=","initialState":"Data","input":"<!--=","inputUtf16":[60,33,45,45,61],"output":[{"Comment":{"data":"="}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_392() {
    tokenize(
        r##"{"description":"<!--?","initialState":"Data","input":"<!--?","inputUtf16":[60,33,45,45,63],"output":[{"Comment":{"data":"?"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_393() {
    tokenize(
        r##"{"description":"<!--@","initialState":"Data","input":"<!--@","inputUtf16":[60,33,45,45,64],"output":[{"Comment":{"data":"@"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_394() {
    tokenize(
        r##"{"description":"<!--A","initialState":"Data","input":"<!--A","inputUtf16":[60,33,45,45,65],"output":[{"Comment":{"data":"A"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_395() {
    tokenize(
        r##"{"description":"<!--B","initialState":"Data","input":"<!--B","inputUtf16":[60,33,45,45,66],"output":[{"Comment":{"data":"B"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_396() {
    tokenize(
        r##"{"description":"<!--Y","initialState":"Data","input":"<!--Y","inputUtf16":[60,33,45,45,89],"output":[{"Comment":{"data":"Y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_397() {
    tokenize(
        r##"{"description":"<!--Z","initialState":"Data","input":"<!--Z","inputUtf16":[60,33,45,45,90],"output":[{"Comment":{"data":"Z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_398() {
    tokenize(
        r##"{"description":"<!--`","initialState":"Data","input":"<!--`","inputUtf16":[60,33,45,45,96],"output":[{"Comment":{"data":"`"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_399() {
    tokenize(
        r##"{"description":"<!--a","initialState":"Data","input":"<!--a","inputUtf16":[60,33,45,45,97],"output":[{"Comment":{"data":"a"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_400() {
    tokenize(
        r##"{"description":"<!--b","initialState":"Data","input":"<!--b","inputUtf16":[60,33,45,45,98],"output":[{"Comment":{"data":"b"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_401() {
    tokenize(
        r##"{"description":"<!--y","initialState":"Data","input":"<!--y","inputUtf16":[60,33,45,45,121],"output":[{"Comment":{"data":"y"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_402() {
    tokenize(
        r##"{"description":"<!--z","initialState":"Data","input":"<!--z","inputUtf16":[60,33,45,45,122],"output":[{"Comment":{"data":"z"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_403() {
    tokenize(
        r##"{"description":"<!--{","initialState":"Data","input":"<!--{","inputUtf16":[60,33,45,45,123],"output":[{"Comment":{"data":"{"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_404() {
    tokenize(
        r##"{"description":"<!--\\uDBC0\\uDC00","initialState":"Data","input":"<!--􀀀","inputUtf16":[60,33,45,45,56256,56320],"output":[{"Comment":{"data":"􀀀"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_405() {
    tokenize(
        r##"{"description":"<!/","initialState":"Data","input":"<!/","inputUtf16":[60,33,47],"output":[{"Comment":{"data":"/"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_406() {
    tokenize(
        r##"{"description":"<!0","initialState":"Data","input":"<!0","inputUtf16":[60,33,48],"output":[{"Comment":{"data":"0"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_407() {
    tokenize(
        r##"{"description":"<!1","initialState":"Data","input":"<!1","inputUtf16":[60,33,49],"output":[{"Comment":{"data":"1"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_408() {
    tokenize(
        r##"{"description":"<!9","initialState":"Data","input":"<!9","inputUtf16":[60,33,57],"output":[{"Comment":{"data":"9"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_409() {
    tokenize(
        r##"{"description":"<!<","initialState":"Data","input":"<!<","inputUtf16":[60,33,60],"output":[{"Comment":{"data":"<"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_410() {
    tokenize(
        r##"{"description":"<!=","initialState":"Data","input":"<!=","inputUtf16":[60,33,61],"output":[{"Comment":{"data":"="}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_411() {
    tokenize(
        r##"{"description":"<!>","initialState":"Data","input":"<!>","inputUtf16":[60,33,62],"output":[{"Comment":{"data":""}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_412() {
    tokenize(
        r##"{"description":"<!?","initialState":"Data","input":"<!?","inputUtf16":[60,33,63],"output":[{"Comment":{"data":"?"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_413() {
    tokenize(
        r##"{"description":"<!@","initialState":"Data","input":"<!@","inputUtf16":[60,33,64],"output":[{"Comment":{"data":"@"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_414() {
    tokenize(
        r##"{"description":"<!A","initialState":"Data","input":"<!A","inputUtf16":[60,33,65],"output":[{"Comment":{"data":"A"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_415() {
    tokenize(
        r##"{"description":"<!B","initialState":"Data","input":"<!B","inputUtf16":[60,33,66],"output":[{"Comment":{"data":"B"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_416() {
    tokenize(
        r##"{"description":"<!DOCTYPE","initialState":"Data","input":"<!DOCTYPE","inputUtf16":[60,33,68,79,67,84,89,80,69],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_417() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u0000","initialState":"Data","input":"<!DOCTYPE\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,0],"output":[{"Doctype":{"name":"�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_418() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u0008","initialState":"Data","input":"<!DOCTYPE\b","inputUtf16":[60,33,68,79,67,84,89,80,69,8],"output":[{"Doctype":{"name":"\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":10}},{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_419() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u0009","initialState":"Data","input":"<!DOCTYPE\t","inputUtf16":[60,33,68,79,67,84,89,80,69,9],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_420() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000A","initialState":"Data","input":"<!DOCTYPE\n","inputUtf16":[60,33,68,79,67,84,89,80,69,10],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_421() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000B","initialState":"Data","input":"<!DOCTYPE\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,11],"output":[{"Doctype":{"name":"\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":10}},{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_422() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000C","initialState":"Data","input":"<!DOCTYPE\f","inputUtf16":[60,33,68,79,67,84,89,80,69,12],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_423() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u000D","initialState":"Data","input":"<!DOCTYPE\r","inputUtf16":[60,33,68,79,67,84,89,80,69,13],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_424() {
    tokenize(
        r##"{"description":"<!DOCTYPE\\u001F","initialState":"Data","input":"<!DOCTYPE\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,31],"output":[{"Doctype":{"name":"\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":10}},{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_425() {
    tokenize(
        r##"{"description":"<!DOCTYPE ","initialState":"Data","input":"<!DOCTYPE ","inputUtf16":[60,33,68,79,67,84,89,80,69,32],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_426() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u0000","initialState":"Data","input":"<!DOCTYPE \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,0],"output":[{"Doctype":{"name":"�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_427() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u0008","initialState":"Data","input":"<!DOCTYPE \b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,8],"output":[{"Doctype":{"name":"\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_428() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u0009","initialState":"Data","input":"<!DOCTYPE \t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,9],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_429() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000A","initialState":"Data","input":"<!DOCTYPE \n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,10],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_430() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000B","initialState":"Data","input":"<!DOCTYPE \u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,11],"output":[{"Doctype":{"name":"\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_431() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000C","initialState":"Data","input":"<!DOCTYPE \f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,12],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_432() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u000D","initialState":"Data","input":"<!DOCTYPE \r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,13],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_433() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\u001F","initialState":"Data","input":"<!DOCTYPE \u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,31],"output":[{"Doctype":{"name":"\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_434() {
    tokenize(
        r##"{"description":"<!DOCTYPE  ","initialState":"Data","input":"<!DOCTYPE  ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,32],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_435() {
    tokenize(
        r##"{"description":"<!DOCTYPE !","initialState":"Data","input":"<!DOCTYPE !","inputUtf16":[60,33,68,79,67,84,89,80,69,32,33],"output":[{"Doctype":{"name":"!","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_436() {
    tokenize(
        r##"{"description":"<!DOCTYPE \"","initialState":"Data","input":"<!DOCTYPE \"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,34],"output":[{"Doctype":{"name":"\"","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_437() {
    tokenize(
        r##"{"description":"<!DOCTYPE &","initialState":"Data","input":"<!DOCTYPE &","inputUtf16":[60,33,68,79,67,84,89,80,69,32,38],"output":[{"Doctype":{"name":"&","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_438() {
    tokenize(
        r##"{"description":"<!DOCTYPE '","initialState":"Data","input":"<!DOCTYPE '","inputUtf16":[60,33,68,79,67,84,89,80,69,32,39],"output":[{"Doctype":{"name":"'","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_439() {
    tokenize(
        r##"{"description":"<!DOCTYPE -","initialState":"Data","input":"<!DOCTYPE -","inputUtf16":[60,33,68,79,67,84,89,80,69,32,45],"output":[{"Doctype":{"name":"-","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_440() {
    tokenize(
        r##"{"description":"<!DOCTYPE /","initialState":"Data","input":"<!DOCTYPE /","inputUtf16":[60,33,68,79,67,84,89,80,69,32,47],"output":[{"Doctype":{"name":"/","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_441() {
    tokenize(
        r##"{"description":"<!DOCTYPE 0","initialState":"Data","input":"<!DOCTYPE 0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,48],"output":[{"Doctype":{"name":"0","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_442() {
    tokenize(
        r##"{"description":"<!DOCTYPE 1","initialState":"Data","input":"<!DOCTYPE 1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,49],"output":[{"Doctype":{"name":"1","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_443() {
    tokenize(
        r##"{"description":"<!DOCTYPE 9","initialState":"Data","input":"<!DOCTYPE 9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,57],"output":[{"Doctype":{"name":"9","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_444() {
    tokenize(
        r##"{"description":"<!DOCTYPE <","initialState":"Data","input":"<!DOCTYPE <","inputUtf16":[60,33,68,79,67,84,89,80,69,32,60],"output":[{"Doctype":{"name":"<","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_445() {
    tokenize(
        r##"{"description":"<!DOCTYPE =","initialState":"Data","input":"<!DOCTYPE =","inputUtf16":[60,33,68,79,67,84,89,80,69,32,61],"output":[{"Doctype":{"name":"=","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_446() {
    tokenize(
        r##"{"description":"<!DOCTYPE >","initialState":"Data","input":"<!DOCTYPE >","inputUtf16":[60,33,68,79,67,84,89,80,69,32,62],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypeName","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_447() {
    tokenize(
        r##"{"description":"<!DOCTYPE ?","initialState":"Data","input":"<!DOCTYPE ?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,63],"output":[{"Doctype":{"name":"?","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_448() {
    tokenize(
        r##"{"description":"<!DOCTYPE @","initialState":"Data","input":"<!DOCTYPE @","inputUtf16":[60,33,68,79,67,84,89,80,69,32,64],"output":[{"Doctype":{"name":"@","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_449() {
    tokenize(
        r##"{"description":"<!DOCTYPE A","initialState":"Data","input":"<!DOCTYPE A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_450() {
    tokenize(
        r##"{"description":"<!DOCTYPE B","initialState":"Data","input":"<!DOCTYPE B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,66],"output":[{"Doctype":{"name":"b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_451() {
    tokenize(
        r##"{"description":"<!DOCTYPE Y","initialState":"Data","input":"<!DOCTYPE Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,89],"output":[{"Doctype":{"name":"y","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_452() {
    tokenize(
        r##"{"description":"<!DOCTYPE Z","initialState":"Data","input":"<!DOCTYPE Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,90],"output":[{"Doctype":{"name":"z","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_453() {
    tokenize(
        r##"{"description":"<!DOCTYPE [","initialState":"Data","input":"<!DOCTYPE [","inputUtf16":[60,33,68,79,67,84,89,80,69,32,91],"output":[{"Doctype":{"name":"[","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_454() {
    tokenize(
        r##"{"description":"<!DOCTYPE `","initialState":"Data","input":"<!DOCTYPE `","inputUtf16":[60,33,68,79,67,84,89,80,69,32,96],"output":[{"Doctype":{"name":"`","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_455() {
    tokenize(
        r##"{"description":"<!DOCTYPE a","initialState":"Data","input":"<!DOCTYPE a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_456() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u0000","initialState":"Data","input":"<!DOCTYPE a\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,0],"output":[{"Doctype":{"name":"a�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_457() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u0008","initialState":"Data","input":"<!DOCTYPE a\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,8],"output":[{"Doctype":{"name":"a\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_458() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u0009","initialState":"Data","input":"<!DOCTYPE a\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_459() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000A","initialState":"Data","input":"<!DOCTYPE a\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_460() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000B","initialState":"Data","input":"<!DOCTYPE a\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,11],"output":[{"Doctype":{"name":"a\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_461() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000C","initialState":"Data","input":"<!DOCTYPE a\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_462() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u000D","initialState":"Data","input":"<!DOCTYPE a\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_463() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\u001F","initialState":"Data","input":"<!DOCTYPE a\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,31],"output":[{"Doctype":{"name":"a\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_464() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ","initialState":"Data","input":"<!DOCTYPE a ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_465() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u0000","initialState":"Data","input":"<!DOCTYPE a \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_466() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u0008","initialState":"Data","input":"<!DOCTYPE a \b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":13}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_467() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u0009","initialState":"Data","input":"<!DOCTYPE a \t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_468() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000A","initialState":"Data","input":"<!DOCTYPE a \n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_469() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000B","initialState":"Data","input":"<!DOCTYPE a \u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":13}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_470() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000C","initialState":"Data","input":"<!DOCTYPE a \f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_471() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u000D","initialState":"Data","input":"<!DOCTYPE a \r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_472() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\u001F","initialState":"Data","input":"<!DOCTYPE a \u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":13}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_473() {
    tokenize(
        r##"{"description":"<!DOCTYPE a  ","initialState":"Data","input":"<!DOCTYPE a  ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_474() {
    tokenize(
        r##"{"description":"<!DOCTYPE a !","initialState":"Data","input":"<!DOCTYPE a !","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_475() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \"","initialState":"Data","input":"<!DOCTYPE a \"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_476() {
    tokenize(
        r##"{"description":"<!DOCTYPE a &","initialState":"Data","input":"<!DOCTYPE a &","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_477() {
    tokenize(
        r##"{"description":"<!DOCTYPE a '","initialState":"Data","input":"<!DOCTYPE a '","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_478() {
    tokenize(
        r##"{"description":"<!DOCTYPE a -","initialState":"Data","input":"<!DOCTYPE a -","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_479() {
    tokenize(
        r##"{"description":"<!DOCTYPE a /","initialState":"Data","input":"<!DOCTYPE a /","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_480() {
    tokenize(
        r##"{"description":"<!DOCTYPE a 0","initialState":"Data","input":"<!DOCTYPE a 0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_481() {
    tokenize(
        r##"{"description":"<!DOCTYPE a 1","initialState":"Data","input":"<!DOCTYPE a 1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_482() {
    tokenize(
        r##"{"description":"<!DOCTYPE a 9","initialState":"Data","input":"<!DOCTYPE a 9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_483() {
    tokenize(
        r##"{"description":"<!DOCTYPE a <","initialState":"Data","input":"<!DOCTYPE a <","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_484() {
    tokenize(
        r##"{"description":"<!DOCTYPE a =","initialState":"Data","input":"<!DOCTYPE a =","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_485() {
    tokenize(
        r##"{"description":"<!DOCTYPE a >","initialState":"Data","input":"<!DOCTYPE a >","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_486() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ?","initialState":"Data","input":"<!DOCTYPE a ?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_487() {
    tokenize(
        r##"{"description":"<!DOCTYPE a @","initialState":"Data","input":"<!DOCTYPE a @","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_488() {
    tokenize(
        r##"{"description":"<!DOCTYPE a A","initialState":"Data","input":"<!DOCTYPE a A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_489() {
    tokenize(
        r##"{"description":"<!DOCTYPE a B","initialState":"Data","input":"<!DOCTYPE a B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_490() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC","initialState":"Data","input":"<!DOCTYPE a PUBLIC","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_491() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_492() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u0008","initialState":"Data","input":"<!DOCTYPE a PUBLIC\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_493() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_494() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_495() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_496() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_497() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u000D","initialState":"Data","input":"<!DOCTYPE a PUBLIC\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_498() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\u001F","initialState":"Data","input":"<!DOCTYPE a PUBLIC\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_499() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC ","initialState":"Data","input":"<!DOCTYPE a PUBLIC ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_500() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC!","initialState":"Data","input":"<!DOCTYPE a PUBLIC!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_501() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_502() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_503() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_504() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_505() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_506() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_507() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\" ","initialState":"Data","input":"<!DOCTYPE a PUBLIC\" ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_508() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"!","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_509() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_510() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\"\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,34,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_511() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\" \\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"\" \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,34,32,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":22}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_512() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"#","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,35],"output":[{"Doctype":{"name":"a","public_id":"#","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_513() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"&","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_514() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"'","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,39],"output":[{"Doctype":{"name":"a","public_id":"'","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_515() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"-","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_516() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"/","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_517() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"0","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_518() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"1","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_519() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"9","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_520() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"<","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_521() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"=","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_522() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\">","initialState":"Data","input":"<!DOCTYPE a PUBLIC\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_523() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"?","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,63],"output":[{"Doctype":{"name":"a","public_id":"?","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_524() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"@","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,64],"output":[{"Doctype":{"name":"a","public_id":"@","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_525() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"A","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,65],"output":[{"Doctype":{"name":"a","public_id":"A","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_526() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"B","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,66],"output":[{"Doctype":{"name":"a","public_id":"B","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_527() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"Y","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,89],"output":[{"Doctype":{"name":"a","public_id":"Y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_528() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"Z","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,90],"output":[{"Doctype":{"name":"a","public_id":"Z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_529() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"`","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,96],"output":[{"Doctype":{"name":"a","public_id":"`","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_530() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"a","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,97],"output":[{"Doctype":{"name":"a","public_id":"a","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_531() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"b","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,98],"output":[{"Doctype":{"name":"a","public_id":"b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_532() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"y","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,121],"output":[{"Doctype":{"name":"a","public_id":"y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_533() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"z","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,122],"output":[{"Doctype":{"name":"a","public_id":"z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_534() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"{","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,123],"output":[{"Doctype":{"name":"a","public_id":"{","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_535() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\"\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC\"􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,34,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"􀀀","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_536() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC#","initialState":"Data","input":"<!DOCTYPE a PUBLIC#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_537() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC&","initialState":"Data","input":"<!DOCTYPE a PUBLIC&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_538() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'","initialState":"Data","input":"<!DOCTYPE a PUBLIC'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_539() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_540() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_541() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_542() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_543() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_544() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC' ","initialState":"Data","input":"<!DOCTYPE a PUBLIC' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_545() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'!","initialState":"Data","input":"<!DOCTYPE a PUBLIC'!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_546() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC'\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,34],"output":[{"Doctype":{"name":"a","public_id":"\"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_547() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'&","initialState":"Data","input":"<!DOCTYPE a PUBLIC'&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_548() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''","initialState":"Data","input":"<!DOCTYPE a PUBLIC''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_549() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_550() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u0008","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,8],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_551() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u0009","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,9],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_552() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000A","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,10],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_553() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000B","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,11],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_554() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000C","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,12],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_555() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u000D","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,13],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_556() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\u001F","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,31],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_557() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'' ","initialState":"Data","input":"<!DOCTYPE a PUBLIC'' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,32],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_558() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''!","initialState":"Data","input":"<!DOCTYPE a PUBLIC''!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,33],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_559() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\"","initialState":"Data","input":"<!DOCTYPE a PUBLIC''\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_560() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''#","initialState":"Data","input":"<!DOCTYPE a PUBLIC''#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,35],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_561() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''&","initialState":"Data","input":"<!DOCTYPE a PUBLIC''&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,38],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_562() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'''","initialState":"Data","input":"<!DOCTYPE a PUBLIC'''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_563() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''''\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC''''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":23}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":23}}]}"##,
    );
}

#[test]
fn test_564() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''''x\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC''''x\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,120,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":23}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_565() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'''' \\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC'''' \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,32,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":24}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_566() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'''' x\\u0000","initialState":"Data","input":"<!DOCTYPE a PUBLIC'''' x\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,39,39,32,120,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":24}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":25}}]}"##,
    );
}

#[test]
fn test_567() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''(","initialState":"Data","input":"<!DOCTYPE a PUBLIC''(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,40],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_568() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''-","initialState":"Data","input":"<!DOCTYPE a PUBLIC''-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,45],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_569() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''/","initialState":"Data","input":"<!DOCTYPE a PUBLIC''/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,47],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_570() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''0","initialState":"Data","input":"<!DOCTYPE a PUBLIC''0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,48],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_571() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''1","initialState":"Data","input":"<!DOCTYPE a PUBLIC''1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,49],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_572() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''9","initialState":"Data","input":"<!DOCTYPE a PUBLIC''9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,57],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_573() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''<","initialState":"Data","input":"<!DOCTYPE a PUBLIC''<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,60],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_574() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''=","initialState":"Data","input":"<!DOCTYPE a PUBLIC''=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,61],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_575() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''>","initialState":"Data","input":"<!DOCTYPE a PUBLIC''>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_576() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''?","initialState":"Data","input":"<!DOCTYPE a PUBLIC''?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,63],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_577() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''@","initialState":"Data","input":"<!DOCTYPE a PUBLIC''@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,64],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_578() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''A","initialState":"Data","input":"<!DOCTYPE a PUBLIC''A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,65],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_579() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''B","initialState":"Data","input":"<!DOCTYPE a PUBLIC''B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,66],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_580() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''Y","initialState":"Data","input":"<!DOCTYPE a PUBLIC''Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,89],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_581() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''Z","initialState":"Data","input":"<!DOCTYPE a PUBLIC''Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,90],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_582() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''`","initialState":"Data","input":"<!DOCTYPE a PUBLIC''`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,96],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_583() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''a","initialState":"Data","input":"<!DOCTYPE a PUBLIC''a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,97],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_584() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''b","initialState":"Data","input":"<!DOCTYPE a PUBLIC''b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,98],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_585() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''y","initialState":"Data","input":"<!DOCTYPE a PUBLIC''y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,121],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_586() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''z","initialState":"Data","input":"<!DOCTYPE a PUBLIC''z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,122],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_587() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''{","initialState":"Data","input":"<!DOCTYPE a PUBLIC''{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,123],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_588() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC''\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC''􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_589() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'(","initialState":"Data","input":"<!DOCTYPE a PUBLIC'(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,40],"output":[{"Doctype":{"name":"a","public_id":"(","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_590() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'-","initialState":"Data","input":"<!DOCTYPE a PUBLIC'-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_591() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'/","initialState":"Data","input":"<!DOCTYPE a PUBLIC'/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_592() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'0","initialState":"Data","input":"<!DOCTYPE a PUBLIC'0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_593() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'1","initialState":"Data","input":"<!DOCTYPE a PUBLIC'1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_594() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'9","initialState":"Data","input":"<!DOCTYPE a PUBLIC'9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_595() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'<","initialState":"Data","input":"<!DOCTYPE a PUBLIC'<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_596() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'=","initialState":"Data","input":"<!DOCTYPE a PUBLIC'=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_597() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'>","initialState":"Data","input":"<!DOCTYPE a PUBLIC'>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_598() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'?","initialState":"Data","input":"<!DOCTYPE a PUBLIC'?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,63],"output":[{"Doctype":{"name":"a","public_id":"?","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_599() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'@","initialState":"Data","input":"<!DOCTYPE a PUBLIC'@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,64],"output":[{"Doctype":{"name":"a","public_id":"@","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_600() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'A","initialState":"Data","input":"<!DOCTYPE a PUBLIC'A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,65],"output":[{"Doctype":{"name":"a","public_id":"A","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_601() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'B","initialState":"Data","input":"<!DOCTYPE a PUBLIC'B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,66],"output":[{"Doctype":{"name":"a","public_id":"B","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_602() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'Y","initialState":"Data","input":"<!DOCTYPE a PUBLIC'Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,89],"output":[{"Doctype":{"name":"a","public_id":"Y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_603() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'Z","initialState":"Data","input":"<!DOCTYPE a PUBLIC'Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,90],"output":[{"Doctype":{"name":"a","public_id":"Z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_604() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'`","initialState":"Data","input":"<!DOCTYPE a PUBLIC'`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,96],"output":[{"Doctype":{"name":"a","public_id":"`","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_605() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'a","initialState":"Data","input":"<!DOCTYPE a PUBLIC'a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,97],"output":[{"Doctype":{"name":"a","public_id":"a","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_606() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'b","initialState":"Data","input":"<!DOCTYPE a PUBLIC'b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,98],"output":[{"Doctype":{"name":"a","public_id":"b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_607() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'y","initialState":"Data","input":"<!DOCTYPE a PUBLIC'y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,121],"output":[{"Doctype":{"name":"a","public_id":"y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_608() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'z","initialState":"Data","input":"<!DOCTYPE a PUBLIC'z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,122],"output":[{"Doctype":{"name":"a","public_id":"z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_609() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'{","initialState":"Data","input":"<!DOCTYPE a PUBLIC'{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,123],"output":[{"Doctype":{"name":"a","public_id":"{","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_610() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC'\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC'􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"􀀀","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_611() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC(","initialState":"Data","input":"<!DOCTYPE a PUBLIC(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_612() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC-","initialState":"Data","input":"<!DOCTYPE a PUBLIC-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_613() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC/","initialState":"Data","input":"<!DOCTYPE a PUBLIC/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_614() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC0","initialState":"Data","input":"<!DOCTYPE a PUBLIC0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_615() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC1","initialState":"Data","input":"<!DOCTYPE a PUBLIC1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_616() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC9","initialState":"Data","input":"<!DOCTYPE a PUBLIC9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_617() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC<","initialState":"Data","input":"<!DOCTYPE a PUBLIC<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_618() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC=","initialState":"Data","input":"<!DOCTYPE a PUBLIC=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_619() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC>","initialState":"Data","input":"<!DOCTYPE a PUBLIC>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_620() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC?","initialState":"Data","input":"<!DOCTYPE a PUBLIC?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_621() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC@","initialState":"Data","input":"<!DOCTYPE a PUBLIC@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_622() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICA","initialState":"Data","input":"<!DOCTYPE a PUBLICA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_623() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICB","initialState":"Data","input":"<!DOCTYPE a PUBLICB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_624() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICY","initialState":"Data","input":"<!DOCTYPE a PUBLICY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_625() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICZ","initialState":"Data","input":"<!DOCTYPE a PUBLICZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_626() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC`","initialState":"Data","input":"<!DOCTYPE a PUBLIC`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_627() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICa","initialState":"Data","input":"<!DOCTYPE a PUBLICa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_628() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICb","initialState":"Data","input":"<!DOCTYPE a PUBLICb","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_629() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICy","initialState":"Data","input":"<!DOCTYPE a PUBLICy","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_630() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLICz","initialState":"Data","input":"<!DOCTYPE a PUBLICz","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_631() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC{","initialState":"Data","input":"<!DOCTYPE a PUBLIC{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_632() {
    tokenize(
        r##"{"description":"<!DOCTYPE a PUBLIC\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a PUBLIC􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,80,85,66,76,73,67,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_633() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM","initialState":"Data","input":"<!DOCTYPE a SYSTEM","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_634() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_635() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM \\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_636() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM x\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_637() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u0008","initialState":"Data","input":"<!DOCTYPE a SYSTEM\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_638() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_639() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_640() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_641() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_642() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u000D","initialState":"Data","input":"<!DOCTYPE a SYSTEM\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_643() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\u001F","initialState":"Data","input":"<!DOCTYPE a SYSTEM\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_644() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM ","initialState":"Data","input":"<!DOCTYPE a SYSTEM ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_645() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM!","initialState":"Data","input":"<!DOCTYPE a SYSTEM!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_646() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_647() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"�","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_648() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\t","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_649() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\n","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_650() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\u000b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_651() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\f","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_652() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\" ","initialState":"Data","input":"<!DOCTYPE a SYSTEM\" ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":" ","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_653() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"!","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"!","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_654() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_655() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"#","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"#","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_656() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"&","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"&","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_657() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"'","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"'","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_658() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"-","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"-","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_659() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"/","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"/","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_660() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"0","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"0","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_661() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"1","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"1","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_662() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"9","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"9","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_663() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"<","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"<","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_664() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"=","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"=","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_665() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\">","initialState":"Data","input":"<!DOCTYPE a SYSTEM\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_666() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"?","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"?","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_667() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"@","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"@","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_668() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"A","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"A","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_669() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"B","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"B","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_670() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"Y","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_671() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"Z","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_672() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"`","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"`","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_673() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"a","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"a","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_674() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"b","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_675() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"y","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_676() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"z","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_677() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"{","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"{","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_678() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\"\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM\"􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,34,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"􀀀","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_679() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM#","initialState":"Data","input":"<!DOCTYPE a SYSTEM#","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_680() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM&","initialState":"Data","input":"<!DOCTYPE a SYSTEM&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_681() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'","initialState":"Data","input":"<!DOCTYPE a SYSTEM'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_682() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"�","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_683() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\t","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_684() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\n","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_685() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\u000b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_686() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\f","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_687() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM' ","initialState":"Data","input":"<!DOCTYPE a SYSTEM' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":" ","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_688() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'!","initialState":"Data","input":"<!DOCTYPE a SYSTEM'!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"!","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_689() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM'\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"\"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_690() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'&","initialState":"Data","input":"<!DOCTYPE a SYSTEM'&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"&","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_691() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''","initialState":"Data","input":"<!DOCTYPE a SYSTEM''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_692() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_693() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u0008","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_694() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u0009","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_695() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000A","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_696() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000B","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_697() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000C","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_698() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u000D","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\r","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_699() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\u001F","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":21}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_700() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'' ","initialState":"Data","input":"<!DOCTYPE a SYSTEM'' ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_701() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'' \\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM'' \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":22}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_702() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'' x\\u0000","initialState":"Data","input":"<!DOCTYPE a SYSTEM'' x\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,32,120,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":22}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":23}}]}"##,
    );
}

#[test]
fn test_703() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''!","initialState":"Data","input":"<!DOCTYPE a SYSTEM''!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_704() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\"","initialState":"Data","input":"<!DOCTYPE a SYSTEM''\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_705() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''&","initialState":"Data","input":"<!DOCTYPE a SYSTEM''&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_706() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'''","initialState":"Data","input":"<!DOCTYPE a SYSTEM'''","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_707() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''-","initialState":"Data","input":"<!DOCTYPE a SYSTEM''-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_708() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''/","initialState":"Data","input":"<!DOCTYPE a SYSTEM''/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_709() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''0","initialState":"Data","input":"<!DOCTYPE a SYSTEM''0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_710() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''1","initialState":"Data","input":"<!DOCTYPE a SYSTEM''1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_711() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''9","initialState":"Data","input":"<!DOCTYPE a SYSTEM''9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_712() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''<","initialState":"Data","input":"<!DOCTYPE a SYSTEM''<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_713() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''=","initialState":"Data","input":"<!DOCTYPE a SYSTEM''=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_714() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''>","initialState":"Data","input":"<!DOCTYPE a SYSTEM''>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_715() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''?","initialState":"Data","input":"<!DOCTYPE a SYSTEM''?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_716() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''@","initialState":"Data","input":"<!DOCTYPE a SYSTEM''@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_717() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''A","initialState":"Data","input":"<!DOCTYPE a SYSTEM''A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_718() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''B","initialState":"Data","input":"<!DOCTYPE a SYSTEM''B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_719() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''Y","initialState":"Data","input":"<!DOCTYPE a SYSTEM''Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_720() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''Z","initialState":"Data","input":"<!DOCTYPE a SYSTEM''Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_721() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''`","initialState":"Data","input":"<!DOCTYPE a SYSTEM''`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_722() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''a","initialState":"Data","input":"<!DOCTYPE a SYSTEM''a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_723() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''b","initialState":"Data","input":"<!DOCTYPE a SYSTEM''b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_724() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''y","initialState":"Data","input":"<!DOCTYPE a SYSTEM''y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_725() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''z","initialState":"Data","input":"<!DOCTYPE a SYSTEM''z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_726() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''{","initialState":"Data","input":"<!DOCTYPE a SYSTEM''{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_727() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM''\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM''􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":false}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"UnexpectedCharacterAfterDoctypeSystemIdentifier","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_728() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'(","initialState":"Data","input":"<!DOCTYPE a SYSTEM'(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"(","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_729() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'-","initialState":"Data","input":"<!DOCTYPE a SYSTEM'-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"-","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_730() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'/","initialState":"Data","input":"<!DOCTYPE a SYSTEM'/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"/","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_731() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'0","initialState":"Data","input":"<!DOCTYPE a SYSTEM'0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"0","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_732() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'1","initialState":"Data","input":"<!DOCTYPE a SYSTEM'1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"1","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_733() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'9","initialState":"Data","input":"<!DOCTYPE a SYSTEM'9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"9","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_734() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'<","initialState":"Data","input":"<!DOCTYPE a SYSTEM'<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"<","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_735() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'=","initialState":"Data","input":"<!DOCTYPE a SYSTEM'=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"=","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_736() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'>","initialState":"Data","input":"<!DOCTYPE a SYSTEM'>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"AbruptDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_737() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'?","initialState":"Data","input":"<!DOCTYPE a SYSTEM'?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"?","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_738() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'@","initialState":"Data","input":"<!DOCTYPE a SYSTEM'@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"@","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_739() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'A","initialState":"Data","input":"<!DOCTYPE a SYSTEM'A","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"A","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_740() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'B","initialState":"Data","input":"<!DOCTYPE a SYSTEM'B","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"B","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_741() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'Y","initialState":"Data","input":"<!DOCTYPE a SYSTEM'Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_742() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'Z","initialState":"Data","input":"<!DOCTYPE a SYSTEM'Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"Z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_743() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'`","initialState":"Data","input":"<!DOCTYPE a SYSTEM'`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"`","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_744() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'a","initialState":"Data","input":"<!DOCTYPE a SYSTEM'a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"a","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_745() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'b","initialState":"Data","input":"<!DOCTYPE a SYSTEM'b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"b","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_746() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'y","initialState":"Data","input":"<!DOCTYPE a SYSTEM'y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"y","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_747() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'z","initialState":"Data","input":"<!DOCTYPE a SYSTEM'z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"z","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_748() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'{","initialState":"Data","input":"<!DOCTYPE a SYSTEM'{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"{","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_749() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM'\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM'􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":"􀀀","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceAfterDoctypeSystemKeyword","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_750() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM(","initialState":"Data","input":"<!DOCTYPE a SYSTEM(","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,40],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_751() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM-","initialState":"Data","input":"<!DOCTYPE a SYSTEM-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_752() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM/","initialState":"Data","input":"<!DOCTYPE a SYSTEM/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_753() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM0","initialState":"Data","input":"<!DOCTYPE a SYSTEM0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_754() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM1","initialState":"Data","input":"<!DOCTYPE a SYSTEM1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_755() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM9","initialState":"Data","input":"<!DOCTYPE a SYSTEM9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_756() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM<","initialState":"Data","input":"<!DOCTYPE a SYSTEM<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_757() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM=","initialState":"Data","input":"<!DOCTYPE a SYSTEM=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_758() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM>","initialState":"Data","input":"<!DOCTYPE a SYSTEM>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_759() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM?","initialState":"Data","input":"<!DOCTYPE a SYSTEM?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_760() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM@","initialState":"Data","input":"<!DOCTYPE a SYSTEM@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_761() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMA","initialState":"Data","input":"<!DOCTYPE a SYSTEMA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_762() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMB","initialState":"Data","input":"<!DOCTYPE a SYSTEMB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_763() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMY","initialState":"Data","input":"<!DOCTYPE a SYSTEMY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_764() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMZ","initialState":"Data","input":"<!DOCTYPE a SYSTEMZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_765() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM`","initialState":"Data","input":"<!DOCTYPE a SYSTEM`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_766() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMa","initialState":"Data","input":"<!DOCTYPE a SYSTEMa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_767() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMb","initialState":"Data","input":"<!DOCTYPE a SYSTEMb","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_768() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMy","initialState":"Data","input":"<!DOCTYPE a SYSTEMy","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_769() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEMz","initialState":"Data","input":"<!DOCTYPE a SYSTEMz","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_770() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM{","initialState":"Data","input":"<!DOCTYPE a SYSTEM{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_771() {
    tokenize(
        r##"{"description":"<!DOCTYPE a SYSTEM\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a SYSTEM􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,83,89,83,84,69,77,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_772() {
    tokenize(
        r##"{"description":"<!DOCTYPE a Y","initialState":"Data","input":"<!DOCTYPE a Y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_773() {
    tokenize(
        r##"{"description":"<!DOCTYPE a Z","initialState":"Data","input":"<!DOCTYPE a Z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_774() {
    tokenize(
        r##"{"description":"<!DOCTYPE a `","initialState":"Data","input":"<!DOCTYPE a `","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_775() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a","initialState":"Data","input":"<!DOCTYPE a a","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_776() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u0000","initialState":"Data","input":"<!DOCTYPE a a\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_777() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u0009","initialState":"Data","input":"<!DOCTYPE a a\t","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_778() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u000A","initialState":"Data","input":"<!DOCTYPE a a\n","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_779() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u000B","initialState":"Data","input":"<!DOCTYPE a a\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_780() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\u000C","initialState":"Data","input":"<!DOCTYPE a a\f","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_781() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a ","initialState":"Data","input":"<!DOCTYPE a a ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_782() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a!","initialState":"Data","input":"<!DOCTYPE a a!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_783() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\"","initialState":"Data","input":"<!DOCTYPE a a\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_784() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a&","initialState":"Data","input":"<!DOCTYPE a a&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_785() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a'","initialState":"Data","input":"<!DOCTYPE a a'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_786() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a-","initialState":"Data","input":"<!DOCTYPE a a-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_787() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a/","initialState":"Data","input":"<!DOCTYPE a a/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_788() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a0","initialState":"Data","input":"<!DOCTYPE a a0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_789() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a1","initialState":"Data","input":"<!DOCTYPE a a1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_790() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a9","initialState":"Data","input":"<!DOCTYPE a a9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_791() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a<","initialState":"Data","input":"<!DOCTYPE a a<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_792() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a=","initialState":"Data","input":"<!DOCTYPE a a=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_793() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a>","initialState":"Data","input":"<!DOCTYPE a a>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_794() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a?","initialState":"Data","input":"<!DOCTYPE a a?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_795() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a@","initialState":"Data","input":"<!DOCTYPE a a@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_796() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aA","initialState":"Data","input":"<!DOCTYPE a aA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_797() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aB","initialState":"Data","input":"<!DOCTYPE a aB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_798() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aY","initialState":"Data","input":"<!DOCTYPE a aY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,89],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_799() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aZ","initialState":"Data","input":"<!DOCTYPE a aZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,90],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_800() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a`","initialState":"Data","input":"<!DOCTYPE a a`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,96],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_801() {
    tokenize(
        r##"{"description":"<!DOCTYPE a aa","initialState":"Data","input":"<!DOCTYPE a aa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_802() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ab","initialState":"Data","input":"<!DOCTYPE a ab","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_803() {
    tokenize(
        r##"{"description":"<!DOCTYPE a ay","initialState":"Data","input":"<!DOCTYPE a ay","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_804() {
    tokenize(
        r##"{"description":"<!DOCTYPE a az","initialState":"Data","input":"<!DOCTYPE a az","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_805() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a{","initialState":"Data","input":"<!DOCTYPE a a{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_806() {
    tokenize(
        r##"{"description":"<!DOCTYPE a a\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a a􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,97,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_807() {
    tokenize(
        r##"{"description":"<!DOCTYPE a b","initialState":"Data","input":"<!DOCTYPE a b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,98],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_808() {
    tokenize(
        r##"{"description":"<!DOCTYPE a y","initialState":"Data","input":"<!DOCTYPE a y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,121],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_809() {
    tokenize(
        r##"{"description":"<!DOCTYPE a z","initialState":"Data","input":"<!DOCTYPE a z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,122],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_810() {
    tokenize(
        r##"{"description":"<!DOCTYPE a {","initialState":"Data","input":"<!DOCTYPE a {","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,123],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_811() {
    tokenize(
        r##"{"description":"<!DOCTYPE a \\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a 􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,32,56256,56320],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_812() {
    tokenize(
        r##"{"description":"<!DOCTYPE a!","initialState":"Data","input":"<!DOCTYPE a!","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,33],"output":[{"Doctype":{"name":"a!","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_813() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\"","initialState":"Data","input":"<!DOCTYPE a\"","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,34],"output":[{"Doctype":{"name":"a\"","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_814() {
    tokenize(
        r##"{"description":"<!DOCTYPE a&","initialState":"Data","input":"<!DOCTYPE a&","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,38],"output":[{"Doctype":{"name":"a&","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_815() {
    tokenize(
        r##"{"description":"<!DOCTYPE a'","initialState":"Data","input":"<!DOCTYPE a'","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,39],"output":[{"Doctype":{"name":"a'","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_816() {
    tokenize(
        r##"{"description":"<!DOCTYPE a-","initialState":"Data","input":"<!DOCTYPE a-","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,45],"output":[{"Doctype":{"name":"a-","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_817() {
    tokenize(
        r##"{"description":"<!DOCTYPE a/","initialState":"Data","input":"<!DOCTYPE a/","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,47],"output":[{"Doctype":{"name":"a/","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_818() {
    tokenize(
        r##"{"description":"<!DOCTYPE a0","initialState":"Data","input":"<!DOCTYPE a0","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,48],"output":[{"Doctype":{"name":"a0","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_819() {
    tokenize(
        r##"{"description":"<!DOCTYPE a1","initialState":"Data","input":"<!DOCTYPE a1","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,49],"output":[{"Doctype":{"name":"a1","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_820() {
    tokenize(
        r##"{"description":"<!DOCTYPE a9","initialState":"Data","input":"<!DOCTYPE a9","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,57],"output":[{"Doctype":{"name":"a9","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_821() {
    tokenize(
        r##"{"description":"<!DOCTYPE a<","initialState":"Data","input":"<!DOCTYPE a<","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,60],"output":[{"Doctype":{"name":"a<","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_822() {
    tokenize(
        r##"{"description":"<!DOCTYPE a=","initialState":"Data","input":"<!DOCTYPE a=","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,61],"output":[{"Doctype":{"name":"a=","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_823() {
    tokenize(
        r##"{"description":"<!DOCTYPE a>","initialState":"Data","input":"<!DOCTYPE a>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_824() {
    tokenize(
        r##"{"description":"<!DOCTYPE a?","initialState":"Data","input":"<!DOCTYPE a?","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,63],"output":[{"Doctype":{"name":"a?","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_825() {
    tokenize(
        r##"{"description":"<!DOCTYPE a@","initialState":"Data","input":"<!DOCTYPE a@","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,64],"output":[{"Doctype":{"name":"a@","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_826() {
    tokenize(
        r##"{"description":"<!DOCTYPE aA","initialState":"Data","input":"<!DOCTYPE aA","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,65],"output":[{"Doctype":{"name":"aa","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_827() {
    tokenize(
        r##"{"description":"<!DOCTYPE aB","initialState":"Data","input":"<!DOCTYPE aB","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,66],"output":[{"Doctype":{"name":"ab","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_828() {
    tokenize(
        r##"{"description":"<!DOCTYPE aY","initialState":"Data","input":"<!DOCTYPE aY","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,89],"output":[{"Doctype":{"name":"ay","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_829() {
    tokenize(
        r##"{"description":"<!DOCTYPE aZ","initialState":"Data","input":"<!DOCTYPE aZ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,90],"output":[{"Doctype":{"name":"az","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_830() {
    tokenize(
        r##"{"description":"<!DOCTYPE a[","initialState":"Data","input":"<!DOCTYPE a[","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,91],"output":[{"Doctype":{"name":"a[","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_831() {
    tokenize(
        r##"{"description":"<!DOCTYPE a`","initialState":"Data","input":"<!DOCTYPE a`","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,96],"output":[{"Doctype":{"name":"a`","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_832() {
    tokenize(
        r##"{"description":"<!DOCTYPE aa","initialState":"Data","input":"<!DOCTYPE aa","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,97],"output":[{"Doctype":{"name":"aa","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_833() {
    tokenize(
        r##"{"description":"<!DOCTYPE ab","initialState":"Data","input":"<!DOCTYPE ab","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,98],"output":[{"Doctype":{"name":"ab","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_834() {
    tokenize(
        r##"{"description":"<!DOCTYPE ay","initialState":"Data","input":"<!DOCTYPE ay","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,121],"output":[{"Doctype":{"name":"ay","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_835() {
    tokenize(
        r##"{"description":"<!DOCTYPE az","initialState":"Data","input":"<!DOCTYPE az","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,122],"output":[{"Doctype":{"name":"az","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_836() {
    tokenize(
        r##"{"description":"<!DOCTYPE a{","initialState":"Data","input":"<!DOCTYPE a{","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,123],"output":[{"Doctype":{"name":"a{","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_837() {
    tokenize(
        r##"{"description":"<!DOCTYPE a\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE a􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,97,56256,56320],"output":[{"Doctype":{"name":"a􀀀","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_838() {
    tokenize(
        r##"{"description":"<!DOCTYPE b","initialState":"Data","input":"<!DOCTYPE b","inputUtf16":[60,33,68,79,67,84,89,80,69,32,98],"output":[{"Doctype":{"name":"b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_839() {
    tokenize(
        r##"{"description":"<!DOCTYPE y","initialState":"Data","input":"<!DOCTYPE y","inputUtf16":[60,33,68,79,67,84,89,80,69,32,121],"output":[{"Doctype":{"name":"y","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_840() {
    tokenize(
        r##"{"description":"<!DOCTYPE z","initialState":"Data","input":"<!DOCTYPE z","inputUtf16":[60,33,68,79,67,84,89,80,69,32,122],"output":[{"Doctype":{"name":"z","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_841() {
    tokenize(
        r##"{"description":"<!DOCTYPE {","initialState":"Data","input":"<!DOCTYPE {","inputUtf16":[60,33,68,79,67,84,89,80,69,32,123],"output":[{"Doctype":{"name":"{","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_842() {
    tokenize(
        r##"{"description":"<!DOCTYPE \\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPE 􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,32,56256,56320],"output":[{"Doctype":{"name":"􀀀","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_843() {
    tokenize(
        r##"{"description":"<!DOCTYPE!","initialState":"Data","input":"<!DOCTYPE!","inputUtf16":[60,33,68,79,67,84,89,80,69,33],"output":[{"Doctype":{"name":"!","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_844() {
    tokenize(
        r##"{"description":"<!DOCTYPE\"","initialState":"Data","input":"<!DOCTYPE\"","inputUtf16":[60,33,68,79,67,84,89,80,69,34],"output":[{"Doctype":{"name":"\"","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_845() {
    tokenize(
        r##"{"description":"<!DOCTYPE&","initialState":"Data","input":"<!DOCTYPE&","inputUtf16":[60,33,68,79,67,84,89,80,69,38],"output":[{"Doctype":{"name":"&","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_846() {
    tokenize(
        r##"{"description":"<!DOCTYPE'","initialState":"Data","input":"<!DOCTYPE'","inputUtf16":[60,33,68,79,67,84,89,80,69,39],"output":[{"Doctype":{"name":"'","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_847() {
    tokenize(
        r##"{"description":"<!DOCTYPE-","initialState":"Data","input":"<!DOCTYPE-","inputUtf16":[60,33,68,79,67,84,89,80,69,45],"output":[{"Doctype":{"name":"-","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_848() {
    tokenize(
        r##"{"description":"<!DOCTYPE/","initialState":"Data","input":"<!DOCTYPE/","inputUtf16":[60,33,68,79,67,84,89,80,69,47],"output":[{"Doctype":{"name":"/","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_849() {
    tokenize(
        r##"{"description":"<!DOCTYPE0","initialState":"Data","input":"<!DOCTYPE0","inputUtf16":[60,33,68,79,67,84,89,80,69,48],"output":[{"Doctype":{"name":"0","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_850() {
    tokenize(
        r##"{"description":"<!DOCTYPE1","initialState":"Data","input":"<!DOCTYPE1","inputUtf16":[60,33,68,79,67,84,89,80,69,49],"output":[{"Doctype":{"name":"1","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_851() {
    tokenize(
        r##"{"description":"<!DOCTYPE9","initialState":"Data","input":"<!DOCTYPE9","inputUtf16":[60,33,68,79,67,84,89,80,69,57],"output":[{"Doctype":{"name":"9","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_852() {
    tokenize(
        r##"{"description":"<!DOCTYPE<","initialState":"Data","input":"<!DOCTYPE<","inputUtf16":[60,33,68,79,67,84,89,80,69,60],"output":[{"Doctype":{"name":"<","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_853() {
    tokenize(
        r##"{"description":"<!DOCTYPE=","initialState":"Data","input":"<!DOCTYPE=","inputUtf16":[60,33,68,79,67,84,89,80,69,61],"output":[{"Doctype":{"name":"=","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_854() {
    tokenize(
        r##"{"description":"<!DOCTYPE>","initialState":"Data","input":"<!DOCTYPE>","inputUtf16":[60,33,68,79,67,84,89,80,69,62],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingDoctypeName","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_855() {
    tokenize(
        r##"{"description":"<!DOCTYPE?","initialState":"Data","input":"<!DOCTYPE?","inputUtf16":[60,33,68,79,67,84,89,80,69,63],"output":[{"Doctype":{"name":"?","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_856() {
    tokenize(
        r##"{"description":"<!DOCTYPE@","initialState":"Data","input":"<!DOCTYPE@","inputUtf16":[60,33,68,79,67,84,89,80,69,64],"output":[{"Doctype":{"name":"@","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_857() {
    tokenize(
        r##"{"description":"<!DOCTYPEA","initialState":"Data","input":"<!DOCTYPEA","inputUtf16":[60,33,68,79,67,84,89,80,69,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_858() {
    tokenize(
        r##"{"description":"<!DOCTYPEB","initialState":"Data","input":"<!DOCTYPEB","inputUtf16":[60,33,68,79,67,84,89,80,69,66],"output":[{"Doctype":{"name":"b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_859() {
    tokenize(
        r##"{"description":"<!DOCTYPEY","initialState":"Data","input":"<!DOCTYPEY","inputUtf16":[60,33,68,79,67,84,89,80,69,89],"output":[{"Doctype":{"name":"y","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_860() {
    tokenize(
        r##"{"description":"<!DOCTYPEZ","initialState":"Data","input":"<!DOCTYPEZ","inputUtf16":[60,33,68,79,67,84,89,80,69,90],"output":[{"Doctype":{"name":"z","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_861() {
    tokenize(
        r##"{"description":"<!DOCTYPE`","initialState":"Data","input":"<!DOCTYPE`","inputUtf16":[60,33,68,79,67,84,89,80,69,96],"output":[{"Doctype":{"name":"`","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_862() {
    tokenize(
        r##"{"description":"<!DOCTYPEa","initialState":"Data","input":"<!DOCTYPEa","inputUtf16":[60,33,68,79,67,84,89,80,69,97],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_863() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u0000","initialState":"Data","input":"<!DOCTYPEa\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,0],"output":[{"Doctype":{"name":"a�","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_864() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u0008","initialState":"Data","input":"<!DOCTYPEa\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,8],"output":[{"Doctype":{"name":"a\b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_865() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u0009","initialState":"Data","input":"<!DOCTYPEa\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_866() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000A","initialState":"Data","input":"<!DOCTYPEa\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_867() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000B","initialState":"Data","input":"<!DOCTYPEa\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,11],"output":[{"Doctype":{"name":"a\u000b","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_868() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000C","initialState":"Data","input":"<!DOCTYPEa\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_869() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u000D","initialState":"Data","input":"<!DOCTYPEa\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_870() {
    tokenize(
        r##"{"description":"<!DOCTYPEa\\u001F","initialState":"Data","input":"<!DOCTYPEa\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,31],"output":[{"Doctype":{"name":"a\u001f","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":11}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_871() {
    tokenize(
        r##"{"description":"<!DOCTYPEa ","initialState":"Data","input":"<!DOCTYPEa ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_872() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u0000","initialState":"Data","input":"<!DOCTYPEa \u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_873() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u0008","initialState":"Data","input":"<!DOCTYPEa \b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_874() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u0009","initialState":"Data","input":"<!DOCTYPEa \t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_875() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000A","initialState":"Data","input":"<!DOCTYPEa \n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_876() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000B","initialState":"Data","input":"<!DOCTYPEa \u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_877() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000C","initialState":"Data","input":"<!DOCTYPEa \f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_878() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u000D","initialState":"Data","input":"<!DOCTYPEa \r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_879() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \\u001F","initialState":"Data","input":"<!DOCTYPEa \u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":12}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_880() {
    tokenize(
        r##"{"description":"<!DOCTYPEa  ","initialState":"Data","input":"<!DOCTYPEa  ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_881() {
    tokenize(
        r##"{"description":"<!DOCTYPEa !","initialState":"Data","input":"<!DOCTYPEa !","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_882() {
    tokenize(
        r##"{"description":"<!DOCTYPEa \"","initialState":"Data","input":"<!DOCTYPEa \"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,34],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_883() {
    tokenize(
        r##"{"description":"<!DOCTYPEa &","initialState":"Data","input":"<!DOCTYPEa &","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_884() {
    tokenize(
        r##"{"description":"<!DOCTYPEa '","initialState":"Data","input":"<!DOCTYPEa '","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,39],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_885() {
    tokenize(
        r##"{"description":"<!DOCTYPEa -","initialState":"Data","input":"<!DOCTYPEa -","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,45],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_886() {
    tokenize(
        r##"{"description":"<!DOCTYPEa /","initialState":"Data","input":"<!DOCTYPEa /","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,47],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_887() {
    tokenize(
        r##"{"description":"<!DOCTYPEa 0","initialState":"Data","input":"<!DOCTYPEa 0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,48],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_888() {
    tokenize(
        r##"{"description":"<!DOCTYPEa 1","initialState":"Data","input":"<!DOCTYPEa 1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,49],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_889() {
    tokenize(
        r##"{"description":"<!DOCTYPEa 9","initialState":"Data","input":"<!DOCTYPEa 9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,57],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_890() {
    tokenize(
        r##"{"description":"<!DOCTYPEa <","initialState":"Data","input":"<!DOCTYPEa <","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,60],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_891() {
    tokenize(
        r##"{"description":"<!DOCTYPEa =","initialState":"Data","input":"<!DOCTYPEa =","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,61],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_892() {
    tokenize(
        r##"{"description":"<!DOCTYPEa >","initialState":"Data","input":"<!DOCTYPEa >","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,62],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_893() {
    tokenize(
        r##"{"description":"<!DOCTYPEa ?","initialState":"Data","input":"<!DOCTYPEa ?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,63],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_894() {
    tokenize(
        r##"{"description":"<!DOCTYPEa @","initialState":"Data","input":"<!DOCTYPEa @","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,64],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_895() {
    tokenize(
        r##"{"description":"<!DOCTYPEa A","initialState":"Data","input":"<!DOCTYPEa A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,65],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_896() {
    tokenize(
        r##"{"description":"<!DOCTYPEa B","initialState":"Data","input":"<!DOCTYPEa B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,66],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_897() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC","initialState":"Data","input":"<!DOCTYPEa PUBLIC","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_898() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,0],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_899() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u0008","initialState":"Data","input":"<!DOCTYPEa PUBLIC\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,8],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_900() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,9],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_901() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,10],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_902() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,11],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_903() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,12],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_904() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u000D","initialState":"Data","input":"<!DOCTYPEa PUBLIC\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,13],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_905() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\\u001F","initialState":"Data","input":"<!DOCTYPEa PUBLIC\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,31],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_906() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC ","initialState":"Data","input":"<!DOCTYPEa PUBLIC ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,32],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_907() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC!","initialState":"Data","input":"<!DOCTYPEa PUBLIC!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,33],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_908() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_909() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_910() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_911() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_912() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_913() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_914() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\" ","initialState":"Data","input":"<!DOCTYPEa PUBLIC\" ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_915() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"!","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_916() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_917() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"#","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,35],"output":[{"Doctype":{"name":"a","public_id":"#","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_918() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"&","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_919() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"'","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,39],"output":[{"Doctype":{"name":"a","public_id":"'","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_920() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"-","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_921() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"/","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_922() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"0","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_923() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"1","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_924() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"9","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_925() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"<","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_926() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"=","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_927() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\">","initialState":"Data","input":"<!DOCTYPEa PUBLIC\">","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_928() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"?","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,63],"output":[{"Doctype":{"name":"a","public_id":"?","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_929() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"@","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,64],"output":[{"Doctype":{"name":"a","public_id":"@","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_930() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"A","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,65],"output":[{"Doctype":{"name":"a","public_id":"A","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_931() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"B","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,66],"output":[{"Doctype":{"name":"a","public_id":"B","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_932() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"Y","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,89],"output":[{"Doctype":{"name":"a","public_id":"Y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_933() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"Z","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,90],"output":[{"Doctype":{"name":"a","public_id":"Z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_934() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"`","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,96],"output":[{"Doctype":{"name":"a","public_id":"`","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_935() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"a","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,97],"output":[{"Doctype":{"name":"a","public_id":"a","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_936() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"b","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,98],"output":[{"Doctype":{"name":"a","public_id":"b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_937() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"y","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,121],"output":[{"Doctype":{"name":"a","public_id":"y","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_938() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"z","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,122],"output":[{"Doctype":{"name":"a","public_id":"z","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_939() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"{","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,123],"output":[{"Doctype":{"name":"a","public_id":"{","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_940() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC\"\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa PUBLIC\"􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,34,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"􀀀","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_941() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC#","initialState":"Data","input":"<!DOCTYPEa PUBLIC#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,35],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_942() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC&","initialState":"Data","input":"<!DOCTYPEa PUBLIC&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,38],"output":[{"Doctype":{"name":"a","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingQuoteBeforeDoctypePublicIdentifier","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_943() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'","initialState":"Data","input":"<!DOCTYPEa PUBLIC'","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_944() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,0],"output":[{"Doctype":{"name":"a","public_id":"�","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_945() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,9],"output":[{"Doctype":{"name":"a","public_id":"\t","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_946() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,10],"output":[{"Doctype":{"name":"a","public_id":"\n","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_947() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,11],"output":[{"Doctype":{"name":"a","public_id":"\u000b","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":19}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_948() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,12],"output":[{"Doctype":{"name":"a","public_id":"\f","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_949() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC' ","initialState":"Data","input":"<!DOCTYPEa PUBLIC' ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,32],"output":[{"Doctype":{"name":"a","public_id":" ","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_950() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'!","initialState":"Data","input":"<!DOCTYPEa PUBLIC'!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,33],"output":[{"Doctype":{"name":"a","public_id":"!","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_951() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC'\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,34],"output":[{"Doctype":{"name":"a","public_id":"\"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_952() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'&","initialState":"Data","input":"<!DOCTYPEa PUBLIC'&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,38],"output":[{"Doctype":{"name":"a","public_id":"&","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_953() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''","initialState":"Data","input":"<!DOCTYPEa PUBLIC''","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_954() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u0000","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\u0000","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,0],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_955() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u0008","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,8],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_956() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u0009","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\t","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,9],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_957() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000A","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\n","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,10],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_958() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000B","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\u000b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,11],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_959() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000C","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,12],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_960() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u000D","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\r","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,13],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_961() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\u001F","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\u001f","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,31],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":20}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_962() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'' ","initialState":"Data","input":"<!DOCTYPEa PUBLIC'' ","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,32],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_963() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''!","initialState":"Data","input":"<!DOCTYPEa PUBLIC''!","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,33],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_964() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\"","initialState":"Data","input":"<!DOCTYPEa PUBLIC''\"","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,34],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_965() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''#","initialState":"Data","input":"<!DOCTYPEa PUBLIC''#","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,35],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_966() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''&","initialState":"Data","input":"<!DOCTYPEa PUBLIC''&","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,38],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_967() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'''","initialState":"Data","input":"<!DOCTYPEa PUBLIC'''","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,39],"output":[{"Doctype":{"name":"a","public_id":"","system_id":"","force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers","location":{"line":1,"column":20}},{"code":"EofInDoctype","location":{"line":1,"column":21}}]}"##,
    );
}

#[test]
fn test_968() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''(","initialState":"Data","input":"<!DOCTYPEa PUBLIC''(","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,40],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_969() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''-","initialState":"Data","input":"<!DOCTYPEa PUBLIC''-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,45],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_970() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''/","initialState":"Data","input":"<!DOCTYPEa PUBLIC''/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,47],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_971() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''0","initialState":"Data","input":"<!DOCTYPEa PUBLIC''0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,48],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_972() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''1","initialState":"Data","input":"<!DOCTYPEa PUBLIC''1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,49],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_973() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''9","initialState":"Data","input":"<!DOCTYPEa PUBLIC''9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,57],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_974() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''<","initialState":"Data","input":"<!DOCTYPEa PUBLIC''<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,60],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_975() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''=","initialState":"Data","input":"<!DOCTYPEa PUBLIC''=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,61],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_976() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''>","initialState":"Data","input":"<!DOCTYPEa PUBLIC''>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":false}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}}]}"##,
    );
}

#[test]
fn test_977() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''?","initialState":"Data","input":"<!DOCTYPEa PUBLIC''?","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,63],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_978() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''@","initialState":"Data","input":"<!DOCTYPEa PUBLIC''@","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,64],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_979() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''A","initialState":"Data","input":"<!DOCTYPEa PUBLIC''A","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,65],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_980() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''B","initialState":"Data","input":"<!DOCTYPEa PUBLIC''B","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,66],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_981() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''Y","initialState":"Data","input":"<!DOCTYPEa PUBLIC''Y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,89],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_982() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''Z","initialState":"Data","input":"<!DOCTYPEa PUBLIC''Z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,90],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_983() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''`","initialState":"Data","input":"<!DOCTYPEa PUBLIC''`","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,96],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_984() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''a","initialState":"Data","input":"<!DOCTYPEa PUBLIC''a","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,97],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_985() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''b","initialState":"Data","input":"<!DOCTYPEa PUBLIC''b","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,98],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_986() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''y","initialState":"Data","input":"<!DOCTYPEa PUBLIC''y","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,121],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_987() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''z","initialState":"Data","input":"<!DOCTYPEa PUBLIC''z","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,122],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_988() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''{","initialState":"Data","input":"<!DOCTYPEa PUBLIC''{","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,123],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_989() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC''\\uDBC0\\uDC00","initialState":"Data","input":"<!DOCTYPEa PUBLIC''􀀀","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,39,56256,56320],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"MissingQuoteBeforeDoctypeSystemIdentifier","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_990() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'(","initialState":"Data","input":"<!DOCTYPEa PUBLIC'(","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,40],"output":[{"Doctype":{"name":"a","public_id":"(","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_991() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'-","initialState":"Data","input":"<!DOCTYPEa PUBLIC'-","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,45],"output":[{"Doctype":{"name":"a","public_id":"-","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_992() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'/","initialState":"Data","input":"<!DOCTYPEa PUBLIC'/","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,47],"output":[{"Doctype":{"name":"a","public_id":"/","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_993() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'0","initialState":"Data","input":"<!DOCTYPEa PUBLIC'0","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,48],"output":[{"Doctype":{"name":"a","public_id":"0","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_994() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'1","initialState":"Data","input":"<!DOCTYPEa PUBLIC'1","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,49],"output":[{"Doctype":{"name":"a","public_id":"1","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_995() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'9","initialState":"Data","input":"<!DOCTYPEa PUBLIC'9","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,57],"output":[{"Doctype":{"name":"a","public_id":"9","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_996() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'<","initialState":"Data","input":"<!DOCTYPEa PUBLIC'<","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,60],"output":[{"Doctype":{"name":"a","public_id":"<","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_997() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'=","initialState":"Data","input":"<!DOCTYPEa PUBLIC'=","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,61],"output":[{"Doctype":{"name":"a","public_id":"=","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"EofInDoctype","location":{"line":1,"column":20}}]}"##,
    );
}

#[test]
fn test_998() {
    tokenize(
        r##"{"description":"<!DOCTYPEa PUBLIC'>","initialState":"Data","input":"<!DOCTYPEa PUBLIC'>","inputUtf16":[60,33,68,79,67,84,89,80,69,97,32,80,85,66,76,73,67,39,62],"output":[{"Doctype":{"name":"a","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"MissingWhitespaceBeforeDoctypeName","location":{"line":1,"column":10}},{"code":"MissingWhitespaceAfterDoctypePublicKeyword","location":{"line":1,"column":18}},{"code":"AbruptDoctypePublicIdentifier","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_999() {
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
