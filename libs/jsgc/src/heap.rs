use std::alloc::Layout;
use std::collections::VecDeque;
use std::ptr::NonNull;

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

use crate::handle::Handle;
use crate::handle::HandleMut;

/// A heap memory managed by GC.
pub struct Heap {
    // TODO(perf): inefficient
    holders: FxHashMap<usize, MemoryHolder>,
}

impl Heap {
    /// Creates a heap.
    pub fn new() -> Self {
        Self {
            holders: Default::default(),
        }
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc<T>(&mut self, object: T) -> Handle<T>
    where
        T: Sized + Unknown,
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            assert!(!ptr.is_null());
            std::ptr::write(ptr, object);
            ptr
        };

        self.holders.insert(
            ptr as usize,
            MemoryHolder {
                vtable: T::vtable(),
                layout: Layout::new::<T>(),
                addr: ptr as usize,
            },
        );

        Handle::from_ptr(ptr).unwrap()
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_mut<T>(&mut self, object: T) -> HandleMut<T>
    where
        T: Sized + Unknown,
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = std::alloc::alloc(Layout::new::<T>()) as *mut T;
            assert!(!ptr.is_null());
            std::ptr::write(ptr, object);
            ptr
        };

        self.holders.insert(
            ptr as usize,
            MemoryHolder {
                vtable: T::vtable(),
                layout: Layout::new::<T>(),
                addr: ptr as usize,
            },
        );

        HandleMut::from_ptr(ptr).unwrap()
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_layout<T, F>(&mut self, layout: Layout, init: F) -> Handle<T>
    where
        T: Sized + Unknown,
        F: FnOnce(NonNull<u8>),
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            NonNull::new(std::alloc::alloc(layout)).unwrap()
        };
        init(ptr);
        self.holders.insert(
            ptr.addr().get(),
            MemoryHolder {
                vtable: T::vtable(),
                layout,
                addr: ptr.addr().get(),
            },
        );
        Handle::from_ref(unsafe { ptr.cast::<T>().as_ref() })
    }

    /// Populates a specified object on memory allocated from the heap.
    pub fn alloc_layout_mut<T, F>(&mut self, layout: Layout, init: F) -> HandleMut<T>
    where
        T: Sized + Unknown,
        F: FnOnce(NonNull<u8>),
    {
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            NonNull::new(std::alloc::alloc(layout)).unwrap()
        };
        init(ptr);
        self.holders.insert(
            ptr.addr().get(),
            MemoryHolder {
                vtable: T::vtable(),
                layout,
                addr: ptr.addr().get(),
            },
        );
        HandleMut::from_ref(unsafe { ptr.cast::<T>().as_mut() })
    }

    // TODO: return HandleMut
    // TODO: there is no way to restrict the type of `T` to an integer type.
    pub fn alloc_slice_copy<T>(&mut self, src: &[T]) -> &mut [T]
    where
        T: Copy,
    {
        static VTABLE: UnknownVtable = UnknownVtable {
            tidy: None,
            trace: None,
        };
        let len = src.len();
        let layout = Layout::array::<T>(len).unwrap();
        let ptr = unsafe {
            // TODO(perf): use a dedicated memory pool
            let ptr = NonNull::new(std::alloc::alloc(layout)).unwrap().cast::<T>();
            ptr.as_ptr().copy_from(src.as_ptr(), len);
            ptr
        };
        let addr = ptr.as_ptr() as usize;
        self.holders.insert(
            addr,
            MemoryHolder {
                vtable: &VTABLE,
                layout,
                addr,
            },
        );
        unsafe { std::slice::from_raw_parts_mut(ptr.as_ptr(), len) }
    }

    /// Reclaims objects that are not reachable from a specified root objects.
    pub fn collect_garbage(&mut self, roots: &[usize]) {
        let mut state = GcState::new(roots);
        self.mark(&mut state);
        self.sweep(&mut state);
    }

    /// Performs the mark phase.
    fn mark(&mut self, state: &mut GcState) {
        while let Some(addr) = state.visit_list.pop() {
            if state.visited.contains(&addr) {
                continue;
            }
            let holder = self.holders.get(&addr).unwrap();
            if let Some(trace) = holder.vtable.trace {
                trace(addr, &mut state.visit_list);
            }
            state.visited.insert(addr);
        }
    }

    /// Performs the sweep phase.
    fn sweep(&mut self, state: &mut GcState) {
        self.holders.retain(|addr, _| state.visited.contains(addr));
    }

    /// Returns statistics.
    pub fn stats(&self) -> Stats {
        Stats {
            num_objects: self.holders.len(),
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics of a heap at some point.
pub struct Stats {
    pub num_objects: usize,
}

/// A data type holds GC states.
struct GcState {
    visit_list: VisitList,
    visited: FxHashSet<usize>,
}

impl GcState {
    fn new(roots: &[usize]) -> Self {
        let mut visit_list: VisitList = Default::default();
        visit_list.extend(roots.iter().cloned());
        Self {
            visit_list,
            visited: Default::default(),
        }
    }
}

/// A list to which reachable objects will be added.
#[derive(Default)]
pub struct VisitList(VecDeque<usize>);

impl VisitList {
    /// Appends a handle to the back of the visit list.
    pub fn push(&mut self, addr: usize) {
        self.0.push_back(addr);
    }

    /// Appends handles of an iterator.
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = usize>,
    {
        self.0.extend(iter);
    }

    /// Removes the first handle and returns it, or `None` if the visit list is empty.
    fn pop(&mut self) -> Option<usize> {
        self.0.pop_front()
    }
}

struct MemoryHolder {
    vtable: &'static UnknownVtable,
    layout: Layout,
    addr: usize,
}

impl Drop for MemoryHolder {
    fn drop(&mut self) {
        if let Some(tidy) = self.vtable.tidy {
            tidy(self.addr);
        }
        unsafe {
            std::alloc::dealloc(self.addr as *mut u8, self.layout);
        }
    }
}

pub struct UnknownVtable {
    pub tidy: Option<TidyFn>,
    pub trace: Option<TraceFn>,
}

type TidyFn = fn(usize);
type TraceFn = fn(usize, &mut VisitList);

pub trait Unknown {
    fn vtable() -> &'static UnknownVtable;
}

impl Unknown for u16 {
    fn vtable() -> &'static UnknownVtable {
        &UnknownVtable {
            tidy: None,
            trace: None,
        }
    }
}
