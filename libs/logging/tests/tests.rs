logging::init!();

#[test]
fn test_error() {
    logging::error!(target: logging::targets::TESTS, "msg");
}

#[test]
fn test_warn() {
    logging::warn!(target: logging::targets::TESTS, "msg");
}

#[test]
fn test_info() {
    logging::info!(target: logging::targets::TESTS, "msg");
}

#[test]
fn test_debug0() {
    logging::debug0!(target: logging::targets::TESTS, "msg");
}

#[test]
fn test_debug1() {
    logging::debug1!(target: logging::targets::TESTS, "msg");
}

#[test]
fn test_debug2() {
    logging::debug2!(target: logging::targets::TESTS, "msg");
}

#[test]
fn test_trace() {
    logging::trace!(target: logging::targets::TESTS, "msg");
}

#[test]
fn test_debug() {
    logging::debug!(target: logging::targets::TESTS, "msg");
}
