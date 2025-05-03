#[macro_use]
mod macros;

// NOTE: Crates using this crate will rebuild when this file changes even if only code inside the
// `tests` module changes.  For avoiding the meaningless rebuilds, separate the `tests` module to
// tests.rs.
#[cfg(test)]
mod tests;

pub mod imp;

use std::str::FromStr;

use bitflags::bitflags;

/// Initializes the log system.
///
/// This function must be called only once in a process before calling any functions that use
/// logging macros.
///
/// This function is **NOT** thread-safe.  So, it's strongly recommended to call this function
/// before creating any threads other than the main thread.
pub fn init() {
    imp::init();
}

pub fn load_flags(target: &str) -> Flags {
    let default = std::env::var("BEE_LOG_DEFAULT")
        .map(|flags| match Flags::from_str(&flags) {
            Ok(flags) => flags,
            Err(_) => panic!("invalid default filter: {flags}"),
        })
        .unwrap_or(Flags::empty());
    let filters = std::env::var("BEE_LOG").unwrap_or_default();
    load_flags_from_filters(&filters, target, default)
}

fn load_flags_from_filters(filters: &str, target: &str, default: Flags) -> Flags {
    filters
        .split(',')
        .map(str::trim)
        .filter(|filter| !filter.is_empty())
        .map(parse_filter)
        .filter(|(filter_target, _, _)| target.starts_with(filter_target))
        .fold(default, |prev, (_, op, flags)| match op {
            Op::Assign => flags,
            Op::Add => prev | flags,
            Op::Remove => prev & !flags,
        })
}

fn parse_filter(filter: &str) -> (&str, Op, Flags) {
    let result = if let Some((target, flags)) = filter.split_once("+=") {
        Flags::from_str(flags).map(|flags| (target, Op::Add, flags))
    } else if let Some((target, flags)) = filter.split_once("-=") {
        Flags::from_str(flags).map(|flags| (target, Op::Remove, flags))
    } else if let Some((target, flags)) = filter.split_once('=') {
        Flags::from_str(flags).map(|flags| (target, Op::Assign, flags))
    } else {
        Flags::from_str(filter).map(|flags| ("", Op::Assign, flags))
    };
    match result {
        Ok(v) => v,
        Err(_) => panic!("invalid filter: {filter}"),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Assign,
    Add,
    Remove,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Flags: u8 {
        const ERROR  = 0b00000001;
        const WARN   = 0b00000010;
        const INFO   = 0b00000100;
        const DEBUG0 = 0b00001000;
        const DEBUG1 = 0b00010000;
        const DEBUG2 = 0b00100000;
        const TRACE  = 0b01000000;
    }
}

impl FromStr for Flags {
    type Err = ();

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        flags
            .split('|')
            .map(str::trim)
            .filter(|flag| !flag.is_empty())
            .try_fold(Flags::empty(), |flags, flag| {
                Ok(flags
                    | match flag {
                        "error" => Flags::ERROR,
                        "warn" => Flags::WARN,
                        "info" => Flags::INFO,
                        "debug0" => Flags::DEBUG0,
                        "debug1" => Flags::DEBUG1,
                        "debug2" => Flags::DEBUG2,
                        "trace" => Flags::TRACE,
                        "off" => Flags::empty(),
                        "all" => Flags::all(),
                        "debug" => Flags::DEBUG0 | Flags::DEBUG1 | Flags::DEBUG2,
                        _ => return Err(()),
                    })
            })
    }
}
