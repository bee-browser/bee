#[macro_export]
macro_rules! init {
    () => {
        #[logging::ctor]
        fn logging_init() {
            logging::init();
        }
    };
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($tokens:tt)+) => {
        if $target.error_enabled() {
            logging::imp::error!(target: $target.name(), $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($tokens:tt)+) => {
        if $target.warn_enabled() {
            logging::imp::warn!(target: $target.name(), $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($tokens:tt)+) => {
        if $target.info_enabled() {
            logging::imp::info!(target: $target.name(), $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug0 {
    (target: $target:expr, $($tokens:tt)+) => {
        if $target.debug0_enabled() {
            logging::imp::debug!(target: $target.name(), $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug1 {
    (target: $target:expr, $($tokens:tt)+) => {
        if $target.debug1_enabled() {
            logging::imp::debug!(target: $target.name(), $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug2 {
    (target: $target:expr, $($tokens:tt)+) => {
        if $target.debug2_enabled() {
            logging::imp::debug!(target: $target.name(), $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($tokens:tt)+) => {
        if $target.trace_enabled() {
            logging::imp::trace!(target: $target.name(), $($tokens)+);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($tokens:tt)+) => {
        logging::debug0!($($tokens)+);
    };
}
