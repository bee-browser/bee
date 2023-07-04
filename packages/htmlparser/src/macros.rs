macro_rules! tag {
    ($tag:ident) => {
        crate::localnames::LocalName::$tag
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
        tags!($($tags),+)
    };
    ($($tags:ident),+) => {
        [$(tags!($tags)),+]
    };
}
