macro_rules! lr_item {
    ($rule:expr, $dot:expr, $lookahead:expr) => {
        crate::lr::LrItem {
            rule: std::sync::Arc::new($rule),
            dot: $dot,
            lookahead: $lookahead,
        }
    };
}

macro_rules! lr0_item {
    ($rule:expr, $dot:expr) => {
        lr_item!($rule, $dot, crate::phrase::Phrase::empty())
    };
}

pub(crate) use lr0_item;
pub(crate) use lr_item;
