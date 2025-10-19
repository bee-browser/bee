use std::ops::Index;
use std::ptr::NonNull;

use bitflags::bitflags;
use bitflags::bitflags_match;
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

base::static_assert_eq!(align_of::<StringHandle>(), align_of::<usize>());

impl StringHandle {
    /// An empty string.
    pub const EMPTY: Self = Self::new(&StringFragment::EMPTY);

    /// A single U+0020 character.
    pub const SPACE: Self = Self::new(&StringFragment::SPACE);

    /// Creates a new UTF-16 string.
    pub const fn new(frag: &StringFragment) -> Self {
        Self(NonNull::from_ref(frag))
    }

    /// Creates a new constant UTF-16 string.
    pub const fn new_const(frag: &'static StringFragment) -> Self {
        debug_assert!(frag.is_const());
        Self(NonNull::from_ref(frag))
    }

    pub const fn is_simple(&self) -> bool {
        self.fragment().is_simple()
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

    pub fn code_points(&self) -> impl Iterator<Item = CodePointAt> {
        self.fragment().code_points()
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

    pub fn at(&self, index: u32) -> Option<u16> {
        self.fragment().at(index as usize)
    }

    pub fn code_point_at(&self, index: u32) -> CodePointAt {
        let first = self.at(index).unwrap();
        let size = self.len();

        if !is_leading_surrogate(first) && !is_trailing_surrogate(first) {
            return CodePointAt {
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: false,
            };
        }

        if is_trailing_surrogate(first) || index + 1 == size {
            return CodePointAt {
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: true,
            };
        }

        // TODO(perf): inefficient
        let second = self.at(index + 1).unwrap();
        if !is_trailing_surrogate(second) {
            return CodePointAt {
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: true,
            };
        }

        let cp = utf16_surrogate_pair_to_code_point(first, second);
        CodePointAt {
            code_point: cp,
            code_unit_count: 2,
            is_unpaired_surrogate: false,
        }
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

    // 6.1.4.2 StringLastIndexOf ( string, searchValue, fromIndex )
    pub fn last_index_of(&self, search_value: Self, from_index: u32) -> Option<u32> {
        // TODO(perf): slow and inefficient
        let len = self.len();
        let search_len = search_value.len();
        debug_assert!(from_index + search_len <= len);
        let string = self.make_utf16();
        let search = search_value.make_utf16();
        for i in (0..from_index).rev() {
            let canditate = &string[(i as usize)..((i + search_len) as usize)];
            if canditate == search {
                return Some(i);
            }
        }
        None
    }

    // 7.2.7 Static Semantics: IsStringWellFormedUnicode ( string )
    pub fn is_well_formed(&self) -> bool {
        for code_unit_at in self.code_points() {
            if code_unit_at.is_unpaired_surrogate {
                return false;
            }
        }
        true
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

    /// A pointer to the UTF-16 code unit sequence if it exists.
    ///
    /// The `ptr` points to one of the following memory block:
    ///
    ///   * A constant array of UTF-16 code units
    ///   * An array of UTF-16 code units allocated in the string pool (not yet implemented)
    ///   * A memory block allocated in the GC heap
    ptr: *const u16,

    /// The number of the UTF-16 code units in the string fragment.
    len: u32,

    flags: StringFragmentFlags,

    /// The number of repetitions of the UTF-16 code unit sequence.
    repetitions: u8,
}

base::static_assert_eq!(align_of::<StringFragment>(), align_of::<usize>());

impl StringFragment {
    pub(crate) const EMPTY: Self = Self::new_const(&[]);
    pub(crate) const SPACE: Self = Self::new_const(&[0x0020]);

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const NEXT_OFFSET: usize = std::mem::offset_of!(Self, next);
    pub(crate) const PTR_OFFSET: usize = std::mem::offset_of!(Self, ptr);
    pub(crate) const LEN_OFFSET: usize = std::mem::offset_of!(Self, len);
    pub(crate) const FLAGS_OFFSET: usize = std::mem::offset_of!(Self, flags);
    pub(crate) const REPETITIONS_OFFSET: usize = std::mem::offset_of!(Self, repetitions);

    // TODO(refactor): should be private
    pub const fn new_const(slice: &'static [u16]) -> Self {
        Self {
            next: std::ptr::null(),
            ptr: slice.as_ptr(),
            len: slice.len() as u32,
            flags: StringFragmentFlags::CONST,
            repetitions: 1,
        }
    }

    // TODO(feat): support DYNAMIC
    pub(crate) const fn new_stack(slice: &[u16], dynamic: bool) -> Self {
        Self {
            next: std::ptr::null(),
            ptr: slice.as_ptr(),
            len: slice.len() as u32,
            flags: if dynamic {
                StringFragmentFlags::STACK.union(StringFragmentFlags::DYNAMIC)
            } else {
                StringFragmentFlags::STACK
            },
            repetitions: 1,
        }
    }

    pub(crate) fn new_heap(next: *const Self, frag: &StringFragment) -> Self {
        Self {
            next,
            ptr: frag.ptr,
            len: frag.len,
            flags: StringFragmentFlags::HEAP
                | frag.flags.intersection(StringFragmentFlags::DYNAMIC),
            repetitions: frag.repetitions,
        }
    }

    pub(crate) const fn is_simple(&self) -> bool {
        self.next.is_null() && self.repetitions == 1
    }

    pub(crate) const fn is_empty(&self) -> bool {
        debug_assert!(self.len > 0 || self.next.is_null());
        self.len == 0
    }

    pub(crate) const fn is_const(&self) -> bool {
        self.flags.contains(StringFragmentFlags::CONST)
    }

    pub(crate) const fn on_stack(&self) -> bool {
        self.flags.contains(StringFragmentFlags::STACK)
    }

    pub(crate) fn total_len(&self) -> u32 {
        // SAFETY: `self.next` is null or a valid pointer to a `StringFragment`.
        if let Some(next) = unsafe { self.next.as_ref() } {
            debug_assert!(self.len > 0);
            self.len() + next.total_len()
        } else {
            self.len()
        }
    }

    pub(crate) fn len(&self) -> u32 {
        debug_assert!(self.repetitions > 0);
        self.len * self.repetitions as u32
    }

    // Returns a *raw* UTF-16 code unit sequence.
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

    pub(crate) fn code_points(&self) -> impl Iterator<Item = CodePointAt> {
        CodePoints::new(self)
    }

    pub(crate) fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    fn at(&self, index: usize) -> Option<u16> {
        let slice = self.as_slice();
        match slice.get(index) {
            Some(code_unit) => Some(*code_unit),
            None => self.next().and_then(|next| next.at(index - slice.len())),
        }
    }

    pub(crate) fn repeat(&self, repetitions: u8) -> Self {
        debug_assert!(self.is_simple());
        debug_assert!(repetitions > 0);
        Self {
            next: std::ptr::null_mut(),
            ptr: self.ptr,
            len: self.len,
            flags: self.flags,
            repetitions,
        }
    }

    pub(crate) fn sub_fragment(&self, start: u32, end: u32) -> Self {
        debug_assert!(self.is_simple());
        let slice = self.as_slice();
        let sub = &slice[(start as usize)..(end as usize)];
        Self {
            next: std::ptr::null_mut(),
            ptr: sub.as_ptr(),
            len: sub.len() as u32,
            flags: self.flags,
            repetitions: 1,
        }
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
        assert!(index < self.len());
        let index = index % self.len;
        // SAFETY: `self.ptr` points to `[u16; self.len]`.
        unsafe { &*self.ptr.add(index as usize) }
    }
}

impl std::fmt::Debug for StringFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = bitflags_match!(self.flags, {
            StringFragmentFlags::CONST => r#"const""#,
            StringFragmentFlags::STACK => r#"stack""#,
            StringFragmentFlags::STACK | StringFragmentFlags::DYNAMIC => r#"stack!""#,
            StringFragmentFlags::HEAP => r#"heap""#,
            StringFragmentFlags::HEAP | StringFragmentFlags::DYNAMIC => r#"heap!""#,
            _ => unreachable!(),
        });
        write!(f, "{prefix}")?;
        let utf16 = self.as_slice().iter().cloned();
        for c in std::char::decode_utf16(utf16).map(|r| r.map_err(|e| e.unpaired_surrogate())) {
            match c {
                Ok(c) => write!(f, "{}", c.escape_debug())?,
                Err(code_unit) => write!(f, "\\u{code_unit:04X}")?,
            }
        }
        write!(f, r#"""#)?;
        if self.repetitions > 1 {
            write!(f, "*{}", self.repetitions)?;
        }
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

bitflags! {
    #[derive(Clone, Copy, PartialEq)]
    pub struct StringFragmentFlags: u8 {
        /// The object is a constant value.
        const CONST   = 1 << 0;

        /// The object has been allocated on the stack.
        const STACK   = 1 << 1;

        /// The object has been allocated in the heap.
        const HEAP    = 1 << 2;

        /// The UTF-16 code units has allocated in the heap at runtime.
        const DYNAMIC = 1 << 3;
    }
}

struct CodeUnits<'a> {
    fragment: &'a StringFragment,
    pos: u32,
    repetitions: u8,
}

impl<'a> CodeUnits<'a> {
    fn new(fragment: &'a StringFragment) -> Self {
        Self {
            fragment,
            pos: 0,
            repetitions: 0,
        }
    }

    fn has_next(&self) -> bool {
        // We can solve the following warning by changing like this:
        //
        // ```
        // if self.pos < self.fragment.len || self.repetitions < self.fragment.repetitions {
        //     true
        // } else if let Some(next) = self.fragment.next() {
        //     ...
        // ```
        //
        // But we keep the code for readability.
        #[allow(clippy::if_same_then_else)]
        if self.pos < self.fragment.len {
            true
        } else if self.repetitions < self.fragment.repetitions {
            true
        } else if let Some(next) = self.fragment.next() {
            !next.is_empty()
        } else {
            false
        }
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

        self.repetitions += 1;
        self.pos = 0;

        if self.repetitions < self.fragment.repetitions {
            return self.next();
        }

        if let Some(next) = self.fragment.next() {
            self.fragment = next;
            return self.next();
        }

        None
    }
}

struct CodePoints<'a> {
    code_units: CodeUnits<'a>,
}

impl<'a> CodePoints<'a> {
    fn new(fragment: &'a StringFragment) -> Self {
        Self {
            code_units: CodeUnits::new(fragment),
        }
    }
}

impl<'a> Iterator for CodePoints<'a> {
    type Item = CodePointAt;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.code_units.next()?;

        if !is_leading_surrogate(first) && !is_trailing_surrogate(first) {
            return Some(CodePointAt {
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: false,
            });
        }

        if is_trailing_surrogate(first) || !self.code_units.has_next() {
            return Some(CodePointAt {
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: true,
            });
        }

        let second = self.code_units.next().unwrap();

        if !is_trailing_surrogate(second) {
            return Some(CodePointAt {
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: true,
            });
        }

        Some(CodePointAt {
            code_point: utf16_surrogate_pair_to_code_point(first, second),
            code_unit_count: 2,
            is_unpaired_surrogate: false,
        })
    }
}

pub struct CodePointAt {
    pub code_point: u32,
    pub code_unit_count: u32,
    pub is_unpaired_surrogate: bool,
}

fn is_leading_surrogate(code_unit: u16) -> bool {
    (0xD800..=0xDBFF).contains(&code_unit)
}

fn is_trailing_surrogate(code_unit: u16) -> bool {
    (0xDC00..=0xDFFF).contains(&code_unit)
}

// 11.1.3 Static Semantics: UTF16SurrogatePairToCodePoint ( lead, trail )
fn utf16_surrogate_pair_to_code_point(lead: u16, trail: u16) -> u32 {
    (lead as u32 - 0xD800) * 0x400 + (trail as u32 - 0xDC00) + 0x10000
}
