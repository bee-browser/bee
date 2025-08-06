#[macro_export]
macro_rules! delegate_all {
    ($type:ty => $target:ty) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    ($type:ty => $target:ty, $lifetime:lifetime) => {
        impl<$lifetime> std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$lifetime> std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    ($type:ty => $field:ident: $target:ty) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($lhs:expr, $rhs:expr) => {
        std::assert_eq!($lhs, $rhs, std::stringify!($lhs == $rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::assert_eq!($lhs, $rhs, $($t)*);
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($lhs:expr, $rhs:expr) => {
        std::assert_ne!($lhs, $rhs, std::stringify!($lhs != $rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::assert_ne!($lhs, $rhs, $($t)*);
    };
}

#[macro_export]
macro_rules! debug_assert_eq {
    ($lhs:expr, $rhs:expr) => {
        std::debug_assert_eq!($lhs, $rhs, std::stringify!($lhs == $rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::debug_assert_eq!($lhs, $rhs, $($t)*);
    };
}

#[macro_export]
macro_rules! debug_assert_ne {
    ($lhs:expr, $rhs:expr) => {
        std::debug_assert_ne!($lhs, $rhs, std::stringify!($lhs != $rhs));
    };
    ($lhs:expr, $rhs:expr, $($t:tt)*) => {
        std::debug_assert_ne!($lhs, $rhs, $($t)*);
    };
}

#[macro_export]
macro_rules! static_assert_eq {
    ($lhs:expr, $rhs:expr) => {
        // assert_eq!() cannot be used as a compile-time assertion.
        const _: () = std::assert!($lhs == $rhs);
    };
}

#[macro_export]
macro_rules! static_assert_ne {
    ($lhs:expr, $rhs:expr) => {
        // assert_eq!() cannot be used as a compile-time assertion.
        const _: () = std::assert!($lhs != $rhs);
    };
}

#[macro_export]
macro_rules! static_assert_size {
    ($type:ty, $num_bytes:literal) => {
        $crate::static_assert_eq!(std::mem::size_of::<$type>(), $num_bytes);
    };
}

#[macro_export]
macro_rules! static_assert_size_eq {
    ($lhs_type:ty, $rhs_type:ty) => {
        $crate::static_assert_eq!(
            std::mem::size_of::<$lhs_type>(),
            std::mem::size_of::<$rhs_type>()
        );
    };
}

#[macro_export]
macro_rules! utf16 {
    ($utf8:literal) => {
        $crate::procmacros::utf16_array!($utf8)
    };
    (# $utf8:literal) => {
        $crate::procmacros::utf16_size!($utf8)
    };
    (& $utf8:literal) => {
        $crate::procmacros::utf16_slice!($utf8)
    };
}

#[macro_export]
macro_rules! const_utf16 {
    ($name: ident, $utf8:literal) => {
const $name: [u16; $crate::utf16!(# $utf8)] = $crate::utf16!($utf8);
    };
    ($name: ident, & $utf8:literal) => {
        const $name: &[u16] = $crate::utf16!(&$utf8);
    };
}

pub use assert_eq;
pub use assert_ne;
pub use const_utf16;
pub use debug_assert_eq;
pub use debug_assert_ne;
pub use delegate_all;
pub use static_assert_eq;
pub use static_assert_ne;
pub use static_assert_size;
pub use static_assert_size_eq;
pub use utf16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_eq() {
        assert_eq!(0, 0);
    }

    #[test]
    #[should_panic(expected = "0 == 1")]
    fn test_assert_eq_failed() {
        assert_eq!(0, 1);
    }

    #[test]
    fn test_assert_ne() {
        assert_ne!(0, 1);
    }

    #[test]
    #[should_panic(expected = "0 != 0")]
    fn test_assert_ne_failed() {
        assert_ne!(0, 0);
    }

    #[test]
    fn test_debug_assert_eq() {
        debug_assert_eq!(0, 0);
    }

    #[test]
    #[should_panic(expected = "0 == 1")]
    fn test_debug_assert_eq_failed() {
        debug_assert_eq!(0, 1);
    }

    #[test]
    fn test_debug_assert_ne() {
        debug_assert_ne!(0, 1);
    }

    #[test]
    #[should_panic(expected = "0 != 0")]
    fn test_debug_assert_ne_failed() {
        debug_assert_ne!(0, 0);
    }

    static_assert_eq!(0, 0);
    static_assert_ne!(0, 1);
    static_assert_size!(u32, 4);
    static_assert_size_eq!(u32, f32);

    #[test]
    fn test_utf16_array() {
        let actual = utf16!("test");
        // TODO(test): actual should be [u16; 4].
        let expected = "test".encode_utf16().collect::<Vec<_>>();
        assert_eq!(actual.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_utf16_size() {
        assert_eq!(utf16!(# "test"), 4);
    }

    #[test]
    fn test_utf16_slice() {
        let actual = utf16!(&"test");
        let expected = "test".encode_utf16().collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_const_utf16_array() {
        const_utf16!(ACTUAL, "test");
        let expected = "test".encode_utf16().collect::<Vec<_>>();
        assert_eq!(ACTUAL.as_slice(), expected.as_slice());
    }

    #[test]
    fn test_const_utf16_slice() {
        const_utf16!(ACTUAL, &"test");
        // TODO(test): actual should be &[u16].
        let expected = "test".encode_utf16().collect::<Vec<_>>();
        assert_eq!(ACTUAL, expected);
    }
}
