mod helper;

use helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE without name","initialState":"Data","input":"<!DOCTYPE>","inputUtf16":[60,33,68,79,67,84,89,80,69,62],"output":[{"Doctype":{"name":null,"public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"missing-doctype-name","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0001() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE without space before name","initialState":"Data","input":"<!DOCTYPEhtml>","inputUtf16":[60,33,68,79,67,84,89,80,69,104,116,109,108,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[{"code":"missing-whitespace-before-doctype-name","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0002() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Incorrect DOCTYPE without a space before name","initialState":"Data","input":"<!DOCTYPEfoo>","inputUtf16":[60,33,68,79,67,84,89,80,69,102,111,111,62],"output":[{"Doctype":{"name":"foo","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[{"code":"missing-whitespace-before-doctype-name","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0003() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with publicId","initialState":"Data","input":"<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML Transitional 4.01//EN\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,34,45,47,47,87,51,67,47,47,68,84,68,32,72,84,77,76,32,84,114,97,110,115,105,116,105,111,110,97,108,32,52,46,48,49,47,47,69,78,34,62],"output":[{"Doctype":{"name":"html","public_id":"-//W3C//DTD HTML Transitional 4.01//EN","system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0004() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with EOF after PUBLIC","initialState":"Data","input":"<!DOCTYPE html PUBLIC","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"eof-in-doctype","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_0005() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with EOF after PUBLIC '","initialState":"Data","input":"<!DOCTYPE html PUBLIC '","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,39],"output":[{"Doctype":{"name":"html","public_id":"","system_id":null,"force_quirks":true}}],"errors":[{"code":"eof-in-doctype","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_0006() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with EOF after PUBLIC 'x","initialState":"Data","input":"<!DOCTYPE html PUBLIC 'x","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,39,120],"output":[{"Doctype":{"name":"html","public_id":"x","system_id":null,"force_quirks":true}}],"errors":[{"code":"eof-in-doctype","location":{"line":1,"column":25}}]}"##,
    );
}

#[test]
fn test_0007() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with systemId","initialState":"Data","input":"<!DOCTYPE html SYSTEM \"-//W3C//DTD HTML Transitional 4.01//EN\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,83,89,83,84,69,77,32,34,45,47,47,87,51,67,47,47,68,84,68,32,72,84,77,76,32,84,114,97,110,115,105,116,105,111,110,97,108,32,52,46,48,49,47,47,69,78,34,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":"-//W3C//DTD HTML Transitional 4.01//EN","force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0008() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with single-quoted systemId","initialState":"Data","input":"<!DOCTYPE html SYSTEM '-//W3C//DTD HTML Transitional 4.01//EN'>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,83,89,83,84,69,77,32,39,45,47,47,87,51,67,47,47,68,84,68,32,72,84,77,76,32,84,114,97,110,115,105,116,105,111,110,97,108,32,52,46,48,49,47,47,69,78,39,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":"-//W3C//DTD HTML Transitional 4.01//EN","force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0009() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with publicId and systemId","initialState":"Data","input":"<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML Transitional 4.01//EN\" \"-//W3C//DTD HTML Transitional 4.01//EN\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,34,45,47,47,87,51,67,47,47,68,84,68,32,72,84,77,76,32,84,114,97,110,115,105,116,105,111,110,97,108,32,52,46,48,49,47,47,69,78,34,32,34,45,47,47,87,51,67,47,47,68,84,68,32,72,84,77,76,32,84,114,97,110,115,105,116,105,111,110,97,108,32,52,46,48,49,47,47,69,78,34,62],"output":[{"Doctype":{"name":"html","public_id":"-//W3C//DTD HTML Transitional 4.01//EN","system_id":"-//W3C//DTD HTML Transitional 4.01//EN","force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0010() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with > in double-quoted publicId","initialState":"Data","input":"<!DOCTYPE html PUBLIC \">x","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,34,62,120],"output":[{"Doctype":{"name":"html","public_id":"","system_id":null,"force_quirks":true}},{"Character":{"data":"x"}}],"errors":[{"code":"abrupt-doctype-public-identifier","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_0011() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with > in single-quoted publicId","initialState":"Data","input":"<!DOCTYPE html PUBLIC '>x","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,39,62,120],"output":[{"Doctype":{"name":"html","public_id":"","system_id":null,"force_quirks":true}},{"Character":{"data":"x"}}],"errors":[{"code":"abrupt-doctype-public-identifier","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_0012() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with > in double-quoted systemId","initialState":"Data","input":"<!DOCTYPE html PUBLIC \"foo\" \">x","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,34,102,111,111,34,32,34,62,120],"output":[{"Doctype":{"name":"html","public_id":"foo","system_id":"","force_quirks":true}},{"Character":{"data":"x"}}],"errors":[{"code":"abrupt-doctype-system-identifier","location":{"line":1,"column":30}}]}"##,
    );
}

#[test]
fn test_0013() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"DOCTYPE with > in single-quoted systemId","initialState":"Data","input":"<!DOCTYPE html PUBLIC 'foo' '>x","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,67,32,39,102,111,111,39,32,39,62,120],"output":[{"Doctype":{"name":"html","public_id":"foo","system_id":"","force_quirks":true}},{"Character":{"data":"x"}}],"errors":[{"code":"abrupt-doctype-system-identifier","location":{"line":1,"column":30}}]}"##,
    );
}

#[test]
fn test_0014() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Incomplete doctype","initialState":"Data","input":"<!DOCTYPE html ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"eof-in-doctype","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_0015() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Numeric entity representing the NUL character","initialState":"Data","input":"&#0000;","inputUtf16":[38,35,48,48,48,48,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"null-character-reference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0016() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Hexadecimal entity representing the NUL character","initialState":"Data","input":"&#x0000;","inputUtf16":[38,35,120,48,48,48,48,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"null-character-reference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_0017() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Numeric entity representing a codepoint after 1114111 (U+10FFFF)","initialState":"Data","input":"&#2225222;","inputUtf16":[38,35,50,50,50,53,50,50,50,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0018() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Hexadecimal entity representing a codepoint after 1114111 (U+10FFFF)","initialState":"Data","input":"&#x1010FFFF;","inputUtf16":[38,35,120,49,48,49,48,70,70,70,70,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"character-reference-outside-unicode-range","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_0019() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Hexadecimal entity pair representing a surrogate pair","initialState":"Data","input":"&#xD869;&#xDED6;","inputUtf16":[38,35,120,68,56,54,57,59,38,35,120,68,69,68,54,59],"output":[{"Character":{"data":"��"}}],"errors":[{"code":"surrogate-character-reference","location":{"line":1,"column":9}},{"code":"surrogate-character-reference","location":{"line":1,"column":17}}]}"##,
    );
}

#[test]
fn test_0020() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Hexadecimal entity with mixed uppercase and lowercase","initialState":"Data","input":"&#xaBcD;","inputUtf16":[38,35,120,97,66,99,68,59],"output":[{"Character":{"data":"ꯍ"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0021() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Entity without a name","initialState":"Data","input":"&;","inputUtf16":[38,59],"output":[{"Character":{"data":"&;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0022() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Unescaped ampersand in attribute value","initialState":"Data","input":"<h a='&'>","inputUtf16":[60,104,32,97,61,39,38,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0023() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"StartTag containing <","initialState":"Data","input":"<a<b>","inputUtf16":[60,97,60,98,62],"output":[{"StartTag":{"name":"a<b","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0024() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Non-void element containing trailing /","initialState":"Data","input":"<h/>","inputUtf16":[60,104,47,62],"output":[{"StartTag":{"name":"h","attrs":{},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_0025() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Void element with permitted slash","initialState":"Data","input":"<br/>","inputUtf16":[60,98,114,47,62],"output":[{"StartTag":{"name":"br","attrs":{},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_0026() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Void element with permitted slash (with attribute)","initialState":"Data","input":"<br foo='bar'/>","inputUtf16":[60,98,114,32,102,111,111,61,39,98,97,114,39,47,62],"output":[{"StartTag":{"name":"br","attrs":{"foo":"bar"},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_0027() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"StartTag containing /","initialState":"Data","input":"<h/a='b'>","inputUtf16":[60,104,47,97,61,39,98,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"b"},"self_closing":false}}],"errors":[{"code":"unexpected-solidus-in-tag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_0028() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Double-quoted attribute value","initialState":"Data","input":"<h a=\"b\">","inputUtf16":[60,104,32,97,61,34,98,34,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"b"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0029() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Unescaped </","initialState":"Data","input":"</","inputUtf16":[60,47],"output":[{"Character":{"data":"</"}}],"errors":[{"code":"eof-before-tag-name","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0030() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Illegal end tag name","initialState":"Data","input":"</1>","inputUtf16":[60,47,49,62],"output":[{"Comment":{"data":"1"}}],"errors":[{"code":"invalid-first-character-of-tag-name","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_0031() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Simili processing instruction","initialState":"Data","input":"<?namespace>","inputUtf16":[60,63,110,97,109,101,115,112,97,99,101,62],"output":[{"Comment":{"data":"?namespace"}}],"errors":[{"code":"unexpected-question-mark-instead-of-tag-name","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0032() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"A bogus comment stops at >, even if preceded by two dashes","initialState":"Data","input":"<?foo-->","inputUtf16":[60,63,102,111,111,45,45,62],"output":[{"Comment":{"data":"?foo--"}}],"errors":[{"code":"unexpected-question-mark-instead-of-tag-name","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_0033() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Unescaped <","initialState":"Data","input":"foo < bar","inputUtf16":[102,111,111,32,60,32,98,97,114],"output":[{"Character":{"data":"foo < bar"}}],"errors":[{"code":"invalid-first-character-of-tag-name","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0034() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Null Byte Replacement","initialState":"Data","input":"\u0000","inputUtf16":[0],"output":[{"Character":{"data":"\u0000"}}],"errors":[{"code":"unexpected-null-character","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_0035() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Comment with dash","initialState":"Data","input":"<!---x","inputUtf16":[60,33,45,45,45,120],"output":[{"Comment":{"data":"-x"}}],"errors":[{"code":"eof-in-comment","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0036() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Entity + newline","initialState":"Data","input":"\nx\n&gt;\n","inputUtf16":[10,120,10,38,103,116,59,10],"output":[{"Character":{"data":"\nx\n>\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_0037() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Start tag with no attributes but space before the greater-than sign","initialState":"Data","input":"<h >","inputUtf16":[60,104,32,62],"output":[{"StartTag":{"name":"h","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0038() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Empty attribute followed by uppercase attribute","initialState":"Data","input":"<h a B=''>","inputUtf16":[60,104,32,97,32,66,61,39,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"","b":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0039() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Double-quote after attribute name","initialState":"Data","input":"<h a \">","inputUtf16":[60,104,32,97,32,34,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"","\"":""},"self_closing":false}}],"errors":[{"code":"unexpected-character-in-attribute-name","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0040() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Single-quote after attribute name","initialState":"Data","input":"<h a '>","inputUtf16":[60,104,32,97,32,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"","'":""},"self_closing":false}}],"errors":[{"code":"unexpected-character-in-attribute-name","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_0041() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Empty end tag with following characters","initialState":"Data","input":"a</>bc","inputUtf16":[97,60,47,62,98,99],"output":[{"Character":{"data":"abc"}}],"errors":[{"code":"missing-end-tag-name","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_0042() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Empty end tag with following tag","initialState":"Data","input":"a</><b>c","inputUtf16":[97,60,47,62,60,98,62,99],"output":[{"Character":{"data":"a"}},{"StartTag":{"name":"b","attrs":{},"self_closing":false}},{"Character":{"data":"c"}}],"errors":[{"code":"missing-end-tag-name","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_0043() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Empty end tag with following comment","initialState":"Data","input":"a</><!--b-->c","inputUtf16":[97,60,47,62,60,33,45,45,98,45,45,62,99],"output":[{"Character":{"data":"a"}},{"Comment":{"data":"b"}},{"Character":{"data":"c"}}],"errors":[{"code":"missing-end-tag-name","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_0044() {
    tokenize(
        #[allow(clippy::invisible_characters)]
        r##"{"description":"Empty end tag with following end tag","initialState":"Data","input":"a</></b>c","inputUtf16":[97,60,47,62,60,47,98,62,99],"output":[{"Character":{"data":"a"}},{"EndTag":{"name":"b"}},{"Character":{"data":"c"}}],"errors":[{"code":"missing-end-tag-name","location":{"line":1,"column":4}}]}"##,
    );
}
