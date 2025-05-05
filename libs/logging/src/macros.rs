#[macro_export]
macro_rules! init {
    () => {
        #[ctor::ctor]
        fn logging_init() {
            $crate::init();
        }
    };
}

#[macro_export]
macro_rules! define_logger {
    ($target:literal) => {
        logging::define_logger_inner! {$target, $}
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
    ($target:literal, $d:tt) => {
        mod logger {
            use std::sync::LazyLock;

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
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::ERROR, $($tokens)+)
    };
}

#[macro_export]
macro_rules! warn {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::WARN, $($tokens)+)
    };
}

#[macro_export]
macro_rules! info {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::INFO, $($tokens)+)
    };
}

#[macro_export]
macro_rules! debug {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::DEBUG, $($tokens)+)
    };
}

#[macro_export]
macro_rules! trace {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::TRACE, $($tokens)+)
    };
}
