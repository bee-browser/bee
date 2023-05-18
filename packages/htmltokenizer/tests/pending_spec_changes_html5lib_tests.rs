//<coverage:exclude>
mod helper;

use helper::tokenize;

#[test]
fn test_0000() {
    tokenize(
        r##"{"description":"<!---- >","initialState":"Data","input":"<!---- >","inputUtf16":[60,33,45,45,45,45,32,62],"output":[{"Comment":{"data":"-- >"}}],"errors":[{"code":"eof-in-comment","location":{"line":1,"column":9}}]}"##,
    );
}
//</coverage:exclude>