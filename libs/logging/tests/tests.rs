use paste::paste;

macro_rules! impl_test_log {
    ($flag:ident) => {
        paste! {
            #[test]
            fn [<test_ $flag>]() {
                let a = 1;
                logging::$flag!(logging::targets::TESTS, "msg");
                logging::$flag!(logging::targets::TESTS, a);
                logging::$flag!(logging::targets::TESTS, %a);
                logging::$flag!(logging::targets::TESTS, ?a);
                logging::$flag!(logging::targets::TESTS, b = 1);
                logging::$flag!(logging::targets::TESTS, b = %1);
                logging::$flag!(logging::targets::TESTS, b = ?1);
                logging::$flag!(logging::targets::TESTS, c.d = 1);
                logging::$flag!(logging::targets::TESTS, c.d = %1);
                logging::$flag!(logging::targets::TESTS, c.d = ?1);
            }
        }
    };
}

impl_test_log! { error }
impl_test_log! { warn }
impl_test_log! { info }
impl_test_log! { debug0 }
impl_test_log! { debug1 }
impl_test_log! { debug2 }
impl_test_log! { trace }
impl_test_log! { debug }
