use super::*;

#[test]
fn test_load_flags_from_filters_empty() {
    assert_eq!(
        load_flags_from_filters("", "a", Flags::empty()),
        Flags::empty()
    );
    assert_eq!(
        load_flags_from_filters("", "b", Flags::empty()),
        Flags::empty()
    );
}

#[test]
fn test_load_flags_from_filters_no_target() {
    assert_eq!(
        load_flags_from_filters("error|warn", "a", Flags::empty()),
        Flags::ERROR | Flags::WARN
    );
    assert_eq!(
        load_flags_from_filters("error|warn", "b", Flags::empty()),
        Flags::ERROR | Flags::WARN
    );
}

#[test]
fn test_load_flags_from_filters_single_filter() {
    assert_eq!(
        load_flags_from_filters("a=error|warn", "a", Flags::empty()),
        Flags::ERROR | Flags::WARN
    );
    assert_eq!(
        load_flags_from_filters("a=error|warn", "b", Flags::empty()),
        Flags::empty()
    );
}

#[test]
fn test_load_flags_from_filters_multiple_filters() {
    assert_eq!(
        load_flags_from_filters("a=error|warn,b=info|trace", "a", Flags::empty()),
        Flags::ERROR | Flags::WARN
    );
    assert_eq!(
        load_flags_from_filters("a=error|warn,b=info|trace", "b", Flags::empty()),
        Flags::INFO | Flags::TRACE
    );
}

#[test]
fn test_load_flags_from_filters_override() {
    assert_eq!(
        load_flags_from_filters("a=error|warn,a=info|trace", "a", Flags::empty()),
        Flags::INFO | Flags::TRACE
    );
}

#[test]
fn test_load_flags_from_filters_last_comma() {
    assert_eq!(
        load_flags_from_filters("a=error|warn,", "a", Flags::empty()),
        Flags::ERROR | Flags::WARN
    );
}

#[test]
fn test_load_flags_from_filters_add() {
    assert_eq!(
        load_flags_from_filters("a+=error|warn", "a", Flags::INFO),
        Flags::ERROR | Flags::WARN | Flags::INFO
    );
}

#[test]
fn test_load_flags_from_filters_remove() {
    assert_eq!(
        load_flags_from_filters("a-=error|warn", "a", Flags::INFO | Flags::ERROR),
        Flags::INFO
    );
}

#[test]
fn test_parse_filter_empty() {
    assert_eq!(parse_filter(""), ("", Op::Assign, Flags::empty()))
}

#[test]
fn test_parse_filter_add() {
    assert_eq!(parse_filter("a+=info"), ("a", Op::Add, Flags::INFO))
}

#[test]
fn test_parse_filter_remove() {
    assert_eq!(parse_filter("a-=info"), ("a", Op::Remove, Flags::INFO))
}

#[test]
fn test_parse_filter_assign() {
    assert_eq!(parse_filter("a=info"), ("a", Op::Assign, Flags::INFO))
}

#[test]
#[should_panic(expected = "invalid filter: a==info")]
fn test_parse_filter_invalid() {
    parse_filter("a==info");
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
