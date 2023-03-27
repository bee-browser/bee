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
