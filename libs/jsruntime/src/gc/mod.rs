use std::cmp::PartialEq;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

/// A data type to hold a non-null pointer to a data type managed on the heap memory.
///
/// This type treats the pointee type as an opaque type and simply copy the pointer when the value
/// is cloned.
// TODO(issue#237): GcCellRef
#[derive(Eq)]
#[repr(transparent)]
pub struct Handle<T>(NonNull<T>);

base::static_assert_eq!(size_of::<Handle<u8>>(), size_of::<usize>());
base::static_assert_eq!(size_of::<Option<Handle<u8>>>(), size_of::<usize>());

impl<T> Handle<T> {
    pub const fn from_ref(r: &T) -> Self {
        Self(NonNull::from_ref(r))
    }

    pub fn from_ptr(p: *mut T) -> Option<Self> {
        NonNull::new(p).map(Handle)
    }

    pub fn from_addr(addr: usize) -> Option<Self> {
        debug_assert_ne!(addr, 0);
        debug_assert_eq!(addr % std::mem::align_of::<T>(), 0);
        Self::from_ptr(addr as *mut T)
    }

    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    pub fn as_addr(&self) -> usize {
        self.0.addr().get()
    }

    pub fn dummy_for_testing() -> Self {
        // SAFETY: it's just a dummy data for testing.
        Self(unsafe { NonNull::new_unchecked(16 as *mut T) })
    }

    fn as_ref<'a>(&self) -> &'a T {
        //debug_assert!(!self.0.as_ptr().is_null());
        debug_assert!(self.0.as_ptr().is_aligned());
        // SAFETY: `self` holds a valid pointer to `T`.
        unsafe { self.0.as_ref() }
    }

    fn as_mut<'a>(&mut self) -> &'a mut T {
        //debug_assert!(!self.0.as_ptr().is_null());
        debug_assert!(self.0.as_ptr().is_aligned());
        // SAFETY: `self` holds a valid pointer to `T`.
        unsafe { self.0.as_mut() }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> Debug for Handle<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handle({:?})", self.as_ref())
    }
}

impl<T> Display for Handle<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handle({})", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check sizes of Handle types at runtime for safety.
    // Though, these are checked w/ base::static_assert_eq!().
    #[test]
    fn test_size() {
        assert_eq!(size_of::<Handle<u8>>(), size_of::<usize>());
        assert_eq!(size_of::<Option<Handle<u8>>>(), size_of::<usize>());
    }
}
