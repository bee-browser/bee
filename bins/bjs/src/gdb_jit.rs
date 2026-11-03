//! GDB JIT Interface control module.
//!
//! Official GDB Documentation:
//! https://sourceware.org/gdb/onlinedocs/gdb/JIT-Interface.html

use jsruntime::LambdaId;

/// Empty function where debuggers set a breakpoint to catch JIT registration events.
///
/// Corresponds to the `jit_code_register` function hook in the GDB JIT interface.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn __jit_debug_register_code() {
    unsafe {
        std::ptr::read_volatile(&raw const __jit_debug_descriptor.action_flag);
    }
}

/// Global descriptor monitored by debuggers.
///
/// Corresponds to the global descriptor instance in the GDB JIT interface.
#[unsafe(no_mangle)]
pub static mut __jit_debug_descriptor: JitDescriptor = JitDescriptor {
    version: 1,
    action_flag: 0,
    relevant_entry: std::ptr::null_mut(),
    first_entry: std::ptr::null_mut(),
};

/// Registers a JIT-compiled function with the debugger via the GDB JIT interface.
///
/// This function is application-agnostic and unconditionally registers the JIT artifact so that
/// debuggers (even if attached later) can traverse the linked list.
pub fn register_function(id: LambdaId, addr: usize, size: usize) {
    // SAFETY:
    // - `addr` must be a valid, non-null pointer to a readable block of memory allocated for
    //   JIT-compiled machine code
    // - The memory region starting at `addr` must remain valid and allocated for at least `size`
    //   bytes
    // - The memory must not be mutated while this slice reference is active
    let code = unsafe { std::slice::from_raw_parts(addr as *const u8, size) };

    // Build the ELF binary using the elf module.
    let elf_bytes = crate::elf::build_jit_elf(id, code, addr);

    // Allocate a persistent buffer for the ELF image required by the debugger.
    let elf_box = elf_bytes.into_boxed_slice();
    let symfile_addr = elf_box.as_ptr();
    let symfile_size = elf_box.len() as u64;
    std::mem::forget(elf_box);

    unsafe {
        let entry = Box::into_raw(Box::new(JitCodeEntry {
            next_entry: std::ptr::null_mut(),
            prev_entry: std::ptr::null_mut(),
            symfile_addr,
            symfile_size,
        }));

        (*entry).next_entry = __jit_debug_descriptor.first_entry;
        if !__jit_debug_descriptor.first_entry.is_null() {
            (*__jit_debug_descriptor.first_entry).prev_entry = entry;
        }
        __jit_debug_descriptor.first_entry = entry;
        __jit_debug_descriptor.relevant_entry = entry;
        __jit_debug_descriptor.action_flag = JitAction::Register as u32;

        // Notify the debugger by triggering the breakpoint hook.
        __jit_debug_register_code();

        __jit_debug_descriptor.action_flag = JitAction::NoAction as u32;
    }
}

/// Action types for the GDB JIT interface action flag.
///
/// Corresponds to `enum jit_actions_t` in the GDB JIT interface.
#[repr(u32)]
#[allow(dead_code)]
pub enum JitAction {
    NoAction = 0,
    Register = 1,
    Unregister = 2,
}

/// Global descriptor structure that tracks registered JIT code segments.
///
/// Corresponds to `struct jit_descriptor` in the GDB JIT interface.
#[repr(C)]
#[derive(Debug)]
pub struct JitDescriptor {
    pub version: u32,
    pub action_flag: u32,
    pub relevant_entry: *mut JitCodeEntry,
    pub first_entry: *mut JitCodeEntry,
}

/// Linked list entry structure representing a single JIT code segment and its symbol file.
///
/// Corresponds to `struct jit_code_entry` in the GDB JIT interface.
#[repr(C)]
#[derive(Debug)]
pub struct JitCodeEntry {
    pub next_entry: *mut JitCodeEntry,
    pub prev_entry: *mut JitCodeEntry,
    pub symfile_addr: *const u8,
    pub symfile_size: u64,
}
