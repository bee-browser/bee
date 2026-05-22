macro_rules! const_string {
    () => {
        &$crate::types::String::EMPTY
    };
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
    () => {
        jsgc::Handle::from_ref(const_string!())
    };
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
    ($runtime:expr, $retv:expr) => {{
        *$retv = $crate::Value::Object($runtime.create_type_error(None));
        $crate::Status::Exception
    }};
    ($runtime:expr, $retv:expr, $message:literal) => {{
        *$retv = $crate::Value::Object($runtime.create_type_error(Some(const_string!($message))));
        $crate::Status::Exception
    }};
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
