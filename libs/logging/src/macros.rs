#[macro_export]
macro_rules! init {
    () => {
        #[ctor::ctor]
        fn logging_init() {
            $crate::init();
        }
    };
}

/// Defines the `logger` module inside the current module.
///
/// The current module path is used as the logging target if no logging target is specified.
#[macro_export]
macro_rules! define_logger {
    () => {
        $crate::define_logger_inner! {::std::concat!("bee::", ::std::module_path!()), $}
    };
    ($target:path) => {
        $crate::define_logger_inner! {::std::stringify!($target), $}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_prefix {
    ($target:expr) => {
        const _: () = {
            let s = $target.as_bytes();
            if s.len() < 5
                || s[0] != b'b'
                || s[1] != b'e'
                || s[2] != b'e'
                || s[3] != b':'
                || s[4] != b':'
            {
                panic!(r#"the loggin target must starts with "bee::""#);
            }
        };
    };
}

// See https://github.com/rust-lang/rust/issues/35853 for the reason why we define
// the `define_logger_inner` macro.
//
// Limitation: It's assumed that `logger` defined in the current module scope.  We cannot use
// `$crate` in macro definitions in this macro.  `$crate` always expands to `logging` even if we
// replace `$crate` with `$d crate`.
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! define_logger_inner {
    ($target:expr, $d:tt) => {
        $crate::assert_prefix!($target);

        mod logger {
            use ::std::sync::LazyLock;

            use $crate::imp::Level;
            use $crate::imp::LevelFilter;

            static LEVEL: LazyLock<LevelFilter> = LazyLock::new(|| $crate::load_level($target));

            #[allow(unused)]
            #[inline(always)]
            pub(crate) fn error_enabled() -> bool {
                *LEVEL >= Level::ERROR
            }

            #[allow(unused)]
            #[inline(always)]
            pub(crate) fn warn_enabled() -> bool {
                *LEVEL >= Level::WARN
            }

            #[allow(unused)]
            #[inline(always)]
            pub(crate) fn info_enabled() -> bool {
                *LEVEL >= Level::INFO
            }

            #[allow(unused)]
            #[inline(always)]
            pub(crate) fn debug_enabled() -> bool {
                *LEVEL >= Level::DEBUG
            }

            #[allow(unused)]
            #[inline(always)]
            pub(crate) fn trace_enabled() -> bool {
                *LEVEL >= Level::TRACE
            }

            #[allow(unused_macros)]
            macro_rules! error {
                ($d ($d tokens:tt)+) => {
                    if logger::error_enabled() {
                        logging::error!(target: $target, $d ($d tokens)+);
                    }
                };
            }

            #[allow(unused_macros)]
            macro_rules! warn_ {
                ($d ($d tokens:tt)+) => {
                    if logger::warn_enabled() {
                        logging::warn!(target: $target, $d ($d tokens)+);
                    }
                };
            }

            #[allow(unused_macros)]
            macro_rules! info {
                ($d ($d tokens:tt)+) => {
                    if logger::info_enabled() {
                        logging::info!(target: $target, $d ($d tokens)+);
                    }
                };
            }

            #[allow(unused_macros)]
            macro_rules! debug {
                ($d ($d tokens:tt)+) => {
                    if logger::debug_enabled() {
                        logging::debug!(target: $target, $d ($d tokens)+);
                    }
                };
            }

            #[allow(unused_macros)]
            macro_rules! trace {
                ($d ($d tokens:tt)+) => {
                    if logger::trace_enabled() {
                        logging::trace!(target: $target, $d ($d tokens)+);
                    }
                };
            }

            #[allow(unused_imports)]
            pub(crate) use debug;
            #[allow(unused_imports)]
            pub(crate) use error;
            #[allow(unused_imports)]
            pub(crate) use info;
            #[allow(unused_imports)]
            pub(crate) use trace;
            #[allow(unused_imports)]
            pub(crate) use warn_ as warn;
        }
    };
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::ERROR, $($tokens)+)
    };
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::WARN, $($tokens)+)
    };
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::INFO, $($tokens)+)
    };
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::DEBUG, $($tokens)+)
    };
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::TRACE, $($tokens)+)
    };
}
