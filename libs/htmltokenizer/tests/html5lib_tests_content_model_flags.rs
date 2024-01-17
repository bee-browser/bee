mod helper;

use helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"PLAINTEXT content model flag","initialState":"Plaintext","lastStartTag":"plaintext","input":"<head>&body;","inputUtf16":[60,104,101,97,100,62,38,98,111,100,121,59],"output":[{"Character":{"data":"<head>&body;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0001() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"PLAINTEXT with seeming close tag","initialState":"Plaintext","lastStartTag":"plaintext","input":"</plaintext>&body;","inputUtf16":[60,47,112,108,97,105,110,116,101,120,116,62,38,98,111,100,121,59],"output":[{"Character":{"data":"</plaintext>&body;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0002() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp>","inputUtf16":[102,111,111,60,47,120,109,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0003() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp>","inputUtf16":[102,111,111,60,47,120,109,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0004() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (case-insensitivity)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xMp>","inputUtf16":[102,111,111,60,47,120,77,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0005() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (case-insensitivity)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xMp>","inputUtf16":[102,111,111,60,47,120,77,112,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0006() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with space)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp ","inputUtf16":[102,111,111,60,47,120,109,112,32],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"eof-in-tag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0007() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with space)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp ","inputUtf16":[102,111,111,60,47,120,109,112,32],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"eof-in-tag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0008() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with EOF)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp","inputUtf16":[102,111,111,60,47,120,109,112],"output":[{"Character":{"data":"foo</xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0009() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with EOF)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp","inputUtf16":[102,111,111,60,47,120,109,112],"output":[{"Character":{"data":"foo</xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0010() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with slash)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp/","inputUtf16":[102,111,111,60,47,120,109,112,47],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"eof-in-tag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0011() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT (ending with slash)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp/","inputUtf16":[102,111,111,60,47,120,109,112,47],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"eof-in-tag","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0012() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag not closing RCDATA or RAWTEXT (ending with left-angle-bracket)","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp<","inputUtf16":[102,111,111,60,47,120,109,112,60],"output":[{"Character":{"data":"foo</xmp<"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0013() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag not closing RCDATA or RAWTEXT (ending with left-angle-bracket)","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp<","inputUtf16":[102,111,111,60,47,120,109,112,60],"output":[{"Character":{"data":"foo</xmp<"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0014() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"</foo>bar</xmp>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,62],"output":[{"Character":{"data":"</foo>bar"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0015() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"</foo>bar</xmp>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,62],"output":[{"Character":{"data":"</foo>bar"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0016() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Partial end tags leading straight into partial end tags","initialState":"Rcdata","lastStartTag":"xmp","input":"</xmp</xmp</xmp>","inputUtf16":[60,47,120,109,112,60,47,120,109,112,60,47,120,109,112,62],"output":[{"Character":{"data":"</xmp</xmp"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0017() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Partial end tags leading straight into partial end tags","initialState":"Rawtext","lastStartTag":"xmp","input":"</xmp</xmp</xmp>","inputUtf16":[60,47,120,109,112,60,47,120,109,112,60,47,120,109,112,62],"output":[{"Character":{"data":"</xmp</xmp"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0018() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT (starting like correct name)","initialState":"Rcdata","lastStartTag":"xmp","input":"</foo>bar</xmpaar>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,97,97,114,62],"output":[{"Character":{"data":"</foo>bar</xmpaar>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0019() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag with incorrect name in RCDATA or RAWTEXT (starting like correct name)","initialState":"Rawtext","lastStartTag":"xmp","input":"</foo>bar</xmpaar>","inputUtf16":[60,47,102,111,111,62,98,97,114,60,47,120,109,112,97,97,114,62],"output":[{"Character":{"data":"</foo>bar</xmpaar>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0020() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT, switching back to PCDATA","initialState":"Rcdata","lastStartTag":"xmp","input":"foo</xmp></baz>","inputUtf16":[102,111,111,60,47,120,109,112,62,60,47,98,97,122,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}},{"EndTag":{"name":"baz"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0021() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"End tag closing RCDATA or RAWTEXT, switching back to PCDATA","initialState":"Rawtext","lastStartTag":"xmp","input":"foo</xmp></baz>","inputUtf16":[102,111,111,60,47,120,109,112,62,60,47,98,97,122,62],"output":[{"Character":{"data":"foo"}},{"EndTag":{"name":"xmp"}},{"EndTag":{"name":"baz"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0022() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"RAWTEXT w/ something looking like an entity","initialState":"Rawtext","lastStartTag":"xmp","input":"&foo;","inputUtf16":[38,102,111,111,59],"output":[{"Character":{"data":"&foo;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0023() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"RCDATA w/ an entity","initialState":"Rcdata","lastStartTag":"textarea","input":"&lt;","inputUtf16":[38,108,116,59],"output":[{"Character":{"data":"<"}}],"errors":[]}"##,
    );
}
