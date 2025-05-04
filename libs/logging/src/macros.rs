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
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::ERROR, $($tokens)+);
    };
}

#[macro_export]
macro_rules! warn {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::WARN, $($tokens)+);
    };
}

#[macro_export]
macro_rules! info {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::INFO, $($tokens)+);
    };
}

#[macro_export]
macro_rules! debug {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::DEBUG, $($tokens)+);
    };
}

#[macro_export]
macro_rules! trace {
    (target: $target:literal, $($tokens:tt)+) => {
        $crate::imp::event!(target: $target, $crate::imp::Level::TRACE, $($tokens)+);
    };
}
