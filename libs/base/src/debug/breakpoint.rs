/// Triggers a debugger breakpoint at runtime during debug builds.
///
/// This macro expands to zero-overhead code in release builds (`#[cfg(not(debug_assertions))]`).
/// In debug builds, it executes the target-specific trap instruction (e.g., `int3` on x86_64,
/// `brk #0` on aarch64). If targeted on an unsupported architecture during debug builds,
/// it causes a compile-time error via `compile_error!`.
#[macro_export]
macro_rules! breakpoint {
    () => {
        core::cfg_select! {
            not(debug_assertions) => {}
            target_arch = "x86_64" => {
                unsafe { core::arch::asm!("int3"); }
            }
            target_arch = "aarch64" => {
                unsafe { core::arch::asm!("brk #0"); }
            }
            _ => {
                compile_error!("The breakpoint! macro is not supported on this target architecture.");
            }
        }
    };
}

pub use breakpoint;
