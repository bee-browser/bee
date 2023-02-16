//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"Correct Doctype lowercase","initialState":"Data","input":"<!DOCTYPE html>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,104,116,109,108,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_1() {
    tokenize(
        r##"{"description":"Correct Doctype uppercase","initialState":"Data","input":"<!DOCTYPE HTML>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,72,84,77,76,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_2() {
    tokenize(
        r##"{"description":"Correct Doctype mixed case","initialState":"Data","input":"<!DOCTYPE HtMl>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,72,116,77,108,62],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_3() {
    tokenize(
        r##"{"description":"Correct Doctype case with EOF","initialState":"Data","input":"<!DOCTYPE HtMl","inputUtf16":[60,33,68,79,67,84,89,80,69,32,72,116,77,108],"output":[{"Doctype":{"name":"html","public_id":null,"system_id":null,"force_quirks":true}}],"errors":[{"code":"EofInDoctype","location":{"line":1,"column":15}}]}"##,
    );
}

#[test]
fn test_4() {
    tokenize(
        r##"{"description":"Truncated doctype start","initialState":"Data","input":"<!DOC>","inputUtf16":[60,33,68,79,67,62],"output":[{"Comment":{"data":"DOC"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_5() {
    tokenize(
        r##"{"description":"Doctype in error","initialState":"Data","input":"<!DOCTYPE foo>","inputUtf16":[60,33,68,79,67,84,89,80,69,32,102,111,111,62],"output":[{"Doctype":{"name":"foo","public_id":null,"system_id":null,"force_quirks":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_6() {
    tokenize(
        r##"{"description":"Single Start Tag","initialState":"Data","input":"<h>","inputUtf16":[60,104,62],"output":[{"StartTag":{"name":"h","attrs":{},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_7() {
    tokenize(
        r##"{"description":"Empty end tag","initialState":"Data","input":"</>","inputUtf16":[60,47,62],"output":[],"errors":[{"code":"MissingEndTagName","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_8() {
    tokenize(
        r##"{"description":"Empty start tag","initialState":"Data","input":"<>","inputUtf16":[60,62],"output":[{"Character":{"data":"<>"}}],"errors":[{"code":"InvalidFirstCharacterOfTagName","location":{"line":1,"column":2}}]}"##,
    );
}

#[test]
fn test_9() {
    tokenize(
        r##"{"description":"Start Tag w/attribute","initialState":"Data","input":"<h a='b'>","inputUtf16":[60,104,32,97,61,39,98,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"b"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_10() {
    tokenize(
        r##"{"description":"Start Tag w/attribute no quotes","initialState":"Data","input":"<h a=b>","inputUtf16":[60,104,32,97,61,98,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"b"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_11() {
    tokenize(
        r##"{"description":"Start/End Tag","initialState":"Data","input":"<h></h>","inputUtf16":[60,104,62,60,47,104,62],"output":[{"StartTag":{"name":"h","attrs":{},"self_closing":false}},{"EndTag":{"name":"h"}}],"errors":[]}"##,
    );
}

#[test]
fn test_12() {
    tokenize(
        r##"{"description":"Two unclosed start tags","initialState":"Data","input":"<p>One<p>Two","inputUtf16":[60,112,62,79,110,101,60,112,62,84,119,111],"output":[{"StartTag":{"name":"p","attrs":{},"self_closing":false}},{"Character":{"data":"One"}},{"StartTag":{"name":"p","attrs":{},"self_closing":false}},{"Character":{"data":"Two"}}],"errors":[]}"##,
    );
}

#[test]
fn test_13() {
    tokenize(
        r##"{"description":"End Tag w/attribute","initialState":"Data","input":"<h></h a='b'>","inputUtf16":[60,104,62,60,47,104,32,97,61,39,98,39,62],"output":[{"StartTag":{"name":"h","attrs":{},"self_closing":false}},{"EndTag":{"name":"h"}}],"errors":[{"code":"EndTagWithAttributes","location":{"line":1,"column":13}}]}"##,
    );
}

#[test]
fn test_14() {
    tokenize(
        r##"{"description":"Multiple atts","initialState":"Data","input":"<h a='b' c='d'>","inputUtf16":[60,104,32,97,61,39,98,39,32,99,61,39,100,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"b","c":"d"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_15() {
    tokenize(
        r##"{"description":"Multiple atts no space","initialState":"Data","input":"<h a='b'c='d'>","inputUtf16":[60,104,32,97,61,39,98,39,99,61,39,100,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"b","c":"d"},"self_closing":false}}],"errors":[{"code":"MissingWhitespaceBetweenAttributes","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_16() {
    tokenize(
        r##"{"description":"Repeated attr","initialState":"Data","input":"<h a='b' a='d'>","inputUtf16":[60,104,32,97,61,39,98,39,32,97,61,39,100,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"b"},"self_closing":false}}],"errors":[{"code":"DuplicateAttribute","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_17() {
    tokenize(
        r##"{"description":"Simple comment","initialState":"Data","input":"<!--comment-->","inputUtf16":[60,33,45,45,99,111,109,109,101,110,116,45,45,62],"output":[{"Comment":{"data":"comment"}}],"errors":[]}"##,
    );
}

#[test]
fn test_18() {
    tokenize(
        r##"{"description":"Comment, Central dash no space","initialState":"Data","input":"<!----->","inputUtf16":[60,33,45,45,45,45,45,62],"output":[{"Comment":{"data":"-"}}],"errors":[]}"##,
    );
}

#[test]
fn test_19() {
    tokenize(
        r##"{"description":"Comment, two central dashes","initialState":"Data","input":"<!-- --comment -->","inputUtf16":[60,33,45,45,32,45,45,99,111,109,109,101,110,116,32,45,45,62],"output":[{"Comment":{"data":" --comment "}}],"errors":[]}"##,
    );
}

#[test]
fn test_20() {
    tokenize(
        r##"{"description":"Comment, central less-than bang","initialState":"Data","input":"<!--<!-->","inputUtf16":[60,33,45,45,60,33,45,45,62],"output":[{"Comment":{"data":"<!"}}],"errors":[]}"##,
    );
}

#[test]
fn test_21() {
    tokenize(
        r##"{"description":"Unfinished comment","initialState":"Data","input":"<!--comment","inputUtf16":[60,33,45,45,99,111,109,109,101,110,116],"output":[{"Comment":{"data":"comment"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_22() {
    tokenize(
        r##"{"description":"Unfinished comment after start of nested comment","initialState":"Data","input":"<!-- <!--","inputUtf16":[60,33,45,45,32,60,33,45,45],"output":[{"Comment":{"data":" <!"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_23() {
    tokenize(
        r##"{"description":"Start of a comment","initialState":"Data","input":"<!-","inputUtf16":[60,33,45],"output":[{"Comment":{"data":"-"}}],"errors":[{"code":"IncorrectlyOpenedComment","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_24() {
    tokenize(
        r##"{"description":"Short comment","initialState":"Data","input":"<!-->","inputUtf16":[60,33,45,45,62],"output":[{"Comment":{"data":""}}],"errors":[{"code":"AbruptClosingOfEmptyComment","location":{"line":1,"column":5}}]}"##,
    );
}

#[test]
fn test_25() {
    tokenize(
        r##"{"description":"Short comment two","initialState":"Data","input":"<!--->","inputUtf16":[60,33,45,45,45,62],"output":[{"Comment":{"data":""}}],"errors":[{"code":"AbruptClosingOfEmptyComment","location":{"line":1,"column":6}}]}"##,
    );
}

#[test]
fn test_26() {
    tokenize(
        r##"{"description":"Short comment three","initialState":"Data","input":"<!---->","inputUtf16":[60,33,45,45,45,45,62],"output":[{"Comment":{"data":""}}],"errors":[]}"##,
    );
}

#[test]
fn test_27() {
    tokenize(
        r##"{"description":"< in comment","initialState":"Data","input":"<!-- <test-->","inputUtf16":[60,33,45,45,32,60,116,101,115,116,45,45,62],"output":[{"Comment":{"data":" <test"}}],"errors":[]}"##,
    );
}

#[test]
fn test_28() {
    tokenize(
        r##"{"description":"<< in comment","initialState":"Data","input":"<!--<<-->","inputUtf16":[60,33,45,45,60,60,45,45,62],"output":[{"Comment":{"data":"<<"}}],"errors":[]}"##,
    );
}

#[test]
fn test_29() {
    tokenize(
        r##"{"description":"<! in comment","initialState":"Data","input":"<!-- <!test-->","inputUtf16":[60,33,45,45,32,60,33,116,101,115,116,45,45,62],"output":[{"Comment":{"data":" <!test"}}],"errors":[]}"##,
    );
}

#[test]
fn test_30() {
    tokenize(
        r##"{"description":"<!- in comment","initialState":"Data","input":"<!-- <!-test-->","inputUtf16":[60,33,45,45,32,60,33,45,116,101,115,116,45,45,62],"output":[{"Comment":{"data":" <!-test"}}],"errors":[]}"##,
    );
}

#[test]
fn test_31() {
    tokenize(
        r##"{"description":"Nested comment","initialState":"Data","input":"<!-- <!--test-->","inputUtf16":[60,33,45,45,32,60,33,45,45,116,101,115,116,45,45,62],"output":[{"Comment":{"data":" <!--test"}}],"errors":[{"code":"NestedComment","location":{"line":1,"column":10}}]}"##,
    );
}

#[test]
fn test_32() {
    tokenize(
        r##"{"description":"Nested comment with extra <","initialState":"Data","input":"<!-- <<!--test-->","inputUtf16":[60,33,45,45,32,60,60,33,45,45,116,101,115,116,45,45,62],"output":[{"Comment":{"data":" <<!--test"}}],"errors":[{"code":"NestedComment","location":{"line":1,"column":11}}]}"##,
    );
}

#[test]
fn test_33() {
    tokenize(
        r##"{"description":"< in script data","initialState":"ScriptData","input":"<test-->","inputUtf16":[60,116,101,115,116,45,45,62],"output":[{"Character":{"data":"<test-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_34() {
    tokenize(
        r##"{"description":"<! in script data","initialState":"ScriptData","input":"<!test-->","inputUtf16":[60,33,116,101,115,116,45,45,62],"output":[{"Character":{"data":"<!test-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_35() {
    tokenize(
        r##"{"description":"<!- in script data","initialState":"ScriptData","input":"<!-test-->","inputUtf16":[60,33,45,116,101,115,116,45,45,62],"output":[{"Character":{"data":"<!-test-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_36() {
    tokenize(
        r##"{"description":"Escaped script data","initialState":"ScriptData","input":"<!--test-->","inputUtf16":[60,33,45,45,116,101,115,116,45,45,62],"output":[{"Character":{"data":"<!--test-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_37() {
    tokenize(
        r##"{"description":"< in script HTML comment","initialState":"ScriptData","input":"<!-- < test -->","inputUtf16":[60,33,45,45,32,60,32,116,101,115,116,32,45,45,62],"output":[{"Character":{"data":"<!-- < test -->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_38() {
    tokenize(
        r##"{"description":"</ in script HTML comment","initialState":"ScriptData","input":"<!-- </ test -->","inputUtf16":[60,33,45,45,32,60,47,32,116,101,115,116,32,45,45,62],"output":[{"Character":{"data":"<!-- </ test -->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_39() {
    tokenize(
        r##"{"description":"Start tag in script HTML comment","initialState":"ScriptData","input":"<!-- <test> -->","inputUtf16":[60,33,45,45,32,60,116,101,115,116,62,32,45,45,62],"output":[{"Character":{"data":"<!-- <test> -->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_40() {
    tokenize(
        r##"{"description":"End tag in script HTML comment","initialState":"ScriptData","input":"<!-- </test> -->","inputUtf16":[60,33,45,45,32,60,47,116,101,115,116,62,32,45,45,62],"output":[{"Character":{"data":"<!-- </test> -->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_41() {
    tokenize(
        r##"{"description":"- in script HTML comment double escaped","initialState":"ScriptData","input":"<!--<script>-</script>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,45,60,47,115,99,114,105,112,116,62,45,45,62],"output":[{"Character":{"data":"<!--<script>-</script>-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_42() {
    tokenize(
        r##"{"description":"-- in script HTML comment double escaped","initialState":"ScriptData","input":"<!--<script>--</script>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,45,45,60,47,115,99,114,105,112,116,62,45,45,62],"output":[{"Character":{"data":"<!--<script>--</script>-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_43() {
    tokenize(
        r##"{"description":"--- in script HTML comment double escaped","initialState":"ScriptData","input":"<!--<script>---</script>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,45,45,45,60,47,115,99,114,105,112,116,62,45,45,62],"output":[{"Character":{"data":"<!--<script>---</script>-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_44() {
    tokenize(
        r##"{"description":"- spaced in script HTML comment double escaped","initialState":"ScriptData","input":"<!--<script> - </script>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,32,45,32,60,47,115,99,114,105,112,116,62,45,45,62],"output":[{"Character":{"data":"<!--<script> - </script>-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_45() {
    tokenize(
        r##"{"description":"-- spaced in script HTML comment double escaped","initialState":"ScriptData","input":"<!--<script> -- </script>-->","inputUtf16":[60,33,45,45,60,115,99,114,105,112,116,62,32,45,45,32,60,47,115,99,114,105,112,116,62,45,45,62],"output":[{"Character":{"data":"<!--<script> -- </script>-->"}}],"errors":[]}"##,
    );
}

#[test]
fn test_46() {
    tokenize(
        r##"{"description":"Ampersand EOF","initialState":"Data","input":"&","inputUtf16":[38],"output":[{"Character":{"data":"&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_47() {
    tokenize(
        r##"{"description":"Ampersand ampersand EOF","initialState":"Data","input":"&&","inputUtf16":[38,38],"output":[{"Character":{"data":"&&"}}],"errors":[]}"##,
    );
}

#[test]
fn test_48() {
    tokenize(
        r##"{"description":"Ampersand space EOF","initialState":"Data","input":"& ","inputUtf16":[38,32],"output":[{"Character":{"data":"& "}}],"errors":[]}"##,
    );
}

#[test]
fn test_49() {
    tokenize(
        r##"{"description":"Unfinished entity","initialState":"Data","input":"&f","inputUtf16":[38,102],"output":[{"Character":{"data":"&f"}}],"errors":[]}"##,
    );
}

#[test]
fn test_50() {
    tokenize(
        r##"{"description":"Ampersand, number sign","initialState":"Data","input":"&#","inputUtf16":[38,35],"output":[{"Character":{"data":"&#"}}],"errors":[{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":3}}]}"##,
    );
}

#[test]
fn test_51() {
    tokenize(
        r##"{"description":"Unfinished numeric entity","initialState":"Data","input":"&#x","inputUtf16":[38,35,120],"output":[{"Character":{"data":"&#x"}}],"errors":[{"code":"AbsenceOfDigitsInNumericCharacterReference","location":{"line":1,"column":4}}]}"##,
    );
}

#[test]
fn test_52() {
    tokenize(
        r##"{"description":"Entity with trailing semicolon (1)","initialState":"Data","input":"I'm &not;it","inputUtf16":[73,39,109,32,38,110,111,116,59,105,116],"output":[{"Character":{"data":"I'm ¬it"}}],"errors":[]}"##,
    );
}

#[test]
fn test_53() {
    tokenize(
        r##"{"description":"Entity with trailing semicolon (2)","initialState":"Data","input":"I'm &notin;","inputUtf16":[73,39,109,32,38,110,111,116,105,110,59],"output":[{"Character":{"data":"I'm ∉"}}],"errors":[]}"##,
    );
}

#[test]
fn test_54() {
    tokenize(
        r##"{"description":"Entity without trailing semicolon (1)","initialState":"Data","input":"I'm &notit","inputUtf16":[73,39,109,32,38,110,111,116,105,116],"output":[{"Character":{"data":"I'm ¬it"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_55() {
    tokenize(
        r##"{"description":"Entity without trailing semicolon (2)","initialState":"Data","input":"I'm &notin","inputUtf16":[73,39,109,32,38,110,111,116,105,110],"output":[{"Character":{"data":"I'm ¬in"}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":9}}]}"##,
    );
}

#[test]
fn test_56() {
    tokenize(
        r##"{"description":"Partial entity match at end of file","initialState":"Data","input":"I'm &no","inputUtf16":[73,39,109,32,38,110,111],"output":[{"Character":{"data":"I'm &no"}}],"errors":[]}"##,
    );
}

#[test]
fn test_57() {
    tokenize(
        r##"{"description":"Non-ASCII character reference name","initialState":"Data","input":"&¬;","inputUtf16":[38,172,59],"output":[{"Character":{"data":"&¬;"}}],"errors":[]}"##,
    );
}

#[test]
fn test_58() {
    tokenize(
        r##"{"description":"ASCII decimal entity","initialState":"Data","input":"&#0036;","inputUtf16":[38,35,48,48,51,54,59],"output":[{"Character":{"data":"$"}}],"errors":[]}"##,
    );
}

#[test]
fn test_59() {
    tokenize(
        r##"{"description":"ASCII hexadecimal entity","initialState":"Data","input":"&#x3f;","inputUtf16":[38,35,120,51,102,59],"output":[{"Character":{"data":"?"}}],"errors":[]}"##,
    );
}

#[test]
fn test_60() {
    tokenize(
        r##"{"description":"Hexadecimal entity in attribute","initialState":"Data","input":"<h a='&#x3f;'></h>","inputUtf16":[60,104,32,97,61,39,38,35,120,51,102,59,39,62,60,47,104,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"?"},"self_closing":false}},{"EndTag":{"name":"h"}}],"errors":[]}"##,
    );
}

#[test]
fn test_61() {
    tokenize(
        r##"{"description":"Entity in attribute without semicolon ending in x","initialState":"Data","input":"<h a='&notx'>","inputUtf16":[60,104,32,97,61,39,38,110,111,116,120,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&notx"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_62() {
    tokenize(
        r##"{"description":"Entity in attribute without semicolon ending in 1","initialState":"Data","input":"<h a='&not1'>","inputUtf16":[60,104,32,97,61,39,38,110,111,116,49,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&not1"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_63() {
    tokenize(
        r##"{"description":"Entity in attribute without semicolon ending in i","initialState":"Data","input":"<h a='&noti'>","inputUtf16":[60,104,32,97,61,39,38,110,111,116,105,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"&noti"},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_64() {
    tokenize(
        r##"{"description":"Entity in attribute without semicolon","initialState":"Data","input":"<h a='&COPY'>","inputUtf16":[60,104,32,97,61,39,38,67,79,80,89,39,62],"output":[{"StartTag":{"name":"h","attrs":{"a":"©"},"self_closing":false}}],"errors":[{"code":"MissingSemicolonAfterCharacterReference","location":{"line":1,"column":12}}]}"##,
    );
}

#[test]
fn test_65() {
    tokenize(
        r##"{"description":"Unquoted attribute ending in ampersand","initialState":"Data","input":"<s o=& t>","inputUtf16":[60,115,32,111,61,38,32,116,62],"output":[{"StartTag":{"name":"s","attrs":{"o":"&","t":""},"self_closing":false}}],"errors":[]}"##,
    );
}

#[test]
fn test_66() {
    tokenize(
        r##"{"description":"Unquoted attribute at end of tag with final character of &, with tag followed by characters","initialState":"Data","input":"<a a=a&>foo","inputUtf16":[60,97,32,97,61,97,38,62,102,111,111],"output":[{"StartTag":{"name":"a","attrs":{"a":"a&"},"self_closing":false}},{"Character":{"data":"foo"}}],"errors":[]}"##,
    );
}

#[test]
fn test_67() {
    tokenize(
        r##"{"description":"plaintext element","initialState":"Data","input":"<plaintext>foobar","inputUtf16":[60,112,108,97,105,110,116,101,120,116,62,102,111,111,98,97,114],"output":[{"StartTag":{"name":"plaintext","attrs":{},"self_closing":false}},{"Character":{"data":"foobar"}}],"errors":[]}"##,
    );
}

#[test]
fn test_68() {
    tokenize(
        r##"{"description":"Open angled bracket in unquoted attribute value state","initialState":"Data","input":"<a a=f<>","inputUtf16":[60,97,32,97,61,102,60,62],"output":[{"StartTag":{"name":"a","attrs":{"a":"f<"},"self_closing":false}}],"errors":[{"code":"UnexpectedCharacterInUnquotedAttributeValue","location":{"line":1,"column":7}}]}"##,
    );
}
//</coverage:exclude>
