use std::alloc::Layout;
use std::ptr::NonNull;

use crate::Handle;

pub struct Heap {
    bump: bumpalo::Bump,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            bump: bumpalo::Bump::new(),
        }
    }

    pub fn alloc<T>(&self, value: T) -> Handle<T> {
        Handle::from_ref(self.bump.alloc(value))
    }

    pub fn alloc_layout<T, F>(&self, layout: Layout, init: F) -> Handle<T>
    where
        F: FnOnce(NonNull<u8>),
    {
        let ptr = self.bump.alloc_layout(layout);
        init(ptr);
        Handle::from_ref(unsafe { ptr.cast::<T>().as_ref() })
    }

    // TODO: return Handle
    pub fn alloc_slice_copy<T>(&self, src: &[T]) -> &mut [T]
    where
        T: Copy,
    {
        self.bump.alloc_slice_copy(src)
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
