macro_rules! mode {
    ($mode:ident) => {
        InsertionMode::$mode
    };
    ($mode:ident, $($more:ident),+) => {
        mode!($mode) | mode!($($more),+)
    };
}

macro_rules! flags {
    ($flag:ident) => {
        TreeBuildFlags::$flag
    };
    ($flag:ident, $($more:ident),+) => {
        flags!($flag) | flags!($($more),+)
    };
    ($($more:ident),+ $(,)?) => {
        flags!($($more),+)
    };
}

macro_rules! char_class {
    ($c:literal) => {
        $c
    };
    ($c:literal, $($more:literal),+) => {
        char_class!($c) | char_class!($($more),+)
    };
    (whitespace) => {
        char_class!['\u{0009}', '\u{000A}', '\u{000C}', '\u{000D}', '\u{0020}']
    };
}
