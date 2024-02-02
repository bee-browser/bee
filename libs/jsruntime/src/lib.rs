mod llvmir;
mod logger;

pub use llvmir::Runtime;

#[no_mangle]
pub extern "C" fn print_str(s: *const std::ffi::c_char) {
    // std::ffi::CStr::from_ptr(s).to_str() is safer but slower than the following code.
    let s = unsafe { std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(s).to_bytes()) };
    println!("{s}");
}

#[no_mangle]
pub extern "C" fn print_f64(v: f64) {
    println!("{v}");
}
