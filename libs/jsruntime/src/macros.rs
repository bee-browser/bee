macro_rules! const_string {
    ($utf8:literal) => {{
        const STRING: $crate::types::String =
            $crate::types::String::new_const(base::utf16!(&$utf8));
        &STRING
    }};
    ($slice:expr) => {{
        const STRING: $crate::types::String = $crate::types::String::new_const($slice);
        &STRING
    }};
}

macro_rules! const_string_handle {
    ($utf8:literal) => {
        jsgc::Handle::from_ref(const_string!($utf8))
    };
    ($slice:expr) => {
        jsgc::Handle::from_ref(const_string!($slice))
    };
}

macro_rules! syntax_error {
    () => {
        Err($crate::Error::new($crate::ErrorKind::SyntaxError, None))
    };
    ($message:literal) => {
        Err($crate::Error::new(
            $crate::ErrorKind::SyntaxError,
            Some(const_string!($message)),
        ))
    };
}

macro_rules! error {
    ($kind:expr) => {
        Err($crate::Error::new($kind, None))
    };
    ($kind:expr, $message:literal) => {
        Err($crate::Error::new($kind, Some(const_string!($message))))
    };
}

macro_rules! type_error {
    () => {
        error!($crate::ErrorKind::TypeError)
    };
    ($message:literal) => {
        error!($crate::ErrorKind::TypeError, $message)
    };
}

macro_rules! range_error {
    () => {
        error!($crate::ErrorKind::RangeError)
    };
    ($message:literal) => {
        error!($crate::ErrorKind::RangeError, $message)
    };
}

macro_rules! runtime_todo {
    () => {
        error!($crate::ErrorKind::InternalError)
    };
    ($message:literal) => {
        error!($crate::ErrorKind::InternalError, $message)
    };
}
