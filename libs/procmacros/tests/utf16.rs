use procmacros::utf16_array;
use procmacros::utf16_size;
use procmacros::utf16_slice;

#[test]
fn test_size() {
    assert_eq!(utf16_size!("test"), 4);
}

#[test]
fn test_array() {
    let actual = utf16_array!("test");
    // TODO(test): actual should be [u16; 4].
    let expected = "test".encode_utf16().collect::<Vec<_>>();
    assert_eq!(actual.as_slice(), expected.as_slice());
}

#[test]
fn test_const_array() {
    const ACTUAL: [u16; utf16_size!("test")] = utf16_array!("test");
    let expected = "test".encode_utf16().collect::<Vec<_>>();
    assert_eq!(ACTUAL.as_slice(), expected.as_slice());
}

#[test]
fn test_slice() {
    let actual = utf16_slice!("test");
    let expected = "test".encode_utf16().collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn test_const_slice() {
    const ACTUAL: &[u16] = utf16_slice!("test");
    // TODO(test): actual should be &[u16].
    let expected = "test".encode_utf16().collect::<Vec<_>>();
    assert_eq!(ACTUAL, expected);
}
