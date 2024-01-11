mod helper;

use helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+DFFF","initialState":"Data","input":"","inputUtf16":[57343],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"surrogate-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0001() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+D800","initialState":"Data","input":"","inputUtf16":[55296],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"surrogate-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0002() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+DFFF with valid preceding character","initialState":"Data","input":"","inputUtf16":[97,57343],"output":[{"Character":{"data":"a�"}}],"errors":[{"code":"surrogate-in-input-stream","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0003() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+D800 with valid following character","initialState":"Data","input":"","inputUtf16":[55296,97],"output":[{"Character":{"data":"�a"}}],"errors":[{"code":"surrogate-in-input-stream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0004() {
    tokenize(
        r##"{"description":"CR followed by U+0000","initialState":"Data","input":"\r\u0000","inputUtf16":[13,0],"output":[{"Character":{"data":"\n\u0000"}}],"errors":[{"code":"unexpected-null-character","location":{"line":2,"column":1}}]}"##,
    );
}
