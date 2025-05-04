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

impl Flags {
    /// Loads flags for a logging target from the `BEE_LOG` environment variable.
    pub fn load(target: &str) -> Flags {
        Self::load_from_env(target, "BEE_LOG")
    }

    /// Loads flags for a logging target from an environment variable.
    pub fn load_from_env(target: &str, name: &str) -> Flags {
        let filters = std::env::var(name).unwrap_or_default();
        Flags::load_from_str(target, &filters)
    }

    /// Loads flags for a logging target from a string.
    pub fn load_from_str(target: &str, filters: &str) -> Flags {
        filters
            .split(',')
            .map(str::trim)
            .filter(|filter| !filter.is_empty())
            .map(Filter::parse)
            .filter(|filter| filter.matches(target))
            .fold(Flags::empty(), |flags, filter| filter.fold(flags))
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Assign,
    Add,
    Remove,
}

#[derive(Debug, PartialEq)]
struct Filter<'a>(&'a str, Op, Flags);

impl<'a> Filter<'a> {
    fn parse(filter: &'a str) -> Self {
        let result = if let Some((target, flags)) = filter.split_once("+=") {
            Flags::from_str(flags).map(|flags| Self(target, Op::Add, flags))
        } else if let Some((target, flags)) = filter.split_once("-=") {
            Flags::from_str(flags).map(|flags| Self(target, Op::Remove, flags))
        } else if let Some((target, flags)) = filter.split_once('=') {
            Flags::from_str(flags).map(|flags| Self(target, Op::Assign, flags))
        } else {
            Flags::from_str(filter).map(|flags| Self("", Op::Assign, flags))
        };
        match result {
            Ok(v) => v,
            Err(_) => panic!("invalid filter: {filter}"),
        }
    }

    fn matches(&self, target: &str) -> bool {
        target.starts_with(self.0)
    }

    fn fold(&self, flags: Flags) -> Flags {
        match self.1 {
            Op::Assign => self.2,
            Op::Add => flags | self.2,
            Op::Remove => flags & !self.2,
        }
    }
}
