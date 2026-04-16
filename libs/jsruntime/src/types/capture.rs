use std::mem::offset_of;
use std::ptr::addr_eq;

use jsgc::Trace;
use jsgc::VisitList;

use crate::types::Value;

/// A data type to track a captured value.
//
// NOTE: The `target` may point to the `escaped`.  In this case, the `target` must be updated if
// the capture is moved during GC, so that the `target` points to the `escaped` correctly.
#[repr(C)]
pub struct Capture {
    /// A captured value.
    ///
    /// This may point to the `escaped`.
    pub target: *mut Value,

    /// Data storage for escaped value.
    pub escaped: Value,
}

base::static_assert_eq!(size_of::<Capture>(), 24);
base::static_assert_eq!(align_of::<Capture>(), 8);
base::static_assert_eq!(offset_of!(Capture, escaped), 8);

impl Capture {
    pub(crate) const TARGET_OFFSET: usize = std::mem::offset_of!(Self, target);
    pub(crate) const ESCAPED_OFFSET: usize = std::mem::offset_of!(Self, escaped);

    fn is_escaped(&self) -> bool {
        debug_assert!(!self.target.is_null());
        addr_eq(self.target, &self.escaped)
    }
}

impl std::fmt::Debug for Capture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_escaped() {
            write!(
                f,
                "capture(escaped: {:?}, value: {:?})",
                self.target, self.escaped
            )
        } else {
            write!(f, "capture(onstack: {:?})", self.target)
        }
    }
}

impl Trace for Capture {
    fn trace(&self, visit_list: &mut VisitList) {
        if !self.is_escaped() {
            return;
        }

        match self.escaped {
            Value::String(string) => visit_list.push(string.as_addr()),
            Value::Object(object) => visit_list.push(object.as_addr()),
            _ => (),
        }
    }
}
