use jsgc::HandleMut;
use jsgc::Trace;
use jsgc::VisitList;

use crate::types::Closure;
use crate::types::Value;

/// A data type to represent a coroutine.
///
/// Memory layout:
///
/// ```
/// +----------------------------------------+
/// | Coroutine                              |
/// +----------------------------------------+
/// | locals[Value; num_locals]              |
/// +----------------------------------------+
/// | scratch_buffer[u8; scratch_buffer_len] |
/// +----------------------------------------+
/// | capture_buffer[u8; capture_buffer_len] |
/// +----------------------------------------+
/// ```
#[derive(Debug)]
#[repr(C)]
pub struct Coroutine {
    /// The closure of the coroutine.
    pub closure: HandleMut<Closure>,

    /// The state of the coroutine.
    pub state: u32,

    /// The number of the local variables used in the coroutine.
    pub num_locals: u16,

    /// The current scope ID used by the scope cleanup checker.
    pub scope_id: u16,

    /// The size of the scratch buffer in bytes.
    pub scratch_buffer_len: u16,

    /// The size of the capture buffer in bytes.
    pub capture_buffer_len: u16,

    /// A variable-length list of local variables used in the coroutine.
    ///
    /// `Capture::target` may point to one of `locals[]`.
    pub locals: [Value; 0],
}

base::static_assert_eq!(align_of::<Coroutine>(), 8);

impl Coroutine {
    pub(crate) const CLOSURE_OFFSET: usize = std::mem::offset_of!(Self, closure);
    pub(crate) const STATE_OFFSET: usize = std::mem::offset_of!(Self, state);
    pub(crate) const NUM_LOCALS_OFFSET: usize = std::mem::offset_of!(Self, num_locals);
    pub(crate) const SCOPE_ID_OFFSET: usize = std::mem::offset_of!(Self, scope_id);
    pub(crate) const SCRATCH_BUFFER_LEN_OFFSET: usize =
        std::mem::offset_of!(Self, scratch_buffer_len);
    pub(crate) const LOCALS_OFFSET: usize = std::mem::offset_of!(Self, locals);

    fn locals(&self) -> &[Value] {
        let len = self.num_locals as usize;
        let data = self.locals.as_ptr();
        // SAFETY: `data` is a non-null pointer to an array of pointers.
        unsafe {
            debug_assert!(!data.is_null());
            debug_assert!(data.is_aligned());
            std::slice::from_raw_parts(data, len)
        }
    }
}

impl Trace for Coroutine {
    fn trace(&self, visit_list: &mut VisitList) {
        visit_list.push(self.closure.as_addr());

        for local in self.locals() {
            match local {
                Value::String(string) => visit_list.push(string.as_addr()),
                Value::Object(object) => visit_list.push(object.as_addr()),
                _ => (),
            }
        }

        // TODO: scan the scratch buffer and the capture buffer
    }
}
