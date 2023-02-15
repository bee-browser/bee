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
        TagKind::Html(bee_htmltags::HtmlTag::$tag)
    };
    ($tag:ident, $($more:ident),+) => {
        tag!($tag) | tag!($($more),+)
    };
}
