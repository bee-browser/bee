macro_rules! mode {
    ($mode:ident) => {
        InsertionMode::$mode
    };
    ($mode:ident, $($more:ident),+) => {
        mode!($mode) | mode!($($more),+)
    };
}

macro_rules! tag {
    ($tag:ident) => {
        crate::local_names::LocalName::$tag
    };
    ($tag:ident, $($more:ident),+) => {
        tag!($tag) | tag!($($more),+)
    };
    (svg: $tag:ident) => {
        tag!($tag)
    };
    (svg: $tag:ident, $($more:ident),+) => {
        tag!(svg: $tag) | tag!(svg: $($more),+)
    };
    (mathml: $tag:ident) => {
        tag!($tag)
    };
    (mathml: $tag:ident, $($more:ident),+) => {
        tag!(mathml: $tag) | tag!(mathml: $($more),+)
    };
}

macro_rules! tags {
    ($tag:ident) => {
        tag!($tag)
    };
    ($($tags:ident,)+) => {
        tags!($($tags:ident),+)
    };
    ($($tags:ident),+) => {
        [$(tags!($tags)),+]
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
