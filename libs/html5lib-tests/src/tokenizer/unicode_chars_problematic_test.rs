//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+DFFF","initialState":"Data","input":"","inputUtf16":[57343],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"SurrogateInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+D800","initialState":"Data","input":"","inputUtf16":[55296],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"SurrogateInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+DFFF with valid preceding character","initialState":"Data","input":"","inputUtf16":[97,57343],"output":[{"Character":{"data":"a�"}}],"errors":[{"code":"SurrogateInInputStream","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"Invalid Unicode character U+D800 with valid following character","initialState":"Data","input":"","inputUtf16":[55296,97],"output":[{"Character":{"data":"�a"}}],"errors":[{"code":"SurrogateInInputStream","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"CR followed by U+0000","initialState":"Data","input":"\r\u0000","inputUtf16":[13,0],"output":[{"Character":{"data":"\n\u0000"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":2,"column":1}}]}"##,
    );
}
//</coverage:exclude>
