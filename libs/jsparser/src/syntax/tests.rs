use super::*;

#[test]
fn test_to_string_value_empty() {
    assert!(matches!(to_string_value(""), Ok(v) if v.is_empty()));
}

#[test]
fn test_to_string_value_hex_digits() {
    assert!(matches!(to_string_value("\x40"), Ok(v) if v == vec![0x0040]));
}

#[test]
fn test_to_string_value_unicode_hex4() {
    assert!(matches!(to_string_value("\\u0040"), Ok(v) if v == vec![0x0040]));
}

#[test]
fn test_to_string_value_unicode_code_point() {
    assert!(matches!(to_string_value("\\u{2F804}"), Ok(v) if v == vec![0xD87E, 0xDC04]));
}
