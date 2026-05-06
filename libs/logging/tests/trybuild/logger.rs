logging::define_logger! {}

macro_rules! test_logger {
    ($flag:ident) => {{
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
    }};
}

fn main() {
    test_logger!(error);
    test_logger!(warn);
    test_logger!(info);
    test_logger!(debug);
    test_logger!(trace);
}
