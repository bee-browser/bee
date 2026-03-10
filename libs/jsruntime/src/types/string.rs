use std::alloc::Layout;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::ops::Index;

use bitflags::bitflags;
use bitflags::bitflags_match;
use itertools::Itertools;

use jsgc::Handle;
use jsgc::Heap;
use jsgc::Seq;
use jsgc::Unknown;
use jsgc::UnknownVtable;
use jsgc::VisitList;

/// An empty string.
pub const EMPTY: Handle<StringFragment> = Handle::from_ref(&StringFragment::EMPTY);

/// A single U+0020 character.
pub const SPACE: Handle<StringFragment> = Handle::from_ref(&StringFragment::SPACE);

/// A data type representing an **immutable** fragment of UTF-16 code units.
///
/// A UTF-16 string is represented as a *chain* of **immutable** fragments of UTF-16 code units.
///
/// This type may be allocated on the stack.
#[derive(Clone)]
#[repr(C)]
pub struct StringFragment {
    /// A pointer to the UTF-16 code unit sequence if it exists.
    ///
    /// The `ptr` points to one of the following memory block:
    ///
    ///   * A constant array of UTF-16 code units
    ///   * An array of UTF-16 code units allocated in the string pool (not yet implemented)
    ///   * A memory block allocated in the GC heap
    // TODO(issue#237): GcCellRef
    ptr: Handle<u16>,

    offset: u32,

    /// The number of the UTF-16 code units in the string fragment.
    len: u32,

    flags: StringFragmentFlags,
}

base::static_assert_eq!(align_of::<StringFragment>(), align_of::<usize>());

impl StringFragment {
    pub(crate) const EMPTY: Self = Self::new_const(&[]);
    pub(crate) const SPACE: Self = Self::new_const(&[0x0020]);

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const PTR_OFFSET: usize = std::mem::offset_of!(Self, ptr);
    pub(crate) const OFFSET_OFFSET: usize = std::mem::offset_of!(Self, offset);
    pub(crate) const LEN_OFFSET: usize = std::mem::offset_of!(Self, len);
    pub(crate) const FLAGS_OFFSET: usize = std::mem::offset_of!(Self, flags);

    // TODO(refactor): should be private
    pub const fn new_const(slice: &'static [u16]) -> Self {
        Self {
            ptr: Handle::from_ptr(slice.as_ptr()).unwrap(),
            offset: 0,
            len: slice.len() as u32,
            flags: StringFragmentFlags::CONST,
        }
    }

    pub(crate) const fn new_stack(seq: Seq<u16>) -> Self {
        Self {
            ptr: seq.data,
            offset: 0,
            len: seq.len as u32,
            flags: StringFragmentFlags::STACK,
        }
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) const fn is_const(&self) -> bool {
        self.flags.contains(StringFragmentFlags::CONST)
    }

    pub(crate) const fn on_stack(&self) -> bool {
        self.flags.contains(StringFragmentFlags::STACK)
    }

    /// Returns the number of UTF-16 code units in the string.
    pub(crate) fn len(&self) -> u32 {
        self.len
    }

    // Returns a *raw* UTF-16 code unit sequence.
    pub(crate) fn as_slice(&self) -> &[u16] {
        debug_assert_ne!(self.len, 0);
        // SAFETY: `self.ptr.as_ptr()` is always pointer to an array of `u16`.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len as usize) }
    }

    pub fn at(&self, index: u32) -> Option<u16> {
        self.as_slice().get(index as usize).copied()
    }

    pub fn code_point_at(&self, index: u32) -> CodePointAt {
        let first = self.at(index).unwrap();
        let size = self.len();

        if !is_leading_surrogate(first) && !is_trailing_surrogate(first) {
            return CodePointAt {
                index,
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: false,
            };
        }

        if is_trailing_surrogate(first) || index + 1 == size {
            return CodePointAt {
                index,
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: true,
            };
        }

        // TODO(perf): inefficient
        let second = self.at(index + 1).unwrap();
        if !is_trailing_surrogate(second) {
            return CodePointAt {
                index,
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: true,
            };
        }

        let cp = utf16_surrogate_pair_to_code_point(first, second);
        CodePointAt {
            index,
            code_point: cp,
            code_unit_count: 2,
            is_unpaired_surrogate: false,
        }
    }

    pub fn position<P>(&self, predicate: P) -> Option<u32>
    where
        P: Fn(u32) -> bool,
    {
        for code_point_at in self.code_points() {
            if predicate(code_point_at.code_point) {
                return Some(code_point_at.index);
            }
        }
        None
    }

    // TODO(perf): inefficient
    pub fn last_position<P>(&self, predicate: P) -> Option<u32>
    where
        P: Fn(u32) -> bool,
    {
        let mut candidate = None;
        for code_point_at in self.code_points() {
            if predicate(code_point_at.code_point) {
                candidate = Some(code_point_at.index);
            }
        }
        candidate
    }

    // 6.1.4.1 StringIndexOf ( string, searchValue, fromIndex )
    pub fn index_of(&self, search_value: Handle<Self>, from_index: u32) -> Option<u32> {
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
    pub fn last_index_of(&self, search_value: Handle<Self>, from_index: u32) -> Option<u32> {
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

    pub fn concat(&self, tail: Handle<Self>, heap: &mut Heap) -> Handle<Self> {
        if self.is_empty() {
            return tail.ensure_return_safe(heap);
        }
        if tail.is_empty() {
            return self.ensure_return_safe(heap);
        }

        let len = (self.len() + tail.len()) as usize;
        let layout = Layout::array::<u16>(len).unwrap();

        let handle: Handle<u16> = heap.alloc_layout(layout, |ptr| unsafe {
            let code_units = ptr.cast::<u16>();
            dbg!(self.offset);
            dbg!(self.len);
            dbg!(tail.offset);
            dbg!(tail.len);
            code_units.as_ptr().copy_from(
                self.ptr.as_ptr().add(self.offset as usize),
                self.len as usize,
            );
            code_units.offset(self.len as isize).as_ptr().copy_from(
                tail.ptr.as_ptr().add(tail.offset as usize),
                tail.len as usize,
            );
        });

        heap.alloc(StringFragment {
            ptr: handle,
            offset: 0,
            len: len as u32,
            flags: StringFragmentFlags::HEAP,
        })
    }

    pub fn ensure_return_safe(&self, heap: &mut Heap) -> Handle<Self> {
        if !self.on_stack() {
            return Handle::from_ref(self);
        }

        if self.is_empty() {
            return EMPTY;
        }

        self.migrate_to_heap(heap)
    }

    // Migrate a UTF-16 string from the stack to the heap.
    fn migrate_to_heap(&self, heap: &mut Heap) -> Handle<Self> {
        heap.alloc(self.clone())
    }

    pub(crate) fn code_units(&self) -> impl Iterator<Item = u16> {
        CodeUnits::new(self)
    }

    pub(crate) fn code_points(&self) -> impl Iterator<Item = CodePointAt> {
        CodePoints::new(self)
    }

    /// Creates a `Vec` containing UTF-16 code units of the string.
    pub(crate) fn make_utf16(&self) -> Vec<u16> {
        self.code_units().collect_vec()
    }

    pub(crate) fn repeat(&self, repetitions: u32, heap: &mut Heap) -> Handle<Self> {
        debug_assert!(repetitions > 0);
        let mut head = Handle::from_ref(self);
        for _ in 0..repetitions {
            head = head.concat(head, heap);
        }
        head
    }

    pub(crate) fn sub_fragment(&self, start: u32, end: u32) -> Self {
        Self {
            ptr: self.ptr,
            offset: start,
            len: end - start,
            flags: StringFragmentFlags::STACK,
        }
    }

    fn trace(&self, visit_list: &mut VisitList) {
        visit_list.push(self.ptr.as_addr());
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
        self.as_slice().get(index as usize).unwrap()
    }
}

impl std::fmt::Debug for StringFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = bitflags_match!(self.flags, {
            StringFragmentFlags::CONST => r#"const""#,
            StringFragmentFlags::STACK => r#"stack""#,
            StringFragmentFlags::HEAP => r#"heap""#,
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

impl Unknown for StringFragment {
    fn vtable() -> &'static UnknownVtable {
        fn trace(addr: usize, visit_list: &mut VisitList) {
            Handle::<StringFragment>::from_addr(addr)
                .unwrap()
                .trace(visit_list);
        }

        static VTABLE: UnknownVtable = UnknownVtable {
            tidy: None,
            trace: Some(trace),
        };

        &VTABLE
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
    }
}

struct CodeUnits<'a> {
    fragment: &'a StringFragment,
    pos: u32,
}

impl<'a> CodeUnits<'a> {
    fn new(fragment: &'a StringFragment) -> Self {
        Self { fragment, pos: 0 }
    }

    #[allow(unused)]
    fn has_next(&self) -> bool {
        self.pos < self.fragment.len
    }
}

impl<'a> Iterator for CodeUnits<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let fragment = self.fragment;

        if self.pos < fragment.len {
            let code_unit = fragment[self.pos];
            self.pos += 1;
            return Some(code_unit);
        }

        None
    }
}

struct CodePoints<'a> {
    code_units: Peekable<Enumerate<CodeUnits<'a>>>,
}

impl<'a> CodePoints<'a> {
    fn new(fragment: &'a StringFragment) -> Self {
        Self {
            code_units: CodeUnits::new(fragment).enumerate().peekable(),
        }
    }
}

impl<'a> Iterator for CodePoints<'a> {
    type Item = CodePointAt;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, first) = self.code_units.next()?;

        if !is_leading_surrogate(first) && !is_trailing_surrogate(first) {
            return Some(CodePointAt {
                index: index as u32,
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: false,
            });
        }

        if is_trailing_surrogate(first) {
            return Some(CodePointAt {
                index: index as u32,
                code_point: first as u32,
                code_unit_count: 1,
                is_unpaired_surrogate: true,
            });
        }

        let second = match self
            .code_units
            .next_if(|(_, second)| is_trailing_surrogate(*second))
        {
            Some((_, second)) => second,
            None => {
                return Some(CodePointAt {
                    index: index as u32,
                    code_point: first as u32,
                    code_unit_count: 1,
                    is_unpaired_surrogate: true,
                });
            }
        };

        Some(CodePointAt {
            index: index as u32,
            code_point: utf16_surrogate_pair_to_code_point(first, second),
            code_unit_count: 2,
            is_unpaired_surrogate: false,
        })
    }
}

pub struct CodePointAt {
    pub index: u32,
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
