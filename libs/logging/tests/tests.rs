use paste::paste;

macro_rules! impl_test_log {
    ($flag:ident) => {
        paste! {
            #[test]
            fn [<test_ $flag>]() {
                let a = 1;
                logging::$flag!(target: "bee::tests", "msg");
                logging::$flag!(target: "bee::tests", a);
                logging::$flag!(target: "bee::tests", %a);
                logging::$flag!(target: "bee::tests", ?a);
                logging::$flag!(target: "bee::tests", b = 1);
                logging::$flag!(target: "bee::tests", b = %1);
                logging::$flag!(target: "bee::tests", b = ?1);
                logging::$flag!(target: "bee::tests", c.d = 1);
                logging::$flag!(target: "bee::tests", c.d = %1);
                logging::$flag!(target: "bee::tests", c.d = ?1);
            }
        }
    };
}

impl_test_log! { error }
impl_test_log! { warn }
impl_test_log! { info }
impl_test_log! { debug }
impl_test_log! { trace }
