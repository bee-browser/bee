macro_rules! empty {
    () => {
        crate::grammar::Term::Empty
    };
}

macro_rules! token {
    ($name:literal) => {
        crate::grammar::Term::Token($name.to_string())
    };
}

macro_rules! non_terminal {
    ($non_terminal:expr) => {
        crate::grammar::Term::NonTerminal($non_terminal.into())
    };
}

macro_rules! lookahead {
    ($phrase_set:expr) => {
        crate::grammar::Term::Lookahead(std::sync::Arc::new(crate::grammar::Lookahead::Include(
            $phrase_set,
        )))
    };
    (x: $phrase_set:expr) => {
        crate::grammar::Term::Lookahead(std::sync::Arc::new(crate::grammar::Lookahead::Exclude(
            $phrase_set,
        )))
    };
}

macro_rules! disallow {
    ($name:literal) => {
        crate::grammar::Term::Disallow($name.to_string())
    };
}

macro_rules! rule {
    ($name:literal ->) => {
        crate::grammar::Rule {
            name: $name.into(),
            production: vec![],
        }
    };
    ($name:literal -> $($term:expr) +) => {
        crate::grammar::Rule {
            name: $name.into(),
            production: vec![$($term),+],
        }
    };
}

pub(crate) use disallow;
pub(crate) use empty;
pub(crate) use lookahead;
pub(crate) use non_terminal;
pub(crate) use rule;
pub(crate) use token;
