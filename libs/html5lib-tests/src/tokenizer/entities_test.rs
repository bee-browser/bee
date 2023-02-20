//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        r##"{"description":"Undefined named entity in a double-quoted attribute value ending in semicolon and whose name starts with a known entity name.","initialState":"Data","input":"<h a=\"&noti;\">","inputUtf16":[60,104,32,97,61,34,38,110,111,116,105,59,34,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&noti;"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0001() {
    tokenize(
        r##"{"description":"Entity name requiring semicolon instead followed by the equals sign in a double-quoted attribute value.","initialState":"Data","input":"<h a=\"&lang=\">","inputUtf16":[60,104,32,97,61,34,38,108,97,110,103,61,34,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&lang="},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0002() {
    tokenize(
        r##"{"description":"Valid entity name followed by the equals sign in a double-quoted attribute value.","initialState":"Data","input":"<h a=\"&not=\">","inputUtf16":[60,104,32,97,61,34,38,110,111,116,61,34,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&not="},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0003() {
    tokenize(
        r##"{"description":"Undefined named entity in a single-quoted attribute value ending in semicolon and whose name starts with a known entity name.","initialState":"Data","input":"<h a='&noti;'>","inputUtf16":[60,104,32,97,61,39,38,110,111,116,105,59,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&noti;"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0004() {
    tokenize(
        r##"{"description":"Entity name requiring semicolon instead followed by the equals sign in a single-quoted attribute value.","initialState":"Data","input":"<h a='&lang='>","inputUtf16":[60,104,32,97,61,39,38,108,97,110,103,61,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&lang="},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0005() {
    tokenize(
        r##"{"description":"Valid entity name followed by the equals sign in a single-quoted attribute value.","initialState":"Data","input":"<h a='&not='>","inputUtf16":[60,104,32,97,61,39,38,110,111,116,61,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&not="},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0006() {
    tokenize(
        r##"{"description":"Undefined named entity in an unquoted attribute value ending in semicolon and whose name starts with a known entity name.","initialState":"Data","input":"<h a=&noti;>","inputUtf16":[60,104,32,97,61,38,110,111,116,105,59,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&noti;"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_0007() {
    tokenize(
        r##"{"description":"Entity name requiring semicolon instead followed by the equals sign in an unquoted attribute value.","initialState":"Data","input":"<h a=&lang=>","inputUtf16":[60,104,32,97,61,38,108,97,110,103,61,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&lang="},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_0008() {
    tokenize(
        r##"{"description":"Valid entity name followed by the equals sign in an unquoted attribute value.","initialState":"Data","input":"<h a=&not=>","inputUtf16":[60,104,32,97,61,38,110,111,116,61,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&not="},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_0009() {
    tokenize(
        r##"{"description":"Ambiguous ampersand.","initialState":"Data","input":"&rrrraannddom;","inputUtf16":[38,114,114,114,114,97,97,110,110,100,100,111,109,59],"output":[{"Character":{"data":"&rrrraannddom;"}}],"errors":[{"code":"UnknownNamedCharacterReference","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_0010() {
    tokenize(
        r##"{"description":"Semicolonless named entity 'not' followed by 'i;' in body","initialState":"Data","input":"&noti;","inputUtf16":[38,110,111,116,105,59],"output":[{"Character":{"data":"¬i;"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_0011() {
    tokenize(
        r##"{"description":"Very long undefined named entity in body","initialState":"Data","input":"&ammmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmp;","inputUtf16":[38,97,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,109,112,59],"output":[{"Character":{"data":"&ammmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmp;"}}],"errors":[{"code":"UnknownNamedCharacterReference","location":{"line":1,"column":950}}]}"##,
    );
}

#[test]
fn test_0012() {
    tokenize(
        r##"{"description":"CR as numeric entity","initialState":"Data","input":"&#013;","inputUtf16":[38,35,48,49,51,59],"output":[{"Character":{"data":"\r"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":7}}]}"##,
    );
}

#[test]
fn test_0013() {
    tokenize(
        r##"{"description":"CR as hexadecimal numeric entity","initialState":"Data","input":"&#x00D;","inputUtf16":[38,35,120,48,48,68,59],"output":[{"Character":{"data":"\r"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0014() {
    tokenize(
        r##"{"description":"Windows-1252 EURO SIGN numeric entity.","initialState":"Data","input":"&#0128;","inputUtf16":[38,35,48,49,50,56,59],"output":[{"Character":{"data":"€"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0015() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR numeric entity.","initialState":"Data","input":"&#0129;","inputUtf16":[38,35,48,49,50,57,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0016() {
    tokenize(
        r##"{"description":"Windows-1252 SINGLE LOW-9 QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0130;","inputUtf16":[38,35,48,49,51,48,59],"output":[{"Character":{"data":"‚"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0017() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN SMALL LETTER F WITH HOOK numeric entity.","initialState":"Data","input":"&#0131;","inputUtf16":[38,35,48,49,51,49,59],"output":[{"Character":{"data":"ƒ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0018() {
    tokenize(
        r##"{"description":"Windows-1252 DOUBLE LOW-9 QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0132;","inputUtf16":[38,35,48,49,51,50,59],"output":[{"Character":{"data":"„"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0019() {
    tokenize(
        r##"{"description":"Windows-1252 HORIZONTAL ELLIPSIS numeric entity.","initialState":"Data","input":"&#0133;","inputUtf16":[38,35,48,49,51,51,59],"output":[{"Character":{"data":"…"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0020() {
    tokenize(
        r##"{"description":"Windows-1252 DAGGER numeric entity.","initialState":"Data","input":"&#0134;","inputUtf16":[38,35,48,49,51,52,59],"output":[{"Character":{"data":"†"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0021() {
    tokenize(
        r##"{"description":"Windows-1252 DOUBLE DAGGER numeric entity.","initialState":"Data","input":"&#0135;","inputUtf16":[38,35,48,49,51,53,59],"output":[{"Character":{"data":"‡"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0022() {
    tokenize(
        r##"{"description":"Windows-1252 MODIFIER LETTER CIRCUMFLEX ACCENT numeric entity.","initialState":"Data","input":"&#0136;","inputUtf16":[38,35,48,49,51,54,59],"output":[{"Character":{"data":"ˆ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0023() {
    tokenize(
        r##"{"description":"Windows-1252 PER MILLE SIGN numeric entity.","initialState":"Data","input":"&#0137;","inputUtf16":[38,35,48,49,51,55,59],"output":[{"Character":{"data":"‰"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0024() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN CAPITAL LETTER S WITH CARON numeric entity.","initialState":"Data","input":"&#0138;","inputUtf16":[38,35,48,49,51,56,59],"output":[{"Character":{"data":"Š"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0025() {
    tokenize(
        r##"{"description":"Windows-1252 SINGLE LEFT-POINTING ANGLE QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0139;","inputUtf16":[38,35,48,49,51,57,59],"output":[{"Character":{"data":"‹"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0026() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN CAPITAL LIGATURE OE numeric entity.","initialState":"Data","input":"&#0140;","inputUtf16":[38,35,48,49,52,48,59],"output":[{"Character":{"data":"Œ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0027() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR numeric entity.","initialState":"Data","input":"&#0141;","inputUtf16":[38,35,48,49,52,49,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0028() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN CAPITAL LETTER Z WITH CARON numeric entity.","initialState":"Data","input":"&#0142;","inputUtf16":[38,35,48,49,52,50,59],"output":[{"Character":{"data":"Ž"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0029() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR numeric entity.","initialState":"Data","input":"&#0143;","inputUtf16":[38,35,48,49,52,51,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0030() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR numeric entity.","initialState":"Data","input":"&#0144;","inputUtf16":[38,35,48,49,52,52,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0031() {
    tokenize(
        r##"{"description":"Windows-1252 LEFT SINGLE QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0145;","inputUtf16":[38,35,48,49,52,53,59],"output":[{"Character":{"data":"‘"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0032() {
    tokenize(
        r##"{"description":"Windows-1252 RIGHT SINGLE QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0146;","inputUtf16":[38,35,48,49,52,54,59],"output":[{"Character":{"data":"’"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0033() {
    tokenize(
        r##"{"description":"Windows-1252 LEFT DOUBLE QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0147;","inputUtf16":[38,35,48,49,52,55,59],"output":[{"Character":{"data":"“"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0034() {
    tokenize(
        r##"{"description":"Windows-1252 RIGHT DOUBLE QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0148;","inputUtf16":[38,35,48,49,52,56,59],"output":[{"Character":{"data":"”"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0035() {
    tokenize(
        r##"{"description":"Windows-1252 BULLET numeric entity.","initialState":"Data","input":"&#0149;","inputUtf16":[38,35,48,49,52,57,59],"output":[{"Character":{"data":"•"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0036() {
    tokenize(
        r##"{"description":"Windows-1252 EN DASH numeric entity.","initialState":"Data","input":"&#0150;","inputUtf16":[38,35,48,49,53,48,59],"output":[{"Character":{"data":"–"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0037() {
    tokenize(
        r##"{"description":"Windows-1252 EM DASH numeric entity.","initialState":"Data","input":"&#0151;","inputUtf16":[38,35,48,49,53,49,59],"output":[{"Character":{"data":"—"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0038() {
    tokenize(
        r##"{"description":"Windows-1252 SMALL TILDE numeric entity.","initialState":"Data","input":"&#0152;","inputUtf16":[38,35,48,49,53,50,59],"output":[{"Character":{"data":"˜"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0039() {
    tokenize(
        r##"{"description":"Windows-1252 TRADE MARK SIGN numeric entity.","initialState":"Data","input":"&#0153;","inputUtf16":[38,35,48,49,53,51,59],"output":[{"Character":{"data":"™"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0040() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN SMALL LETTER S WITH CARON numeric entity.","initialState":"Data","input":"&#0154;","inputUtf16":[38,35,48,49,53,52,59],"output":[{"Character":{"data":"š"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0041() {
    tokenize(
        r##"{"description":"Windows-1252 SINGLE RIGHT-POINTING ANGLE QUOTATION MARK numeric entity.","initialState":"Data","input":"&#0155;","inputUtf16":[38,35,48,49,53,53,59],"output":[{"Character":{"data":"›"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0042() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN SMALL LIGATURE OE numeric entity.","initialState":"Data","input":"&#0156;","inputUtf16":[38,35,48,49,53,54,59],"output":[{"Character":{"data":"œ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0043() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR numeric entity.","initialState":"Data","input":"&#0157;","inputUtf16":[38,35,48,49,53,55,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0044() {
    tokenize(
        r##"{"description":"Windows-1252 EURO SIGN hexadecimal numeric entity.","initialState":"Data","input":"&#x080;","inputUtf16":[38,35,120,48,56,48,59],"output":[{"Character":{"data":"€"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0045() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR hexadecimal numeric entity.","initialState":"Data","input":"&#x081;","inputUtf16":[38,35,120,48,56,49,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0046() {
    tokenize(
        r##"{"description":"Windows-1252 SINGLE LOW-9 QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x082;","inputUtf16":[38,35,120,48,56,50,59],"output":[{"Character":{"data":"‚"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0047() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN SMALL LETTER F WITH HOOK hexadecimal numeric entity.","initialState":"Data","input":"&#x083;","inputUtf16":[38,35,120,48,56,51,59],"output":[{"Character":{"data":"ƒ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0048() {
    tokenize(
        r##"{"description":"Windows-1252 DOUBLE LOW-9 QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x084;","inputUtf16":[38,35,120,48,56,52,59],"output":[{"Character":{"data":"„"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0049() {
    tokenize(
        r##"{"description":"Windows-1252 HORIZONTAL ELLIPSIS hexadecimal numeric entity.","initialState":"Data","input":"&#x085;","inputUtf16":[38,35,120,48,56,53,59],"output":[{"Character":{"data":"…"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0050() {
    tokenize(
        r##"{"description":"Windows-1252 DAGGER hexadecimal numeric entity.","initialState":"Data","input":"&#x086;","inputUtf16":[38,35,120,48,56,54,59],"output":[{"Character":{"data":"†"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0051() {
    tokenize(
        r##"{"description":"Windows-1252 DOUBLE DAGGER hexadecimal numeric entity.","initialState":"Data","input":"&#x087;","inputUtf16":[38,35,120,48,56,55,59],"output":[{"Character":{"data":"‡"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0052() {
    tokenize(
        r##"{"description":"Windows-1252 MODIFIER LETTER CIRCUMFLEX ACCENT hexadecimal numeric entity.","initialState":"Data","input":"&#x088;","inputUtf16":[38,35,120,48,56,56,59],"output":[{"Character":{"data":"ˆ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0053() {
    tokenize(
        r##"{"description":"Windows-1252 PER MILLE SIGN hexadecimal numeric entity.","initialState":"Data","input":"&#x089;","inputUtf16":[38,35,120,48,56,57,59],"output":[{"Character":{"data":"‰"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0054() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN CAPITAL LETTER S WITH CARON hexadecimal numeric entity.","initialState":"Data","input":"&#x08A;","inputUtf16":[38,35,120,48,56,65,59],"output":[{"Character":{"data":"Š"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0055() {
    tokenize(
        r##"{"description":"Windows-1252 SINGLE LEFT-POINTING ANGLE QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x08B;","inputUtf16":[38,35,120,48,56,66,59],"output":[{"Character":{"data":"‹"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0056() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN CAPITAL LIGATURE OE hexadecimal numeric entity.","initialState":"Data","input":"&#x08C;","inputUtf16":[38,35,120,48,56,67,59],"output":[{"Character":{"data":"Œ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0057() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR hexadecimal numeric entity.","initialState":"Data","input":"&#x08D;","inputUtf16":[38,35,120,48,56,68,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0058() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN CAPITAL LETTER Z WITH CARON hexadecimal numeric entity.","initialState":"Data","input":"&#x08E;","inputUtf16":[38,35,120,48,56,69,59],"output":[{"Character":{"data":"Ž"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0059() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR hexadecimal numeric entity.","initialState":"Data","input":"&#x08F;","inputUtf16":[38,35,120,48,56,70,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0060() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR hexadecimal numeric entity.","initialState":"Data","input":"&#x090;","inputUtf16":[38,35,120,48,57,48,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0061() {
    tokenize(
        r##"{"description":"Windows-1252 LEFT SINGLE QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x091;","inputUtf16":[38,35,120,48,57,49,59],"output":[{"Character":{"data":"‘"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0062() {
    tokenize(
        r##"{"description":"Windows-1252 RIGHT SINGLE QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x092;","inputUtf16":[38,35,120,48,57,50,59],"output":[{"Character":{"data":"’"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0063() {
    tokenize(
        r##"{"description":"Windows-1252 LEFT DOUBLE QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x093;","inputUtf16":[38,35,120,48,57,51,59],"output":[{"Character":{"data":"“"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0064() {
    tokenize(
        r##"{"description":"Windows-1252 RIGHT DOUBLE QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x094;","inputUtf16":[38,35,120,48,57,52,59],"output":[{"Character":{"data":"”"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0065() {
    tokenize(
        r##"{"description":"Windows-1252 BULLET hexadecimal numeric entity.","initialState":"Data","input":"&#x095;","inputUtf16":[38,35,120,48,57,53,59],"output":[{"Character":{"data":"•"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0066() {
    tokenize(
        r##"{"description":"Windows-1252 EN DASH hexadecimal numeric entity.","initialState":"Data","input":"&#x096;","inputUtf16":[38,35,120,48,57,54,59],"output":[{"Character":{"data":"–"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0067() {
    tokenize(
        r##"{"description":"Windows-1252 EM DASH hexadecimal numeric entity.","initialState":"Data","input":"&#x097;","inputUtf16":[38,35,120,48,57,55,59],"output":[{"Character":{"data":"—"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0068() {
    tokenize(
        r##"{"description":"Windows-1252 SMALL TILDE hexadecimal numeric entity.","initialState":"Data","input":"&#x098;","inputUtf16":[38,35,120,48,57,56,59],"output":[{"Character":{"data":"˜"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0069() {
    tokenize(
        r##"{"description":"Windows-1252 TRADE MARK SIGN hexadecimal numeric entity.","initialState":"Data","input":"&#x099;","inputUtf16":[38,35,120,48,57,57,59],"output":[{"Character":{"data":"™"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0070() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN SMALL LETTER S WITH CARON hexadecimal numeric entity.","initialState":"Data","input":"&#x09A;","inputUtf16":[38,35,120,48,57,65,59],"output":[{"Character":{"data":"š"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0071() {
    tokenize(
        r##"{"description":"Windows-1252 SINGLE RIGHT-POINTING ANGLE QUOTATION MARK hexadecimal numeric entity.","initialState":"Data","input":"&#x09B;","inputUtf16":[38,35,120,48,57,66,59],"output":[{"Character":{"data":"›"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0072() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN SMALL LIGATURE OE hexadecimal numeric entity.","initialState":"Data","input":"&#x09C;","inputUtf16":[38,35,120,48,57,67,59],"output":[{"Character":{"data":"œ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0073() {
    tokenize(
        r##"{"description":"Windows-1252 REPLACEMENT CHAR hexadecimal numeric entity.","initialState":"Data","input":"&#x09D;","inputUtf16":[38,35,120,48,57,68,59],"output":[{"Character":{"data":""}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0074() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN SMALL LETTER Z WITH CARON hexadecimal numeric entity.","initialState":"Data","input":"&#x09E;","inputUtf16":[38,35,120,48,57,69,59],"output":[{"Character":{"data":"ž"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0075() {
    tokenize(
        r##"{"description":"Windows-1252 LATIN CAPITAL LETTER Y WITH DIAERESIS hexadecimal numeric entity.","initialState":"Data","input":"&#x09F;","inputUtf16":[38,35,120,48,57,70,59],"output":[{"Character":{"data":"Ÿ"}}],"errors":[{"code":"ControlCharacterReference","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_0076() {
    tokenize(
        r##"{"description":"Decimal numeric entity followed by hex character a.","initialState":"Data","input":"&#97a","inputUtf16":[38,35,57,55,97],"output":[{"Character":{"data":"aa"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_0077() {
    tokenize(
        r##"{"description":"Decimal numeric entity followed by hex character A.","initialState":"Data","input":"&#97A","inputUtf16":[38,35,57,55,65],"output":[{"Character":{"data":"aA"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_0078() {
    tokenize(
        r##"{"description":"Decimal numeric entity followed by hex character f.","initialState":"Data","input":"&#97f","inputUtf16":[38,35,57,55,102],"output":[{"Character":{"data":"af"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_0079() {
    tokenize(
        r##"{"description":"Decimal numeric entity followed by hex character A.","initialState":"Data","input":"&#97F","inputUtf16":[38,35,57,55,70],"output":[{"Character":{"data":"aF"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":5}}]}"##,
    );
}
//</coverage:exclude>
