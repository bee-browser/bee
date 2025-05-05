use paste::paste;

logging::define_logger! {"bee::tests"}

macro_rules! impl_test_log {
    ($flag:ident) => {
        paste! {
            #[test]
            fn [<test_ $flag>]() {
                let a = 1;
                logger::$flag!("msg");
                logger::$flag!(a);
                logger::$flag!(%a);
                logger::$flag!(?a);
                logger::$flag!(b = 1);
                logger::$flag!(b = %1);
                logger::$flag!(b = ?1);
                logger::$flag!(c.d = 1);
                logger::$flag!(c.d = %1);
                logger::$flag!(c.d = ?1);
            }
        }
    };
}

impl_test_log! { error }
impl_test_log! { warn }
impl_test_log! { info }
impl_test_log! { debug }
impl_test_log! { trace }
