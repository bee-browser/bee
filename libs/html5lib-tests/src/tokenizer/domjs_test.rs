//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"CR in bogus comment state","initialState":"Data","input":"<?\r","inputUtf16":[60,63,13],"output":[{"Comment":{"data":"?\n"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"CRLF in bogus comment state","initialState":"Data","input":"<?\r\n","inputUtf16":[60,63,13,10],"output":[{"Comment":{"data":"?\n"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"CRLFLF in bogus comment state","initialState":"Data","input":"<?\r\n\n","inputUtf16":[60,63,13,10,10],"output":[{"Comment":{"data":"?\n\n"}}],"errors":[{"code":"UnexpectedQuestionMarkInsteadOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"Raw NUL replacement","initialState":"Rcdata","input":"\u0000","inputUtf16":[0],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"Raw NUL replacement","initialState":"Rawtext","input":"\u0000","inputUtf16":[0],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_5() {
    tokenize(
        r##"{"description":"Raw NUL replacement","initialState":"Plaintext","input":"\u0000","inputUtf16":[0],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_6() {
    tokenize(
        r##"{"description":"Raw NUL replacement","initialState":"ScriptData","input":"\u0000","inputUtf16":[0],"output":[{"Character":{"data":"�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":1}}]}"##,
    );
}

#[test]
fn test_7() {
    tokenize(
        r##"{"description":"NUL in CDATA section","initialState":"CdataSection","input":"\u0000]]>","inputUtf16":[0,93,93,62],"output":[{"Character":{"data":"\u0000"}}],"errors":[]}"##,
    );
}

#[test]
fn test_8() {
    tokenize(
        r##"{"description":"NUL in script HTML comment","initialState":"ScriptData","input":"<!--test\u0000--><!--test-\u0000--><!--test--\u0000-->","inputUtf16":[60,33,45,45,116,101,115,116,0,45,45,62,60,33,45,45,116,101,115,116,45,0,45,45,62,60,33,45,45,116,101,115,116,45,45,0,45,45,62],"output":[{"Character":{"data":"<!--test�--><!--test-�--><!--test--�-->"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":9}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":22}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":36}}]}"##,
    );
}

#[test]
fn test_9() {
    tokenize(
        r##"{"description":"NUL in script HTML comment - double escaped","initialState":"ScriptData","input":"<!--<script>\u0000--><!--<script>-\u0000--><!--<script>--\u0000-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,0,45,45,62,60,33,45,45,60,115,99,114,105,112,116,62,45,0,45,45,62,60,33,45,45,60,115,99,114,105,112,116,62,45,45,0,45,45,62],"output":[{"Character":{"data":"<!--<script>�--><!--<script>-�--><!--<script>--�-->"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":13}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":30}},{"code":"UnexpectedNullCharacter","location":{"line":1,"column":48}}]}"##,
    );
}

#[test]
fn test_10() {
    tokenize(
        r##"{"description":"EOF in script HTML comment","initialState":"ScriptData","input":"<!--test","inputUtf16":[60,33,45,45,116,101,115,116],"output":[{"Character":{"data":"<!--test"}}],"errors":[{"code":"EofInScriptHtmlCommentLikeText","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_11() {
    tokenize(
        r##"{"description":"EOF in script HTML comment after dash","initialState":"ScriptData","input":"<!--test-","inputUtf16":[60,33,45,45,116,101,115,116,45],"output":[{"Character":{"data":"<!--test-"}}],"errors":[{"code":"EofInScriptHtmlCommentLikeText","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_12() {
    tokenize(
        r##"{"description":"EOF in script HTML comment after dash dash","initialState":"ScriptData","input":"<!--test--","inputUtf16":[60,33,45,45,116,101,115,116,45,45],"output":[{"Character":{"data":"<!--test--"}}],"errors":[{"code":"EofInScriptHtmlCommentLikeText","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_13() {
    tokenize(
        r##"{"description":"EOF in script HTML comment double escaped after dash","initialState":"ScriptData","input":"<!--<script>-","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,45],"output":[{"Character":{"data":"<!--<script>-"}}],"errors":[{"code":"EofInScriptHtmlCommentLikeText","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_14() {
    tokenize(
        r##"{"description":"EOF in script HTML comment double escaped after dash dash","initialState":"ScriptData","input":"<!--<script>--","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,45,45],"output":[{"Character":{"data":"<!--<script>--"}}],"errors":[{"code":"EofInScriptHtmlCommentLikeText","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_15() {
    tokenize(
        r##"{"description":"EOF in script HTML comment - double escaped","initialState":"ScriptData","input":"<!--<script>","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62],"output":[{"Character":{"data":"<!--<script>"}}],"errors":[{"code":"EofInScriptHtmlCommentLikeText","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_16() {
    tokenize(
        r##"{"description":"Dash in script HTML comment","initialState":"ScriptData","input":"<!-- - -->","inputUtf16":[60,33,45,45,32,45,32,45,45,62],"output":[{"Character":{"data":"<!-- - -->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_17() {
    tokenize(
        r##"{"description":"Dash less-than in script HTML comment","initialState":"ScriptData","input":"<!-- -< -->","inputUtf16":[60,33,45,45,32,45,60,32,45,45,62],"output":[{"Character":{"data":"<!-- -< -->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_18() {
    tokenize(
        r##"{"description":"Dash at end of script HTML comment","initialState":"ScriptData","input":"<!--test--->","inputUtf16":[60,33,45,45,116,101,115,116,45,45,45,62],"output":[{"Character":{"data":"<!--test--->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_19() {
    tokenize(
        r##"{"description":"</script> in script HTML comment","initialState":"ScriptData","lastStartTag":"script","input":"<!-- </script> --></script>","inputUtf16":[60,33,45,45,32,60,47,115,99,114,105,112,116,62,32,45,45,62,60,47,115,99,114,105,112,116,62],"output":[{"Character":{"data":"<!-- "}},{"EndTag":{"name":"script"}},{"Character":{"data":" -->"}},{"EndTag":{"name":"script"}}],"errors":[]}"##,
    );
}

#[test]
fn test_20() {
    tokenize(
        r##"{"description":"</script> in script HTML comment - double escaped","initialState":"ScriptData","lastStartTag":"script","input":"<!-- <script></script> --></script>","inputUtf16":[60,33,45,45,32,60,115,99,114,105,112,116,62,60,47,115,99,114,105,112,116,62,32,45,45,62,60,47,115,99,114,105,112,116,62],"output":[{"Character":{"data":"<!-- <script></script> -->"}},{"EndTag":{"name":"script"}}],"errors":[]}"##,
    );
}

#[test]
fn test_21() {
    tokenize(
        r##"{"description":"</script> in script HTML comment - double escaped with nested <script>","initialState":"ScriptData","lastStartTag":"script","input":"<!-- <script><script></script></script> --></script>","inputUtf16":[60,33,45,45,32,60,115,99,114,105,112,116,62,60,115,99,114,105,112,116,62,60,47,115,99,114,105,112,116,62,60,47,115,99,114,105,112,116,62,32,45,45,62,60,47,115,99,114,105,112,116,62],"output":[{"Character":{"data":"<!-- <script><script></script>"}},{"EndTag":{"name":"script"}},{"Character":{"data":" -->"}},{"EndTag":{"name":"script"}}],"errors":[]}"##,
    );
}

#[test]
fn test_22() {
    tokenize(
        r##"{"description":"</script> in script HTML comment - double escaped with abrupt end","initialState":"ScriptData","lastStartTag":"script","input":"<!-- <script>--></script> --></script>","inputUtf16":[60,33,45,45,32,60,115,99,114,105,112,116,62,45,45,62,60,47,115,99,114,105,112,116,62,32,45,45,62,60,47,115,99,114,105,112,116,62],"output":[{"Character":{"data":"<!-- <script>-->"}},{"EndTag":{"name":"script"}},{"Character":{"data":" -->"}},{"EndTag":{"name":"script"}}],"errors":[]}"##,
    );
}

#[test]
fn test_23() {
    tokenize(
        r##"{"description":"Incomplete start tag in script HTML comment double escaped","initialState":"ScriptData","lastStartTag":"script","input":"<!--<scrip></script>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,62,60,47,115,99,114,105,112,116,62,45,45,62],"output":[{"Character":{"data":"<!--<scrip>"}},{"EndTag":{"name":"script"}},{"Character":{"data":"-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_24() {
    tokenize(
        r##"{"description":"Unclosed start tag in script HTML comment double escaped","initialState":"ScriptData","lastStartTag":"script","input":"<!--<script</script>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,60,47,115,99,114,105,112,116,62,45,45,62],"output":[{"Character":{"data":"<!--<script"}},{"EndTag":{"name":"script"}},{"Character":{"data":"-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_25() {
    tokenize(
        r##"{"description":"Incomplete end tag in script HTML comment double escaped","initialState":"ScriptData","lastStartTag":"script","input":"<!--<script></scrip>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,60,47,115,99,114,105,112,62,45,45,62],"output":[{"Character":{"data":"<!--<script></scrip>-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_26() {
    tokenize(
        r##"{"description":"Unclosed end tag in script HTML comment double escaped","initialState":"ScriptData","lastStartTag":"script","input":"<!--<script></script-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,60,47,115,99,114,105,112,116,45,45,62],"output":[{"Character":{"data":"<!--<script></script-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_27() {
    tokenize(
        r##"{"description":"leading U+FEFF must pass through","initialState":"Data","input":"﻿foo﻿bar","inputUtf16":[65279,102,111,111,65279,98,97,114],"output":[{"Character":{"data":"﻿foo﻿bar"}}],"errors":[]}"##,
    );
}

#[test]
fn test_28() {
    tokenize(
        r##"{"description":"leading U+FEFF must pass through","initialState":"Rcdata","input":"﻿foo﻿bar","inputUtf16":[65279,102,111,111,65279,98,97,114],"output":[{"Character":{"data":"﻿foo﻿bar"}}],"errors":[]}"##,
    );
}

#[test]
fn test_29() {
    tokenize(
        r##"{"description":"leading U+FEFF must pass through","initialState":"Rawtext","input":"﻿foo﻿bar","inputUtf16":[65279,102,111,111,65279,98,97,114],"output":[{"Character":{"data":"﻿foo﻿bar"}}],"errors":[]}"##,
    );
}

#[test]
fn test_30() {
    tokenize(
        r##"{"description":"leading U+FEFF must pass through","initialState":"ScriptData","input":"﻿foo﻿bar","inputUtf16":[65279,102,111,111,65279,98,97,114],"output":[{"Character":{"data":"﻿foo﻿bar"}}],"errors":[]}"##,
    );
}

#[test]
fn test_31() {
    tokenize(
        r##"{"description":"Non BMP-charref in RCDATA","initialState":"Rcdata","input":"&NotEqualTilde;","inputUtf16":[38,78,111,116,69,113,117,97,108,84,105,108,100,101,59],"output":[{"Character":{"data":"≂̸"}}],"errors":[]}"##,
    );
}

#[test]
fn test_32() {
    tokenize(
        r##"{"description":"Bad charref in RCDATA","initialState":"Rcdata","input":"&NotEqualTild;","inputUtf16":[38,78,111,116,69,113,117,97,108,84,105,108,100,59],"output":[{"Character":{"data":"&NotEqualTild;"}}],"errors":[{"code":"UnknownNamedCharacterReference","location":{"line":1,"column":14}}]}"##,
    );
}

#[test]
fn test_33() {
    tokenize(
        r##"{"description":"lowercase endtags","initialState":"Rcdata","lastStartTag":"xmp","input":"</XMP>","inputUtf16":[60,47,88,77,80,62],"output":[{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_34() {
    tokenize(
        r##"{"description":"lowercase endtags","initialState":"Rawtext","lastStartTag":"xmp","input":"</XMP>","inputUtf16":[60,47,88,77,80,62],"output":[{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_35() {
    tokenize(
        r##"{"description":"lowercase endtags","initialState":"ScriptData","lastStartTag":"xmp","input":"</XMP>","inputUtf16":[60,47,88,77,80,62],"output":[{"EndTag":{"name":"xmp"}}],"errors":[]}"##,
    );
}

#[test]
fn test_36() {
    tokenize(
        r##"{"description":"bad endtag (space before name)","initialState":"Rcdata","lastStartTag":"xmp","input":"</ XMP>","inputUtf16":[60,47,32,88,77,80,62],"output":[{"Character":{"data":"</ XMP>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_37() {
    tokenize(
        r##"{"description":"bad endtag (space before name)","initialState":"Rawtext","lastStartTag":"xmp","input":"</ XMP>","inputUtf16":[60,47,32,88,77,80,62],"output":[{"Character":{"data":"</ XMP>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_38() {
    tokenize(
        r##"{"description":"bad endtag (space before name)","initialState":"ScriptData","lastStartTag":"xmp","input":"</ XMP>","inputUtf16":[60,47,32,88,77,80,62],"output":[{"Character":{"data":"</ XMP>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_39() {
    tokenize(
        r##"{"description":"bad endtag (not matching last start tag)","initialState":"Rcdata","lastStartTag":"xmp","input":"</xm>","inputUtf16":[60,47,120,109,62],"output":[{"Character":{"data":"</xm>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_40() {
    tokenize(
        r##"{"description":"bad endtag (not matching last start tag)","initialState":"Rawtext","lastStartTag":"xmp","input":"</xm>","inputUtf16":[60,47,120,109,62],"output":[{"Character":{"data":"</xm>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_41() {
    tokenize(
        r##"{"description":"bad endtag (not matching last start tag)","initialState":"ScriptData","lastStartTag":"xmp","input":"</xm>","inputUtf16":[60,47,120,109,62],"output":[{"Character":{"data":"</xm>"}}],"errors":[]}"##,
    );
}

#[test]
fn test_42() {
    tokenize(
        r##"{"description":"bad endtag (without close bracket)","initialState":"Rcdata","lastStartTag":"xmp","input":"</xm ","inputUtf16":[60,47,120,109,32],"output":[{"Character":{"data":"</xm "}}],"errors":[]}"##,
    );
}

#[test]
fn test_43() {
    tokenize(
        r##"{"description":"bad endtag (without close bracket)","initialState":"Rawtext","lastStartTag":"xmp","input":"</xm ","inputUtf16":[60,47,120,109,32],"output":[{"Character":{"data":"</xm "}}],"errors":[]}"##,
    );
}

#[test]
fn test_44() {
    tokenize(
        r##"{"description":"bad endtag (without close bracket)","initialState":"ScriptData","lastStartTag":"xmp","input":"</xm ","inputUtf16":[60,47,120,109,32],"output":[{"Character":{"data":"</xm "}}],"errors":[]}"##,
    );
}

#[test]
fn test_45() {
    tokenize(
        r##"{"description":"bad endtag (trailing solidus)","initialState":"Rcdata","lastStartTag":"xmp","input":"</xm/","inputUtf16":[60,47,120,109,47],"output":[{"Character":{"data":"</xm/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_46() {
    tokenize(
        r##"{"description":"bad endtag (trailing solidus)","initialState":"Rawtext","lastStartTag":"xmp","input":"</xm/","inputUtf16":[60,47,120,109,47],"output":[{"Character":{"data":"</xm/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_47() {
    tokenize(
        r##"{"description":"bad endtag (trailing solidus)","initialState":"ScriptData","lastStartTag":"xmp","input":"</xm/","inputUtf16":[60,47,120,109,47],"output":[{"Character":{"data":"</xm/"}}],"errors":[]}"##,
    );
}

#[test]
fn test_48() {
    tokenize(
        r##"{"description":"Non BMP-charref in attribute","initialState":"Data","input":"<p id=\"&NotEqualTilde;\">","inputUtf16":[60,112,32,105,100,61,34,38,78,111,116,69,113,117,97,108,84,105,108,100,101,59,34,62],"output":[{"StartTag":{"name":"p","attrs":{"id":"≂̸"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_49() {
    tokenize(
        r##"{"description":"--!NUL in comment ","initialState":"Data","input":"<!----!\u0000-->","inputUtf16":[60,33,45,45,45,45,33,0,45,45,62],"output":[{"Comment":{"data":"--!�"}}],"errors":[{"code":"UnexpectedNullCharacter","location":{"line":1,"column":8}}]}"##,
    );
}

#[test]
fn test_50() {
    tokenize(
        r##"{"description":"space EOF after doctype ","initialState":"Data","input":"<!DOCTYPE html ","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,32],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":16}}]}"##,
    );
}

#[test]
fn test_51() {
    tokenize(
        r##"{"description":"CDATA in HTML content","initialState":"Data","input":"<![CDATA[foo]]>","inputUtf16":[60,33,91,67,68,65,84,65,91,102,111,111,93,93,62],"output":[{"Comment":{"data":"[CDATA[foo]]"}}],"errors":[{"code":"CdataInHtmlContent","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_52() {
    tokenize(
        r##"{"description":"CDATA content","initialState":"CdataSection","input":"foo&#32;]]>","inputUtf16":[102,111,111,38,35,51,50,59,93,93,62],"output":[{"Character":{"data":"foo&#32;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_53() {
    tokenize(
        r##"{"description":"CDATA followed by HTML content","initialState":"CdataSection","input":"foo&#32;]]>&#32;","inputUtf16":[102,111,111,38,35,51,50,59,93,93,62,38,35,51,50,59],"output":[{"Character":{"data":"foo&#32; "}}],"errors":[]}"##,
    );
}

#[test]
fn test_54() {
    tokenize(
        r##"{"description":"CDATA with extra bracket","initialState":"CdataSection","input":"foo]]]>","inputUtf16":[102,111,111,93,93,93,62],"output":[{"Character":{"data":"foo]"}}],"errors":[]}"##,
    );
}

#[test]
fn test_55() {
    tokenize(
        r##"{"description":"CDATA without end marker","initialState":"CdataSection","input":"foo","inputUtf16":[102,111,111],"output":[{"Character":{"data":"foo"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_56() {
    tokenize(
        r##"{"description":"CDATA with single bracket ending","initialState":"CdataSection","input":"foo]","inputUtf16":[102,111,111,93],"output":[{"Character":{"data":"foo]"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_57() {
    tokenize(
        r##"{"description":"CDATA with two brackets ending","initialState":"CdataSection","input":"foo]]","inputUtf16":[102,111,111,93,93],"output":[{"Character":{"data":"foo]]"}}],"errors":[{"code":"EofInCdata","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_58() {
    tokenize(
        r##"{"description":"HTML tag in script data","initialState":"ScriptData","input":"<b>hello world</b>","inputUtf16":[60,98,62,104,101,108,108,111,32,119,111,114,108,100,60,47,98,62],"output":[{"Character":{"data":"<b>hello world</b>"}}],"errors":[]}"##,
    );
}
//</coverage:exclude>
