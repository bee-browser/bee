//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"PLAINTEXT content model flag","initialState":"Plaintext","lastStartTag":"plaintext","input":"<head>&body;","inputUtf16":[60,104,101,97,100,62,38,98,111,100,121,59],"output":[{"Character":{"data":"<head>&body;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"PLAINTEXT with seeming close tag","initialState":"Plaintext","lastStartTag":"plaintext","input":"</plaintext>&body;","inputUtf16":[60,47,112,108,97,105,110,116,101,120,116,62,38,98,111,100,121,59],"output":[{"Character":{"data":"</plaintext>&body;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp>","inputUtf16":[102,111,111,60,47,120,109,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp>","inputUtf16":[102,111,111,60,47,120,109,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (case-insensitivity)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xMp>","inputUtf16":[102,111,111,60,47,120,77,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_5() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (case-insensitivity)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xMp>","inputUtf16":[102,111,111,60,47,120,77,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_6() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with space)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp ","inputUtf16":[102,111,111,60,47,120,109,112,32],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"EofInTag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_7() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with space)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp ","inputUtf16":[102,111,111,60,47,120,109,112,32],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"EofInTag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_8() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with EOF)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp","inputUtf16":[102,111,111,60,47,120,109,112],"output":[{"Character":{"data":"foo</xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_9() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with EOF)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp","inputUtf16":[102,111,111,60,47,120,109,112],"output":[{"Character":{"data":"foo</xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_10() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with slash)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp/","inputUtf16":[102,111,111,60,47,120,109,112,47],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"EofInTag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_11() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with slash)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp/","inputUtf16":[102,111,111,60,47,120,109,112,47],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"EofInTag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_12() {
    tokenize(
        r##"{"description":"End tag not closing RCDATA or RAWTEXT (ending with left-angle-bracket)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp<","inputUtf16":[102,111,111,60,47,120,109,112,60],"output":[{"Character":{"data":"foo</xmp<"}}],"errors":[]}"##,
    );
}

#[test]
fn test_13() {
    tokenize(
        r##"{"description":"End tag not closing RCDATA or RAWTEXT (ending with left-angle-bracket)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp<","inputUtf16":[102,111,111,60,47,120,109,112,60],"output":[{"Character":{"data":"foo</xmp<"}}],"errors":[]}"##,
    );
}

#[test]
fn test_14() {
    tokenize(
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"</foo>bar</xmp>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,62],"output":[{"Character":{"data":"</foo>bar"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_15() {
    tokenize(
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"</foo>bar</xmp>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,62],"output":[{"Character":{"data":"</foo>bar"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_16() {
    tokenize(
        r##"{"description":"Partial end tags leading straight into partial end tags","initialState":"Rcdata","lastStartTag":"xmp","input":"</xmp</xmp</xmp>","inputUtf16":[60,47,120,109,112,60,47,120,109,112,60,47,120,109,112,62],"output":[{"Character":{"data":"</xmp</xmp"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_17() {
    tokenize(
        r##"{"description":"Partial end tags leading straight into partial end tags","initialState":"Rawtext","lastStartTag":"xmp","input":"</xmp</xmp</xmp>","inputUtf16":[60,47,120,109,112,60,47,120,109,112,60,47,120,109,112,62],"output":[{"Character":{"data":"</xmp</xmp"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_18() {
    tokenize(
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT (starting like correct name)","initialState":"Rcdata","lastStartTag":"xmp","input":"</foo>bar</xmpaar>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,97,97,114,62],"output":[{"Character":{"data":"</foo>bar</xmpaar>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_19() {
    tokenize(
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT (starting like correct name)","initialState":"Rawtext","lastStartTag":"xmp","input":"</foo>bar</xmpaar>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,97,97,114,62],"output":[{"Character":{"data":"</foo>bar</xmpaar>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_20() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT, switching back to PCDATA","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp></baz>","inputUtf16":[102,111,111,60,47,120,109,112,62,60,47,98,97,122,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}},{"EndTag":{"name":"baz"}}],"errors":[]}"##,
    );
}

#[test]
fn test_21() {
    tokenize(
        r##"{"description":"End tag closing RCDATA or RAWTEXT, switching back to PCDATA","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp></baz>","inputUtf16":[102,111,111,60,47,120,109,112,62,60,47,98,97,122,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}},{"EndTag":{"name":"baz"}}],"errors":[]}"##,
    );
}

#[test]
fn test_22() {
    tokenize(
        r##"{"description":"RAWTEXT w/ something looking like an entity","initialState":"Rawtext","lastStartTag":"xmp","input":"&foo;","inputUtf16":[38,102,111,111,59],"output":[{"Character":{"data":"&foo;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_23() {
    tokenize(
        r##"{"description":"RCDATA w/ an entity","initialState":"Rcdata","lastStartTag":"textarea","input":"&lt;","inputUtf16":[38,108,116,59],"output":[{"Character":{"data":"<"}}],"errors":[]}"##,
    );
}
//</coverage:exclude>
