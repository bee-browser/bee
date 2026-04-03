macro_rules! const_string {
    ($utf8:literal) => {{
        const STRING: $crate::types::String =
            $crate::types::String::new_const(base::utf16!(&$utf8));
        jsgc::Handle::from_ref(&STRING)
    }};
    ($slice:expr) => {{
        const STRING: $crate::types::String = $crate::types::String::new_const($slice);
        jsgc::Handle::from_ref(&STRING)
    }};
}

macro_rules! runtime_todo {
    ($runtime:expr, $message:literal, $retv:expr) => {
        $runtime.throw_internal_error(const_string!($message), $retv)
    };
}
