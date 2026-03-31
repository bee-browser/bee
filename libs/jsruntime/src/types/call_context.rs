use std::ffi::c_void;

use jsgc::HandleMut;

use crate::types::Closure;
use crate::types::Coroutine;
use crate::types::Object;
use crate::types::Value;

// TODO: collect addresses stored on the stack (gc)
#[derive(Debug)]
#[repr(C)]
pub struct CallContext {
    /// A pointer to the call environment.
    ///
    /// The actual type of the value varies depending on the type of the lambda function:
    ///
    /// * Entry functions: 0 (null pointer)
    /// * Regular functions: the address of the `Closure`
    /// * Coroutine functions: the address of the `Coroutine`
    ///
    envp: *mut c_void,

    /// The `this` argument.
    this: Value,

    /// The active function object.
    #[allow(unused)]
    func: Option<HandleMut<Object>>,

    /// A pointer to the call context of the caller.
    #[allow(unused)]
    caller: *const CallContext,

    /// Flags.
    flags: CallContextFlags,

    /// The depth of the call.
    depth: u16,

    /// The number of the arguments.
    argc: u16,

    /// The maximum number of the arguments.
    argc_max: u16,

    /// A pointer to the arguments.
    argv: *mut Value,
}

impl CallContext {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const ALIGNMENT: usize = std::mem::align_of::<Self>();
    pub const ENVP_OFFSET: usize = std::mem::offset_of!(Self, envp);
    pub const THIS_OFFSET: usize = std::mem::offset_of!(Self, this);
    pub const FUNC_OFFSET: usize = std::mem::offset_of!(Self, func);
    pub const CALLER_OFFSET: usize = std::mem::offset_of!(Self, caller);
    pub const FLAGS_OFFSET: usize = std::mem::offset_of!(Self, flags);
    pub const DEPTH_OFFSET: usize = std::mem::offset_of!(Self, depth);
    pub const ARGC_OFFSET: usize = std::mem::offset_of!(Self, argc);
    pub const ARGC_MAX_OFFSET: usize = std::mem::offset_of!(Self, argc_max);
    pub const ARGV_OFFSET: usize = std::mem::offset_of!(Self, argv);

    pub(crate) fn new_for_entry(args: &mut [Value]) -> Self {
        Self {
            envp: std::ptr::null_mut(),
            this: Value::Undefined,
            func: None,
            caller: std::ptr::null(),
            flags: CallContextFlags::empty(),
            depth: 0,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_mut_ptr(),
        }
    }

    pub(crate) fn new_for_promise(coroutine: HandleMut<Coroutine>, args: &mut [Value]) -> Self {
        Self {
            envp: coroutine.as_ptr() as *mut std::ffi::c_void,
            this: Value::Undefined,
            func: None,
            caller: std::ptr::null(),
            flags: CallContextFlags::empty(),
            depth: 0,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_mut_ptr(),
        }
    }

    pub(crate) fn new_child(
        &self,
        func: HandleMut<Object>,
        closure: HandleMut<Closure>,
        args: &mut [Value],
    ) -> Self {
        Self {
            envp: closure.as_ptr() as *mut std::ffi::c_void,
            this: Value::Undefined,
            func: Some(func),
            caller: self,
            flags: CallContextFlags::empty(),
            depth: self.depth + 1,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_mut_ptr(),
        }
    }

    pub(crate) fn is_new(&self) -> bool {
        self.flags.contains(CallContextFlags::NEW)
    }

    pub(crate) fn this(&self) -> &Value {
        debug_assert!(self.this.is_valid());
        &self.this
    }

    pub(crate) fn func(&self) -> Option<HandleMut<Object>> {
        self.func
    }

    pub(crate) fn closure(&self) -> HandleMut<Closure> {
        HandleMut::from_ptr(self.envp as *mut Closure)
            .expect("must be a non-null pointer to a Closure")
    }

    #[allow(unused)]
    pub(crate) fn coroutine(&self) -> &Coroutine {
        // SAFETY: `envp` is always a non-null pointer to a `Coroutine`.
        unsafe {
            debug_assert!(!self.envp.is_null());
            debug_assert!(self.envp.is_aligned());
            &*(self.envp as *const Coroutine)
        }
    }

    pub(crate) fn coroutine_mut(&mut self) -> &mut Coroutine {
        // SAFETY: `envp` is always a non-null pointer to a `Coroutine`.
        unsafe {
            debug_assert!(!self.envp.is_null());
            debug_assert!(self.envp.is_aligned());
            &mut *(self.envp as *mut Coroutine)
        }
    }

    pub(crate) fn args(&self) -> &[Value] {
        // SAFETY: `argv` is always non-null and a valid pointer to an array of `Value`s.
        unsafe {
            debug_assert!(!self.argv.is_null());
            debug_assert!(self.argv.is_aligned());
            std::slice::from_raw_parts(self.argv as *const Value, self.argc as usize)
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct CallContextFlags: u16  {
        const NEW = 1 << 1;
    }
}
