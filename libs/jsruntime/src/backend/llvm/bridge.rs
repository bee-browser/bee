pub fn initialize() {
    unsafe {
        llvmir_initialize();
    }
}

#[link(name = "backend-llvm")]
unsafe extern "C" {
    fn llvmir_initialize();
}
