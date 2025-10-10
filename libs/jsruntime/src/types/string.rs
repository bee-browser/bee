use std::ops::Index;
use std::ptr::NonNull;

use itertools::Itertools;

/// A data type to hold an **immutable** UTF-16 string.
///
/// A UTF-16 string is represented as a *chain* of **immutable** fragments of UTF-16 code units.
///
/// This type is usually allocated on the stack and holds a pointer to a `StringFragment` that is
/// allocated in the heap or on the stack.
// TODO(issue#237): GcCell
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct StringHandle(NonNull<StringFragment>);

static_assertions::const_assert_eq!(align_of::<StringHandle>(), align_of::<usize>());

impl StringHandle {
    /// An empty string.
    pub const EMPTY: Self = Self::new(&StringFragment::EMPTY);

    /// Creates a new UTF-16 string.
    pub const fn new(frag: &StringFragment) -> Self {
        Self(NonNull::from_ref(frag))
    }

    /// Creates a new constant UTF-16 string.
    pub const fn new_const(frag: &'static StringFragment) -> Self {
        debug_assert!(frag.is_const());
        Self(NonNull::from_ref(frag))
    }

    /// Returns `true` if the string is empty.
    pub const fn is_empty(&self) -> bool {
        self.fragment().is_empty()
    }

    pub(crate) fn is_const(&self) -> bool {
        let frag = self.fragment();
        frag.is_const() && frag.next().is_none()
    }

    /// Returns `true` if the string is allocated on the stack.
    pub(crate) fn on_stack(&self) -> bool {
        self.fragment().on_stack()
    }

    /// Returns the number of UTF-16 code units in the string.
    pub fn len(&self) -> u32 {
        self.fragment().total_len()
    }

    /// Returns the first string fragment.
    pub(crate) const fn fragment(&self) -> &StringFragment {
        // SAFETY: `self.0` is always convertible to a reference.
        unsafe { self.0.as_ref() }
    }

    pub(crate) fn code_units(&self) -> impl Iterator<Item = u16> {
        self.fragment().code_units()
    }

    /// Creates a `Vec` containing UTF-16 code units of the string.
    pub(crate) fn make_utf16(&self) -> Vec<u16> {
        self.code_units().collect_vec()
    }

    pub(crate) unsafe fn from_addr(addr: usize) -> Self {
        debug_assert_ne!(addr, 0);
        Self::new(unsafe {
            let ptr = addr as *const StringFragment;
            debug_assert!(ptr.is_aligned());
            &*ptr
        })
    }

    pub(crate) fn as_addr(&self) -> usize {
        self.0.addr().get()
    }

    // 6.1.4.1 StringIndexOf ( string, searchValue, fromIndex )
    pub fn index_of(&self, search_value: Self, from_index: u32) -> Option<u32> {
        // TODO(perf): slow and inefficient
        let len = self.len();
        if search_value.is_empty() && from_index <= len {
            return Some(from_index);
        }
        let search_len = search_value.len();
        if len < search_len {
            return None;
        }
        let string = self.make_utf16();
        let search = search_value.make_utf16();
        for i in from_index..(len - search_len + 1) {
            let canditate = &string[(i as usize)..((i + search_len) as usize)];
            if canditate == search {
                return Some(i);
            }
        }
        None
    }
}

impl PartialEq for StringHandle {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }
        self.fragment() == other.fragment()
    }
}

impl std::fmt::Debug for StringHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "StringHandle()")
        } else {
            write!(f, "StringHandle({:?})", self.fragment())
        }
    }
}

impl std::fmt::Display for StringHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            Ok(())
        } else {
            write!(f, "{}", self.fragment())
        }
    }
}

/// A data type representing an **immutable** fragment of UTF-16 code units.
///
/// This type may be allocated on the stack.
// TODO(issue#237): GcCell
#[derive(Clone)]
#[repr(C)]
pub struct StringFragment {
    /// A pointer to the next string fragment if it exists.
    next: *const StringFragment,

    /// A pointer to the array of UTF-16 code units if it exists.
    ptr: *const u16,

    /// The number of the UTF-16 code units in the string fragment.
    len: u32,

    kind: StringFragmentKind,
}

static_assertions::const_assert_eq!(align_of::<StringFragment>(), align_of::<usize>());

impl StringFragment {
    pub(crate) const EMPTY: Self = Self::new_const_from_raw_parts(std::ptr::null(), 0);

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const NEXT_OFFSET: usize = std::mem::offset_of!(Self, next);
    pub(crate) const PTR_OFFSET: usize = std::mem::offset_of!(Self, ptr);
    pub(crate) const LEN_OFFSET: usize = std::mem::offset_of!(Self, len);
    pub(crate) const KIND_OFFSET: usize = std::mem::offset_of!(Self, kind);

    // TODO(refactor): should be private
    pub const fn new_const(slice: &'static [u16]) -> Self {
        Self::new_const_from_raw_parts(slice.as_ptr(), slice.len() as u32)
    }

    pub(crate) const fn new_stack(slice: &[u16]) -> Self {
        Self::new_stack_from_raw_parts(slice.as_ptr(), slice.len() as u32)
    }

    pub(crate) const fn new_const_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: StringFragmentKind::Const,
        }
    }

    pub(crate) const fn new_stack_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: StringFragmentKind::Stack,
        }
    }

    pub(crate) const fn new_heap_from_raw_parts(
        next: *const Self,
        ptr: *const u16,
        len: u32,
    ) -> Self {
        Self {
            next,
            ptr,
            len,
            kind: StringFragmentKind::Heap,
        }
    }

    pub(crate) const fn is_empty(&self) -> bool {
        debug_assert!(self.len > 0 || self.next.is_null());
        self.len == 0
    }

    pub(crate) const fn is_const(&self) -> bool {
        matches!(self.kind, StringFragmentKind::Const)
    }

    pub(crate) const fn on_stack(&self) -> bool {
        matches!(self.kind, StringFragmentKind::Stack)
    }

    pub(crate) fn total_len(&self) -> u32 {
        // SAFETY: `self.next` is null or a valid pointer to a `StringFragment`.
        if let Some(next) = unsafe { self.next.as_ref() } {
            debug_assert!(self.len > 0);
            self.len + next.total_len()
        } else {
            self.len
        }
    }

    pub(crate) fn raw_ptr(&self) -> *const u16 {
        self.ptr
    }

    pub(crate) fn len(&self) -> u32 {
        self.len
    }

    pub(crate) fn as_slice(&self) -> &[u16] {
        debug_assert_ne!(self.len, 0);
        debug_assert!(!self.ptr.is_null());
        debug_assert!(self.ptr.is_aligned());
        // SAFETY: `self.ptr` is always pointer to an array of `u16`.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) }
    }

    pub(crate) fn next(&self) -> Option<&Self> {
        // SAFETY: `self.next` is null or convertible to a reference.
        debug_assert!(self.next.is_null() || self.next.is_aligned());
        unsafe { self.next.as_ref() }
    }

    pub(crate) fn code_units(&self) -> impl Iterator<Item = u16> {
        CodeUnits::new(self)
    }

    pub(crate) fn as_ptr(&self) -> *const Self {
        self as *const Self
    }
}

// The UTF-16 code units never change.
unsafe impl Send for StringFragment {}
unsafe impl Sync for StringFragment {}

impl PartialEq for StringFragment {
    fn eq(&self, other: &Self) -> bool {
        self.code_units().eq(other.code_units())
    }
}

impl Index<u32> for StringFragment {
    type Output = u16;

    fn index(&self, index: u32) -> &Self::Output {
        assert!(index < self.len);
        // SAFETY: `self.ptr` points to `[u16; self.len]`.
        unsafe { &*self.ptr.add(index as usize) }
    }
}

impl std::fmt::Debug for StringFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            StringFragmentKind::Const => write!(f, r#"const""#)?,
            StringFragmentKind::Stack => write!(f, r#"stack""#)?,
            StringFragmentKind::Heap => write!(f, r#"heap""#)?,
        }
        let utf16 = self.as_slice().iter().cloned();
        for c in std::char::decode_utf16(utf16).map(|r| r.map_err(|e| e.unpaired_surrogate())) {
            match c {
                Ok(c) => write!(f, "{}", c.escape_debug())?,
                Err(code_unit) => write!(f, "\\u{code_unit:04X}")?,
            }
        }
        write!(f, r#"""#)?;
        if let Some(next) = self.next() {
            write!(f, " ")?;
            std::fmt::Debug::fmt(next, f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for StringFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in std::char::decode_utf16(self.code_units())
            .map(|r| r.map_err(|e| e.unpaired_surrogate()))
        {
            match c {
                Ok(c) => write!(f, "{}", c.escape_debug())?,
                Err(code_unit) => write!(f, "\\u{code_unit:04X}")?,
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum StringFragmentKind {
    Const = 0,
    Stack,
    Heap,
}

struct CodeUnits<'a> {
    fragment: &'a StringFragment,
    pos: u32,
}

impl<'a> CodeUnits<'a> {
    fn new(fragment: &'a StringFragment) -> Self {
        Self { fragment, pos: 0 }
    }
}

impl<'a> Iterator for CodeUnits<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.fragment.len {
            let code_unit = self.fragment[self.pos];
            self.pos += 1;
            return Some(code_unit);
        }

        if let Some(next) = self.fragment.next() {
            self.fragment = next;
            self.pos = 0;
            return self.next();
        }

        None
    }
}
