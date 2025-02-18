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
macro_rules! error {
    ($target:expr, $($tokens:tt)+) => {
        if $target.error_enabled() {
            $crate::imp::event!(target: $target.name(), $crate::imp::Level::ERROR, $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($target:expr, $($tokens:tt)+) => {
        if $target.warn_enabled() {
            $crate::imp::event!(target: $target.name(), $crate::imp::Level::WARN, $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($target:expr, $($tokens:tt)+) => {
        if $target.info_enabled() {
            $crate::imp::event!(target: $target.name(), $crate::imp::Level::INFO, $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug0 {
    ($target:expr, $($tokens:tt)+) => {
        if $target.debug0_enabled() {
            $crate::imp::event!(target: $target.name(), $crate::imp::Level::DEBUG, $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug1 {
    ($target:expr, $($tokens:tt)+) => {
        if $target.debug1_enabled() {
            $crate::imp::event!(target: $target.name(), $crate::imp::Level::DEBUG, $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug2 {
    ($target:expr, $($tokens:tt)+) => {
        if $target.debug2_enabled() {
            $crate::imp::event!(target: $target.name(), $crate::imp::Level::DEBUG, $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($target:expr, $($tokens:tt)+) => {
        if $target.trace_enabled() {
            $crate::imp::event!(target: $target.name(), $crate::imp::Level::TRACE, $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($target:expr, $($tokens:tt)+) => {
        logging::debug0!($target, $($tokens)+);
    };
}
