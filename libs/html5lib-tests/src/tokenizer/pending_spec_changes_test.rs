//<coverage:exclude>
use super::helper::tokenize;

#[test]
fn test_0() {
    tokenize(
        r##"{"description":"<!---- >","initialState":"Data","input":"<!---- >","inputUtf16":[60,33,45,45,45,45,32,62],"output":[{"Comment":{"data":"-- >"}}],"errors":[{"code":"EofInComment","location":{"line":1,"column":9}}]}"##,
    );
}
//</coverage:exclude>
