use std::ffi::c_void;

use jsgc::HandleMut;

use crate::types::Closure;
use crate::types::Coroutine;
use crate::types::Object;
use crate::types::Value;

// TODO: collect addresses stored on the stack (gc)
#[derive(Debug)]
#[repr(C)]
pub struct ExecContext {
    /// A pointer to the environment.
    ///
    /// The actual type of the value varies depending on the type of the lambda function:
    ///
    /// * Entry functions: 0 (null pointer)
    /// * Regular functions: the address of the `Closure`
    /// * Coroutine functions: the address of the `Coroutine`
    ///
    envp: *mut c_void,

    /// The `new` target.
    new_target: Option<HandleMut<Object>>,

    /// The `this` argument.
    this: Value,

    /// The active function object.
    #[allow(unused)]
    func: Option<HandleMut<Object>>,

    /// A pointer to the outer execution context.
    outer: *const ExecContext,

    /// Flags.
    flags: ExecContextFlags,

    /// The depth of the execution context stack.
    depth: u16,

    /// The number of the arguments.
    argc: u16,

    /// The maximum number of the arguments.
    argc_max: u16,

    /// A pointer to the arguments.
    // TODO(feat): arguments object
    argv: *const Value,
}

impl ExecContext {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const ALIGNMENT: usize = std::mem::align_of::<Self>();
    pub const ENVP_OFFSET: usize = std::mem::offset_of!(Self, envp);
    pub const NEW_TARGET_OFFSET: usize = std::mem::offset_of!(Self, new_target);
    pub const THIS_OFFSET: usize = std::mem::offset_of!(Self, this);
    pub const FUNC_OFFSET: usize = std::mem::offset_of!(Self, func);
    pub const OUTER_OFFSET: usize = std::mem::offset_of!(Self, outer);
    pub const FLAGS_OFFSET: usize = std::mem::offset_of!(Self, flags);
    pub const DEPTH_OFFSET: usize = std::mem::offset_of!(Self, depth);
    pub const ARGC_OFFSET: usize = std::mem::offset_of!(Self, argc);
    pub const ARGC_MAX_OFFSET: usize = std::mem::offset_of!(Self, argc_max);
    pub const ARGV_OFFSET: usize = std::mem::offset_of!(Self, argv);

    pub(crate) fn new_for_entry(args: &[Value]) -> Self {
        Self {
            envp: std::ptr::null_mut(),
            new_target: None,
            this: Value::Undefined,
            func: None,
            outer: std::ptr::null(),
            flags: ExecContextFlags::empty(),
            depth: 0,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_ptr(),
        }
    }

    pub(crate) fn new_for_promise(coroutine: HandleMut<Coroutine>, args: &[Value]) -> Self {
        Self {
            envp: coroutine.as_ptr() as *mut std::ffi::c_void,
            new_target: None,
            this: Value::Undefined,
            func: None,
            outer: std::ptr::null(),
            flags: ExecContextFlags::empty(),
            depth: 0,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_ptr(),
        }
    }

    pub(crate) fn new_child(
        &self,
        func: HandleMut<Object>,
        closure: HandleMut<Closure>,
        this: &Value,
        args: &[Value],
    ) -> Self {
        Self {
            envp: closure.as_ptr() as *mut std::ffi::c_void,
            new_target: None,
            this: this.clone(),
            func: Some(func),
            outer: self,
            flags: ExecContextFlags::empty(),
            depth: self.depth + 1,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_ptr(),
        }
    }

    pub(crate) fn new_target(&self) -> Option<HandleMut<Object>> {
        self.new_target
    }

    pub(crate) fn set_new_target(&mut self, new_target: HandleMut<Object>) {
        self.new_target = Some(new_target);
    }

    pub(crate) fn this(&self) -> &Value {
        debug_assert!(self.this.is_valid());
        &self.this
    }

    pub(crate) fn set_this(&mut self, this: Value) {
        self.this = this;
    }

    pub(crate) fn func(&self) -> Option<HandleMut<Object>> {
        self.func
    }

    pub(crate) fn set_func(&mut self, func: HandleMut<Object>) {
        self.func = Some(func);
    }

    pub(crate) fn closure(&self) -> HandleMut<Closure> {
        HandleMut::from_ptr(self.envp as *mut Closure)
            .expect("must be a non-null pointer to a Closure")
    }

    pub(crate) fn set_closure(&mut self, closure: HandleMut<Closure>) {
        self.envp = closure.as_ptr() as *mut c_void
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

    pub(crate) fn arg(&self, nth: usize) -> &Value {
        self.args().get(nth).unwrap_or(&Value::Undefined)
    }

    pub(crate) fn args(&self) -> &[Value] {
        // SAFETY: `argv` is always non-null and a valid pointer to an array of `Value`s.
        unsafe {
            debug_assert!(!self.argv.is_null());
            debug_assert!(self.argv.is_aligned());
            std::slice::from_raw_parts(self.argv, self.argc as usize)
        }
    }
}

base::auto_bitflags! {
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct ExecContextFlags: u16  {
    }
}
