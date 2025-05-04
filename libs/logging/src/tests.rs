use super::*;

#[test]
fn test_flags_load_from_str_empty() {
    assert_eq!(Flags::load_from_str("a", ""), Flags::empty());
    assert_eq!(Flags::load_from_str("b", ""), Flags::empty());
}

#[test]
fn test_flags_load_from_str_no_target() {
    const FILTERS: &str = "error|warn";
    assert_eq!(Flags::load_from_str("a", FILTERS), Flags::ERROR | Flags::WARN);
    assert_eq!(Flags::load_from_str("b", FILTERS), Flags::ERROR | Flags::WARN);
}

#[test]
fn test_flags_load_from_str_single_filter() {
    const FILTERS: &str = "a=error|warn";
    assert_eq!(Flags::load_from_str("a", FILTERS), Flags::ERROR | Flags::WARN);
    assert_eq!(Flags::load_from_str("b", FILTERS), Flags::empty());
}

#[test]
fn test_flags_load_from_str_multiple_targets() {
    const FILTERS: &str = "a=error|warn,b=info|trace";
    assert_eq!(Flags::load_from_str("a", FILTERS), Flags::ERROR | Flags::WARN);
    assert_eq!(Flags::load_from_str("b", FILTERS), Flags::INFO | Flags::TRACE);
}

#[test]
fn test_flags_load_from_str_folding_assign() {
    const FILTERS: &str = "a=error|warn,a=info|trace";
    assert_eq!(Flags::load_from_str("a", FILTERS), Flags::INFO | Flags::TRACE);
}

#[test]
fn test_flags_load_from_str_folding_add() {
    const FILTERS: &str = "a=info,a+=warn";
    assert_eq!(Flags::load_from_str("a", FILTERS), Flags::WARN | Flags::INFO);
}

#[test]
fn test_flags_load_from_str_folding_remove() {
    const FILTERS: &str = "a=error|info,a-=error|warn";
    assert_eq!(Flags::load_from_str("a", FILTERS), Flags::INFO);
}

#[test]
fn test_flags_load_from_str_last_comma() {
    const FILTERS: &str = "a=error|warn,";
    assert_eq!(Flags::load_from_str("a", FILTERS), Flags::ERROR | Flags::WARN);
}

#[test]
fn test_filter_parse_empty() {
    assert_eq!(Filter::parse(""), Filter("", Op::Assign, Flags::empty()));
}

#[test]
fn test_filter_parse_assign() {
    assert_eq!(Filter::parse("a=info"), Filter("a", Op::Assign, Flags::INFO));
}

#[test]
fn test_filter_parse_add() {
    assert_eq!(Filter::parse("a+=info"), Filter("a", Op::Add, Flags::INFO));
}

#[test]
fn test_filter_parse_remove() {
    assert_eq!(Filter::parse("a-=info"), Filter("a", Op::Remove, Flags::INFO));
}

#[test]
#[should_panic(expected = "invalid filter: a==info")]
fn test_filter_parse_invalid() {
    Filter::parse("a==info");
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
