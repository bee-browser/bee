// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/logging/scripts/logger.rs.hbs

#[allow(unused_macros)]
macro_rules! error {
    ($($tokens:tt)+) => {
        logging::error!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

// error[E0659]: `warn` is ambiguous
#[allow(unused_macros)]
macro_rules! warn_ {
    ($($tokens:tt)+) => {
        logging::warn!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

#[allow(unused_macros)]
macro_rules! info {
    ($($tokens:tt)+) => {
        logging::info!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

#[allow(unused_macros)]
macro_rules! debug0 {
    ($($tokens:tt)+) => {
        logging::debug0!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

#[allow(unused_macros)]
macro_rules! debug1 {
    ($($tokens:tt)+) => {
        logging::debug1!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

#[allow(unused_macros)]
macro_rules! debug2 {
    ($($tokens:tt)+) => {
        logging::debug2!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

#[allow(unused_macros)]
macro_rules! trace {
    ($($tokens:tt)+) => {
        logging::trace!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($tokens:tt)+) => {
        logging::debug!(target: logging::targets::ESTREE, $($tokens)+);
    };
}

#[allow(unused_imports)]
pub(crate) use debug;
#[allow(unused_imports)]
pub(crate) use debug0;
#[allow(unused_imports)]
pub(crate) use debug1;
#[allow(unused_imports)]
pub(crate) use debug2;
#[allow(unused_imports)]
pub(crate) use error;
#[allow(unused_imports)]
pub(crate) use info;
#[allow(unused_imports)]
pub(crate) use trace;
#[allow(unused_imports)]
pub(crate) use warn_ as warn;
