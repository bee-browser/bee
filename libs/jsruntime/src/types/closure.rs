use jsgc::HandleMut;
use jsgc::Trace;
use jsgc::VisitList;

use crate::lambda::LambdaId;
use crate::types::Capture;
use crate::types::LambdaAddr;

/// A data type to represent a closure.
#[repr(C)]
pub struct Closure {
    /// An address of a lambda function compiled from a JavaScript function definition.
    ///
    /// This filed is initially set to a runtime function that will perform the lazy compilation of
    /// the JavaScript function and set the actual lambda function to this field.
    //
    // NOTE: Using Lambda<X> instead of LambdaAddr causes some problems.  For example, functions
    // such as `std::mem::offset_of!()` and `std::mem::align_of()` does not work with generic
    // types such as Closure<X> even though the size of Lambda<X> is always equal to the size of
    // usize regardless of the actual type of X.
    pub lambda: LambdaAddr,

    /// The ID of `lambda`.
    pub lambda_id: LambdaId,

    /// The number of captures.
    ///
    /// Usually, this field does not used in the compiled function, but we add this field here for
    /// debugging purposes.  If we need to reduce the heap memory usage and `Closure`s dominant, we
    /// can remove this field.
    pub num_captures: u16,

    /// A variable-length list of captures used in the lambda function.
    pub captures: [HandleMut<Capture>; 0],
}

base::static_assert_eq!(align_of::<Closure>(), 8);

impl Closure {
    pub(crate) const LAMBDA_OFFSET: usize = std::mem::offset_of!(Self, lambda);
    pub(crate) const CAPTURES_OFFSET: usize = std::mem::offset_of!(Self, captures);

    fn captures(&self) -> &[HandleMut<Capture>] {
        let len = self.num_captures as usize;
        let data = self.captures.as_ptr();
        // SAFETY: `data` is a non-null pointer to an array of pointers.
        unsafe {
            debug_assert!(!data.is_null());
            debug_assert!(data.is_aligned());
            std::slice::from_raw_parts(data, len)
        }
    }
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "closure({:?}, [", self.lambda_id)?;
        let mut captures = self.captures().iter();
        if let Some(capture) = captures.next() {
            write!(f, "{:?}", capture)?;
            for capture in captures {
                write!(f, ", {:?}", capture)?
            }
        }
        write!(f, "])")
    }
}

impl Trace for Closure {
    fn trace(&self, visit_list: &mut VisitList) {
        visit_list.extend(self.captures().iter().map(|capture| capture.as_addr()));
    }
}
