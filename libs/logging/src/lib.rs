#[macro_use]
mod macros;

pub mod imp;
pub mod targets;

use std::mem::MaybeUninit;
use std::str::FromStr;

use bitflags::bitflags;

pub use ctor::ctor;

/// Initializes the log system.
pub fn init() {
    imp::init();
}

/// A logging target.
pub struct Target(usize);

impl Target {
    #[inline(always)]
    pub const fn name(&self) -> &'static str {
        targets::name(self.0)
    }

    #[inline(always)]
    pub fn error_enabled(&self) -> bool {
        FILTERS[self.0].contains(Flags::ERROR)
    }

    #[inline(always)]
    pub fn warn_enabled(&self) -> bool {
        FILTERS[self.0].contains(Flags::WARN)
    }

    #[inline(always)]
    pub fn info_enabled(&self) -> bool {
        FILTERS[self.0].contains(Flags::INFO)
    }

    #[inline(always)]
    pub fn debug0_enabled(&self) -> bool {
        FILTERS[self.0].contains(Flags::DEBUG0)
    }

    #[inline(always)]
    pub fn debug1_enabled(&self) -> bool {
        FILTERS[self.0].contains(Flags::DEBUG1)
    }

    #[inline(always)]
    pub fn debug2_enabled(&self) -> bool {
        FILTERS[self.0].contains(Flags::DEBUG2)
    }

    #[inline(always)]
    pub fn trace_enabled(&self) -> bool {
        FILTERS[self.0].contains(Flags::TRACE)
    }
}

#[ctor]
static FILTERS: [Flags; targets::len()] = {
    let default = std::env::var("BEE_LOG_DEFAULT")
        .map(|flags| match Flags::from_str(&flags) {
            Ok(flags) => flags,
            Err(_) => panic!("invalid default filter: {flags}"),
        })
        .unwrap_or(Flags::empty());
    build_filters(default, &std::env::var("BEE_LOG").unwrap_or_default())
};

fn build_filters(default: Flags, filters: &str) -> [Flags; targets::len()] {
    let filters = load_filters(filters);

    let mut data: [MaybeUninit<Flags>; targets::len()] =
        unsafe { MaybeUninit::uninit().assume_init() };

    for (i, item) in data.iter_mut().enumerate() {
        let name = Target(i).name();
        let flags = filters
            .iter()
            .filter(|(target, _, _)| name.starts_with(target))
            .cloned()
            .fold(default, |prev, (_, op, flags)| match op {
                Op::Assign => flags,
                Op::Add => prev | flags,
                Op::Remove => prev & !flags,
            });
        item.write(flags);
    }

    unsafe { std::mem::transmute::<_, [Flags; targets::len()]>(data) }
}

fn load_filters(filters: &str) -> Vec<(&str, Op, Flags)> {
    filters
        .split(',')
        .map(str::trim)
        .filter(|filter| !filter.is_empty())
        .map(|filter| {
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
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Assign,
    Add,
    Remove,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Flags: u8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_filters_empty() {
        assert_eq!(load_filters(""), vec![]);
    }

    #[test]
    fn test_load_filters_no_target() {
        assert_eq!(
            load_filters("error|warn"),
            vec![("", Op::Assign, Flags::ERROR | Flags::WARN)]
        );
    }

    #[test]
    fn test_load_filters_single_filter() {
        assert_eq!(
            load_filters("a=error|warn"),
            vec![("a", Op::Assign, Flags::ERROR | Flags::WARN)]
        );
    }

    #[test]
    fn test_load_filters_multiple_filters() {
        assert_eq!(
            load_filters("a=error|warn,b=info|trace"),
            vec![
                ("a", Op::Assign, Flags::ERROR | Flags::WARN),
                ("b", Op::Assign, Flags::INFO | Flags::TRACE)
            ]
        );
    }

    #[test]
    fn test_load_filters_last_comma() {
        assert_eq!(
            load_filters("a=error|warn,"),
            vec![("a", Op::Assign, Flags::ERROR | Flags::WARN)]
        );
    }

    #[test]
    fn test_load_filters_add() {
        assert_eq!(
            load_filters("a+=error|warn"),
            vec![("a", Op::Add, Flags::ERROR | Flags::WARN)]
        );
    }

    #[test]
    fn test_load_filters_remove() {
        assert_eq!(
            load_filters("a-=error|warn"),
            vec![("a", Op::Remove, Flags::ERROR | Flags::WARN)]
        );
    }

    #[test]
    fn test_build_filters_default() {
        let filters = build_filters(Flags::ERROR | Flags::WARN, "");
        assert_eq!(filters[targets::TESTS.0], Flags::ERROR | Flags::WARN);
    }

    #[test]
    fn test_build_filters_inherit() {
        let filters = build_filters(Flags::empty(), "bee=error|warn");
        assert_eq!(filters[targets::TESTS.0], Flags::ERROR | Flags::WARN);
    }

    #[test]
    fn test_build_filters_assign() {
        let filters = build_filters(Flags::ERROR | Flags::WARN, "bee::tests=info");
        assert_eq!(filters[targets::TESTS.0], Flags::INFO);
    }

    #[test]
    fn test_build_filters_add() {
        let filters = build_filters(Flags::ERROR | Flags::WARN, "bee::tests+=info");
        assert_eq!(
            filters[targets::TESTS.0],
            Flags::ERROR | Flags::WARN | Flags::INFO
        );
    }

    #[test]
    fn test_build_filters_remove() {
        let filters = build_filters(Flags::ERROR | Flags::WARN, "bee::tests-=warn");
        assert_eq!(filters[targets::TESTS.0], Flags::ERROR);
    }

    #[test]
    fn test_build_filters_others() {
        let filters = build_filters(Flags::ERROR | Flags::WARN, "bee::x=info");
        assert_eq!(filters[targets::TESTS.0], Flags::ERROR | Flags::WARN);
    }

    #[test]
    #[should_panic(expected = "invalid filter: invalid")]
    fn test_load_filters_invalid() {
        load_filters("invalid");
    }

    #[test]
    #[should_panic(expected = "invalid filter: a+=invalid")]
    fn test_load_filters_invalid_add() {
        load_filters("a+=invalid");
    }

    #[test]
    #[should_panic(expected = "invalid filter: a-=invalid")]
    fn test_load_filters_invalid_remove() {
        load_filters("a-=invalid");
    }

    #[test]
    #[should_panic(expected = "invalid filter: a=invalid")]
    fn test_load_filters_invalid_assign() {
        load_filters("a=invalid");
    }

    #[test]
    fn test_flags_parse_empty() {
        assert_eq!("".parse(), Ok(Flags::empty()));
    }

    #[test]
    fn test_flags_parse_error() {
        assert_eq!("error".parse(), Ok(Flags::ERROR));
    }

    #[test]
    fn test_flags_parse_warn() {
        assert_eq!("warn".parse(), Ok(Flags::WARN));
    }

    #[test]
    fn test_flags_parse_info() {
        assert_eq!("info".parse(), Ok(Flags::INFO));
    }

    #[test]
    fn test_flags_parse_debug0() {
        assert_eq!("debug0".parse(), Ok(Flags::DEBUG0));
    }

    #[test]
    fn test_flags_parse_debug1() {
        assert_eq!("debug1".parse(), Ok(Flags::DEBUG1));
    }

    #[test]
    fn test_flags_parse_debug2() {
        assert_eq!("debug2".parse(), Ok(Flags::DEBUG2));
    }

    #[test]
    fn test_flags_parse_trace() {
        assert_eq!("trace".parse(), Ok(Flags::TRACE));
    }

    #[test]
    fn test_flags_parse_off() {
        assert_eq!("off".parse(), Ok(Flags::empty()));
    }

    #[test]
    fn test_flags_parse_all() {
        assert_eq!("all".parse(), Ok(Flags::all()));
    }

    #[test]
    fn test_flags_parse_debug() {
        assert_eq!(
            "debug".parse(),
            Ok(Flags::DEBUG0 | Flags::DEBUG1 | Flags::DEBUG2)
        );
    }

    #[test]
    fn test_flags_parse_error_warn() {
        assert_eq!("error|warn".parse(), Ok(Flags::ERROR | Flags::WARN));
    }

    #[test]
    fn test_flags_parse_info_empty() {
        assert_eq!("info|".parse(), Ok(Flags::INFO));
    }

    #[test]
    fn test_flags_parse_trim() {
        assert_eq!(" error | warn ".parse(), Ok(Flags::ERROR | Flags::WARN));
    }

    #[test]
    fn test_flags_parse_unknown() {
        assert_eq!(Flags::from_str("unknown"), Err(()));
    }

    #[test]
    fn test_flags_parse_info_unknown() {
        assert_eq!(Flags::from_str("info|unknown"), Err(()));
    }
}
