//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"< in attribute name","initialState":"Data","input":"<z/0  <>","inputUtf16":[60,122,47,48,32,32,60,62],"output":[{"StartTag":{"name":"z","attrs":{"0":"","<":""},"self_closing":false}}],"errors":[{"code":"UnexpectedSolidusInTag","location":{"line":1,"column":4}},{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"< in unquoted attribute value","initialState":"Data","input":"<z x=<>","inputUtf16":[60,122,32,120,61,60,62],"output":[{"StartTag":{"name":"z","attrs":{"x":"<"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"= in unquoted attribute value","initialState":"Data","input":"<z z=z=z>","inputUtf16":[60,122,32,122,61,122,61,122,62],"output":[{"StartTag":{"name":"z","attrs":{"z":"z=z"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"= attribute","initialState":"Data","input":"<z =>","inputUtf16":[60,122,32,61,62],"output":[{"StartTag":{"name":"z","attrs":{"=":""},"self_closing":false}}],"errors":[{"code":"UnexpectedEqualsSignBeforeAttributeName","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"== attribute","initialState":"Data","input":"<z ==>","inputUtf16":[60,122,32,61,61,62],"output":[{"StartTag":{"name":"z","attrs":{"=":""},"self_closing":false}}],"errors":[{"code":"UnexpectedEqualsSignBeforeAttributeName","location":{"line":1,"column":4}},{"code":"MissingAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_5() {
    tokenize(
        r##"{"description":"=== attribute","initialState":"Data","input":"<z ===>","inputUtf16":[60,122,32,61,61,61,62],"output":[{"StartTag":{"name":"z","attrs":{"=":"="},"self_closing":false}}],"errors":[{"code":"UnexpectedEqualsSignBeforeAttributeName","location":{"line":1,"column":4}},{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_6() {
    tokenize(
        r##"{"description":"==== attribute","initialState":"Data","input":"<z ====>","inputUtf16":[60,122,32,61,61,61,61,62],"output":[{"StartTag":{"name":"z","attrs":{"=":"=="},"self_closing":false}}],"errors":[{"code":"UnexpectedEqualsSignBeforeAttributeName","location":{"line":1,"column":4}},{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":6}},{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_7() {
    tokenize(
        r##"{"description":"\" after ampersand in double-quoted attribute value","initialState":"Data","input":"<z z=\"&\">","inputUtf16":[60,122,32,122,61,34,38,34,62],"output":[{"StartTag":{"name":"z","attrs":{"z":"&"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_8() {
    tokenize(
        r##"{"description":"' after ampersand in double-quoted attribute value","initialState":"Data","input":"<z z=\"&'\">","inputUtf16":[60,122,32,122,61,34,38,39,34,62],"output":[{"StartTag":{"name":"z","attrs":{"z":"&'"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_9() {
    tokenize(
        r##"{"description":"' after ampersand in single-quoted attribute value","initialState":"Data","input":"<z z='&'>","inputUtf16":[60,122,32,122,61,39,38,39,62],"output":[{"StartTag":{"name":"z","attrs":{"z":"&"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_10() {
    tokenize(
        r##"{"description":"\" after ampersand in single-quoted attribute value","initialState":"Data","input":"<z z='&\"'>","inputUtf16":[60,122,32,122,61,39,38,34,39,62],"output":[{"StartTag":{"name":"z","attrs":{"z":"&\""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_11() {
    tokenize(
        r##"{"description":"Text after bogus character reference","initialState":"Data","input":"<z z='&xlink_xmlns;'>bar<z>","inputUtf16":[60,122,32,122,61,39,38,120,108,105,110,107,95,120,109,108,110,115,59,39,62,98,97,114,60,122,62],"output":[{"StartTag":{"name":"z","attrs":{"z":"&xlink_xmlns;"},"self_closing":false}},{"Character":{"data":"bar"}},{"StartTag":{"name":"z","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_12() {
    tokenize(
        r##"{"description":"Text after hex character reference","initialState":"Data","input":"<z z='&#x0020; foo'>bar<z>","inputUtf16":[60,122,32,122,61,39,38,35,120,48,48,50,48,59,32,102,111,111,39,62,98,97,114,60,122,62],"output":[{"StartTag":{"name":"z","attrs":{"z":"  foo"},"self_closing":false}},{"Character":{"data":"bar"}},{"StartTag":{"name":"z","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_13() {
    tokenize(
        r##"{"description":"Attribute name starting with \"","initialState":"Data","input":"<foo \"='bar'>","inputUtf16":[60,102,111,111,32,34,61,39,98,97,114,39,62],"output":[{"StartTag":{"name":"foo","attrs":{"\"":"bar"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_14() {
    tokenize(
        r##"{"description":"Attribute name starting with '","initialState":"Data","input":"<foo '='bar'>","inputUtf16":[60,102,111,111,32,39,61,39,98,97,114,39,62],"output":[{"StartTag":{"name":"foo","attrs":{"'":"bar"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_15() {
    tokenize(
        r##"{"description":"Attribute name containing \"","initialState":"Data","input":"<foo a\"b='bar'>","inputUtf16":[60,102,111,111,32,97,34,98,61,39,98,97,114,39,62],"output":[{"StartTag":{"name":"foo","attrs":{"a\"b":"bar"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_16() {
    tokenize(
        r##"{"description":"Attribute name containing '","initialState":"Data","input":"<foo a'b='bar'>","inputUtf16":[60,102,111,111,32,97,39,98,61,39,98,97,114,39,62],"output":[{"StartTag":{"name":"foo","attrs":{"a'b":"bar"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInAttributeName","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_17() {
    tokenize(
        r##"{"description":"Unquoted attribute value containing '","initialState":"Data","input":"<foo a=b'c>","inputUtf16":[60,102,111,111,32,97,61,98,39,99,62],"output":[{"StartTag":{"name":"foo","attrs":{"a":"b'c"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_18() {
    tokenize(
        r##"{"description":"Unquoted attribute value containing \"","initialState":"Data","input":"<foo a=b\"c>","inputUtf16":[60,102,111,111,32,97,61,98,34,99,62],"output":[{"StartTag":{"name":"foo","attrs":{"a":"b\"c"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_19() {
    tokenize(
        r##"{"description":"Double-quoted attribute value not followed by whitespace","initialState":"Data","input":"<foo a=\"b\"c>","inputUtf16":[60,102,111,111,32,97,61,34,98,34,99,62],"output":[{"StartTag":{"name":"foo","attrs":{"a":"b","c":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_20() {
    tokenize(
        r##"{"description":"Single-quoted attribute value not followed by whitespace","initialState":"Data","input":"<foo a='b'c>","inputUtf16":[60,102,111,111,32,97,61,39,98,39,99,62],"output":[{"StartTag":{"name":"foo","attrs":{"a":"b","c":""},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_21() {
    tokenize(
        r##"{"description":"Quoted attribute followed by permitted /","initialState":"Data","input":"<br a='b'/>","inputUtf16":[60,98,114,32,97,61,39,98,39,47,62],"output":[{"StartTag":{"name":"br","attrs":{"a":"b"},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_22() {
    tokenize(
        r##"{"description":"Quoted attribute followed by non-permitted /","initialState":"Data","input":"<bar a='b'/>","inputUtf16":[60,98,97,114,32,97,61,39,98,39,47,62],"output":[{"StartTag":{"name":"bar","attrs":{"a":"b"},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_23() {
    tokenize(
        r##"{"description":"CR EOF after doctype name","initialState":"Data","input":"<!doctype html \r","inputUtf16":[60,33,100,111,99,116,121,112,101,32,104,116,109,108,32,13],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_24() {
    tokenize(
        r##"{"description":"CR EOF in tag name","initialState":"Data","input":"<z\r","inputUtf16":[60,122,13],"output":[],"errors":[{"code":"EofInTag","location":{"line":2,"column":1}}]}"##,
    );
}

#[test]
fn test_25() {
    tokenize(
        r##"{"description":"Slash EOF in tag name","initialState":"Data","input":"<z/","inputUtf16":[60,122,47],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_26() {
    tokenize(
        r##"{"description":"Zero hex numeric entity","initialState":"Data","input":"&#x0","inputUtf16":[38,35,120,48],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":5}},{"code":"NullCharacterReference","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_27() {
    tokenize(
        r##"{"description":"Zero decimal numeric entity","initialState":"Data","input":"&#0","inputUtf16":[38,35,48],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":4}},{"code":"NullCharacterReference","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_28() {
    tokenize(
        r##"{"description":"Zero-prefixed hex numeric entity","initialState":"Data","input":"&#x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041;","inputUtf16":[38,35,120,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,52,49,59],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_29() {
    tokenize(
        r##"{"description":"Zero-prefixed decimal numeric entity","initialState":"Data","input":"&#000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000065;","inputUtf16":[38,35,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,54,53,59],"output":[{"Character":{"data":"A"}}],"errors":[]}"##,
    );
}

#[test]
fn test_30() {
    tokenize(
        r##"{"description":"Empty hex numeric entities","initialState":"Data","input":"&#x &#X ","inputUtf16":[38,35,120,32,38,35,88,32],"output":[{"Character":{"data":"&#x &#X "}}],"errors":[{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":4}},{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_31() {
    tokenize(
        r##"{"description":"Invalid digit in hex numeric entity","initialState":"Data","input":"&#xZ","inputUtf16":[38,35,120,90],"output":[{"Character":{"data":"&#xZ"}}],"errors":[{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_32() {
    tokenize(
        r##"{"description":"Empty decimal numeric entities","initialState":"Data","input":"&# &#; ","inputUtf16":[38,35,32,38,35,59,32],"output":[{"Character":{"data":"&# &#; "}}],"errors":[{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":3}},{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_33() {
    tokenize(
        r##"{"description":"Invalid digit in decimal numeric entity","initialState":"Data","input":"&#A","inputUtf16":[38,35,65],"output":[{"Character":{"data":"&#A"}}],"errors":[{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_34() {
    tokenize(
        r##"{"description":"Non-BMP numeric entity","initialState":"Data","input":"&#x10000;","inputUtf16":[38,35,120,49,48,48,48,48,59],"output":[{"Character":{"data":"𐀀"}}],"errors":[]}"##,
    );
}

#[test]
fn test_35() {
    tokenize(
        r##"{"description":"Maximum non-BMP numeric entity","initialState":"Data","input":"&#X10FFFF;","inputUtf16":[38,35,88,49,48,70,70,70,70,59],"output":[{"Character":{"data":"􏿿"}}],"errors":[{"code":"NoncharacterCharacterReference","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_36() {
    tokenize(
        r##"{"description":"Above maximum numeric entity","initialState":"Data","input":"&#x110000;","inputUtf16":[38,35,120,49,49,48,48,48,48,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_37() {
    tokenize(
        r##"{"description":"32-bit hex numeric entity","initialState":"Data","input":"&#x80000041;","inputUtf16":[38,35,120,56,48,48,48,48,48,52,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_38() {
    tokenize(
        r##"{"description":"33-bit hex numeric entity","initialState":"Data","input":"&#x100000041;","inputUtf16":[38,35,120,49,48,48,48,48,48,48,52,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_39() {
    tokenize(
        r##"{"description":"33-bit decimal numeric entity","initialState":"Data","input":"&#4294967361;","inputUtf16":[38,35,52,50,57,52,57,54,55,51,54,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_40() {
    tokenize(
        r##"{"description":"65-bit hex numeric entity","initialState":"Data","input":"&#x10000000000000041;","inputUtf16":[38,35,120,49,48,48,48,48,48,48,48,48,48,48,48,48,48,48,52,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":22}}]}"##,
    );
}

#[test]
fn test_41() {
    tokenize(
        r##"{"description":"65-bit decimal numeric entity","initialState":"Data","input":"&#18446744073709551681;","inputUtf16":[38,35,49,56,52,52,54,55,52,52,48,55,51,55,48,57,53,53,49,54,56,49,59],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"CharacterReferenceOutsideUnicodeRange","location":{"line":1,"column":24}}]}"##,
    );
}

#[test]
fn test_42() {
    tokenize(
        r##"{"description":"Surrogate code point edge cases","initialState":"Data","input":"&#xD7FF;&#xD800;&#xD801;&#xDFFE;&#xDFFF;&#xE000;","inputUtf16":[38,35,120,68,55,70,70,59,38,35,120,68,56,48,48,59,38,35,120,68,56,48,49,59,38,35,120,68,70,70,69,59,38,35,120,68,70,70,70,59,38,35,120,69,48,48,48,59],"output":[{"Character":{"data":"퟿����"}}],"errors":[{"code":"SurrogateCharacterReference","location":{"line":1,"column":17}},{"code":"SurrogateCharacterReference","location":{"line":1,"column":25}},{"code":"SurrogateCharacterReference","location":{"line":1,"column":33}},{"code":"SurrogateCharacterReference","location":{"line":1,"column":41}}]}"##,
    );
}

#[test]
fn test_43() {
    tokenize(
        r##"{"description":"Uppercase start tag name","initialState":"Data","input":"<X>","inputUtf16":[60,88,62],"output":[{"StartTag":{"name":"x","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_44() {
    tokenize(
        r##"{"description":"Uppercase end tag name","initialState":"Data","input":"</X>","inputUtf16":[60,47,88,62],"output":[{"EndTag":{"name":"x"}}],"errors":[]}"##,
    );
}

#[test]
fn test_45() {
    tokenize(
        r##"{"description":"Uppercase attribute name","initialState":"Data","input":"<x X>","inputUtf16":[60,120,32,88,62],"output":[{"StartTag":{"name":"x","attrs":{"x":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_46() {
    tokenize(
        r##"{"description":"Tag/attribute name case edge values","initialState":"Data","input":"<x@AZ[`az{ @AZ[`az{>","inputUtf16":[60,120,64,65,90,91,96,97,122,123,32,64,65,90,91,96,97,122,123,62],"output":[{"StartTag":{"name":"x@az[`az{","attrs":{"@az[`az{":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_47() {
    tokenize(
        r##"{"description":"Duplicate different-case attributes","initialState":"Data","input":"<x x=1 x=2 X=3>","inputUtf16":[60,120,32,120,61,49,32,120,61,50,32,88,61,51,62],"output":[{"StartTag":{"name":"x","attrs":{"x":"1"},"self_closing":false}}],"errors":[{"code":"DuplicateAttribute","location":{"line":1,"column":9}},{"code":"DuplicateAttribute","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_48() {
    tokenize(
        r##"{"description":"Uppercase close tag attributes","initialState":"Data","input":"</x X>","inputUtf16":[60,47,120,32,88,62],"output":[{"EndTag":{"name":"x"}}],"errors":[{"code":"EndTagWithAttributes","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_49() {
    tokenize(
        r##"{"description":"Duplicate close tag attributes","initialState":"Data","input":"</x x x>","inputUtf16":[60,47,120,32,120,32,120,62],"output":[{"EndTag":{"name":"x"}}],"errors":[{"code":"DuplicateAttribute","location":{"line":1,"column":8}},{"code":"EndTagWithAttributes","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_50() {
    tokenize(
        r##"{"description":"Permitted slash","initialState":"Data","input":"<br/>","inputUtf16":[60,98,114,47,62],"output":[{"StartTag":{"name":"br","attrs":{},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_51() {
    tokenize(
        r##"{"description":"Non-permitted slash","initialState":"Data","input":"<xr/>","inputUtf16":[60,120,114,47,62],"output":[{"StartTag":{"name":"xr","attrs":{},"self_closing":true}}],"errors":[]}"##,
    );
}

#[test]
fn test_52() {
    tokenize(
        r##"{"description":"Permitted slash but in close tag","initialState":"Data","input":"</br/>","inputUtf16":[60,47,98,114,47,62],"output":[{"EndTag":{"name":"br"}}],"errors":[{"code":"EndTagWithTrailingSolidus","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_53() {
    tokenize(
        r##"{"description":"Doctype public case-sensitivity (1)","initialState":"Data","input":"<!DoCtYpE HtMl PuBlIc \"AbC\" \"XyZ\">","inputUtf16":[60,33,68,111,67,116,89,112,69,32,72,116,77,108,32,80,117,66,108,73,99,32,34,65,98,67,34,32,34,88,121,90,34,62],"output":[{"Doctype":{"name":"html","public_id":"AbC","system_id":"XyZ","force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_54() {
    tokenize(
        r##"{"description":"Doctype public case-sensitivity (2)","initialState":"Data","input":"<!dOcTyPe hTmL pUbLiC \"aBc\" \"xYz\">","inputUtf16":[60,33,100,79,99,84,121,80,101,32,104,84,109,76,32,112,85,98,76,105,67,32,34,97,66,99,34,32,34,120,89,122,34,62],"output":[{"Doctype":{"name":"html","public_id":"aBc","system_id":"xYz","force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_55() {
    tokenize(
        r##"{"description":"Doctype system case-sensitivity (1)","initialState":"Data","input":"<!DoCtYpE HtMl SyStEm \"XyZ\">","inputUtf16":[60,33,68,111,67,116,89,112,69,32,72,116,77,108,32,83,121,83,116,69,109,32,34,88,121,90,34,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":"XyZ","force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_56() {
    tokenize(
        r##"{"description":"Doctype system case-sensitivity (2)","initialState":"Data","input":"<!dOcTyPe hTmL sYsTeM \"xYz\">","inputUtf16":[60,33,100,79,99,84,121,80,101,32,104,84,109,76,32,115,89,115,84,101,77,32,34,120,89,122,34,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":"xYz","force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_57() {
    tokenize(
        r##"{"description":"U+0000 in lookahead region after non-matching character","initialState":"Data","input":"<!doc>\u0000","inputUtf16":[60,33,100,111,99,62,0],"output":[{"Comment":{"data":"doc"}},{"Character":{"data":"\u0000"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_58() {
    tokenize(
        r##"{"description":"U+0000 in lookahead region","initialState":"Data","input":"<!doc\u0000","inputUtf16":[60,33,100,111,99,0],"output":[{"Comment":{"data":"doc�"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_59() {
    tokenize(
        r##"{"description":"U+0080 in lookahead region","initialState":"Data","input":"<!doc","inputUtf16":[60,33,100,111,99,128],"output":[{"Comment":{"data":"doc"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"ControlCharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_60() {
    tokenize(
        r##"{"description":"U+FDD1 in lookahead region","initialState":"Data","input":"<!doc﷑","inputUtf16":[60,33,100,111,99,64977],"output":[{"Comment":{"data":"doc﷑"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"NoncharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_61() {
    tokenize(
        r##"{"description":"U+1FFFF in lookahead region","initialState":"Data","input":"<!doc🿿","inputUtf16":[60,33,100,111,99,55359,57343],"output":[{"Comment":{"data":"doc🿿"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}},{"code":"NoncharacterInInputStream","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_62() {
    tokenize(
        r##"{"description":"CR followed by non-LF","initialState":"Data","input":"\r?","inputUtf16":[13,63],"output":[{"Character":{"data":"\n?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_63() {
    tokenize(
        r##"{"description":"CR at EOF","initialState":"Data","input":"\r","inputUtf16":[13],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_64() {
    tokenize(
        r##"{"description":"LF at EOF","initialState":"Data","input":"\n","inputUtf16":[10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_65() {
    tokenize(
        r##"{"description":"CR LF","initialState":"Data","input":"\r\n","inputUtf16":[13,10],"output":[{"Character":{"data":"\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_66() {
    tokenize(
        r##"{"description":"CR CR","initialState":"Data","input":"\r\r","inputUtf16":[13,13],"output":[{"Character":{"data":"\n\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_67() {
    tokenize(
        r##"{"description":"LF LF","initialState":"Data","input":"\n\n","inputUtf16":[10,10],"output":[{"Character":{"data":"\n\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_68() {
    tokenize(
        r##"{"description":"LF CR","initialState":"Data","input":"\n\r","inputUtf16":[10,13],"output":[{"Character":{"data":"\n\n"}}],"errors":[]}"##,
    );
}

#[test]
fn test_69() {
    tokenize(
        r##"{"description":"text CR CR CR text","initialState":"Data","input":"text\r\r\rtext","inputUtf16":[116,101,120,116,13,13,13,116,101,120,116],"output":[{"Character":{"data":"text\n\n\ntext"}}],"errors":[]}"##,
    );
}

#[test]
fn test_70() {
    tokenize(
        r##"{"description":"Doctype publik","initialState":"Data","input":"<!DOCTYPE html PUBLIK \"AbC\" \"XyZ\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73,75,32,34,65,98,67,34,32,34,88,121,90,34,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_71() {
    tokenize(
        r##"{"description":"Doctype publi","initialState":"Data","input":"<!DOCTYPE html PUBLI","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,80,85,66,76,73],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_72() {
    tokenize(
        r##"{"description":"Doctype sistem","initialState":"Data","input":"<!DOCTYPE html SISTEM \"AbC\">","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,83,73,83,84,69,77,32,34,65,98,67,34,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_73() {
    tokenize(
        r##"{"description":"Doctype sys","initialState":"Data","input":"<!DOCTYPE html SYS","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,83,89,83],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_74() {
    tokenize(
        r##"{"description":"Doctype html x>text","initialState":"Data","input":"<!DOCTYPE html x>text","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32,120,62,116,101,120,116],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}},{"Character":{"data":"text"}}],"errors":[{"code":"InvalidCharacterSequenceAfterDoctypeName","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_75() {
    tokenize(
        r##"{"description":"Grave accent in unquoted attribute","initialState":"Data","input":"<a a=aa`>","inputUtf16":[60,97,32,97,61,97,97,96,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"aa`"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_76() {
    tokenize(
        r##"{"description":"EOF in tag name state ","initialState":"Data","input":"<a","inputUtf16":[60,97],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_77() {
    tokenize(
        r##"{"description":"EOF in before attribute name state","initialState":"Data","input":"<a ","inputUtf16":[60,97,32],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_78() {
    tokenize(
        r##"{"description":"EOF in attribute name state","initialState":"Data","input":"<a a","inputUtf16":[60,97,32,97],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_79() {
    tokenize(
        r##"{"description":"EOF in after attribute name state","initialState":"Data","input":"<a a ","inputUtf16":[60,97,32,97,32],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_80() {
    tokenize(
        r##"{"description":"EOF in before attribute value state","initialState":"Data","input":"<a a =","inputUtf16":[60,97,32,97,32,61],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_81() {
    tokenize(
        r##"{"description":"EOF in attribute value (double quoted) state","initialState":"Data","input":"<a a =\"a","inputUtf16":[60,97,32,97,32,61,34,97],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_82() {
    tokenize(
        r##"{"description":"EOF in attribute value (single quoted) state","initialState":"Data","input":"<a a ='a","inputUtf16":[60,97,32,97,32,61,39,97],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_83() {
    tokenize(
        r##"{"description":"EOF in attribute value (unquoted) state","initialState":"Data","input":"<a a =a","inputUtf16":[60,97,32,97,32,61,97],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_84() {
    tokenize(
        r##"{"description":"EOF in after attribute value state","initialState":"Data","input":"<a a ='a'","inputUtf16":[60,97,32,97,32,61,39,97,39],"output":[],"errors":[{"code":"EofInTag","location":{"line":1,"column":10}}]}"##,
    );
}
//</coverage:exclude>
