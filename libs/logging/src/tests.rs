use super::*;

#[test]
fn test_load_level_from_str_empty() {
    assert_eq!(load_level_from_str("a", ""), imp::LevelFilter::OFF);
    assert_eq!(load_level_from_str("b", ""), imp::LevelFilter::OFF);
}

#[test]
fn test_load_level_from_str_no_target() {
    const FILTERS: &str = "error";
    assert_eq!(load_level_from_str("a", FILTERS), imp::LevelFilter::ERROR);
    assert_eq!(load_level_from_str("b", FILTERS), imp::LevelFilter::ERROR);
}

#[test]
fn test_load_level_from_str_no_level() {
    const FILTERS: &str = "a";
    assert_eq!(load_level_from_str("a", FILTERS), imp::LevelFilter::TRACE);
    assert_eq!(load_level_from_str("b", FILTERS), imp::LevelFilter::OFF);
}

#[test]
fn test_load_level_from_str_single_filter() {
    const FILTERS: &str = "a=error";
    assert_eq!(load_level_from_str("a", FILTERS), imp::LevelFilter::ERROR);
    assert_eq!(load_level_from_str("b", FILTERS), imp::LevelFilter::OFF);
}

#[test]
fn test_load_level_from_str_multiple_targets() {
    const FILTERS: &str = "a=error,b=info";
    assert_eq!(load_level_from_str("a", FILTERS), imp::LevelFilter::ERROR);
    assert_eq!(load_level_from_str("b", FILTERS), imp::LevelFilter::INFO);
}

#[test]
fn test_load_level_from_str_override() {
    const FILTERS: &str = "a=error,a=info";
    assert_eq!(load_level_from_str("a", FILTERS), imp::LevelFilter::INFO);
}

#[test]
fn test_load_level_from_str_last_comma() {
    const FILTERS: &str = "a=error,";
    assert_eq!(load_level_from_str("a", FILTERS), imp::LevelFilter::ERROR);
}
