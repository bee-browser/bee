//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"Commented close tag in RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"foo<!--</xmp>--></xmp>","inputUtf16":[102,111,111,60,33,45,45,60,47,120,109,112,62,45,45,62,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!--"}},{"EndTag":{"name":"xmp"}},{"Character":{"data":"-->"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"Commented close tag in RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"foo<!--</xmp>--></xmp>","inputUtf16":[102,111,111,60,33,45,45,60,47,120,109,112,62,45,45,62,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!--"}},{"EndTag":{"name":"xmp"}},{"Character":{"data":"-->"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"Bogus comment in RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"foo<!-->baz</xmp>","inputUtf16":[102,111,111,60,33,45,45,62,98,97,122,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!-->baz"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"Bogus comment in RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"foo<!-->baz</xmp>","inputUtf16":[102,111,111,60,33,45,45,62,98,97,122,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!-->baz"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"End tag surrounded by bogus comment in RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"foo<!--></xmp><!-->baz</xmp>","inputUtf16":[102,111,111,60,33,45,45,62,60,47,120,109,112,62,60,33,45,45,62,98,97,122,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!-->"}},{"EndTag":{"name":"xmp"}},{"Comment":{"data":""}},{"Character":{"data":"baz"}},{"EndTag":{"name":"xmp"}}],"errors":[{"code":"AbruptClosingOfEmptyComment","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_5() {
    tokenize(
        r##"{"description":"End tag surrounded by bogus comment in RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"foo<!--></xmp><!-->baz</xmp>","inputUtf16":[102,111,111,60,33,45,45,62,60,47,120,109,112,62,60,33,45,45,62,98,97,122,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!-->"}},{"EndTag":{"name":"xmp"}},{"Comment":{"data":""}},{"Character":{"data":"baz"}},{"EndTag":{"name":"xmp"}}],"errors":[{"code":"AbruptClosingOfEmptyComment","location":{"line":1,"column":19}}]}"##,
    );
}

#[test]
fn test_6() {
    tokenize(
        r##"{"description":"Commented entities in RCDATA","initialState":"Rcdata","lastStartTag":"xmp","input":" &amp; <!-- &amp; --> &amp; </xmp>","inputUtf16":[32,38,97,109,112,59,32,60,33,45,45,32,38,97,109,112,59,32,45,45,62,32,38,97,109,112,59,32,60,47,120,109,112,62],"output":[{"Character":{"data":" & <!-- & --> & "}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_7() {
    tokenize(
        r##"{"description":"Incorrect comment ending sequences in RCDATA or RAWTEXT","initialState":"Rcdata","lastStartTag":"xmp","input":"foo<!-- x --x>x-- >x--!>x--<></xmp>","inputUtf16":[102,111,111,60,33,45,45,32,120,32,45,45,120,62,120,45,45,32,62,120,45,45,33,62,120,45,45,60,62,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!-- x --x>x-- >x--!>x--<>"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_8() {
    tokenize(
        r##"{"description":"Incorrect comment ending sequences in RCDATA or RAWTEXT","initialState":"Rawtext","lastStartTag":"xmp","input":"foo<!-- x --x>x-- >x--!>x--<></xmp>","inputUtf16":[102,111,111,60,33,45,45,32,120,32,45,45,120,62,120,45,45,32,62,120,45,45,33,62,120,45,45,60,62,60,47,120,109,112,62],"output":[{"Character":{"data":"foo<!-- x --x>x-- >x--!>x--<>"}},{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}
//</coverage:exclude>
