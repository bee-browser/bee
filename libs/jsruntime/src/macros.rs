macro_rules! const_string {
    ($utf8:literal) => {{
        const STRING: $crate::types::StringFragment =
            $crate::types::StringFragment::new_const(base::utf16!(&$utf8));
        $crate::types::StringHandle::new_const(&STRING)
    }};
    ($slice:expr) => {{
        const STRING: $crate::types::StringFragment =
            $crate::types::StringFragment::new_const($slice);
        $crate::types::StringHandle::new_const(&STRING)
    }};
}

macro_rules! runtime_todo {
    ($runtime:expr, $message:literal, $retv:expr) => {
        $runtime.throw_internal_error(const_string!($message), $retv)
    };
}
