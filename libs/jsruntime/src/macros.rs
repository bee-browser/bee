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

macro_rules! runtime_todo {
    ($message:literal) => {
        Err($crate::Error::InternalError(Some(const_string_handle!(
            $message
        ))))
    };
}
